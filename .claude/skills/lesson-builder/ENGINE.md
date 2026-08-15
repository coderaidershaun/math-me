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
    .vector('x')                              // x is a vector: every occurrence rendered bold
    .matrix('A')                              // A is a matrix: same bold — capitals only
    .build();

debug_assert!(lesson.audit().is_empty());     // quality gate — see below
lesson.show()                                 // opens the viewer
```

- Blocks render in chain order. `.explain` / `.explain_char` attach hover text to terms wherever they appear — **hover text is where depth lives without costing pages**: a definition, a why, a subtlety the prose cannot afford.
- `.vector(ch)` / `.matrix(ch)`: declare that a letter names a whole column or block — every occurrence, in every formula, renders bold (in the viewer and the PDF). `ch` must be lower-case for a vector, a capital for a matrix, or `lesson.audit()` reports `VectorNotLowercase`/`MatrixNotCapital`.
- `Figure`: `Figure::new(svg, caption)` · `.print_svg(svg)` (light/PDF variant) · `.width_percent(u32)`.
- `Plot`: `Plot::new(x_range)` · `.curve(name, "expr")` (expressions read param names) · `.scatter(name, vec![[x,y],…])` · `.secondary()` (bind previous curve to y2) · `.param(name, range, default)` (live slider) · `.hline(y)` / `.vline(x)` · `.x_label`/`.y_label`/`.y2_label` · `.height(f64)` · `.caption(&str)`.
- Env harness: `MATH_ME_SHOT=<path>` screenshot-and-exit (the cheap way to see the lesson) · `MATH_ME_EXPORT_PDF=<path>` · `MATH_ME_SCROLL=<points>`.

## The traps — symptom → cause → fix

1. **LaTeX compiles to "unknown variable" errors.** `src/formula.rs::document()` emits no mitex prelude, so `\sqrt`, `pmatrix` (all matrix environments), `\operatorname`, `\tfrac`, `\text{...}`, `\underbrace`, `\big` are undefined Typst variables. Fix: write `x^{1/2}` instead of `\sqrt x`, `\mathrm` instead of `\operatorname`/`\text`, `\frac` instead of `\tfrac`; describe matrices in prose or split into scalar equations.
2. **A bare operator name with no argument panics inside `terms.rs`** — an engine crash, not a clean error, and the message is the unrelated-looking `terms must tile the formula`. Applies to every named operator (`\ln`, `\sin`, `\cos`, `\tan`, `\arctan`, …): the letters carry no span of their own, so with nothing following to borrow one from, the partition claims no glyphs. Never emit an operator name without an argument; in prose, write the word out.
3. **Plot param names take alphanumerics and `_` only** (`expr.rs::is_name_continuation`). A name with a space or dot fails **silently**: the slider renders but no curve reads it, and the audit reports it as *unused* rather than broken. Use `log10_q`, never `log10 q`.
4. **`.text()` prose renders literally.** Currency in prose is a bare `$` (a `\$` shows the backslash on screen); `\$` belongs only inside math fragments.
5. **The viewer lays out the whole document up front.** A long lesson takes 15+ minutes to open — the ~30 page budget is also the performance fix. Never sit waiting on a full interactive render while iterating; use `MATH_ME_SHOT` for a one-off look. And never check the audit gate through a **debug** `MATH_ME_SHOT` — on a plot-heavy lesson the unoptimized viewer layout can run for an hour. A debug `MATH_ME_EXPORT_PDF` run fires the identical `debug_assert!` in seconds; take screenshots in release.
6. **`\mathbf` is not in the mitex prelude** — it fails with "unknown variable: mitexmathbf" — and it is not wanted anyway. To bold a vector or matrix name, write the plain letter in the LaTeX and declare it with `.vector(ch)` / `.matrix(ch)`; the engine bolds every occurrence itself, in both the viewer and the PDF.
7. **Two gates, different jobs.** `cargo build --bin lesson-{slug}` clean is the existence gate — never report done without it. `lesson.audit()` empty is the quality gate (uncompilable math, unexplained terms, unparsable plot expressions, unread params); it fires the `debug_assert!` on a debug run, so a lesson that only builds in release is not finished.
8. **Page counting lies three ways.** `mdls` returns nothing on unindexed temp dirs; grepping the PDF for `/Type /Page` counts **double** (Typst emits it in the page tree and again in the tagged-structure tree); and the PDF drops every `.note()` (`src/pdf.rs` ignores them), so it undercounts real reading length. Count with `pdfinfo <file> | grep Pages`.
9. **Spacing commands are swallowed into the next term's key.** `\quad`, `\:` and friends carry no glyph, so `terms.rs` folds them into the term that follows: `L\cos\theta \quad L\sin\theta` keys as `quad L sin theta`, and the `.explain` written against the bare fragment never matches — an `UnexplainedTerm` you cannot fix by explaining harder. Worse, a phrase between two fragments (`\quad \mathrm{and} \quad`) glues the whole line into **one** term. Separate side-by-side fragments with a plain `,` and no spacing command; put the connecting word in the surrounding `.text()` prose instead.
10. **Two curves that coincide at their default parameters render as one.** The legend still lists both, so the reader sees a named series that is not on the plot. Choose slider defaults that hold a tunable curve away from any fixed reference — which doubles as the better lesson, since the reader then has something to drag it onto.

## Working discipline

- **Build early and often** — compile after every few sections. Writing two thousand lines and then building is how a day disappears into error archaeology.
- `[[bin]]` registration is a one-stanza **append** to `Cargo.toml` — never a rewrite of the file.
- Page count check: `MATH_ME_EXPORT_PDF=/tmp/l.pdf cargo run --release --bin lesson-{slug}` then `pdfinfo /tmp/l.pdf | grep Pages` (see trap 8 before trusting any other counter). Around 30 pages is the ceiling, not a target.
