//! The on-screen viewer: an eframe/egui application that walks a [`Lesson`]'s
//! blocks into a centred reading column, with an export-to-PDF button and the
//! `MATH_ME_*` env-var harness the screenshot and PDF-export tests drive.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use eframe::egui::{self, AtomExt, Color32};
use egui_plot::{AxisHints, HLine, HPlacement, Line, LineStyle, Plot as EguiPlot, PlotBounds, PlotPoints, Points, VLine};

use crate::formula;
use crate::glossary::Glossary;
use crate::mathview::{self, MathRenderer};
use crate::plot::{ResolvedPlot, ResolvedSeries, SeriesColor, Shape, format_tick};
use crate::{Block, Error, Figure, Inline, Lesson, Plot, Result};

const ATKINSON_REGULAR: &[u8] = include_bytes!("../assets/AtkinsonHyperlegible-Regular.ttf");
const ATKINSON_BOLD: &[u8] = include_bytes!("../assets/AtkinsonHyperlegible-Bold.ttf");

/// Off-white rather than pure white: white on black haloes badly for readers
/// with astigmatism.
const INK: Color32 = Color32::from_rgb(0xE8, 0xE6, 0xE3);
const PAPER: Color32 = Color32::from_rgb(0x11, 0x11, 0x11);
const MUTED: Color32 = Color32::from_rgb(0x9A, 0x97, 0x93);

/// Roughly 70 characters at the body size — the width prose reads best at.
const COLUMN_WIDTH: f32 = 700.0;

/// Wide enough that a parameter slider has real travel in it, and still short
/// of the reading column so its name and value sit alongside.
const SLIDER_WIDTH: f32 = 300.0;

/// How long an export's outcome stays on screen before fading out of the way.
const STATUS_LINGER: Duration = Duration::from_secs(4);

/// A document sheet with a down-arrow into it — the export glyph. Stroked
/// white because the button multiplies it by the text colour, which is what
/// turns it [`INK`] at rest and brighter under the pointer.
const EXPORT_ICON_SVG: &str = r##"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none" stroke="#FFFFFF" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 3H6a1.5 1.5 0 0 0-1.5 1.5v15A1.5 1.5 0 0 0 6 21h12a1.5 1.5 0 0 0 1.5-1.5V8.5z"/><path d="M14 3v5.5h5.5"/><path d="M12 11v6"/><path d="M9.5 14.5 12 17l2.5-2.5"/></svg>"##;

/// How long the window stays hidden before it is destroyed: several 60 Hz
/// display cycles, long enough for the pending AppKit flush (see
/// [`MathApp::advance_close_handshake`]) to run, yet imperceptible — the
/// window is already invisible.
const HIDE_BEFORE_CLOSE: Duration = Duration::from_millis(120);

fn setup_fonts(ctx: &egui::Context) {
    use egui::{FontData, FontDefinitions, FontFamily};

    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert("Atkinson".to_owned(), Arc::new(FontData::from_static(ATKINSON_REGULAR)));
    fonts
        .font_data
        .insert("AtkinsonBold".to_owned(), Arc::new(FontData::from_static(ATKINSON_BOLD)));
    fonts
        .font_data
        .insert("StixTwoMath".to_owned(), Arc::new(FontData::from_static(formula::STIX_MATH)));
    let proportional = fonts.families.entry(FontFamily::Proportional).or_default();
    proportional.insert(0, "Atkinson".to_owned());
    // Last resort, for the maths characters the tooltips quote back.
    proportional.push("StixTwoMath".to_owned());
    fonts.families.insert(
        FontFamily::Name("AtkinsonBold".into()),
        vec!["AtkinsonBold".to_owned(), "Atkinson".to_owned()],
    );
    ctx.set_fonts(fonts);
}

fn setup_theme(ctx: &egui::Context) {
    use egui::{FontFamily, FontId, TextStyle};

    ctx.all_styles_mut(|style| {
        style.text_styles = [
            (TextStyle::Body, FontId::proportional(16.5)),
            (TextStyle::Heading, FontId::new(26.0, FontFamily::Name("AtkinsonBold".into()))),
            (TextStyle::Button, FontId::proportional(15.5)),
            (TextStyle::Small, FontId::proportional(13.0)),
            (TextStyle::Monospace, FontId::monospace(14.0)),
        ]
        .into();

        style.visuals.override_text_color = Some(INK);
        style.visuals.panel_fill = PAPER;
        style.visuals.extreme_bg_color = PAPER;
        style.visuals.window_fill = Color32::from_rgb(0x1B, 0x1B, 0x1B);
        style.visuals.window_stroke = egui::Stroke::new(1.0, Color32::from_rgb(0x3A, 0x3A, 0x3A));
        style.spacing.item_spacing.y = 10.0;
        style.spacing.tooltip_width = mathview::TOOLTIP_MAX_WIDTH;
    });
}

fn caption(ui: &mut egui::Ui, text: &str) {
    if text.is_empty() {
        return;
    }
    ui.add_space(2.0);
    ui.label(egui::RichText::new(text).size(13.5).color(MUTED));
    ui.add_space(6.0);
}

fn paragraph(ui: &mut egui::Ui, renderer: &mut MathRenderer, inlines: &[Inline], glossary: &Glossary) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for inline in inlines {
            match inline {
                Inline::Text(text) => {
                    ui.label(text.as_str());
                }
                Inline::Math(latex) => renderer.show(ui, latex, false, glossary),
            }
        }
    });
    ui.add_space(10.0);
}

fn display_math(ui: &mut egui::Ui, renderer: &mut MathRenderer, latex: &str, glossary: &Glossary) {
    ui.add_space(8.0);
    ui.vertical_centered(|ui| renderer.show(ui, latex, true, glossary));
    ui.add_space(16.0);
}

/// Render a [`Figure`] at its position in the block list. `index` is folded
/// into the image's `bytes://` URI, so every figure on the page gets a
/// stable, distinct cache key even though the SVG is now owned data rather
/// than a `&'static str` whose address egui could key on.
fn figure_block(ui: &mut egui::Ui, figure: &Figure, index: usize) {
    ui.add_space(6.0);
    ui.vertical_centered(|ui| {
        let uri = format!("bytes://figure-{index}.svg");
        let bytes: Arc<[u8]> = Arc::from(figure.svg.as_bytes());
        ui.add(egui::Image::from_bytes(uri, bytes).fit_to_original_size(1.0));
    });
    caption(ui, &figure.caption);
}

/// Where a plot's sliders currently stand, and the geometry last drawn from
/// them.
///
/// The values are the viewer's, not the lesson's: the model holds the
/// defaults, this holds what the reader has done with them. Re-resolving is
/// keyed on the values themselves, so a page nobody is touching costs nothing
/// per frame however many curves it carries.
struct PlotState {
    values: Vec<f64>,
    resolved: ResolvedPlot,
    resolved_at: Vec<f64>,
}

impl PlotState {
    fn new(plot: &Plot) -> Self {
        let values = plot.defaults();
        Self { resolved: plot.resolve(&values), resolved_at: values.clone(), values }
    }

    fn refresh(&mut self, plot: &Plot) {
        if self.resolved_at != self.values {
            self.resolved = plot.resolve(&self.values);
            self.resolved_at.clone_from(&self.values);
        }
    }
}

fn screen(color: SeriesColor) -> Color32 {
    let [red, green, blue] = color.screen;
    Color32::from_rgb(red, green, blue)
}

/// One y-axis strip, labelled or not. The label is owned rather than borrowed
/// so the hints outlive the plot builder they are handed to.
fn y_axis(label: Option<&str>) -> AxisHints<'static> {
    let hints = AxisHints::new_y()
        .label_spacing(egui::Rangef::new(20.0, 40.0))
        .tick_label_color(MUTED);
    match label {
        Some(label) => hints.label(label.to_owned()),
        None => hints,
    }
}

/// A row of swatches above the plot, one per named series. A series with no
/// name earns no entry, so a plot of one unnamed curve shows no legend.
///
/// Above the frame rather than floating in a corner of it, which is where
/// `egui_plot` would put it: these plots cannot be panned, so a legend that
/// lands on a curve stays on it. The PDF draws its own the same way, and now
/// the two renderings read alike.
fn legend(ui: &mut egui::Ui, series: &[ResolvedSeries]) {
    const SWATCH: egui::Vec2 = egui::vec2(16.0, 10.0);

    let mut named = series.iter().filter(|series| !series.name.is_empty()).peekable();
    if named.peek().is_none() {
        return;
    }
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 5.0;
        for series in named {
            let (swatch, _) = ui.allocate_exact_size(SWATCH, egui::Sense::hover());
            let color = screen(series.color);
            match series.shape {
                Shape::Line => {
                    let stroke = egui::Stroke::new(2.2, color);
                    ui.painter().line_segment([swatch.left_center(), swatch.right_center()], stroke);
                }
                Shape::Scatter => {
                    ui.painter().circle_filled(swatch.center(), 2.6, color);
                }
            }
            ui.label(egui::RichText::new(series.name.as_str()).size(13.5));
            ui.add_space(11.0);
        }
    });
    ui.add_space(2.0);
}

fn plot_block(ui: &mut egui::Ui, plot: &Plot, index: usize, state: &mut PlotState) {
    ui.add_space(16.0);
    state.refresh(plot);
    legend(ui, &state.resolved.series);

    let mut egui_plot = EguiPlot::new(format!("plot-{index}"))
        .height(plot.height)
        .allow_drag(false)
        .allow_zoom(false)
        .allow_scroll(false)
        .allow_boxed_zoom(false)
        .show_background(false)
        .grid_color(Color32::from_rgb(0x2A, 0x2A, 0x2A));

    if let Some(label) = &plot.x_label {
        egui_plot = egui_plot.custom_x_axes(vec![
            AxisHints::new_x()
                .label(label.clone())
                // The default label spacing only labels every tenth unit,
                // which on a ten-unit axis means a lone "0". Allow labels
                // much closer together.
                .label_spacing(egui::Rangef::new(24.0, 48.0))
                .tick_label_color(MUTED),
        ]);
    }

    let mut y_axes = vec![y_axis(plot.y_label.as_deref())];
    if let Some(map) = state.resolved.secondary {
        y_axes.push(
            y_axis(plot.y2_label.as_deref())
                .placement(HPlacement::Right)
                .formatter(move |mark, _| {
                    format_tick(map.to_secondary(mark.value), map.span_to_secondary(mark.step_size))
                }),
        );
    }
    egui_plot = egui_plot.custom_y_axes(y_axes);

    let resolved = &state.resolved;
    egui_plot.show(ui, |plot_ui| {
        // Pinned rather than left on auto: the right-hand axis labels are
        // computed by mapping this frame back through `to_secondary`, and a
        // frame egui chose for itself would put them somewhere else. A
        // tunable curve moves its own bounds, so it has to be set each frame.
        plot_ui.set_plot_bounds(PlotBounds::from_min_max(
            [resolved.x[0], resolved.y[0]],
            [resolved.x[1], resolved.y[1]],
        ));

        for series in &resolved.series {
            let color = screen(series.color);
            for segment in &series.segments {
                let points = PlotPoints::new(segment.clone());
                match series.shape {
                    Shape::Line => plot_ui.line(Line::new(series.name.clone(), points).color(color).width(2.2)),
                    Shape::Scatter => {
                        plot_ui.points(Points::new(series.name.clone(), points).color(color).radius(2.6))
                    }
                }
            }
        }
        for &y in &plot.hlines {
            plot_ui.hline(HLine::new("", y).color(MUTED).width(1.4).style(LineStyle::dashed_loose()));
        }
        for &x in &plot.vlines {
            plot_ui.vline(VLine::new("", x).color(MUTED).width(1.4).style(LineStyle::dashed_loose()));
        }
    });

    for error in &state.resolved.errors {
        ui.colored_label(mathview::ERROR_INK, format!("[plot error: {error}]"));
    }
    // Caption first, then the sliders: the caption belongs to the figure and
    // sits in the same place the PDF puts it, and it is usually the line that
    // tells the reader what dragging them will do.
    caption(ui, &plot.caption);
    sliders(ui, plot, &mut state.values);
}

/// One slider per declared parameter, under the plot it retunes.
fn sliders(ui: &mut egui::Ui, plot: &Plot, values: &mut [f64]) {
    if plot.params.is_empty() {
        return;
    }
    ui.add_space(8.0);
    ui.scope(|ui| {
        ui.spacing_mut().slider_width = SLIDER_WIDTH;
        ui.spacing_mut().item_spacing.y = 4.0;
        for (param, value) in plot.params.iter().zip(values) {
            ui.add(
                egui::Slider::new(value, param.min..=param.max)
                    .text(param.name.as_str())
                    .max_decimals(3),
            );
        }
    });
    ui.add_space(4.0);
}

/// The stem of the default export file name: the lesson's title, lowercased,
/// with every run of non-alphanumeric characters collapsed to one dash — so
/// "GARCH: Part 1!" becomes "garch-part-1".
fn sanitize_file_stem(title: &str) -> String {
    let mut stem = String::new();
    let mut last_was_dash = false;
    for ch in title.chars() {
        if ch.is_ascii_alphanumeric() {
            stem.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash && !stem.is_empty() {
            stem.push('-');
            last_was_dash = true;
        }
    }
    if stem.ends_with('-') {
        stem.pop();
    }
    if stem.is_empty() { "lesson".to_owned() } else { stem }
}

/// The window-teardown walk driven by [`MathApp::advance_close_handshake`]:
/// still visible, hidden and waiting out [`HIDE_BEFORE_CLOSE`], or the real
/// close already sent.
enum CloseHandshake {
    Open,
    Hidden(Instant),
    CloseSent,
}

struct MathApp {
    lesson: Lesson,
    renderer: MathRenderer,
    /// Slider state, keyed by the block index of the plot it belongs to.
    plots: HashMap<usize, PlotState>,
    /// When `MATH_ME_SHOT=<path>` is set, self-capture to that path and quit.
    shot_path: Option<String>,
    /// When `MATH_ME_EXPORT_PDF=<path>` is set, write the PDF there on the
    /// first frame. Taken rather than read, so it can only happen once.
    export_path: Option<String>,
    /// When `MATH_ME_SCROLL=<points>` is set, park the page at that offset
    /// so a screenshot can be taken of something below the fold.
    scroll_offset: Option<f32>,
    /// What the last export did, and when — shown under the button until it
    /// has been up for [`STATUS_LINGER`].
    status: Option<(String, Instant)>,
    frame_count: u32,
    /// Set the moment anything — the OS, the screenshot harness, an
    /// export-only run — decides the window should close.
    close_wanted: bool,
    handshake: CloseHandshake,
}

impl MathApp {
    /// Draw every block of the lesson into the current column.
    fn draw_blocks(&mut self, ui: &mut egui::Ui) {
        let blocks = self.lesson.blocks();
        let glossary = self.lesson.glossary();
        for (index, block) in blocks.iter().enumerate() {
            match block {
                Block::Heading(text) => {
                    ui.add_space(8.0);
                    ui.heading(text.as_str());
                    // A note is a subtitle and sits tight under its heading;
                    // body text under a bare heading wants the full gap.
                    let subtitled = matches!(blocks.get(index + 1), Some(Block::Note(_)));
                    ui.add_space(if subtitled { 4.0 } else { 10.0 });
                }
                Block::Note(text) => {
                    ui.label(egui::RichText::new(text.as_str()).size(13.5).color(MUTED));
                    ui.add_space(14.0);
                }
                Block::Para(inlines) => paragraph(ui, &mut self.renderer, inlines, glossary),
                Block::Display(latex) => display_math(ui, &mut self.renderer, latex, glossary),
                Block::Rule => {
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(6.0);
                }
                Block::Figure(figure) => figure_block(ui, figure, index),
                Block::Plot(plot) => {
                    let state = self.plots.entry(index).or_insert_with(|| PlotState::new(plot));
                    plot_block(ui, plot, index, state);
                }
            }
        }
        ui.add_space(24.0);
    }

    /// The export button, floating clear of the reading column so that adding
    /// it moved nothing on the page.
    fn export_button(&mut self, ctx: &egui::Context) {
        egui::Area::new(egui::Id::new("export"))
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-16.0, 12.0))
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                    let icon = egui::Image::from_bytes("bytes://export-icon.svg", EXPORT_ICON_SVG.as_bytes())
                        .fit_to_exact_size(egui::vec2(20.0, 20.0))
                        .atom_size(egui::vec2(20.0, 20.0));
                    let button = egui::Button::new(icon)
                        .image_tint_follows_text_color(true)
                        .frame_when_inactive(false)
                        .fill(Color32::from_rgb(0x1E, 0x1E, 0x1E))
                        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(0x3A, 0x3A, 0x3A)));

                    if ui.add(button).on_hover_text("Export PDF").clicked()
                        && let Some(path) = rfd::FileDialog::new()
                            .set_file_name(format!("{}.pdf", sanitize_file_stem(self.lesson.title())))
                            .save_file()
                    {
                        let message = match crate::pdf::export(&self.lesson, &path) {
                            Ok(()) => {
                                let name = path.file_name().unwrap_or(path.as_os_str());
                                format!("Saved to {}", name.to_string_lossy())
                            }
                            Err(error) => error.to_string(),
                        };
                        self.status = Some((message, Instant::now()));
                    }

                    if let Some((_, shown)) = &self.status
                        && shown.elapsed() >= STATUS_LINGER
                    {
                        self.status = None;
                    }
                    if let Some((message, shown)) = &self.status {
                        ui.label(egui::RichText::new(message).size(12.5).color(MUTED));
                        ctx.request_repaint_after(STATUS_LINGER.saturating_sub(shown.elapsed()));
                    }
                });
            });
    }

    fn request_close(&mut self) {
        self.close_wanted = true;
    }

    /// AppKit quirk (macOS 15, confirmed against winit 0.30.13 / eframe 0.35,
    /// both latest stable): once the window has been key, AppKit's Touch Bar
    /// Finder holds a KVO observation tied to its view, and invalidates that
    /// observation inside a later display-cycle flush. winit's native close
    /// destroys the window mid-loop, so that flush lands on a dead view,
    /// throws an uncaught `NSException` out of
    /// `-[_NSTouchBarFinderObservation invalidate]`, and the process traps
    /// with SIGTRAP ("quit unexpectedly") the moment the unwind tries to
    /// cross back into our objc2 frames. Hiding the window first lets the
    /// flush complete against a still-live view; only after
    /// [`HIDE_BEFORE_CLOSE`] do we send the real close.
    fn advance_close_handshake(&mut self, ctx: &egui::Context) {
        if !self.close_wanted {
            return;
        }
        match self.handshake {
            CloseHandshake::Open => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                self.handshake = CloseHandshake::Hidden(Instant::now());
                ctx.request_repaint();
            }
            CloseHandshake::Hidden(at) if at.elapsed() >= HIDE_BEFORE_CLOSE => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                self.handshake = CloseHandshake::CloseSent;
            }
            CloseHandshake::Hidden(_) => ctx.request_repaint(),
            CloseHandshake::CloseSent => {}
        }
    }
}

impl eframe::App for MathApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // The OS (red close button, Cmd+W, Dock quit) asked to close: cancel
        // eframe's own default handling and route it through our handshake
        // instead, same as every other close trigger below. Once the
        // handshake itself has sent the real close, `ViewportCommand::Close`
        // re-delivers as this same `close_requested()` signal on the next
        // frame — cancelling *that* would cancel our own close and hang the
        // app forever, so once `CloseSent` this check is skipped entirely.
        if !matches!(self.handshake, CloseHandshake::CloseSent)
            && ctx.input(|input| input.viewport().close_requested())
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.request_close();
        }

        if let Some(path) = self.export_path.take() {
            match crate::pdf::export(&self.lesson, Path::new(&path)) {
                Ok(()) => println!("wrote {path}"),
                Err(error) => eprintln!("could not export pdf: {error}"),
            }
            // A screenshot in the same run still needs its frames; otherwise
            // there is nothing left to wait for.
            if self.shot_path.is_none() {
                self.request_close();
            }
        }

        if let Some(path) = self.shot_path.clone() {
            self.frame_count += 1;

            // Give the first frames time to compile and rasterize every
            // equation. The request is repeated because a viewport that is
            // not yet on screen drops it silently, and one lost request
            // means the app spins forever.
            if self.frame_count >= 30 && self.frame_count.is_multiple_of(30) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
            }
            ctx.request_repaint();

            let shot = ctx.input(|input| {
                input.events.iter().find_map(|event| match event {
                    egui::Event::Screenshot { image, .. } => Some(image.clone()),
                    _ => None,
                })
            });

            if let Some(image) = shot {
                let (width, height) = (image.width() as u32, image.height() as u32);
                match image::RgbaImage::from_raw(width, height, image.as_raw().to_vec()) {
                    Some(buffer) => match buffer.save(&path) {
                        Ok(()) => println!("wrote {path} ({width}x{height})"),
                        Err(error) => eprintln!("could not save screenshot: {error}"),
                    },
                    None => eprintln!("screenshot buffer had unexpected size"),
                }
                self.request_close();
            }
        }

        self.advance_close_handshake(ctx);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(PAPER).inner_margin(32.0))
            .show(ui, |ui| {
                let mut scroll_area = egui::ScrollArea::vertical().auto_shrink([false, false]);
                if let Some(offset) = self.scroll_offset {
                    scroll_area = scroll_area.vertical_scroll_offset(offset);
                }
                scroll_area.show(ui, |ui| {
                    // Centre the reading column, however wide the window is.
                    let slack = (ui.available_width() - COLUMN_WIDTH).max(0.0);
                    ui.horizontal(|ui| {
                        ui.add_space(slack / 2.0);
                        ui.vertical(|ui| {
                            ui.set_max_width(COLUMN_WIDTH);
                            self.draw_blocks(ui);
                        });
                    });
                });
            });

        self.export_button(&ui.ctx().clone());
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        let [r, g, b, a] = PAPER.to_normalized_gamma_f32();
        [r, g, b, a]
    }
}

/// Show `lesson` in a native window: fonts, theme, the reading column, the
/// export button, and the `MATH_ME_*` env-var harness the screenshot and
/// PDF-export tests drive.
///
/// # Errors
/// [`Error::Viewer`] if the window, GPU context, or fonts fail to start.
pub(crate) fn run(lesson: Lesson) -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(lesson.title())
            .with_inner_size(egui::vec2(880.0, 820.0)),
        ..Default::default()
    };

    eframe::run_native(
        "math-me",
        options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            setup_fonts(&cc.egui_ctx);
            setup_theme(&cc.egui_ctx);
            Ok(Box::new(MathApp {
                lesson,
                renderer: MathRenderer::new(),
                plots: HashMap::new(),
                shot_path: std::env::var("MATH_ME_SHOT").ok(),
                export_path: std::env::var("MATH_ME_EXPORT_PDF").ok(),
                scroll_offset: std::env::var("MATH_ME_SCROLL").ok().and_then(|value| value.parse().ok()),
                status: None,
                frame_count: 0,
                close_wanted: false,
                handshake: CloseHandshake::Open,
            }))
        }),
    )
    .map_err(|error| Error::Viewer { message: error.to_string() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_file_stem_lowercases_and_dashes_punctuation() {
        assert_eq!(sanitize_file_stem("GARCH: Part 1!"), "garch-part-1");
    }

    #[test]
    fn sanitize_file_stem_falls_back_when_nothing_alphanumeric_survives() {
        assert_eq!(sanitize_file_stem("!!!"), "lesson");
    }

    /// Lay a plot out through egui with no window and no GPU behind it, and
    /// give back every piece of text it drew.
    ///
    /// Laying out at all is most of the point: a plot with awkward data that
    /// panics here would otherwise panic in front of a reader.
    fn lay_out(plot: &Plot) -> Vec<String> {
        let context = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(900.0, 800.0))),
            ..Default::default()
        };
        let mut state = PlotState::new(plot);
        let output = context.run_ui(input, |ui| plot_block(ui, plot, 0, &mut state));

        assert!(!output.shapes.is_empty(), "the plot drew nothing at all: {plot:?}");
        output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect()
    }

    fn busy_plot() -> Plot {
        Plot::new(0.0..=10.0)
            .curve("cooling", "20 + 60 * exp(-k * x)")
            .param("k", 0.0..=1.0, 0.1)
            .scatter("readings", vec![[0.0, 80.0], [5.0, 56.0]])
            .line("share", vec![[0.0, 0.0], [10.0, 100.0]])
            .secondary()
            .hline(55.0)
            .vline(4.0)
            .x_label("minutes")
            .y_label("degrees")
            .y2_label("percent")
    }

    #[test]
    fn a_plot_using_everything_at_once_lays_out() {
        lay_out(&busy_plot());
    }

    /// The screen's counterpart to `pdf::tests::every_named_series_earns_a_
    /// legend_entry`: the two renderings agree on what a legend is, and this
    /// is the only check on the on-screen one that does not need a window.
    #[test]
    fn every_named_series_earns_a_legend_entry_and_the_axes_are_labelled() {
        let drawn = lay_out(&busy_plot());

        for text in ["cooling", "readings", "share", "degrees", "percent", "minutes"] {
            assert!(drawn.iter().any(|found| found == text), "{text:?} was not drawn: {drawn:?}");
        }
    }

    /// The right-hand axis is labelled by reading the left axis's ticks back
    /// through the linear fit, so its labels land wherever that arithmetic
    /// puts them. What they must not do is print more of the answer than the
    /// gap between them justifies, or run below a series that starts at zero
    /// — a percentage axis reading `98.62` down to `-9.07` is what this is.
    #[test]
    fn no_tick_is_labelled_past_the_precision_its_step_earns() {
        let drawn = lay_out(
            &Plot::new(0.0..=10.0)
                .line("degrees", vec![[0.0, 80.0], [10.0, 22.0]])
                .line("share", vec![[0.0, 0.0], [10.0, 91.0]])
                .secondary(),
        );

        for text in &drawn {
            let Ok(value) = text.parse::<f64>() else { continue };
            assert_eq!(value.fract(), 0.0, "{text:?} is finer than its step: {drawn:?}");
            assert!(value >= 0.0, "{text:?} sits under a series that starts at zero: {drawn:?}");
        }
    }

    #[test]
    fn plots_with_nothing_worth_framing_lay_out_too() {
        // Each of these leaves the frame degenerate somewhere, and egui_plot
        // panics in debug builds on bounds whose ends coincide.
        lay_out(&Plot::new(0.0..=1.0));
        lay_out(&Plot::from_points(vec![[3.0, 3.0]]));
        lay_out(&Plot::from_points(Vec::new()).hline(2.0));
        lay_out(&Plot::new(0.0..=1.0).curve("flat", "7"));
        lay_out(&Plot::new(-1.0..=1.0).curve("poles", "1 / x"));
        lay_out(&Plot::new(0.0..=1.0).curve("broken", "x +").curve("adrift", "x * nope"));
    }
}
