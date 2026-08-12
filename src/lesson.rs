//! A lesson as data: the builder an author writes against, and the model it
//! produces — plain strings, vecs and floats, so a lesson can be typeset,
//! shown on screen, saved and reloaded without recompiling anything.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::expr::Expr;
use crate::glossary::{self, Glossary};
use crate::plot::Plot;

/// A lesson: a title, the blocks that make up its page, and the glossary its
/// author taught it. Builds via [`Lesson::builder`]; everything else is
/// read-only, so a `Lesson` in hand is always the finished, audited article.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lesson {
    title: String,
    blocks: Vec<Block>,
    glossary: Glossary,
}

impl Lesson {
    /// Start authoring a lesson titled `title`.
    pub fn builder(title: impl Into<String>) -> LessonBuilder {
        LessonBuilder {
            title: title.into(),
            blocks: Vec::new(),
            glossary: Glossary::default(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub(crate) fn glossary(&self) -> &Glossary {
        &self.glossary
    }

    /// Open the lesson in a window and read it, until the reader closes it.
    ///
    /// Takes the lesson because the window owns it for as long as it is up;
    /// this is the last call in a lesson's `main`. Export first if a run needs
    /// both — [`Self::export_pdf`] only borrows.
    ///
    /// # Errors
    /// [`Error::Viewer`] if the window, GPU context or fonts fail to start.
    pub fn show(self) -> Result<()> {
        crate::viewer::run(self)
    }

    /// Typeset the whole lesson and write it to `path` as a PDF.
    ///
    /// # Errors
    /// - [`Error::MathCompile`] if a formula's LaTeX does not convert to Typst
    /// - [`Error::Pdf`] if Typst fails to typeset the document
    /// - [`Error::Io`] if `path` cannot be written
    pub fn export_pdf(&self, path: impl AsRef<Path>) -> Result<()> {
        crate::pdf::export(self, path.as_ref())
    }

    /// Compile every formula the lesson sets, check every term it produces
    /// against the glossary, and parse every tunable plot curve, reporting
    /// anything a reader would notice: a formula that does not compile, a
    /// term nothing curated explains, a tooltip that would print a character
    /// egui cannot draw, a curve that would not be drawn, or a slider that
    /// would do nothing.
    ///
    /// Infallible by design — a lesson always builds — so this is how
    /// problems surface. Agent-authored lessons should assert
    /// `audit().is_empty()`.
    pub fn audit(&self) -> Vec<AuditFinding> {
        let mut findings = Vec::new();

        for (latex, display) in self.formulas() {
            let rendered = match crate::formula::compile(latex, display) {
                Ok(rendered) => rendered,
                Err(message) => {
                    findings.push(AuditFinding::MathError {
                        latex: latex.to_owned(),
                        message,
                    });
                    continue;
                }
            };

            for term in &rendered.terms {
                let description = term.describe(&self.glossary);

                if description.name == glossary::UNCURATED_TERM_NAME
                    || description.meaning == glossary::UNKNOWN_CHAR_MEANING
                {
                    findings.push(AuditFinding::UnexplainedTerm {
                        latex: latex.to_owned(),
                        key: term.key.clone(),
                    });
                }

                for ch in description.display.chars() {
                    if crate::symbols::is_unrenderable(ch) {
                        findings.push(AuditFinding::UnrenderableTooltip {
                            latex: latex.to_owned(),
                            key: term.key.clone(),
                            character: ch,
                        });
                    }
                }
            }
        }

        for plot in self.plots() {
            findings.extend(
                plot.stray_y2_label()
                    .map(|label| AuditFinding::UnusedSecondaryLabel { label: label.to_owned() }),
            );

            let mut read = Vec::new();
            let mut every_curve_parsed = true;

            for expression in plot.expressions() {
                let parsed = match Expr::parse(expression) {
                    Ok(parsed) => parsed,
                    Err(error) => {
                        findings.push(AuditFinding::ExpressionError {
                            expression: expression.to_owned(),
                            message: error.to_string(),
                        });
                        every_curve_parsed = false;
                        continue;
                    }
                };
                for name in parsed.variables() {
                    if plot.declares(name) {
                        read.push(name.to_owned());
                    } else {
                        findings.push(AuditFinding::UndeclaredParameter {
                            expression: expression.to_owned(),
                            name: name.to_owned(),
                        });
                    }
                }
            }

            // A curve that did not parse reads nothing this can see, so its
            // parameters are only unused in the sense that the curve is
            // already reported broken.
            if every_curve_parsed {
                findings.extend(
                    plot.unread_parameters(&read)
                        .map(|name| AuditFinding::UnusedParameter { name: name.to_owned() }),
                );
            }
        }

        findings
    }

    fn plots(&self) -> impl Iterator<Item = &Plot> {
        self.blocks.iter().filter_map(|block| match block {
            Block::Plot(plot) => Some(plot),
            _ => None,
        })
    }

    /// Every formula the lesson sets, with whether it is set on its own line.
    fn formulas(&self) -> Vec<(&str, bool)> {
        let mut out = Vec::new();
        for block in &self.blocks {
            match block {
                Block::Para(inlines) => out.extend(inlines.iter().filter_map(|inline| match inline {
                    Inline::Math(latex) => Some((latex.as_str(), false)),
                    Inline::Text(_) => None,
                })),
                Block::Display(latex) => out.push((latex.as_str(), true)),
                Block::Heading(_) | Block::Note(_) | Block::Rule | Block::Figure(_) | Block::Plot(_) => {}
            }
        }
        out
    }

    /// Save the lesson as JSON.
    ///
    /// # Errors
    /// - [`Error::Serialization`] if the lesson cannot be encoded
    /// - [`Error::Io`] if `path` cannot be written
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let json = serde_json::to_string_pretty(self)
            .map_err(|source| Error::Serialization { path: path.to_owned(), source })?;
        std::fs::write(path, json).map_err(|source| Error::Io { path: path.to_owned(), source })
    }

    /// Load a lesson previously written by [`Lesson::save`].
    ///
    /// # Errors
    /// - [`Error::Io`] if `path` cannot be read
    /// - [`Error::Serialization`] if its contents are not a valid lesson
    pub fn load(path: impl AsRef<Path>) -> Result<Lesson> {
        let path = path.as_ref();
        let json = std::fs::read_to_string(path).map_err(|source| Error::Io { path: path.to_owned(), source })?;
        serde_json::from_str(&json).map_err(|source| Error::Serialization { path: path.to_owned(), source })
    }
}

/// One element of a lesson's page. The viewer and the PDF exporter both walk
/// a lesson's blocks, so a lesson can only be written once and the two
/// renderings cannot drift apart.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Block {
    Heading(String),
    /// A muted hint line, sitting tight under its heading like a subtitle.
    Note(String),
    Para(Vec<Inline>),
    /// Maths set on its own line.
    Display(String),
    /// A visual break between sections.
    Rule,
    Figure(Figure),
    Plot(Plot),
}

/// One run of a paragraph: plain prose, or inline maths.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Inline {
    Text(String),
    /// Inline LaTeX, set in the flow of the paragraph.
    Math(String),
}

/// Builds a [`Lesson`] one block at a time. Every method takes and returns
/// `Self`, so a lesson reads as a chain: `Lesson::builder("...").heading(...)
/// .para(...).build()`.
pub struct LessonBuilder {
    title: String,
    blocks: Vec<Block>,
    glossary: Glossary,
}

impl LessonBuilder {
    pub fn heading(mut self, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Heading(text.into()));
        self
    }

    pub fn note(mut self, text: impl Into<String>) -> Self {
        self.blocks.push(Block::Note(text.into()));
        self
    }

    /// Add a paragraph, built up from text and inline maths in `f`.
    ///
    /// ```
    /// # use math_me::Lesson;
    /// let lesson = Lesson::builder("Title")
    ///     .para(|p| p.text("The return is ").math(r"r_t = \mu + \varepsilon_t").text("."))
    ///     .build();
    /// ```
    pub fn para(mut self, f: impl FnOnce(ParaBuilder) -> ParaBuilder) -> Self {
        let built = f(ParaBuilder::default());
        self.blocks.push(Block::Para(built.inlines));
        self
    }

    /// Set a formula on its own centred line, larger than the inline maths
    /// [`ParaBuilder::math`] sets inside a paragraph.
    pub fn display(mut self, latex: impl Into<String>) -> Self {
        self.blocks.push(Block::Display(latex.into()));
        self
    }

    pub fn rule(mut self) -> Self {
        self.blocks.push(Block::Rule);
        self
    }

    pub fn figure(mut self, figure: Figure) -> Self {
        self.blocks.push(Block::Figure(figure));
        self
    }

    pub fn plot(mut self, plot: Plot) -> Self {
        self.blocks.push(Block::Plot(plot));
        self
    }

    /// Teach the glossary what a term means, so hovering it in any formula on
    /// the page explains it.
    ///
    /// `latex` is the fragment as written in a [`ParaBuilder::math`] or
    /// [`Self::display`] call elsewhere in the lesson — the same LaTeX, term
    /// for term, so the two agree on what they are talking about. Spelling is
    /// all that has to match, not notation: the fragment is keyed by the same
    /// structural rules a compiled formula's terms are, so `\left(...\right)`
    /// and plain parentheses around the same fragment resolve to one entry.
    ///
    /// If `latex` fails to convert, the raw text is used as the key instead
    /// of panicking; the entry then matches nothing, and [`Lesson::audit`]
    /// reports the formula's real [`AuditFinding::MathError`] instead.
    pub fn explain(mut self, latex: impl Into<String>, name: impl Into<String>, meaning: impl Into<String>) -> Self {
        let latex = latex.into();
        let key = crate::formula::to_typst_math(&latex)
            .map(|typst| crate::terms::explain_key(&typst))
            .unwrap_or(latex);
        self.glossary.insert_term(key, name.into(), meaning.into());
        self
    }

    /// Teach the glossary what one character means, for terms of a single
    /// glyph the lesson never explains as a whole term via [`Self::explain`].
    pub fn explain_char(mut self, ch: char, name: impl Into<String>, meaning: impl Into<String>) -> Self {
        self.glossary.insert_char(ch, name.into(), meaning.into());
        self
    }

    /// Finish the lesson. Infallible — a malformed formula or an unexplained
    /// term still builds; call [`Lesson::audit`] to find them.
    pub fn build(self) -> Lesson {
        Lesson {
            title: self.title,
            blocks: self.blocks,
            glossary: self.glossary,
        }
    }
}

/// Builds one paragraph's run of text and inline maths.
#[derive(Default)]
pub struct ParaBuilder {
    inlines: Vec<Inline>,
}

impl ParaBuilder {
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.inlines.push(Inline::Text(text.into()));
        self
    }

    pub fn math(mut self, latex: impl Into<String>) -> Self {
        self.inlines.push(Inline::Math(latex.into()));
        self
    }
}

/// An image, with an optional print-specific variant for the PDF.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Figure {
    pub(crate) svg: String,
    pub(crate) caption: String,
    pub(crate) print_svg: Option<String>,
    pub(crate) width_percent: Option<u8>,
}

impl Figure {
    pub fn new(svg: impl Into<String>, caption: impl Into<String>) -> Self {
        Self {
            svg: svg.into(),
            caption: caption.into(),
            print_svg: None,
            width_percent: None,
        }
    }

    /// An SVG to use in the PDF instead of `svg` — for artwork tuned to a
    /// screen palette that needs different ink for print.
    pub fn print_svg(mut self, svg: impl Into<String>) -> Self {
        self.print_svg = Some(svg.into());
        self
    }

    /// How wide the figure sits in the PDF's column, as a percentage. The
    /// on-screen viewer fits the figure to its own column regardless.
    pub fn width_percent(mut self, percent: u8) -> Self {
        self.width_percent = Some(percent);
        self
    }
}

/// Something [`Lesson::audit`] noticed that a reader would too.
#[derive(Clone, Debug, PartialEq)]
pub enum AuditFinding {
    /// A formula's LaTeX did not compile.
    MathError { latex: String, message: String },
    /// A term has no curated entry — neither the lesson's `explain()` nor the
    /// built-in dictionary had anything to say about it.
    UnexplainedTerm { latex: String, key: String },
    /// A term's tooltip would print a character egui cannot draw.
    UnrenderableTooltip { latex: String, key: String, character: char },
    /// A tunable plot curve's expression did not parse, so the curve would be
    /// missing from the plot.
    ExpressionError { expression: String, message: String },
    /// A tunable plot curve reads a name that is neither the free variable
    /// `x` nor one of its plot's declared parameters, so it has no value to
    /// draw with.
    UndeclaredParameter { expression: String, name: String },
    /// A plot declares a parameter no curve on it reads, so the reader gets a
    /// slider that moves and changes nothing. Naming a parameter `x` earns
    /// this too: the free variable wins that lookup, whatever the slider says.
    UnusedParameter { name: String },
    /// A plot labels a right-hand axis it has put no series on, so neither
    /// the axis nor the label is drawn. [`crate::Plot::secondary`] is what
    /// moves a series onto it.
    UnusedSecondaryLabel { label: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_produces_the_expected_block_sequence() {
        let lesson = Lesson::builder("Title")
            .heading("H")
            .note("N")
            .para(|p| p.text("hello ").math(r"x^2"))
            .display(r"y = x^2")
            .rule()
            .build();

        assert_eq!(lesson.title(), "Title");
        assert_eq!(
            lesson.blocks(),
            &[
                Block::Heading("H".to_owned()),
                Block::Note("N".to_owned()),
                Block::Para(vec![Inline::Text("hello ".to_owned()), Inline::Math("x^2".to_owned())]),
                Block::Display("y = x^2".to_owned()),
                Block::Rule,
            ]
        );
    }

    #[test]
    fn figure_and_plot_blocks_land_in_order() {
        let lesson = Lesson::builder("Title")
            .figure(Figure::new("<svg/>", "a drawing").width_percent(80))
            .plot(Plot::from_points(vec![[0.0, 0.0], [1.0, 1.0]]).caption("a line").hline(0.5))
            .build();

        let [Block::Figure(figure), Block::Plot(plot)] = lesson.blocks() else {
            panic!("expected exactly a figure then a plot, got {:?}", lesson.blocks());
        };
        assert_eq!(figure.caption, "a drawing");
        assert_eq!(figure.width_percent, Some(80));
        assert_eq!(plot.hlines, vec![0.5]);
        assert_eq!(plot.caption, "a line");
    }

}
