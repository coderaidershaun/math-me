---
name: lesson-critic
description: The lesson pipeline's enhancement lead — takes an existing, finished lessons/lesson-{slug}.rs and runs the whole pipeline again, harder, dispatching subagents at every step. Step 1 dispatches a fresh-eyes interrogation subagent that re-researches the topic from scratch and charges the first pass with everything it left shallow, missing, or disconnected; step 2 re-dispatches all six recommendation lenses (deepen, apply, cement, illustrate, graph, practice) in one read-only parallel batch primed with those findings, then the lead edits the binary; step 3 dispatches lesson-review. Better means deeper and more connected, never longer — the ~30 page budget still rules. Use whenever the user wants an existing lesson made better — "make the X lesson better/deeper", "the lesson didn't go far enough", "critique the X lesson", "second pass on X", "help me make more connections in X", "run lesson-critic". For building a new lesson or finishing a half-built one, use lesson-builder instead.
disable-model-invocation: true
user-invocable: true
---

# Lesson critic — the enhancement lead

You lead the second-generation pass over an existing lesson. This skill exists because first passes settle: they stop at plausible instead of bedrock, they present core ideas as a list instead of a web, and nobody ever went back to ask whether the job was actually done. Your working assumption is that **the first pass did not go deep enough** — the interrogation exists to test that assumption honestly, not to rubber-stamp it.

The shape mirrors lesson-builder: you dispatch steps 1 and 3 to subagents, and in step 2 the lenses report while **you edit the binary yourself**. There is still no scratch document, ever — the artifact is `lessons/lesson-{slug}.rs` throughout.

## Slug and baseline

Resolve the slug from the user's words **against the files that exist** — list `lessons/lesson-*.rs` and match. Never guess a slug into existence: if nothing matches, stop, show what does exist, and point the user at lesson-builder. From then on that one file is the only lesson file anyone touches; `Cargo.toml` is already registered and stays untouched.

Before dispatching anything, capture the baseline: `cargo build --bin lesson-{slug}` clean, `lesson.audit()` empty, page count recorded (commands in `.claude/skills/lesson-builder/ENGINE.md`). If the baseline gates fail, repair the build minimally yourself first — you cannot make enhancement judgments about a lesson that doesn't render. Note the starting page count; the final report compares against it.

## What "better" means — the enhancement contract

- **Deeper** — every core idea driven to bedrock that survives one more "why". Chains the lesson already has are suspects, not settled facts.
- **More connected** — the reason this skill exists is to build connections in the student's head. Four kinds, defined in `INTERROGATE.md`: internal (idea ↔ idea), cross-lesson (to the other binaries in `lessons/`), representational (the same idea shown a second way), and world (physical or financial anchor). Target state: **no core idea stands alone.**
- **Never longer.** ~30 rendered pages remains the law. The enhancement currency is `.explain` hover text (depth at zero page cost), sharper prose, and swapping weak content for strong — not addition. A pass that "improves" a lesson by growing it has failed.

## The three steps

| Step | Who | What | Gate |
|---|---|---|---|
| 1 | interrogation subagent | fresh-eyes read + second research pass → verdict, KEEP list, ranked findings | read-only, no files |
| 2 | six lens subagents, ONE parallel read-only batch → then **you** | lenses re-examine the lesson primed with the findings; you weigh all seven reports and edit | build clean · audit empty · budget held |
| 3 | `lesson-review` subagent | final pass; may cut | build clean · audit empty · budget held |

The lenses: `lesson-deepen` (still the vital one), `lesson-apply`, `lesson-cement`, `lesson-illustrate`, `lesson-graph`, `lesson-practice`. All read the same binary and write nothing. A lens that dies does not block the batch — proceed on the reports you have and say so. **The interrogation is different: it is the foundation of the whole pass. If it dies, re-dispatch it — never proceed on your own cold read instead.**

## Dispatch templates

Every dispatch: "Read `.claude/skills/{path}` and follow it completely. The lesson binary is `lessons/lesson-{slug}.rs`" — **path and slug filled in, never a literal `{slug}`**. Additions per step:

- **Step 1**: skill file is `.claude/skills/lesson-critic/INTERROGATE.md`. Add the user's request verbatim, and the baseline page count.
- **Step 2 (each lens)**: the lens's own SKILL.md, plus: "READ ONLY — write no files. This is a second, deeper pass: an interrogation has judged the first pass insufficient. Treat the lesson's existing treatment skeptically — re-examine it, don't route around it." Then the interrogation VERDICT plus the findings that touch that lens's territory (deepen gets SHALLOW and WRONG; apply and cement get world-DISCONNECTED and MISSING; illustrate and graph get representational-DISCONNECTED; practice gets whatever new material is likely to land). For deepen specifically, add: "Re-run why-why-why on the chains the lesson already contains and check each claimed bedrock survives one more why."
- **Step 3**: `lesson-review`'s SKILL.md, plus: "This is a second-generation lesson that has just been through an enhancement pass — the bar is higher than for a first build. The enhancements must survive your cut on merit, not seniority."

## Weighing and editing (step 2)

Read `.claude/skills/lesson-builder/ENGINE.md` before touching the file. Priority semantics as in lesson-builder: **high** = apply unless two highs conflict or the budget forbids; **medium** = weigh against the budget; **low** = drop freely — with two additions:

- Interrogation **WRONG** findings outrank everything; a factual error ships to a student's head.
- The **KEEP list is protected.** After your edit, re-read those blocks and confirm the lesson's existing strengths survived the churn — an enhancement pass that degrades what was already good is worse than no pass.

Make room by **cutting before compressing**, and integrate rather than paste-dump: placement, transitions, and trimming are yours. Rebuild and re-check pages after your pass. The page budget wins every tie.

## Verification and edges

After every step: `cargo build --bin lesson-{slug}` clean, `lesson.audit()` empty, page count within budget, and `git diff --stat` touches only `lessons/lesson-{slug}.rs`.

- Missing skill file → stop and name it; do not improvise the stage inline.
- Interrogation VERDICT says the first pass is genuinely good → believe it. Apply the few findings that exist, run steps 2–3 lightly, and report that the lesson largely held up. Manufacturing improvement is how good lessons get worse.
- A step that dies mid-flight → inspect the binary state, re-dispatch with what remains stated explicitly.

## Progress and final report

Relay progress as `[{slug}]`-prefixed one-liners at each step boundary. Final report is **before/after**: the interrogation's verdict on the first pass, page count at baseline and now, what changed per step (grouped), which connections were built, and what was deliberately left alone.
