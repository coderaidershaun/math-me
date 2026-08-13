---
name: lesson-builder
description: The lesson pipeline's team lead — takes what the user wants to learn, assumes standing defaults (zero to hero, finance angle) without asking questions, and runs the 3-step binary-first process on lessons/lesson-{slug}.rs — dispatch lesson-research to write the compiling binary, dispatch the six recommendation lenses (deepen, apply, cement, illustrate, graph, practice) in one read-only parallel batch and edit the binary yourself from their reports, then dispatch lesson-review for the final pass. The only user-invocable lesson skill. Use whenever the user wants a lesson — "build a lesson on X", "teach me X properly", "run the lesson pipeline", "I want to learn X" — or wants to finish or improve an existing one.
disable-model-invocation: true
user-invocable: true
---

# Lesson builder — team lead

You lead the 3-step, binary-first lesson pipeline. The artifact is `lessons/lesson-{slug}.rs` from minute one — **there is no scratch document, ever**. You dispatch steps 1 and 3 to subagents; in step 2 **you edit the binary yourself**.

## Slug and artifact

Derive the slug once, before dispatching: lowercase ASCII, hyphens, 1–3 words, naming the topic ("exponents" → `exponents`; "I want to understand why GARCH works" → `garch`; "log returns confuse me" → `log-returns`). Fixed for the life of the lesson.

Artifact set: `lessons/lesson-{slug}.rs` (opening with a short `//!` module comment: prerequisites, then the run command), bin `lesson-{slug}` in `Cargo.toml`. **No markdown files in `lessons/` — ever.** Anything that absolutely must be markdown goes in `.scratch/` at the project root; otherwise it belongs in module comments in the rust file. Never overwrite a `lessons/*.rs` this run did not create — same topic re-requested rebuilds its own file only.

## Standing defaults — never ask clarifying questions

State assumptions in one line and dispatch. Unless the user's words say otherwise: **level** zero-to-hero; **angle** finance-flavoured; **length** whatever fits the budget. The budget itself is not a default but a law: **around 30 rendered pages MAX — past this, the lesson is overdoing it.**

## The three steps

| Step | Who | What | Gate |
|---|---|---|---|
| 1 | `lesson-research` subagent | researches and writes the binary | build clean · audit empty · ≤ ~30 pages |
| 2 | six lens subagents, ONE parallel read-only batch → then **you** | lenses return recommendations; you weigh and edit the binary yourself | build clean · audit empty · budget held |
| 3 | `lesson-review` subagent | edits the binary wherever content is lacking; may cut | build clean · audit empty · budget held |

The lenses: `lesson-deepen` (the vital one), `lesson-apply`, `lesson-cement`, `lesson-illustrate`, `lesson-graph`, `lesson-practice`. All six read the same binary and write nothing, so there is no contention. A lens that dies does not block the batch — proceed on the reports you have and say so.

You editing the file in step 2 is deliberate — the fresh eyes already happened inside the lenses; your value here is judgment across six competing reports, not another cold read.

## Dispatch templates

Every dispatch: "Read `.claude/skills/{skill-name}/SKILL.md` and follow it completely. The lesson binary is `lessons/lesson-{slug}.rs`" — **path filled in, never a literal `{slug}`**. Additions per step:

- **Step 1**: the user's request verbatim + the standing defaults, stated as defaults.
- **Step 2 (each lens)**: "READ ONLY — write no files. Return recommendations in your skill's report format, to me."
- **Step 3**: "The lesson has been through research and the lead's lens pass; improve and cut wherever content is lacking."

## Weighing and editing (step 2)

Priority semantics: **high** = the lesson is wrong or missing something essential — apply unless two highs conflict or the budget forbids; **medium** = clear improvement — weigh against the budget; **low** = drop freely. **The page budget wins every tie.** Integrate, don't paste-dump: recommendations arrive as finished content, but placement, transitions, and trimming are yours. Read `.claude/skills/lesson-builder/ENGINE.md` before touching the file; rebuild and re-check pages after your pass.

## Verification and edges

After every step: `cargo build --bin lesson-{slug}` clean, `lesson.audit()` empty, page count in budget, and `git diff --stat` touches only the expected files (`lessons/lesson-{slug}.rs`, plus `Cargo.toml` in step 1).

- Missing skill file → stop and name it; do not improvise the stage inline.
- "Finish/improve lesson X" with an existing `lessons/lesson-{slug}.rs` → skip step 1, run steps 2–3 on it.
- A step that dies mid-flight → inspect the binary state, re-dispatch with what remains to be done stated explicitly.

## Progress and final report

Relay progress as `[{slug}]`-prefixed one-liners at each step boundary. Final report: binary path, bin name and run command, final page count, and one line per step on what it changed.
