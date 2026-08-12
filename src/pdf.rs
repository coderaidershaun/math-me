//! A lesson as a real PDF: one Typst document built by walking a [`Lesson`]'s
//! blocks, compiled by the same engine that sets the equations on screen.
//!
//! Nothing here is rasterised. The maths goes through the same mitex
//! conversion the viewer uses, the fonts are embedded, and the prose comes
//! back out of the finished file as selectable text. The one thing that does
//! not carry over is the palette: the viewer is off-white on near-black, and
//! paper is the other way up, so every colour below is chosen for print.

use std::fmt::Write as _;
use std::path::Path;

use typst_as_lib::TypstEngine;
use typst_as_lib::typst_kit_options::TypstKitFontOptions;

use crate::error::{Error, Result};
use crate::lesson::{Block, Figure, Inline, Lesson};
use crate::plot::{Plot, ResolvedPlot, ResolvedSeries, SecondaryAxis, Shape, Ticks, Y_TICKS, format_tick};

/// Print ink: not quite black, for the same reason the viewer is not quite
/// white.
const INK: &str = "#1F1F1F";
const MUTED: &str = "#6B6B6B";
const RULE: &str = "#D4D4D4";
const GRID: &str = "#E6E6E6";
const AXIS: &str = "#B0B0B0";

/// The body font's family name inside both Atkinson TTFs; Typst picks the
/// bold file by weight.
const BODY_FONT: &str = "Atkinson Hyperlegible";
const MATH_FONT: &str = "STIX Two Math";

/// Included directly, rather than shared with `viewer.rs`, so this module has
/// no cross-file coupling to the on-screen app.
const ATKINSON_REGULAR: &[u8] = include_bytes!("../assets/AtkinsonHyperlegible-Regular.ttf");
const ATKINSON_BOLD: &[u8] = include_bytes!("../assets/AtkinsonHyperlegible-Bold.ttf");

/// A figure or plot's default width in the PDF's column, when its block
/// doesn't say otherwise.
const DEFAULT_FIGURE_WIDTH_PERCENT: u8 = 100;

/// A virtual filename paired with the binary content the static file
/// resolver serves at it — one per figure or plot the lesson embeds.
type EmbeddedFile = (String, Vec<u8>);

/// Typeset `lesson` as one Typst document and write it to `path` as a PDF.
///
/// # Errors
/// - [`Error::MathCompile`] if a formula's LaTeX does not convert to Typst
/// - [`Error::Pdf`] if Typst fails to typeset or export the document
/// - [`Error::Io`] if `path` cannot be written
pub(crate) fn export(lesson: &Lesson, path: &Path) -> Result<()> {
    let (source, files) = typst_source(lesson)?;

    let engine = TypstEngine::builder()
        .main_file(source)
        .fonts([ATKINSON_REGULAR, ATKINSON_BOLD, crate::formula::STIX_MATH])
        .search_fonts_with(TypstKitFontOptions::default().include_system_fonts(false))
        .with_static_file_resolver(files.iter().map(|(name, bytes)| (name.as_str(), bytes.clone())))
        .build();

    let doc: typst_layout::PagedDocument = engine
        .compile()
        .output
        .map_err(|error| Error::Pdf { message: format!("typst: {error}") })?;

    let pdf = typst_pdf::pdf(&doc, &Default::default()).map_err(|errors| {
        let messages: Vec<_> = errors.iter().map(|error| error.message.to_string()).collect();
        Error::Pdf { message: format!("pdf: {}", messages.join("; ")) }
    })?;

    std::fs::write(path, pdf).map_err(|source| Error::Io { path: path.to_owned(), source })
}

/// Characters Typst markup would otherwise read as syntax rather than prose.
fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        if matches!(
            ch,
            '\\' | '#' | '$' | '[' | ']' | '*' | '_' | '`' | '@' | '<' | '>' | '~'
        ) {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

/// Build the whole lesson as one Typst document, plus the binary files (figure
/// and plot SVGs) it references by name.
fn typst_source(lesson: &Lesson) -> Result<(String, Vec<EmbeddedFile>)> {
    let mut src = format!(
        "#set page(paper: \"a4\", margin: 2.2cm, numbering: \"1\")\n\
         #set text(font: \"{BODY_FONT}\", size: 11pt, fill: rgb(\"{INK}\"), lang: \"en\")\n\
         #set par(justify: true, leading: 0.75em, spacing: 1.05em)\n\
         #show math.equation: set text(font: \"{MATH_FONT}\")\n\
         #show heading: set text(size: 16pt)\n\
         #show heading: set block(above: 1.5em, below: 0.85em)\n\
         #show figure.caption: set text(size: 9pt, fill: rgb(\"{MUTED}\"))\n\
         #set figure(gap: 0.9em)\n\n"
    );
    let mut files = Vec::new();

    for (index, block) in lesson.blocks().iter().enumerate() {
        match block {
            Block::Heading(text) => {
                let _ = writeln!(src, "= {}\n", escape(text));
            }
            // The note tells the reader to hover, which is advice a sheet of
            // paper cannot honour. It is the one block the print rendering
            // drops.
            Block::Note(_) => {}
            Block::Para(inlines) => {
                for inline in inlines {
                    match inline {
                        Inline::Text(text) => src.push_str(&escape(text)),
                        Inline::Math(latex) => {
                            let math = to_typst_math(latex)?;
                            let _ = write!(src, "${math}$");
                        }
                    }
                }
                src.push_str("\n\n");
            }
            Block::Display(latex) => {
                let math = to_typst_math(latex)?;
                let _ = writeln!(src, "$ {math} $\n");
            }
            Block::Rule => {
                let _ = writeln!(
                    src,
                    "#v(1em)\n#line(length: 100%, stroke: 0.6pt + rgb(\"{RULE}\"))\n"
                );
            }
            Block::Figure(figure) => {
                let name = format!("figure-{index}.svg");
                files.push((name.clone(), figure_svg(figure).into_bytes()));
                let width = figure.width_percent.unwrap_or(DEFAULT_FIGURE_WIDTH_PERCENT);
                let _ = writeln!(
                    src,
                    "#figure(image(\"{name}\", width: {width}%), caption: [{}])\n",
                    escape(&figure.caption)
                );
            }
            Block::Plot(plot) => {
                let name = format!("plot-{index}.svg");
                files.push((name.clone(), plot_svg(plot).into_bytes()));
                let _ = writeln!(
                    src,
                    "#figure(image(\"{name}\", width: 100%), caption: [{}])\n",
                    escape(&plot.caption)
                );
            }
        }
    }

    Ok((src, files))
}

/// The image the PDF embeds for a figure: the author's print variant if they
/// gave one, else the same SVG the viewer draws.
fn figure_svg(figure: &Figure) -> String {
    figure.print_svg.clone().unwrap_or_else(|| figure.svg.clone())
}

/// LaTeX to Typst maths, mapping a conversion failure to the crate's error
/// type with the offending formula attached.
fn to_typst_math(latex: &str) -> Result<String> {
    crate::formula::to_typst_math(latex).map_err(|message| Error::MathCompile { latex: latex.to_owned(), message })
}

/// The page a plot is drawn on, in Typst points, and the data range it maps
/// onto — the paper equivalent of the frame `egui_plot` fits on screen.
struct Canvas {
    left: f64,
    right: f64,
    top: f64,
    x: [f64; 2],
    y: [f64; 2],
    secondary: Option<SecondaryAxis>,
}

impl Canvas {
    const WIDTH: f64 = 480.0;
    const HEIGHT: f64 = 240.0;
    const BOTTOM: f64 = 196.0;

    fn new(resolved: &ResolvedPlot) -> Self {
        // Room along the right edge for the second axis's ticks and title
        // when there is one, and none wasted when there is not.
        let right = if resolved.secondary.is_some() { 420.0 } else { 470.0 };
        let rows = legend_rows(&resolved.series, right - 52.0).len() as f64;
        Self {
            left: 52.0,
            right,
            // Room along the top for as many legend rows as the names need.
            top: 14.0 + rows * LEGEND_ROW,
            x: resolved.x,
            y: resolved.y,
            secondary: resolved.secondary,
        }
    }

    fn x_of(&self, value: f64) -> f64 {
        self.left + (value - self.x[0]) / (self.x[1] - self.x[0]) * (self.right - self.left)
    }

    fn y_of(&self, value: f64) -> f64 {
        Self::BOTTOM - (value - self.y[0]) / (self.y[1] - self.y[0]) * (Self::BOTTOM - self.top)
    }
}

/// How many gridlines the x axis carries, near enough; the y axis uses
/// [`Y_TICKS`], which the right-hand axis is also rounded against.
const X_TICKS: u32 = 8;

/// A plot's data, drawn straight to SVG so it stays vector in the PDF.
///
/// The viewer gets a plot from `egui_plot`, which paints into a texture and
/// knows nothing about paper. Rather than screenshot it, the same
/// [`ResolvedPlot`] the viewer draws is rebuilt here in ink — with the
/// parameters frozen at their defaults, paper having no sliders to drag.
fn plot_svg(plot: &Plot) -> String {
    let resolved = plot.resolve(&plot.defaults());
    let canvas = Canvas::new(&resolved);

    let (width, height) = (Canvas::WIDTH, Canvas::HEIGHT);
    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" \
         width=\"{width}\" height=\"{height}\">\n"
    );

    grid(&mut svg, &canvas);
    reference_lines(&mut svg, &canvas, plot);

    for series in &resolved.series {
        let ink = hex(series.color.print);
        for segment in &series.segments {
            match series.shape {
                Shape::Line => {
                    let mut points = String::new();
                    for &[x, y] in segment {
                        let _ = write!(points, "{:.2},{:.2} ", canvas.x_of(x), canvas.y_of(y));
                    }
                    let _ = writeln!(
                        svg,
                        "  <polyline points=\"{}\" fill=\"none\" stroke=\"{ink}\" stroke-width=\"2.2\" \
                         stroke-linejoin=\"round\"/>",
                        points.trim_end()
                    );
                }
                Shape::Scatter => {
                    for &[x, y] in segment {
                        let _ = writeln!(
                            svg,
                            "  <circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"2.4\" fill=\"{ink}\"/>",
                            canvas.x_of(x),
                            canvas.y_of(y)
                        );
                    }
                }
            }
        }
    }

    legend(&mut svg, &canvas, &resolved.series);
    axis_titles(&mut svg, &canvas, plot);

    svg.push_str("</svg>\n");
    svg
}

/// Gridlines, tick labels and spines — including the right-hand spine when a
/// second axis needs one, its labels read back out of the primary
/// coordinates every series was mapped into.
fn grid(svg: &mut String, canvas: &Canvas) {
    let (left, right, top, bottom) = (canvas.left, canvas.right, canvas.top, Canvas::BOTTOM);

    let x_ticks = Ticks::over(canvas.x, X_TICKS);
    for value in &x_ticks.values {
        let x = canvas.x_of(*value);
        let _ = writeln!(
            svg,
            "  <line x1=\"{x:.1}\" y1=\"{top}\" x2=\"{x:.1}\" y2=\"{bottom}\" \
             stroke=\"{GRID}\" stroke-width=\"1\"/>\n  \
             <text x=\"{x:.1}\" y=\"{label_y}\" font-family=\"{BODY_FONT}\" font-size=\"9.5\" \
             fill=\"{MUTED}\" text-anchor=\"middle\">{label}</text>",
            label_y = bottom + 15.0,
            label = format_tick(*value, x_ticks.step),
        );
    }

    let y_ticks = Ticks::over(canvas.y, Y_TICKS);
    for value in &y_ticks.values {
        let y = canvas.y_of(*value);
        let _ = writeln!(
            svg,
            "  <line x1=\"{left}\" y1=\"{y:.1}\" x2=\"{right}\" y2=\"{y:.1}\" \
             stroke=\"{GRID}\" stroke-width=\"1\"/>\n  \
             <text x=\"{label_x}\" y=\"{baseline:.1}\" font-family=\"{BODY_FONT}\" font-size=\"9.5\" \
             fill=\"{MUTED}\" text-anchor=\"end\">{label}</text>",
            label_x = left - 8.0,
            baseline = y + 3.4,
            label = format_tick(*value, y_ticks.step),
        );
    }

    // The right-hand labels are laid out in the second axis's own units and
    // placed where those values fall, rather than relabelling the left axis's
    // gridlines: on paper both sets of ticks can be round, which is worth
    // more than having them line up.
    if let Some(map) = canvas.secondary {
        let ticks = Ticks::over([map.to_secondary(canvas.y[0]), map.to_secondary(canvas.y[1])], Y_TICKS);
        for value in &ticks.values {
            let _ = writeln!(
                svg,
                "  <text x=\"{label_x}\" y=\"{baseline:.1}\" font-family=\"{BODY_FONT}\" font-size=\"9.5\" \
                 fill=\"{MUTED}\" text-anchor=\"start\">{label}</text>",
                label_x = right + 8.0,
                baseline = canvas.y_of(map.to_primary(*value)) + 3.4,
                label = format_tick(*value, ticks.step),
            );
        }
    }

    let _ = writeln!(
        svg,
        "  <path d=\"M {left} {top} L {left} {bottom} L {right} {bottom}\" fill=\"none\" \
         stroke=\"{AXIS}\" stroke-width=\"1.1\"/>"
    );
    if canvas.secondary.is_some() {
        let _ = writeln!(
            svg,
            "  <line x1=\"{right}\" y1=\"{top}\" x2=\"{right}\" y2=\"{bottom}\" \
             stroke=\"{AXIS}\" stroke-width=\"1.1\"/>"
        );
    }
}

fn reference_lines(svg: &mut String, canvas: &Canvas, plot: &Plot) {
    for &value in &plot.hlines {
        let y = canvas.y_of(value);
        let _ = writeln!(
            svg,
            "  <line x1=\"{left}\" y1=\"{y:.1}\" x2=\"{right}\" y2=\"{y:.1}\" \
             stroke=\"{MUTED}\" stroke-width=\"1.2\" stroke-dasharray=\"7 5\"/>",
            left = canvas.left,
            right = canvas.right,
        );
    }
    for &value in &plot.vlines {
        let x = canvas.x_of(value);
        let _ = writeln!(
            svg,
            "  <line x1=\"{x:.1}\" y1=\"{top}\" x2=\"{x:.1}\" y2=\"{bottom}\" \
             stroke=\"{MUTED}\" stroke-width=\"1.2\" stroke-dasharray=\"7 5\"/>",
            top = canvas.top,
            bottom = Canvas::BOTTOM,
        );
    }
}

/// Roughly the width of one character of the body font at 9.5pt — enough to
/// lay entries out side by side without shaping the text.
const CHARACTER_WIDTH: f64 = 5.1;
const SWATCH: f64 = 16.0;
const SWATCH_GAP: f64 = 5.0;
const ENTRY_GAP: f64 = 16.0;
const LEGEND_ROW: f64 = 20.0;

/// The named series laid out into rows no wider than `width`, each entry with
/// its offset from the left of the frame.
///
/// Wrapped rather than run off the edge: the names are the author's, and a
/// series whose entry fell outside the picture would go missing in print
/// without anything to show for it.
fn legend_rows(series: &[ResolvedSeries], width: f64) -> Vec<Vec<(f64, &ResolvedSeries)>> {
    let mut rows: Vec<Vec<(f64, &ResolvedSeries)>> = Vec::new();
    let mut x = width;
    for series in series.iter().filter(|series| !series.name.is_empty()) {
        let entry = SWATCH + SWATCH_GAP + series.name.chars().count() as f64 * CHARACTER_WIDTH + ENTRY_GAP;
        if x + entry > width {
            rows.push(Vec::new());
            x = 0.0;
        }
        if let Some(row) = rows.last_mut() {
            row.push((x, series));
        }
        x += entry;
    }
    rows
}

/// Rows of swatches above the frame, one entry per named series.
///
/// Above rather than inside it, unlike `egui_plot`'s floating legend: neither
/// paper nor a plot the reader cannot pan can be dragged out from under an
/// overlapping curve. The viewer draws its own the same way.
fn legend(svg: &mut String, canvas: &Canvas, series: &[ResolvedSeries]) {
    let rows = legend_rows(series, canvas.right - canvas.left);
    for (index, row) in rows.iter().enumerate() {
        let baseline = canvas.top - 14.0 - (rows.len() - 1 - index) as f64 * LEGEND_ROW;
        for (offset, series) in row {
            let x = canvas.left + offset;
            let ink = hex(series.color.print);
            match series.shape {
                Shape::Line => {
                    let _ = writeln!(
                        svg,
                        "  <line x1=\"{x:.1}\" y1=\"{y:.1}\" x2=\"{end:.1}\" y2=\"{y:.1}\" \
                         stroke=\"{ink}\" stroke-width=\"2.2\"/>",
                        y = baseline - 3.2,
                        end = x + SWATCH,
                    );
                }
                Shape::Scatter => {
                    let _ = writeln!(
                        svg,
                        "  <circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"2.6\" fill=\"{ink}\"/>",
                        cx = x + SWATCH / 2.0,
                        cy = baseline - 3.2,
                    );
                }
            }
            let _ = writeln!(
                svg,
                "  <text x=\"{text_x:.1}\" y=\"{baseline:.1}\" font-family=\"{BODY_FONT}\" font-size=\"9.5\" \
                 fill=\"{INK}\" text-anchor=\"start\">{name}</text>",
                text_x = x + SWATCH + SWATCH_GAP,
                name = escape_xml(&series.name),
            );
        }
    }
}

fn axis_titles(svg: &mut String, canvas: &Canvas, plot: &Plot) {
    let middle_y = (canvas.top + Canvas::BOTTOM) / 2.0;

    if let Some(label) = &plot.x_label {
        let _ = writeln!(
            svg,
            "  <text x=\"{x:.1}\" y=\"{y}\" font-size=\"10.5\" fill=\"{INK}\" \
             text-anchor=\"middle\">{title}</text>",
            x = (canvas.left + canvas.right) / 2.0,
            y = Canvas::HEIGHT - 8.0,
            title = axis_title(label),
        );
    }
    if let Some(label) = &plot.y_label {
        let _ = writeln!(
            svg,
            "  <text transform=\"translate(13 {middle_y:.1}) rotate(-90)\" font-size=\"10.5\" \
             fill=\"{INK}\" text-anchor=\"middle\">{title}</text>",
            title = axis_title(label),
        );
    }
    if let Some(label) = plot.y2_label.as_ref().filter(|_| canvas.secondary.is_some()) {
        let _ = writeln!(
            svg,
            "  <text transform=\"translate({x:.1} {middle_y:.1}) rotate(90)\" font-size=\"10.5\" \
             fill=\"{INK}\" text-anchor=\"middle\">{title}</text>",
            x = Canvas::WIDTH - 10.0,
            title = axis_title(label),
        );
    }
}

fn hex(rgb: [u8; 3]) -> String {
    let [red, green, blue] = rgb;
    format!("#{red:02X}{green:02X}{blue:02X}")
}

/// The three characters an SVG's text nodes would otherwise read as markup.
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// Split an axis title into prose and the maths symbol it ends with, so each
/// half is set in a font that has the glyphs: Atkinson has no Greek, and
/// Typst resolves an SVG's fonts by name against the document's book —
/// generic families like `sans-serif` are not supported, so both must be
/// spelled out.
fn axis_title(label: &str) -> String {
    let split = label.find(|ch: char| !ch.is_ascii()).unwrap_or(label.len());
    let (words, symbol) = label.split_at(split);
    format!(
        "<tspan font-family=\"{BODY_FONT}\">{}</tspan>\
         <tspan font-family=\"{MATH_FONT}\" font-style=\"italic\">{}</tspan>",
        escape_xml(words),
        escape_xml(symbol),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A plot using every part of the drawing at once: a tunable curve and a
    /// scatter on the left axis, a line on the right one, and reference lines
    /// both ways.
    fn busy_plot() -> Plot {
        Plot::new(0.0..=10.0)
            .curve("cooling", "20 + 60 * exp(-k * x)")
            .param("k", 0.0..=1.0, 0.1)
            .scatter("readings", vec![[0.0, 80.0], [5.0, 56.0]])
            .line("share", vec![[0.0, 0.0], [10.0, 100.0]])
            .secondary()
            .hline(55.0)
            .vline(4.0)
            .y2_label("percent")
    }

    #[test]
    fn every_named_series_earns_a_legend_entry() {
        let svg = plot_svg(&busy_plot());

        for name in ["cooling", "readings", "share"] {
            assert!(svg.contains(&format!(">{name}</text>")), "no legend entry for {name} in\n{svg}");
        }
    }

    #[test]
    fn a_legend_too_wide_for_the_frame_wraps_instead_of_running_off_it() {
        let long = "a series named at length by its author";
        let plot = (0..4).fold(Plot::new(0.0..=1.0), |plot, index| {
            plot.line(format!("{long} {index}"), vec![[0.0, 0.0], [1.0, 1.0]])
        });

        let svg = plot_svg(&plot);
        let resolved = plot.resolve(&[]);
        let canvas = Canvas::new(&resolved);
        let entries = legend_rows(&resolved.series, canvas.right - canvas.left);

        assert!(entries.len() > 1, "four names that long should not fit on one row");
        for index in 0..4 {
            assert!(svg.contains(&format!(">{long} {index}</text>")), "entry {index} missing from\n{svg}");
        }
        for row in &entries {
            for (offset, series) in row {
                let width = offset + SWATCH + SWATCH_GAP + series.name.chars().count() as f64 * CHARACTER_WIDTH;
                assert!(width <= Canvas::WIDTH - 52.0, "{:?} runs past the frame", series.name);
            }
        }
    }

    #[test]
    fn a_secondary_series_gets_its_own_spine_and_round_ticks_in_its_own_units() {
        // The share series runs 0..100, so the right-hand axis should be
        // labelled in the round numbers of a percentage.
        let svg = plot_svg(&busy_plot());

        assert_eq!(svg.matches(AXIS).count(), 2, "expected a left spine and a right one in\n{svg}");
        for label in ["0", "20", "40", "60", "80", "100"] {
            assert!(svg.contains(&format!(">{label}</text>")), "no tick labelled {label} in\n{svg}");
        }
    }

    #[test]
    fn a_plot_on_one_axis_gets_one_spine() {
        let svg = plot_svg(&Plot::from_fn(0.0..=1.0, |x| x));

        assert_eq!(svg.matches(AXIS).count(), 1, "expected only the left-and-bottom spine in\n{svg}");
    }
}
