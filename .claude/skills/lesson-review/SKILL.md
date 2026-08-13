---
name: lesson-review
description: Step 3 of the lesson pipeline — the final reviewer: experience lessons/lesson-{slug}.rs as a student would, then edit the binary directly wherever the content is lacking, holding the ~30 page budget and the build and audit gates, inventing nothing. Use when dispatched by lesson-builder, or when the user says "review the lesson", "polish the lesson", "final pass".
disable-model-invocation: false
user-invocable: false
---

# Review — the final pass

You are step 3 of the 3-step lesson pipeline, and unlike the lenses you **edit `lessons/lesson-{slug}.rs` directly**. You are the last eyes before the student's. **Read `.claude/skills/lesson-builder/ENGINE.md` before touching the file.**

## Experience it first

Before editing anything: capture it with `MATH_ME_SHOT=<path> cargo run --release --bin lesson-{slug}` and look at the screenshot, then read the source top to bottom **once, as a student would**. The first read is the only one where confusion is still visible to you — note every stumble as you hit it, because on the second read you'll know too much to feel it.

## What to improve

- Explanations that **restate instead of explain** — they collapse under one "why".
- Flat prose where the idea deserves a beat — and the opposite: devices stacked so densely no single one lands.
- Captions that decorate instead of telling the reader what to see.
- Slider defaults parked at the boring end of the range, where dragging appears to do nothing.
- Missing `.explain` hover coverage on primary formulas (the audit lists these).
- SVGs that crop, mislabel, or assert a shape the numbers don't support.
- **Overdone material — CUT it.** You are the one agent licensed to cut. The ~30 page budget is a hard ceiling; if the lesson is over, cutting is not optional, and the weakest whole section beats a trim of every section.

## Hard rules

- **Never fabricate** a fact, number, or claim. Improvements rephrase, reorder, tighten, and cut — verify anything new before it goes in.
- No sources, citations, or URLs.
- `cargo build --bin lesson-{slug}` clean and `lesson.audit()` empty **after every editing session** — not once at the end.
- Preserve the engine conventions (the `$` / `\$` rule, the LaTeX substitutions) from ENGINE.md.

## Report

What you changed and why (grouped, not a diff dump), what you cut, the final page count, and both gates' status.
