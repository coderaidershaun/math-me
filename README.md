# math-me

Agent-authored, interactive math lessons: a Rust library for building them, and
a nine-stage agent pipeline for writing them.

## Quick start

Run the canonical template lesson — every builder capability, one small topic
(radioactive half-life), ~120 lines:

```sh
cargo run --example template
```

Add `--release` for a much faster first paint (the Typst compile is
unoptimised in debug). Close the window when done. `examples/template.rs` is
a template in the literal sense — it exists to show how to use the tool, one
demonstration of every builder capability, and it is the file to copy when
starting a new lesson binary. Real lessons land beside it in `examples/`.

## How to use this: the lesson pipeline

Lessons are not written by hand — they are carved by an agent pipeline. In a
Claude Code session in this repo there is exactly one entry point:

```
/lesson-builder
```

Tell it what you want to learn ("build me a lesson on log returns", "teach me
GARCH properly"). It clarifies the request if genuinely needed, then acts as
team lead: nine stages, each delegated to a subagent, all working one shared
buffer at `.scratch/lesson.md` that only ever grows until composition.

| # | Stage | What it does |
|---|-------|--------------|
| 1 | `lesson-research` | Exhaustive facts, rules with their whys, history, audience, distilled core ideas (intuitions, not rules) |
| 2 | `lesson-deepen` | Why-why-why / how-how-how chains, 3–5 levels, to labelled bedrock (definition / axiom / convention / empirical fact) |
| 3 | `lesson-apply` | Physical-world pictures + financial time series equations, toy numbers — math topics only |
| 4 | `lesson-cement` | Curated stories, analogies, imagine-invitations (a four-lens panel, then a judge) |
| 5 | `lesson-illustrate` | Actual SVG source with computed-honest coordinates, placement directives |
| 6 | `lesson-graph` | At most two graph specs — static or interactive with sliders — described, not built |
| 7 | `lesson-practice` | Worked → faded → independent questions plus misconception diagnostics |
| 8 | `lesson-compose` | The ordered lesson script, twice: draft, then a cold-read refinement |
| 9 | `lesson-make` | The only Rust stage: builds `examples/{slug}.rs` + its README with this library, then polishes |

Stages 5–7 run in parallel; everything else is sequential. Stages 1–8 produce
only the document; stage 9 turns it into a binary you run exactly like the
template above, with its own README (prerequisites first, then the run
command) beside it in `examples/`. The finished lessons in `examples/` are the
durable library; `.scratch/lesson.md` is per-topic scratch.

## The library

A library for authoring a math lesson in short, readable builder-style Rust:
headings, notes, paragraphs mixing prose and inline maths, display maths,
hoverable term explanations, figures, plots, an on-screen viewer, and
Typst-native PDF export. A lesson is written once, against the pipeline

```
LaTeX string  ->  mitex  ->  Typst math  ->  typst-svg  ->  egui::Image
```

and the same source compiles both what the viewer shows on screen and what
`typst-pdf` prints on paper. Everything is pure Rust — no TeX install, no
external binaries.

A [`Lesson`](src/lesson.rs) is data, not code: the builder produces an owned,
serializable struct — strings, vecs, floats, no closures — so a tutoring
engine can compile a lesson once, [`Lesson::save`](src/lesson.rs) it, and any
viewer built on this library can [`Lesson::load`](src/lesson.rs) and replay it
without recompiling.

## Example

The shape of a lesson binary (`examples/template.rs` is the live version of
this shape, and the file to copy):

```rust
use math_me::prelude::*;

fn main() -> math_me::Result<()> {
    let lesson = Lesson::builder("GARCH")
        .heading("What is a GARCH model?")
        .note("Hover any term to learn what it is telling you.")
        .para(|p| p
            .text("Write the return as a mean term plus a shock, ")
            .math(r"r_t = \mu + \varepsilon_t")
            .text(", and decompose the shock into a scale and an innovation."))
        .display(r"\sigma_t^2 = \omega + \alpha \varepsilon_{t-1}^2 + \beta \sigma_{t-1}^2")
        .explain(r"\alpha \varepsilon_{t-1}^2", "The ARCH term",
            "The reaction: a large shock yesterday raises today's variance.")
        .figure(Figure::new(CAT_SVG, "A stress test.").width_percent(45))
        .plot(Plot::new(-5.0..=5.0)
            .curve("today's variance", "ω + α * x^2 + β * 5.0")
            .param("ω", 0.0..=0.2, 0.05)
            .param("α", 0.0..=0.3, 0.09)
            .param("β", 0.5..=1.0, 0.90)
            .hline(5.0)
            .x_label("yesterday's shock ε")
            .caption("The news-impact curve. Drag α and β."))
        .build();

    debug_assert!(lesson.audit().is_empty(), "{:?}", lesson.audit());
    lesson.show()
}
```

## Features

- **Hover a term to learn what it is telling you.** Not a character: telling
  a reader what a left parenthesis is helps nobody, so the hoverable unit is
  the piece a mathematician would name. `ℓ(θ)` lights up whole and is "the
  log-likelihood"; `α ε²_{t−1}` is "the ARCH term"; `ω / (1 − α − β)` is one
  thing, fraction bar and all. Operators stand between terms as units of
  their own, and a lone symbol still gets a character-level entry.
- **Graphs the reader can retune.** A plot's curves are written as expressions
  in `x` with named parameters; each parameter gets a slider under the graph,
  and every curve is re-evaluated as the reader drags it. Alongside them a plot
  takes fixed lines, scatter series, reference lines and a second y-axis. See
  [Plots](#plots).
- **`audit()` as the coverage gate.** Compiles every formula in a lesson and
  parses every plot curve, then reports a formula that fails to convert, a term
  nothing curated explains, a tooltip that would print a character egui cannot
  draw, a curve that would not be drawn, or a slider that would do nothing —
  the thing a reader would notice, surfaced before anyone opens the viewer.
- **Right-click a formula to copy it.** "Copy LaTeX" gives back the source
  that was written; "Copy Typst" gives the converted Typst.
- **Export PDF**, from the icon at top right (hover shows "Export PDF"), or
  `Lesson::export_pdf` directly. Not a screenshot: the maths pipeline is already Typst, and
  Typst's native output is PDF, so the whole lesson is rebuilt as one Typst
  document and compiled with `typst-pdf` — selectable text, embedded fonts,
  vector maths and vector figures, set for print (black on white, A4,
  justified, page numbers) rather than screen.
- **Save and load.** `Lesson::save`/`Lesson::load` round-trip a lesson
  through JSON, so it can be compiled once and replayed by any viewer built
  on this library without recompiling.

## Commands

Run the canonical template:

```sh
cargo run --example template
```

Run the test suite (unit tests and doctests):

```sh
cargo test
```

Release build (much faster first paint, since the Typst compile is
unoptimised in debug):

```sh
cargo run --example template --release
```

Lint:

```sh
cargo clippy --all-targets -- -D warnings
```

## Environment variables

A lesson author's binary or example inherits this harness for free through
[`Lesson::show`](src/lesson.rs) — useful for screenshotting or exporting a
lesson without watching the window or clicking the button.

| Variable | Effect |
| --- | --- |
| `MATH_ME_SHOT=<path>` | Screenshot the window to `<path>` and exit. |
| `MATH_ME_EXPORT_PDF=<path>` | Export the PDF to `<path>` and exit. Combines with `MATH_ME_SHOT`, which then decides when to quit. |
| `MATH_ME_SCROLL=<points>` | Park the page at a scroll offset, so a screenshot can catch something below the fold. |
| `MATH_ME_DEBUG_BOXES=1` | Outline every term box in green. This is how the hover geometry is verified without a pointer: the outlines must fall where a mathematician would draw them, and must tile the formula with no glyph left out. |

## Fonts

Three fonts ship in `assets/` and are embedded at compile time, so system
fonts are never scanned and rendering is identical on any machine. All three
are under the [SIL Open Font License 1.1](https://openfontlicense.org/):

- **Atkinson Hyperlegible** (Braille Institute of America) — the body font,
  designed to maximise the distinction between characters.
- **Atkinson Hyperlegible Bold** — headings.
- **STIX Two Math** (SIL / STI Pub) — the maths font, handed both to Typst
  (via `TypstEngine::fonts`) and to egui as a fallback family, so tooltips
  can print the very characters Typst drew.

New Computer Modern Math still comes along via `typst-kit`'s
`embedded-fonts` feature and acts as the fallback for anything STIX lacks.

## Module layout

- `src/lib.rs` — crate root: module declarations, the public re-exports, and
  `prelude`.
- `src/lesson.rs` — `Lesson`, `LessonBuilder`, `Block`, `Inline`,
  `ParaBuilder`, `Figure`, `AuditFinding`: the model an author builds and
  `Lesson::audit`.
- `src/plot.rs` — `Plot` and its builder: the surface a lesson author writes
  against, and nothing else.
- `src/plot/resolve.rs` — the step that turns a plot plus a set of parameter
  values into the geometry both renderers draw: expressions evaluated, gaps
  cut, the second axis folded into the first, series coloured from the palette.
- `src/plot/axis.rs` — the numbers along the edges: where ticks fall, how many
  decimals they are worth, and the linear map a right-hand axis is faked with.
- `src/expr.rs` — the expression language a tunable curve is written in: a
  recursive-descent parser and its evaluator.
- `src/glossary.rs` — the describe-fallback chain behind `.explain()`: a
  lesson's own entries, then the built-in dictionary, then a description
  built from a term's parts.
- `src/error.rs` — the crate's `Error` and `Result`.
- `src/formula.rs` — LaTeX → mitex → Typst → SVG compilation, and the glyph
  geometry that makes a formula hoverable. Draws nothing, so the PDF export
  and `audit()` use it without touching egui.
- `src/mathview.rs` — the egui widget that puts a compiled formula on screen:
  hover highlight, tooltip, copy menu.
- `src/terms.rs` — grouping a compiled formula's glyphs into the terms a
  reader would name, from Typst's own syntax tree.
- `src/symbols.rs` — Unicode maths-variant normalisation, and the generic
  built-in character dictionary `Glossary` falls back to.
- `src/viewer.rs` — the on-screen eframe/egui app: fonts, theme, the reading
  column, the export button, the `MATH_ME_*` harness.
- `src/pdf.rs` — `Lesson` → one Typst document → PDF.
- `examples/template.rs` — THE template: how to use the tool, one
  demonstration of every builder capability, the file to copy for a new
  lesson. Lessons the pipeline builds land beside it.

## Authoring lessons

- **`.explain(latex, name, meaning)`** teaches the glossary what a formula
  fragment means. Write the fragment the way it is written in the formula,
  term for term; the library keys it by the same structural rules a compiled
  term is keyed by, so a `\left(...\right)` spelling and a plain-paren
  spelling of the same fragment always resolve to one entry, and no author
  ever sees a Typst key. **`.explain_char(ch, name, meaning)`** does the same
  for a single character, for terms of one glyph a lesson never explains as a
  whole term. An entry only earns its place if it says something the built-in
  dictionary cannot — check `src/symbols.rs` before writing one that repeats
  it.
- **`Lesson::audit()` is the coverage gate.** It compiles every formula in
  the lesson and reports anything a reader would notice: a formula that
  fails to convert, a term with no curated entry, or a tooltip that would
  print a character egui cannot draw. An agent-authored lesson should assert
  `audit().is_empty()` before calling `.show()`.
- **Figures need `print_svg` for paper.** `Figure::new` takes the SVG the
  viewer draws; if that artwork is tuned to the screen's off-white-on-black
  palette, give `.print_svg(...)` a repainted variant for the PDF, or the
  export embeds the screen colours on white paper.
- **Plots get their own section**, below — the one part of the API with more
  than a handful of methods.

## Plots

A `Plot` is one or more series over a shared x domain, rendered twice from the
same numbers: `egui_plot` on screen, a generated SVG in the PDF. Series are
coloured from a fixed five-colour palette in declaration order — each colour a
pair, one value tuned to glow on the viewer's near-black page and one several
shades darker for white paper — and any series given a name earns a legend
entry in both renderings. Pass `""` where a plot's single curve needs no name.

```rust
Plot::new(0.0..=30.0)
    .curve("paper cup", "room + (poured - room) * exp(-k * x)")   // tunable
    .line_fn("ideal", |x| 20.0 + 60.0 * (-0.05 * x).exp())        // sampled closure
    .line("modelled", vec![[0.0, 80.0], [30.0, 24.0]])            // literal points
    .scatter("readings", readings)                                // markers, no line
    .curve("gap closed", "100 * (1 - exp(-k * x))").secondary()   // right-hand axis
    .param("poured", 70.0..=95.0, 85.0)
    .param("room", 15.0..=25.0, 20.0)
    .param("k", 0.01..=0.30, 0.08)
    .hline(55.0)
    .vline(7.7)
    .x_label("minutes since it was poured")
    .y_label("degrees Celsius")
    .y2_label("% of the gap closed")
    .caption("Drag the sliders.")
```

`.secondary()` moves the series added last onto the right-hand axis, which
carries its own scale and `y2_label` — write it as a suffix on the line that
adds that series, as above. The axis exists only for the series moved onto it,
so a `y2_label` with no `.secondary()` anywhere is a label that cannot be
drawn, and `Lesson::audit()` reports it as an
`AuditFinding::UnusedSecondaryLabel`. `Plot::from_fn(range, f)` and
`Plot::from_points(points)` are the short forms for a plot of one unnamed
fixed curve.

`examples/template.rs` demonstrates this plot machinery in a complete,
runnable lesson binary.

### Tunable graphs

`.curve(name, expression)` takes its maths as a string rather than a closure,
because a lesson is data: a closure cannot be saved, reloaded and replayed.
Each `.param(name, range, default)` puts a slider under the graph, and every
expression curve on that plot is re-evaluated as the reader drags it. The PDF
freezes the parameters at their defaults, paper having no sliders to drag.

The expression language is arithmetic, not a program:

| | |
| --- | --- |
| Free variable | `x` — the horizontal position, swept across the plot's domain. |
| Other names | Must be declared by `.param`. Letters (Greek included), digits and `_`, not starting with a digit. Never `x`: the free variable wins that lookup, so a parameter of that name is a slider nothing reads. |
| Operators | `+ - * /`, `^` for powers (right-associative, so `2^3^2` is `2^9`), unary `-`, and parentheses. `-x^2` is `-(x^2)`. |
| Multiplication | Always written out. `2x` and `3(x + 1)` are errors, not products — say `2 * x` and `3 * (x + 1)`. |
| Functions | `sin cos tan exp ln log sqrt abs floor ceil` take one argument; `pow min max` take two. `ln` is the natural log, `log` is base 10. |
| Numbers | `2`, `1.5`, `.5`, `1e-3`. |

There is no assignment, no comparison and no control flow, so an expression is
always a number and always terminates. Three things a curve can get wrong:

- **It does not parse, or it reads a name nothing declares.** `Lesson::audit()`
  reports an `AuditFinding::ExpressionError` or `AuditFinding::UndeclaredParameter`
  carrying the offending expression, and on screen the plot draws an inline
  `[plot error: …]` label in place of that curve. The other curves are
  unaffected, and nothing panics.
- **A parameter is declared that no curve reads**, leaving a slider that moves
  and changes nothing. `Lesson::audit()` reports an
  `AuditFinding::UnusedParameter` naming it — including the `x` case above,
  where the slider is shadowed by the free variable rather than misspelled.
- **A sample comes out non-finite** — a pole, `ln` of a negative. That point is
  dropped and the line picks up after it, so the curve shows a gap rather than
  a spike to infinity.

## Notes and gotchas

- **The Typst page margin must stay above zero.** At `0pt` the page is
  cropped to the glyph box and subscripts get clipped — `r_t` renders as
  `r.`.
- **One Typst point is 4/3 egui points.** usvg reads the SVG's `width="Npt"`
  at 96dpi. Glyph boxes are stored in Typst page points and only scaled at
  the egui layer; mixing the two spaces is the fastest way to get hit boxes
  that are offset or the wrong size.
- **Typst sets maths variables in the Mathematical Alphanumeric Symbols
  block.** Italic `𝑥` is U+1D465, not `x`, so `symbols::normalize` folds
  that whole block (plus the letterlike singletons such as `ℎ`) back to
  plain characters before any dictionary lookup.
- **mitex escapes every parenthesis**, writing `(a + b)` as `\(a + b\)`, so
  Typst never sees a delimited group and the tree has five loose siblings
  where a reader sees one bracket. `terms` pairs the delimiters up itself —
  including the case where the closing bracket is buried as the *base* of an
  attachment, which is how `(α + β)^{h−1}` parses. `\left`/`\right` compiles
  to an `lr(...)` wrapper on top of that same escaped pair, which is why
  keying a `.explain()` fragment has to walk the identical structural path
  `terms::partition` uses rather than keying the raw converted text: without
  it, `\left(...\right)` and plain parens around the same fragment would key
  differently and the entry would silently never match.
- **Some glyphs have no span of their own.** Content the Typst library
  generates rather than the source does — the letters of `ln`, the base of
  an accent — arrives detached, and borrows the range of the next glyph
  that has one. That lands it in the right term, because every such case
  sits inside a term already, but it is not enough to read structure from: a
  borrowed glyph is ignored when deciding what a piece of source draws.
- **The hover colour is a second copy of the SVG, not a second compile.**
  `recolour` string-replaces the ink hex in the `fill` and `stroke`
  attributes `typst_svg` writes, and hovering paints that green copy over
  the normal one clipped to the hovered term. Strokes matter as much as
  fills: a fraction bar is a stroked path, and skipping it draws a grey
  line through the middle of a highlighted fraction.
- **A term is clipped in one piece where that is safe.** egui holds one clip
  rect at a time, and the union of a term's glyph boxes is the useful clip:
  it catches the ink no glyph owns, such as a fraction bar or a macron.
  The union is only used when no glyph of another term overlaps it;
  otherwise the glyph boxes are painted one at a time, which is exact but
  loses the bars.
- Inline maths is vertically centred against the text row rather than
  sharing its baseline. It reads fine, but proper baseline alignment would
  mean extracting the baseline offset from Typst.
- A formula that fails to convert shows a red `[math error: ...]` label in
  place of the equation instead of panicking, both on screen and (as an
  `Error::MathCompile`) in `Lesson::audit()`.
- **Term tooltips spell the maths out by hand.** egui does no Unicode
  shaping, so a tooltip cannot print a real subscript or an overbar — the
  built-in dictionary and any lesson's own entries write `σ_t²` and `σ̄²`
  the long way, and `symbols::is_unrenderable` is what `Lesson::audit()`
  scans a lesson's curated text with to keep an unrenderable character from
  creeping back in.
- **The PDF drops the hint line.** "Hover any term…" is advice a sheet of
  paper cannot honour, so `Block::Note` is the one block the print
  rendering skips.
- **Typst resolves an SVG's fonts by name against the document's font book,
  and generic families are not supported.** `font-family="sans-serif"` in
  an embedded SVG silently finds nothing, so `pdf.rs`'s generated plot SVG
  names "Atkinson Hyperlegible" and "STIX Two Math" outright — the latter
  for axis titles that end in a Greek symbol, since Atkinson has no Greek.
- **Neither renderer has a real second y-axis, so both fake the same one.**
  `egui_plot` gives a plot one coordinate space however many axis strips are
  drawn on it, and the generated SVG has one frame. `Plot::resolve` therefore
  stretches every secondary series onto the primary range and hands back the
  linear map that reads a tick back out in the series' own units; each renderer
  plots one coordinate space and labels the right-hand strip through that map.
  It is exact for the linear scales this library draws. On screen it also means
  the plot's bounds have to be set explicitly on every frame — a frame
  `egui_plot` chose for itself would put the right-hand labels somewhere else,
  and a tunable curve moves its own bounds as the reader drags. Setting them
  inside the plot's own closure is what makes that safe: `egui_plot` applies
  the bounds a frame asks for before it lays out either axis strip, so the
  curve and the labels can never come from different frames.
- **The right-hand axis is rounded, not padded.** Its series is framed against
  round numbers (0..100 for a percentage that reaches 91) rather than its data
  ±10%, which is what stops a percentage axis from being labelled down to
  `-9.07`. Its labels are then written to the precision the gap between them
  earns — two significant digits of the step — so mapping a left-hand tick
  through the fit prints `99`, not `98.62`. In the PDF, where the tick
  positions are ours, the right-hand ticks are laid out in the second axis's
  own units instead of relabelling the left's gridlines, so both sides read in
  round numbers; on screen `egui_plot` owns tick placement, so the right-hand
  labels land wherever the left's ticks map to.
- **The legend is drawn above the frame, in both renderings.** `egui_plot`'s
  own legend floats in a corner of the plot, and these plots cannot be panned
  or zoomed, so a legend that lands on a curve stays on it permanently. Both
  renderers therefore lay out their own row of swatches above the frame
  instead.
- **`serde_json` needs its `float_roundtrip` feature here.** Without it floats
  are parsed to best-effort precision, and a plot parameter saved and reloaded
  comes back a bit-width off what was written. A save/load round-trip property
  test is what originally caught it.
- The first build pulls in the whole Typst and eframe dependency trees and
  takes a few minutes. Builds after that are seconds.
