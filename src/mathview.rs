//! A compiled formula on screen: the egui widget that draws it, lights up the
//! term under the pointer, explains that term in a tooltip, and offers the
//! source back through a right-click menu.
//!
//! Everything above the pixels — the Typst compile, the glyph geometry, the
//! grouping into terms — happens in [`crate::formula`] and [`crate::terms`];
//! this module is the only place that knows about egui points.

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash as _, Hasher as _};
use std::sync::Arc;

use eframe::egui::{self, Color32};

use crate::formula::{self, RenderedMath};
use crate::glossary::Glossary;

/// usvg parses the SVG's `width="Npt"` at 96dpi, so one Typst point becomes
/// 4/3 egui points on screen. This factor belongs at the egui layer only —
/// every glyph rectangle is stored in Typst page points.
const PT_TO_POINTS: f32 = 4.0 / 3.0;

/// The green used for the hovered character, tooltips and debug outlines.
pub(crate) const HIGHLIGHT: Color32 = Color32::from_rgb(0x4A, 0xDE, 0x80);

/// The red an inline `[… error: …]` label is set in, wherever the page has to
/// admit that something did not compile.
pub(crate) const ERROR_INK: Color32 = Color32::from_rgb(255, 100, 100);

/// The width a tooltip wraps its text at, in egui points. The viewer puts this
/// into `spacing.tooltip_width`, where egui reads it as the widest a tooltip
/// may be.
pub(crate) const TOOLTIP_MAX_WIDTH: f32 = 420.0;

/// Floor under the tooltip's width.
///
/// egui otherwise shrinks a tooltip to its content, so a short entry such as
/// "Equals sign" got a box a fraction of the width of the paragraph shown for
/// the term beside it, and the box resized under the reader as the pointer
/// crossed a formula. Must not exceed [`TOOLTIP_MAX_WIDTH`], or the floor wins
/// everywhere and the wrapping width stops meaning anything.
const TOOLTIP_MIN_WIDTH: f32 = 400.0;
const _: () = assert!(TOOLTIP_MIN_WIDTH <= TOOLTIP_MAX_WIDTH);

/// Compiles each formula once and then draws it, hover behaviour included.
#[derive(Default)]
pub(crate) struct MathRenderer {
    cache: HashMap<(String, bool), Result<Arc<RenderedMath>, String>>,
    /// Set by `MATH_ME_DEBUG_BOXES=1`: outline every hit box for screenshots.
    debug_boxes: bool,
}

impl MathRenderer {
    pub(crate) fn new() -> Self {
        Self {
            cache: HashMap::new(),
            debug_boxes: std::env::var("MATH_ME_DEBUG_BOXES").is_ok_and(|value| value != "0"),
        }
    }

    /// The `(latex, display)` cache key leaves the glossary out on purpose:
    /// one `MathRenderer` serves one lesson for its whole life, so its
    /// glossary never changes underneath a cached compile.
    fn get(&mut self, latex: &str, display: bool, glossary: &Glossary) -> Result<Arc<RenderedMath>, String> {
        self.cache
            .entry((latex.to_owned(), display))
            .or_insert_with(|| formula::compile(latex, display, glossary).map(Arc::new))
            .clone()
    }

    pub(crate) fn show(&mut self, ui: &mut egui::Ui, latex: &str, display: bool, glossary: &Glossary) {
        let rendered = match self.get(latex, display, glossary) {
            Ok(rendered) => rendered,
            Err(error) => {
                ui.colored_label(ERROR_INK, format!("[math error: {error}]"));
                return;
            }
        };

        // Compute the size ourselves rather than asking the image: a second
        // size hint would rasterize the SVG twice every frame.
        let size = rendered.page_size_pt * PT_TO_POINTS;
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
        // The click sense is only there for the context menu and the hit test.
        // A formula is not a button, so the pointer stays an ordinary arrow.
        let response = response.on_hover_cursor(egui::CursorIcon::Default);
        let scale = if rendered.page_size_pt.x > 0.0 {
            rect.width() / rendered.page_size_pt.x
        } else {
            PT_TO_POINTS
        };
        let on_screen = |rect_pt: egui::Rect| {
            egui::Rect::from_min_size(rect.min + rect_pt.min.to_vec2() * scale, rect_pt.size() * scale)
        };

        // A glyph is the sharper target, so it is asked first; the term's whole
        // box only settles what no glyph owns, such as a fraction bar or the
        // space under an overline.
        let hovered = response.hover_pos().and_then(|pointer| {
            let touches = |rect_pt: egui::Rect| on_screen(rect_pt).expand(1.5).contains(pointer);
            rendered
                .terms
                .iter()
                .find(|term| term.glyphs.iter().any(|glyph| touches(glyph.rect_pt)))
                .or_else(|| rendered.terms.iter().find(|term| touches(term.rect_pt)))
        });

        let mut hasher = DefaultHasher::new();
        (latex, display).hash(&mut hasher);
        let key = hasher.finish();
        let uri = format!("bytes://math-{key:016x}.svg");
        egui::Image::from_bytes(uri, Arc::clone(&rendered.svg)).paint_at(ui, rect);

        // Repaint the green copy over the top, clipped to the hovered term, so
        // only that term changes colour. egui holds one clip rect at a time, so
        // a term that cannot be clipped in one piece is painted once per
        // rectangle; every paint uses the same widget rect, so the drawings land
        // on exactly the same pixels.
        if let Some(term) = hovered {
            let outside = ui.clip_rect();
            for clip in &term.highlight_pt {
                ui.set_clip_rect(on_screen(*clip).expand(1.0).intersect(outside));
                egui::Image::from_bytes(
                    format!("bytes://math-{key:016x}-hover.svg"),
                    Arc::clone(&rendered.svg_hover),
                )
                // The green copy is only rasterized on the first hover; a
                // spinner in the hole left by one term would be worse than
                // nothing.
                .show_loading_spinner(false)
                .paint_at(ui, rect);
            }
            ui.set_clip_rect(outside);
        }

        if self.debug_boxes {
            for term in &rendered.terms {
                ui.painter().rect_stroke(
                    on_screen(term.rect_pt),
                    0,
                    egui::Stroke::new(0.7, HIGHLIGHT),
                    egui::StrokeKind::Outside,
                );
            }
        }

        if let Some(term) = hovered {
            let description = term.describe(glossary);
            egui::Tooltip::always_open(
                ui.ctx().clone(),
                ui.layer_id(),
                response.id,
                egui::PopupAnchor::Pointer,
            )
            .gap(12.0)
            .show(|ui| {
                ui.set_min_width(TOOLTIP_MIN_WIDTH);
                // Headline: the term's own SVG cropped and enlarged, since a
                // glyph-concat string loses fractions and subscripts.
                let term_pt = term.rect_pt;
                if term_pt.is_positive() {
                    let scale =
                        (2.0 * PT_TO_POINTS).min((TOOLTIP_MIN_WIDTH - 24.0) / term_pt.width());
                    let (headline, _) =
                        ui.allocate_exact_size(term_pt.size() * scale, egui::Sense::hover());
                    let outside = ui.clip_rect();
                    ui.set_clip_rect(headline.intersect(outside));
                    egui::Image::from_bytes(
                        format!("bytes://math-{key:016x}-zoom.svg"),
                        Arc::clone(&rendered.svg_hover),
                    )
                    .show_loading_spinner(false)
                    .paint_at(
                        ui,
                        egui::Rect::from_min_size(
                            headline.min - term_pt.min.to_vec2() * scale,
                            rendered.page_size_pt * scale,
                        ),
                    );
                    ui.set_clip_rect(outside);
                } else {
                    ui.label(
                        egui::RichText::new(&description.display)
                            .size(20.0)
                            .color(HIGHLIGHT),
                    );
                }
                ui.add_space(6.0);
                ui.label(egui::RichText::new(&description.name).strong());
                ui.add_space(2.0);
                ui.label(&description.meaning);
            });
        }

        response.context_menu(|ui| {
            if ui.button("Copy LaTeX").clicked() {
                ui.ctx().copy_text(rendered.latex.clone());
                ui.close();
            }
            if ui.button("Copy Typst").clicked() {
                ui.ctx().copy_text(rendered.typst.clone());
                ui.close();
            }
        });
    }
}
