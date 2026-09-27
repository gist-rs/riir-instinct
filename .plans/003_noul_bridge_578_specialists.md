# Plan 003 — the noul bridge: seat the Issue 578 specialists (typed_decisions + prompt_injections)

**Status:** PLANNED — filed 2026-09-28. Consumer half of riir-train Issue
578 T3/T4; train-side tasks T1/T2 are DONE (Bench 610/611 —
`typed_decisions_armA_v1.bin` `6f644ccfaf7f7eaf`,
`prompt_injections_armA_v1.bin` `ee0b4eb4336299b2`, both still carrying
their `armA` names deliberately: **the rename is THIS plan's act** —
measured 2026-09-28, a bare rename dies the arena at the suite's shape
gate, `run_suite_n`'s "one non-noul question per case" assertion; the
file was renamed back within the hour, no half-state left).

## Why the gate refuses today

Both 578 suites are **NOUL-shaped** — one yes/no judgment question per
case (prompt_injections: 116 × 1; typed_decisions: 400 cases × 5
questions, 2000 questions total — the multi-question axis is the second
gap). The specialist bridge is defined only over NON-noul single-question
cases, in THREE gated places:

1. `arena.rs::run_suite_n` — the shape-law error (dies the whole suite).
2. `server.rs::from_parts` — the same shape guard (dies the serve boot).
3. `SuiteCtx::fill_positions` — reads `criteria` keys; a noul question's
   criteria is `Null`, so there is no presented-option space to map.

The 578 T2 note already prepared the design's anchor: *"the noul label
spelling is unified (`no`/`yes`) across both suites so ONE consumer
bridge rule serves both"* — the artifacts' label universes ARE the
presented-option space for a noul question.

## The design (one rule, two suites, three gates)

**The noul bridge rule**: for a `QKind::Noul` question, the presented
options are the artifact's own label universe filtered to the seat's
labels (`[no, yes]`, order = the artifact's label order filtered —
deterministic, never the string sort); every option resolves through the
existing `key_map` BY NAME (both are artifact labels — the map already
carries them, including the artifact-known/seat-unknown sentinel path);
the specialist scores both class rows; argmax/`scores_classes_into`
work verbatim. The bridge's two existing rules (all-named → by name;
count == seat labels → by index; else refuse) gain a THIRD arm: noul →
the no/yes rule. Nothing about the non-noul paths moves.

Per-question rows: `eval_a0` already zips questions (typed's A0 row is
per-question today — 2000 rows); `eval_h1`/`eval_a1_h2` must do the same
(zip `case.questions` × `se.cases[ci]`, per-question bag + gold +
positions), replacing the "first question only" reading and its
per-question == per-case comment. The per-question `class_of_pos` map is
the same no/yes pair for every question of these suites.

Gate/face semantics to settle IN the plan before coding (the decisions
that make this a plan, not a patch):
- H1 cascade per question: reflex's fused gate abstains per question
  already (`qo.abstained`); the consult rate becomes per-question —
  disclose the convention change (typed's n=2000 questions vs n=400
  cases — the A0 row already speaks per-question; the hybrid rows join
  that convention).
- The G1 calibration face: per-question confidences (the specialist's
  sigmoid on noul questions — is the fused-gate floor comparison honest
  at 2 classes? Record the face, judge on read).
- The serve side: `from_parts`' shape guard relaxes to "one question per
  case, non-noul OR noul-with-the-bridge" (noul suites serve through the
  SAME `decide()` — its synthesized case carries the suite's own noul
  template and its bridge resolve already handles all-named options).
- **Serve arity**: prompt_injections has 2 labels — `AnySuiteServer`'s
  dispatch has S3..S77, NO S2. Add `S2` (SuiteServer::<2>) + the arena's
  N=2 instantiation (the arena seats it today as A0-only — verify which
  N it instantiates and match). typed_decisions: 3 labels → S3 exists.

## Tasks

- [ ] T1 — the noul bridge rule in `SuiteCtx::fill_positions` (and the
      serve `decide`'s equivalent resolve arm) + the per-question
      eval_h1/eval_a1_h2 zip. Structure-pinned tests: a toy noul case
      scores both class rows; a typed-shaped 2-question case yields
      per-question rows; the non-noul paths byte-identical.
- [ ] T2 — relax the two shape gates (arena + serve) to admit noul
      single-kind cases; the refusal text updated. The gates must STILL
      refuse a mixed-kind or zero-question case loud.
- [ ] T3 — S2 dispatch (serve `AnySuiteServer` + the arena's N=2 path).
- [ ] T4 — the rename (train-side `data/instinct_specialists/`):
      `prompt_injections_armA_v1.bin` → `prompt_injections_winner_v1.bin`
      and `typed_decisions_armA_v1.bin` → `typed_decisions_winner_v1.bin`
      (byte-identical copies; the armA files stay). ONE commit with T1–T3
      — the 610 lesson: the rename without the bridge dies the arena.
- [ ] T5 — the frozen reads (two arena runs; records
      `.benchmarks/013_prompt_injections_specialist` +
      `.benchmarks/014_typed_decisions_specialist`): gates per the
      standing law — T2 paired LB95 (the advertising row), best-measured
      serves. prompt_injections' n=116 support disclosed (the 578 math:
      the LB95 bar ≈ +2 pt over the mean delta at this n). The serve-side
      follow-through where a hybrid arm serves: the manifest gains rows
      (the 6-row pin in `serve_gates` grows with it, both directions),
      `deploy.yaml` ships the two winners, and the site publish doc
      (`build_hybrid_doc.py`) re-runs.
- [ ] T6 — records: instinct Bench rows + AGENTS current-state +
      HISTORY; cite the consumer runs in riir-train Issue 578 T3/T4 and
      close it per the noise-reduction rule.

## Honest risks

- typed_decisions' published reflex row is the OC-ARMED posture (0.4655,
  selected_scale 2.0) — the train-side Arm A read 0.5375 workflow-level
  on ITS holdout; the transfer at per-question granularity is the open
  question the frozen read answers.
- prompt_injections at n=116: a refusal is the likely honest verdict
  unless the specialist's edge clears ~+2 pt + the mean delta — a
  refusal is a publishable answer (the 578 T2 clause), never a fished
  pass.
- The hoard gate + centroids: unchanged (count bags — both suites are
  v1-recipe artifacts; `winner_bridge` gains NO new convention rows).
