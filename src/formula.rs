//! LaTeX -> mitex -> Typst -> SVG, plus the geometry that makes the result
//! hoverable.
//!
//! Typst hands back a laid-out page whose frames still know where every glyph
//! sits, which characters it came from and which piece of source produced it,
//! so the same compile that produces the picture also produces the hit boxes —
//! and, via [`crate::terms`], the terms those boxes are grouped into. Nothing
//! here draws: [`crate::mathview`] is what puts a [`RenderedMath`] on screen,
//! and [`crate::pdf`] puts the same conversion on paper.

use std::sync::Arc;

use eframe::egui;
use typst::World as _;
use typst::layout::{Abs, Frame, FrameItem, Point, Transform};
use typst::text::{BottomEdge, BottomEdgeMetric, TextEdgeBounds, TopEdge, TopEdgeMetric};

use crate::symbols;
use crate::terms::{self, RawGlyph, Term};

/// STIX Two Math: sturdier hairlines than New Computer Modern at 1x density.
/// egui gets it too, as a fallback family, so tooltips can print the very
/// characters Typst drew — Atkinson has no Greek and no Mathematical
/// Alphanumeric Symbols, and a missing glyph would show as tofu.
pub(crate) const STIX_MATH: &[u8] = include_bytes!("../assets/STIXTwoMath-Regular.ttf");

/// Font size of inline maths, in Typst points. Chosen so its x-height matches
/// the 16.5pt body text once scaled up by `mathview::PT_TO_POINTS`.
const INLINE_PT: f32 = 12.5;
/// Font size of display maths, in Typst points.
const DISPLAY_PT: f32 = 17.0;

/// Ink colour of the maths, matched to the body text.
const INK: &str = "#E8E6E3";
/// Ink colour of a hovered character. `recolour` swaps one for the other.
const INK_HOVER: &str = "#4ADE80";
/// Sky blue, for characters a lesson declares vectors or matrices.
const INK_ACCENT: &str = "#87CEEB";

/// A compiled formula: the picture, and everything needed to interact with it.
pub(crate) struct RenderedMath {
    pub svg: Arc<[u8]>,
    /// The same drawing with the ink recoloured green. Painting it over the
    /// normal one, clipped to a term, turns that term green without a second
    /// Typst compile or any pixel-level trickery.
    pub svg_hover: Arc<[u8]>,
    /// The same drawing in sky blue, for vector/matrix glyphs.
    pub svg_accent: Arc<[u8]>,
    pub page_size_pt: egui::Vec2,
    pub terms: Vec<Term>,
    pub latex: String,
    pub typst: String,
}

/// Swap the ink colour in a finished SVG.
///
/// `typst_svg` writes the colour we asked for as a lowercase `#rrggbb` on every
/// piece of ink, so this is a plain string substitution. Both attributes have
/// to be swapped: glyphs are filled, but a fraction bar is a stroked path, and
/// leaving strokes alone draws a grey line through the middle of a highlighted
/// fraction. Returns the rewritten SVG and how many attributes it touched; a
/// count of zero means the emitted format has moved and the hover recolour
/// would silently do nothing (see the `recolour_finds_the_ink` test).
fn recolour(svg: &str, ink: &str) -> (String, usize) {
    let mut out = svg.to_owned();
    let mut touched = 0;
    for attribute in ["fill", "stroke"] {
        let from = format!("{attribute}=\"{}\"", INK.to_ascii_lowercase());
        let to = format!("{attribute}=\"{}\"", ink.to_ascii_lowercase());
        touched += out.matches(&from).count();
        out = out.replace(&from, &to);
    }
    (out, touched)
}

/// LaTeX to Typst maths, the one place the conversion is spelled out.
///
/// The PDF export goes through here too, so a formula cannot end up typeset one
/// way on screen and another way on paper.
pub(crate) fn to_typst_math(latex: &str) -> Result<String, String> {
    Ok(mitex::convert_math(latex, None)?.trim().to_owned())
}

/// The one-equation Typst document a formula is compiled as.
fn document(body: &str, display: bool) -> String {
    // `$ x $` (padded) selects Typst display style; `$x$` stays inline.
    let math = if display {
        format!("$ {body} $")
    } else {
        format!("${body}$")
    };
    let size = if display { DISPLAY_PT } else { INLINE_PT };
    // The margin must stay positive: at 0pt Typst crops the page to the glyph
    // box and subscripts/descenders get clipped off.
    let margin = if display { "2pt" } else { "1.5pt" };

    format!(
        "#set page(width: auto, height: auto, margin: {margin}, fill: none)\n\
         #set text(size: {size}pt, fill: rgb(\"{INK}\"))\n\
         #show math.equation: set text(font: \"STIX Two Math\")\n\
         {math}"
    )
}

/// A Typst engine over one source, with only the fonts this app ships and the
/// ones Typst embeds — never the system's, so a machine cannot change how a
/// formula sets.
fn engine(src: String) -> typst_as_lib::TypstEngine<typst_as_lib::TypstTemplateMainFile> {
    typst_as_lib::TypstEngine::builder()
        .main_file(src)
        .fonts([STIX_MATH])
        .search_fonts_with(
            typst_as_lib::typst_kit_options::TypstKitFontOptions::default()
                .include_system_fonts(false)
                .include_embedded_fonts(true),
        )
        .build()
}

/// Compile one formula, `display` selecting Typst's display style over inline.
///
/// The error is a bare `String` rather than [`crate::Error`] on purpose:
/// mitex and Typst both report in prose, and every caller turns that prose
/// straight back into text a person reads — an `Error::MathCompile`, an
/// `AuditFinding::MathError`, or the red `[math error: …]` label. Wrapping it
/// in a typed error here would only mean unwrapping it again at all three.
pub(crate) fn compile(latex: &str, display: bool) -> Result<RenderedMath, String> {
    let body = to_typst_math(latex)?;
    let engine = engine(document(&body, display));

    // Driving the compile through the world by hand, rather than through
    // `engine.compile()`, is what makes the terms possible: the glyph spans
    // only mean anything against the `Source` the world parsed, and this is the
    // only way to hold both at once.
    let (doc, page_size_pt, terms) = engine
        .with_world(|world| {
            let doc: typst_layout::PagedDocument = typst::compile(world).output.map_err(|errors| {
                let messages: Vec<_> = errors.iter().map(|error| error.message.to_string()).collect();
                messages.join("; ")
            })?;

            let page = doc.pages().first().ok_or("typst produced no pages")?;
            let size = page.frame.size();
            let page_size_pt = egui::vec2(size.x.to_pt() as f32, size.y.to_pt() as f32);

            let mut glyphs = Vec::new();
            collect_glyphs(&page.frame, Transform::identity(), &mut glyphs);
            let source = world
                .source(world.main())
                .map_err(|error| error.to_string())?;

            Ok::<_, String>((doc, page_size_pt, terms::partition(&source, glyphs)))
        })
        .map_err(|error| error.to_string())??;

    let svg = typst_svg::svg_merged(&doc, &Default::default(), Abs::zero());
    let (hover, recoloured) = recolour(&svg, INK_HOVER);
    debug_assert!(
        recoloured > 0,
        "no ink to recolour in the emitted SVG — hover would not turn green"
    );
    let (accent, _) = recolour(&svg, INK_ACCENT);

    Ok(RenderedMath {
        svg: Arc::from(svg.into_bytes()),
        svg_hover: Arc::from(hover.into_bytes()),
        svg_accent: Arc::from(accent.into_bytes()),
        page_size_pt,
        terms,
        latex: latex.to_owned(),
        typst: body,
    })
}

/// Walk a frame, accumulating the transform, and record one box per glyph.
///
/// This mirrors typst-ide's `jump_from_click_in_frame`: item positions are
/// relative to the frame's top-left, a text item's position is its *baseline*
/// left edge, and a group's children live in the group's own transformed
/// space. The horizontal extent of a glyph is its advance (so neighbouring
/// characters tile with no dead gaps), the vertical extent is the glyph's own
/// ink bounding box (so a box round a digit does not also cover the ascender
/// space above it). Each glyph keeps the span of the source that produced it,
/// which is what [`crate::terms`] groups on.
fn collect_glyphs(frame: &Frame, ts: Transform, out: &mut Vec<RawGlyph>) {
    for (pos, item) in frame.items() {
        match item {
            FrameItem::Group(group) => {
                let ts = ts
                    .pre_concat(Transform::translate(pos.x, pos.y))
                    .pre_concat(group.transform);
                collect_glyphs(&group.frame, ts, out);
            }
            FrameItem::Text(text) => {
                let mut pen = Abs::zero();

                for glyph in &text.glyphs {
                    let advance = glyph.x_advance.at(text.size);
                    let x = pos.x + pen + glyph.x_offset.at(text.size);
                    // `y_offset` is measured upwards from the baseline.
                    let baseline = pos.y - glyph.y_offset.at(text.size);
                    pen += advance;

                    // `edges` with the `Bounds` metrics returns the glyph's own
                    // ink extent: `top` above the baseline, `bottom` below it.
                    let (top, bottom) = text.font.edges(
                        TopEdge::Metric(TopEdgeMetric::Bounds),
                        BottomEdge::Metric(BottomEdgeMetric::Bounds),
                        text.size,
                        TextEdgeBounds::Glyph(glyph.id),
                    );
                    if top + bottom <= Abs::zero() || advance <= Abs::zero() {
                        continue; // spaces and other inkless glyphs
                    }

                    let character: String = text
                        .text
                        .get(glyph.range())
                        .unwrap_or_default()
                        .chars()
                        .filter(|&ch| !symbols::is_invisible(ch))
                        .collect::<String>()
                        .trim()
                        .to_owned();
                    if character.is_empty() {
                        continue;
                    }

                    let corners = [
                        Point::new(x, baseline - top),
                        Point::new(x + advance, baseline - top),
                        Point::new(x, baseline + bottom),
                        Point::new(x + advance, baseline + bottom),
                    ];
                    out.push(RawGlyph {
                        rect_pt: bounding_rect(corners, ts),
                        text: character,
                        span: glyph.span.0,
                    });
                }
            }
            _ => {}
        }
    }
}

/// Transform all four corners and take their bounding box, so a rotated group
/// still yields a sane (if slightly generous) hit box.
fn bounding_rect(corners: [Point; 4], ts: Transform) -> egui::Rect {
    let mut rect = egui::Rect::NOTHING;
    for corner in corners {
        let point = corner.transform(ts);
        rect.extend_with(egui::pos2(point.x.to_pt() as f32, point.y.to_pt() as f32));
    }
    rect
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Guards against the `#show math.equation` rule silently falling back to
    /// the embedded New Computer Modern when the family name is wrong.
    #[test]
    fn math_is_set_in_stix() {
        fn families(frame: &Frame, out: &mut Vec<String>) {
            for (_, item) in frame.items() {
                match item {
                    FrameItem::Group(group) => families(&group.frame, out),
                    FrameItem::Text(text) => {
                        out.push(text.font.font().info().family.clone());
                    }
                    _ => {}
                }
            }
        }

        let engine = engine(document("sigma_t^2 = omega + alpha", true));
        let doc: typst_layout::PagedDocument = engine.compile().output.unwrap();
        let mut out = Vec::new();
        families(&doc.pages()[0].frame, &mut out);
        out.sort();
        out.dedup();
        assert_eq!(out, vec!["STIX Two Math".to_owned()], "font fallback: {out:?}");
    }

    /// The hover highlight is a string substitution on the finished SVG, so it
    /// breaks silently if `typst_svg` ever writes the fill differently.
    #[test]
    fn recolour_finds_the_ink() {
        let rendered = compile(r"\sigma_t^2 = \omega", true).unwrap();
        let svg = std::str::from_utf8(&rendered.svg).unwrap();
        let (hover, recoloured) = recolour(svg, INK_HOVER);

        assert_eq!(recoloured, 5, "one fill per glyph");
        assert!(!hover.contains(&INK.to_ascii_lowercase()));
        assert_eq!(hover.as_bytes(), &rendered.svg_hover[..]);

        // A fraction bar is a stroked path rather than a glyph. A whole
        // fraction is one term now, so a bar left off-white would run visibly
        // through the middle of the highlight.
        let fraction = compile(r"\frac{\omega}{2}", true).unwrap();
        let svg = std::str::from_utf8(&fraction.svg).unwrap();
        let stroke = format!("stroke=\"{}\"", INK.to_ascii_lowercase());
        assert!(svg.contains(&stroke), "the bar is drawn some other way now");
        assert!(!recolour(svg, INK_HOVER).0.contains(&INK.to_ascii_lowercase()));
    }

    fn keys(latex: &str) -> Vec<String> {
        compile(latex, true)
            .expect("compile")
            .terms
            .iter()
            .map(|term| term.key.clone())
            .collect()
    }

    /// The body of a sum is where the model lives, so the brackets round it are
    /// opened up and each summand is a term of its own — while `ℓ(θ)`, the
    /// fraction and the summation each stay whole.
    #[test]
    fn a_sum_is_read_summand_by_summand() {
        assert_eq!(
            keys(
                r"\ell(\theta) = -\frac{1}{2} \sum_{t=1}^{T} \left[ \ln(2\pi) + \ln \sigma_t^2 + \frac{\varepsilon_t^2}{\sigma_t^2} \right]"
            ),
            [
                "ell(theta)",
                "=",
                "-frac(1,2)",
                "sum_(t=1)^(T)",
                "[",
                "ln(2 pi)",
                "+",
                "ln sigma_(t)^(2)",
                "+",
                "frac(epsilon_(t)^(2),sigma_(t)^(2))",
                "]",
            ]
        );
    }

    /// A fraction is one term however much is inside it, and a bracket that is
    /// not being summed over stays whole even though it holds a subtraction.
    #[test]
    fn brackets_and_fractions_stay_whole() {
        assert_eq!(
            keys(r"\bar{\sigma}^2 = \frac{\omega}{1 - \alpha - \beta}"),
            ["macron(sigma)^(2)", "=", "frac(omega,1-alpha-beta)"]
        );
        assert_eq!(
            keys(
                r"E[\sigma_{t+h}^2] = \bar{\sigma}^2 + (\alpha + \beta)^{h-1} \left( \sigma_{t+1}^2 - \bar{\sigma}^2 \right)"
            ),
            [
                "E[sigma_(t+h)^(2)]",
                "=",
                "macron(sigma)^(2)",
                "+",
                "(alpha+beta)^(h-1)",
                "(sigma_(t+1)^(2)-macron(sigma)^(2))",
            ]
        );
    }

    /// Every formula the POC's GARCH page set, with whether it was set on its
    /// own line. Kept as a local fixture so the tiling test still exercises
    /// the hardest real formulas this renderer has to handle, not just the
    /// handful inline above.
    fn page_formulas() -> Vec<(&'static str, bool)> {
        vec![
            (r"r_t", false),
            (r"r_t = \mu + \varepsilon_t", false),
            (r"\varepsilon_t = \sigma_t z_t", false),
            (r"z_t \sim \mathcal{N}(0,1)", false),
            (r"\sigma_t^2", false),
            (
                r"\sigma_t^2 = \omega + \alpha \varepsilon_{t-1}^2 + \beta \sigma_{t-1}^2",
                true,
            ),
            (r"\omega > 0", false),
            (r"\alpha \varepsilon_{t-1}^2", false),
            (r"\beta \sigma_{t-1}^2", false),
            (r"\alpha + \beta", false),
            (r"\alpha + \beta < 1", false),
            (r"\bar{\sigma}^2 = \frac{\omega}{1 - \alpha - \beta}", true),
            (r"\alpha + \beta", false),
            (r"\theta = (\omega, \alpha, \beta)", false),
            (
                r"\ell(\theta) = -\frac{1}{2} \sum_{t=1}^{T} \left[ \ln(2\pi) + \ln \sigma_t^2 + \frac{\varepsilon_t^2}{\sigma_t^2} \right]",
                true,
            ),
            (r"\sigma_1^2", false),
            (r"\ell(\theta)", false),
            (r"\omega, \alpha, \beta \geq 0", false),
            (r"h", false),
            (
                r"E[\sigma_{t+h}^2] = \bar{\sigma}^2 + (\alpha + \beta)^{h-1} \left( \sigma_{t+1}^2 - \bar{\sigma}^2 \right)",
                true,
            ),
            (r"\varepsilon_{t-1}^2", false),
            (
                r"\sigma_t^2 = \omega + \alpha \varepsilon_{t-1}^2 + \beta \bar{\sigma}^2",
                false,
            ),
            (r"\omega = 0.05", false),
            (r"\alpha = 0.09", false),
            (r"\beta = 0.90", false),
            (r"\bar{\sigma}^2 = 5", false),
        ]
    }

    /// Terms have to tile: no glyph may go missing between them, and none may
    /// be claimed twice, or the highlight would leave holes or spill.
    #[test]
    fn terms_tile_every_formula() {
        for (latex, display) in page_formulas() {
            let body = to_typst_math(latex).expect("mitex");
            let doc: typst_layout::PagedDocument = engine(document(&body, display))
                .compile()
                .output
                .expect("typst");
            let mut laid_out = Vec::new();
            collect_glyphs(&doc.pages()[0].frame, Transform::identity(), &mut laid_out);

            let rendered = compile(latex, display).expect("compile");
            let grouped: usize = rendered.terms.iter().map(|term| term.glyphs.len()).sum();
            assert_eq!(grouped, laid_out.len(), "glyphs lost or doubled in {latex}");
        }
    }

    /// Typst attaches a variation selector to the script 𝒩; egui would draw
    /// it as a tofu box in the tooltip. No glyph text may carry invisibles.
    #[test]
    fn glyph_texts_contain_no_invisible_characters() {
        let rendered = compile(r"z_t \sim \mathcal{N}(0,1)", true).expect("compile");
        let glyphs: Vec<_> = rendered.terms.iter().flat_map(|term| &term.glyphs).collect();
        assert!(glyphs.iter().any(|g| g.text == "𝒩"), "𝒩 present, selector gone");
        for glyph in glyphs {
            assert!(
                !glyph.text.chars().any(crate::symbols::is_invisible),
                "invisible character in {:?}",
                glyph.text
            );
        }
    }
}
