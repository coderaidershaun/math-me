//! A TEMPLATE, not a lesson: copy this file when starting a new lesson
//! binary. Its job is to show how to use the tool — every builder capability
//! a lesson needs, demonstrated once on one small, self-consistent topic —
//! and nothing else. The half-life content exists to make the demonstration
//! coherent, not to teach half-life; real lessons are carved by the lesson
//! pipeline (see the README) and land beside this file in `lessons/`.
//!
//! Covers: headings, a note, paragraphs mixing prose and inline math,
//! `explain`/`explain_char`, display maths, a rule, a figure with a
//! print-specific variant, and a plot with multiple tunable curves, a
//! scatter series, reference lines and a secondary axis.

use math_me::prelude::*;

fn main() -> math_me::Result<()> {
    let lesson = Lesson::builder("Half-Life")
        .heading("How fast does a radioactive sample decay?")
        .note("Hover any term to learn what it is telling you. Drag the slider under the plot and the curves redraw as you go.")
        .para(|p| p
            .text("A radioactive sample does not lose the same number of atoms every second. Each atom instead carries the same tiny chance of decaying in the next instant, so the count that has not yet decayed, ")
            .math(r"N(t)")
            .text(", falls by a fixed fraction of itself with every tick of time, whatever the sample started at."))
        .explain(r"N(t)", "Atoms remaining at time t",
            "How many atoms of the original sample have not yet decayed, at time t.")
        .para(|p| p
            .text("That constant fraction is set by the decay constant, ")
            .math(r"\lambda")
            .text(", how quickly the sample disappears in units of one over time. It turns the count remaining into a clean exponential:"))
        .explain_char('λ', "Greek small lambda",
            "The decay constant: how quickly the sample disappears, in units of 1/time. A larger λ makes the count fall away faster.")
        .display(r"N(t) = N_0 e^{-\lambda t}")
        .explain(r"N_0 e^{-\lambda t}", "The decay law",
            "The starting count N_0, shrunk by the fraction e^{-λt} still left after time t.")
        .para(|p| p
            .text("The moment exactly half the sample has decayed is called the half-life, ")
            .math(r"t_{1/2}")
            .text(", and every isotope has its own fixed value for it, however large the sample was to start with:"))
        .explain(r"t_{1/2}", "The half-life",
            "The time it takes for exactly half of any sample of this isotope to decay, whatever size it started at.")
        .display(r"t_{1/2} = \frac{\ln 2}{\lambda}")
        .explain(r"\frac{\ln 2}{\lambda}", "Half-life from the decay constant",
            "The constant ln 2, about 0.693, divided by the decay rate: a faster-decaying isotope — a larger λ — has a shorter half-life.")
        .rule()
        .heading("Watch it decay")
        .para(|p| p.text("Two things that are not equations, to see how far the same window can be pushed: a drawing, and a plot of the law this page has already defined."))
        .figure(Figure::new(ATOM_SVG, "A helium-4 nucleus with its two electrons, drawn as plain SVG shapes.")
            .print_svg(ATOM_SVG_PRINT)
            .width_percent(30))
        .plot(Plot::new(0.0..=20.0)
            .curve("sample A", "exp(-lambda * x)")
            .curve("sample B", "exp(-0.25 * x)")
            .scatter("measured counts", readings())
            .curve("percent decayed", "100 * (1 - exp(-lambda * x))").secondary()
            .param("lambda", 0.02..=0.50, 0.10)
            .hline(HALF_REMAINING)
            .vline(HALF_LIFE)
            .x_label("days since the sample was measured")
            .y_label("fraction of atoms remaining")
            .y2_label("percent decayed")
            .height(280.0)
            .caption("Drag λ and sample A's curve compresses or stretches; sample B never moves. The dashed lines mark half remaining and sample A's half-life at the slider's starting rate — the measured counts were taken at that rate, so they drift off the curve as soon as you move it."))
        .build();

    // The audit call belongs inside the assert: left outside it, a release
    // build would still compile every formula and then throw the answer away.
    debug_assert!(
        lesson.audit().is_empty(),
        "math errors, unexplained terms or unusable curves: {:?}",
        lesson.audit()
    );

    lesson.show()
}

/// A helium-4 nucleus with two electron orbits, drawn as plain ellipses and
/// circles — small enough to write by hand, unlike the equations above.
const ATOM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120" width="120" height="120">
  <g fill="none" stroke="#E8E6E3" stroke-width="3">
    <ellipse cx="60" cy="60" rx="50" ry="20"/>
    <ellipse cx="60" cy="60" rx="50" ry="20" transform="rotate(60 60 60)"/>
    <ellipse cx="60" cy="60" rx="50" ry="20" transform="rotate(120 60 60)"/>
  </g>
  <circle cx="60" cy="60" r="8" fill="#4ADE80"/>
  <circle cx="108" cy="60" r="4" fill="#E8E6E3"/>
</svg>"##;

/// [`ATOM_SVG`] repainted for white paper: light strokes and the accent green
/// swap for ink and a print-safe green, the same trade the PDF needs for any
/// screen-tuned artwork.
const ATOM_SVG_PRINT: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 120" width="120" height="120">
  <g fill="none" stroke="#1F1F1F" stroke-width="3">
    <ellipse cx="60" cy="60" rx="50" ry="20"/>
    <ellipse cx="60" cy="60" rx="50" ry="20" transform="rotate(60 60 60)"/>
    <ellipse cx="60" cy="60" rx="50" ry="20" transform="rotate(120 60 60)"/>
  </g>
  <circle cx="60" cy="60" r="8" fill="#16A34A"/>
  <circle cx="108" cy="60" r="4" fill="#1F1F1F"/>
</svg>"##;

/// Half of the sample remaining: the plot's horizontal reference line.
const HALF_REMAINING: f64 = 0.5;

/// Sample A's half-life at the slider's starting rate, ln(2) / 0.10.
const HALF_LIFE: f64 = 6.93;

/// Measured counts for sample A, taken while the slider sat at its starting
/// rate of 0.10.
fn readings() -> Vec<[f64; 2]> {
    vec![
        [0.0, 1.00],
        [2.0, 0.82],
        [5.0, 0.61],
        [10.0, 0.37],
        [15.0, 0.22],
        [20.0, 0.14],
    ]
}
