---
name: lesson-research
description: Step 1 of the lesson pipeline — research a topic and write the lesson straight into a compiling math-me binary at lessons/lesson-{slug}.rs (registered as a [[bin]] in Cargo.toml, with a short //! module-comment header), around 30 rendered pages max, built around 3–7 core ideas, no scratch documents and no citations. Use when dispatched by lesson-builder, or when the user says "research X into a lesson", "write the lesson binary", "start a lesson on X".
disable-model-invocation: false
user-invocable: false
---

# Lesson research — straight into the binary

You are step 1 of the 3-step lesson pipeline. You research the topic and write the lesson **directly into a compiling Rust binary**. There is no scratch document, no advisor, no second draft waiting behind you — everything you write ships to steps 2 and 3, so write only what a student needs and make every sentence earn its place.

**Read `.claude/skills/lesson-builder/ENGINE.md` before writing a line of Rust.** It carries the API and the traps that cost earlier builds hours.

## Artifact contract

- The lesson is `lessons/lesson-{slug}.rs`, registered by **appending** one `[[bin]]` stanza to `Cargo.toml` with `name = "lesson-{slug}"`. Start by copying the shape of `lessons/template.rs`.
- At the top of the binary, a `//!` module comment of **≤15 lines** opening with `Prerequisites:` (what to run it, what to take it — never invent a prerequisite the lesson doesn't actually lean on), then the exact run command, which you have verified works.
- **Write no markdown files in `lessons/` — ever.** If something absolutely must be written as markdown, put it in `.scratch/` at the project root; otherwise it belongs in module comments in the rust file.
- **Never overwrite any `lessons/*.rs` you did not create this run.** The dispatch tells you the slug; that file is the only lesson file you touch.

## Research method

Research first, write second — but interleave: research one section deep, write it, build, move on. Cover, for the topic:

- **Definitions and notation** — every symbol the lesson uses, introduced before first use.
- **Rules, each with its why** — a rule stated without its reason is a fact the student must memorize; a rule with its reason is one they can rebuild.
- **Edge cases** — where the clean story bends (zero, negatives, limits, degenerate inputs).
- **Misconceptions** — what learners typically get wrong, stated as the tempting belief and why it fails.
- **Applications** — where this actually gets used, concretely.
- **History** — only where it genuinely illuminates; a dateline for its own sake is filler.

**Web-search anything you are not certain of BEFORE writing it.** Never fabricate a fact, a number, or an attribution. Then write what you verified as plain prose — **no sources section, no citations, no URLs anywhere in the lesson**. The verification happens; the paper trail doesn't ship.

## Core ideas — the spine

Distill **3–7 core ideas**, exactly one marked primary in your own planning. A core idea is the intuition that makes the rules inevitable, not a topic heading:

- Exponents: "repeated multiplication *starting from 1* — which is why $n^0 = 1$ is forced, not decreed."
- Logarithms: "a log *is* an exponent — the question '$b$ to what power gives $x$?' asked as a function."

The lesson is built around the core ideas: depth goes to them, breadth elsewhere gets trimmed first. If a section serves no core idea, it probably doesn't belong.

## The page budget

**Around 30 rendered pages, maximum.** Past that you are overdoing it — cut, don't compress the font. Nothing written for the sake of writing: no filler transitions, no recap sections, no encyclopedic completeness. Check with the PDF export command in ENGINE.md before reporting done. Depth that won't fit in prose goes into `.explain` hover text, which costs no pages.

## Writing into the engine

- Copy `lessons/template.rs` as the skeleton; keep its idioms.
- **Build after every few sections** (`cargo build --bin lesson-{slug}`) — never write the whole lesson and then compile.
- All math lives in `.math()` / `.display()` LaTeX; prose in `.text()`; respect the `$` / `\$` convention and the LaTeX substitutions from ENGINE.md.
- Give the primary formulas `.explain` hover coverage as you go — an audit full of `UnexplainedTerm` findings at the end is a debt someone must pay.

## Checklist before reporting

- [ ] `lessons/lesson-{slug}.rs` exists and `cargo build --bin lesson-{slug}` is clean
- [ ] `[[bin]]` stanza appended; nothing else in `Cargo.toml` touched
- [ ] `lesson.audit()` is empty (debug run does not fire the assert)
- [ ] PDF page count is ≤30
- [ ] `//!` module comment header present, ≤15 lines, run command verified; no markdown files written in `lessons/`
- [ ] 3–7 core ideas present, each rule carrying its why
- [ ] No citations, sources, or URLs anywhere in the lesson
- [ ] No pre-existing `lessons/*.rs` modified
- [ ] Report: page count, core ideas chosen (primary named), what you deliberately left out
