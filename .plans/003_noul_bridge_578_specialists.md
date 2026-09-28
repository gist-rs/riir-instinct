# Plan 003 — the noul bridge: seat the Issue 578 specialists (typed_decisions + prompt_injections)

**Status:** LANDED 2026-09-28 — Benches 013 + 014. prompt_injections
serves the T2-certified A1 (0.8534 vs 0.7672, paired LB95 +0.0082 > 0 —
the plan's "likely refusal" risk inverted); typed_decisions' artifact is
UNSEATABLE (the context seat's coverage gate: 100/500 cases present keys
with no class row — the train pool never carried the reflex test
templates) → a0_stands with the reason; the retrain is filed as riir-train
Issue 581. Consumer half of riir-train Issue 578 T3/T4; train-side tasks
T1/T2 were DONE (Bench 610/611 — `typed_decisions_armA_v1.bin`
`6f644ccfaf7f7eaf`, `prompt_injections_armA_v1.bin` `ee0b4eb4336299b2`,
both carried their `armA` names until this plan's T4 rename). The
"measured 2026-09-28, a bare rename dies the arena" note below was the
pre-bridge state; the bridge landed the same day.

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

- [x] T1 — the noul bridge rule in `SuiteCtx::fill_positions` (and the
      serve `decide`'s equivalent resolve arm) + the per-question
      eval_h1/eval_a1_h2 zip. Structure-pinned tests: a toy noul case
      scores both class rows; a typed-shaped 2-question case yields
      per-question rows; the non-noul paths byte-identical. DONE
      (`1e9f1b1`): arena noul-by-name via the key map, serve
      positional-always (`noul_suite_serves_positionally_through_the_bridge`
      — three presentation spellings, one pick; the toy typed-shaped
      per-question pin rides the 014 A0-per-question convention and the
      banking77 byte-identity control), banking77 re-run == the 012
      predictions byte-identically.
- [x] T2 — relax the two shape gates (arena + serve) to admit noul
      single-kind cases; the refusal text updated. The gates must STILL
      refuse a mixed-kind or zero-question case loud. DONE (`1e9f1b1`):
      arena = ≥1-question-per-case (zero-question refuses); serve = one
      question per case (a multi-question set has no single serving kind
      — typed stays arena-side), noul admitted.
- [x] T3 — S2 dispatch (serve `AnySuiteServer` + the arena's N=2 path).
      DONE (`1e9f1b1`); the plan's "typed_decisions: 3 labels" guess was
      stale — measured 4 workflow labels (the arena's `::<4>` and `S4`
      already existed; the seat labels turned out to be CONTEXT, see T1'
      s context join in `ed592ba`).
- [x] T4 — the rename (train-side `data/instinct_specialists/`, gitignored
      — weights never committed; the files are runtime data):
      `prompt_injections_armA_v1.bin` → `prompt_injections_winner_v1.bin`
      and `typed_decisions_armA_v1.bin` → `typed_decisions_winner_v1.bin`
      (byte-identical copies, sha256-verified; the armA files stay). ONE
      window with T1–T3 — the 610 lesson held: the bridge landed first,
      the renames second, the arena never died.
- [x] T5 — the frozen reads + records: **Bench 013**
      (`.benchmarks/013_prompt_injections_specialist/`) — A1 0.8534 vs A0
      0.7672, T2 PASS (LB95 +0.0082 > 0, the second certified suite
      ever), G1 disclosed FAIL; **Bench 014**
      (`.benchmarks/014_typed_decisions_specialist/`) — the typed
      artifact UNSEATABLE (the context coverage gate: 100/500 cases, 9
      missing keys, all security_incidents) → a0_stands with the reason,
      A0 0.4655 re-pinned site ✓. Serve follow-through: the manifest's
      SEVENTH row (prompt = A1, digest = Bench 611's mint),
      `PINNED_MANIFEST_DIGEST` re-pinned, the posture table 6→7 with
      `typed_decisions` asserted ABSENT, the prompt parity test reads
      the 013 record, `deploy.yaml` ships the prompt winner + dataset
      dir (typed ships nowhere), the merged lane doc for the reflex-site
      session at `014_…/hybrid_lane_doc.json`.
- [x] T6 — records: instinct Bench rows + AGENTS current-state + this
      plan; the consumer runs cited in riir-train Issue 578 T3/T4 and
      the issue closed per the noise-reduction rule (record in
      HISTORY.md); the typed retrain filed as riir-train Issue 581.

## Honest risks (resolved by the reads)

- typed_decisions' transfer: REFUTED before accuracy could even be
  measured — the artifact is unseatable (coverage gap, not accuracy). The
  train-side holdout edge (+7 pt over a train-pool split) was real but
  never faced the reflex test templates.
- prompt_injections at n=116: the plan's "a refusal is the likely honest
  verdict" inverted — the specialist's +8.6 pt mean cleared the ~+2 pt
  LB95 bar with room; T2-certified.
- The hoard gate + centroids: unchanged (count bags — both suites are
  v1-recipe artifacts; `winner_bridge` gained NO new convention rows),
  as planned.
