# The math-me engine — shared reference

Read this before writing or editing any `lessons/lesson-{slug}.rs`. It is the single home for the API quick reference, the known engine traps, and the gates. Source of truth beyond it: `lessons/template.rs` (the pattern every lesson copies), `src/lesson.rs`, `src/plot.rs`. When a new trap is discovered, append it here — nowhere else.

## API quick reference

```rust
use math_me::prelude::*;

let lesson = Lesson::builder("Title")
    .heading("Section")                       // top-level section heading
    .note("aside")                            // callout box
    .para(|p| p.text("prose ").math("x^2"))   // mixed prose + inline math
    .display(r"e^{i\pi} + 1 = 0")             // display equation
    .rule()                                   // horizontal rule
    .figure(Figure::new(svg_str, "caption"))  // SVG figure
    .plot(Plot::new(0.0..=10.0) /* … */)      // interactive plot
    .explain(r"\pi", "pi", "circle constant") // hover text for a term
    .explain_char('e', "e", "Euler's number") // hover text for a single char
    .build();

debug_assert!(lesson.audit().is_empty());     // quality gate — see below
lesson.show()                                 // opens the viewer
```

- Blocks render in chain order. `.explain` / `.explain_char` attach hover text to terms wherever they appear — **hover text is where depth lives without costing pages**: a definition, a why, a subtlety the prose cannot afford.
- `Figure`: `Figure::new(svg, caption)` · `.print_svg(svg)` (light/PDF variant) · `.width_percent(u32)`.
- `Plot`: `Plot::new(x_range)` · `.curve(name, "expr")` (expressions read param names) · `.scatter(name, vec![[x,y],…])` · `.secondary()` (bind previous curve to y2) · `.param(name, range, default)` (live slider) · `.hline(y)` / `.vline(x)` · `.x_label`/`.y_label`/`.y2_label` · `.height(f64)` · `.caption(&str)`.
- Env harness: `MATH_ME_SHOT=<path>` screenshot-and-exit (the cheap way to see the lesson) · `MATH_ME_EXPORT_PDF=<path>` · `MATH_ME_SCROLL=<points>`.

## The traps — symptom → cause → fix

1. **LaTeX compiles to "unknown variable" errors.** `src/formula.rs::document()` emits no mitex prelude, so `\sqrt`, `pmatrix` (all matrix environments), `\operatorname`, `\tfrac`, `\text{...}`, `\underbrace`, `\big` are undefined Typst variables. Fix: write `x^{1/2}` instead of `\sqrt x`, `\mathrm` instead of `\operatorname`/`\text`, `\frac` instead of `\tfrac`; describe matrices in prose or split into scalar equations.
2. **A bare `\ln` with no argument panics inside `terms.rs`** — an engine crash, not a clean error. Never emit `\ln` without an argument; in prose, write the word "ln".
3. **Plot param names take alphanumerics and `_` only** (`expr.rs::is_name_continuation`). A name with a space or dot fails **silently**: the slider renders but no curve reads it, and the audit reports it as *unused* rather than broken. Use `log10_q`, never `log10 q`.
4. **`.text()` prose renders literally.** Currency in prose is a bare `$` (a `\$` shows the backslash on screen); `\$` belongs only inside math fragments.
5. **The viewer lays out the whole document up front.** A long lesson takes 15+ minutes to open — the ~30 page budget is also the performance fix. Never sit waiting on a full interactive render while iterating; use `MATH_ME_SHOT` for a one-off look.
6. **Two gates, different jobs.** `cargo build --bin lesson-{slug}` clean is the existence gate — never report done without it. `lesson.audit()` empty is the quality gate (uncompilable math, unexplained terms, unparsable plot expressions, unread params); it fires the `debug_assert!` on a debug run, so a lesson that only builds in release is not finished.

## Working discipline

- **Build early and often** — compile after every few sections. Writing two thousand lines and then building is how a day disappears into error archaeology.
- `[[bin]]` registration is a one-stanza **append** to `Cargo.toml` — never a rewrite of the file.
- Page count check: `MATH_ME_EXPORT_PDF=/tmp/l.pdf cargo run --release --bin lesson-{slug}` then `mdls -name kMDItemNumberOfPages /tmp/l.pdf` (macOS). Around 30 pages is the ceiling, not a target.
