# Bench 005 — the hybrid lane over EVERY dataset suite (Issue 010 T1/T2/T3 + T5's arena half)

**Status:** MEASURED — the single frozen test read; 2026-09-27, M3 (AC, charged).

## What ran

The arena population extended from the six specialist suites to **every reflex
dataset suite** (eight): `typed_decisions` + `prompt_injections` join at the
**A0/G0-only posture** — no winner artifact exists, so no A1/H1/H2 arms are
constructed (`run_suite_a0_only`; a PRESENT-but-corrupt artifact stays fatal,
only a missing file degrades to `a0_stands`). The other six run the full arm
grid exactly as Bench 004.

A0 arms are flattened **per question** (reflex's own hard-metrics convention;
the old per-case rows are byte-identical for the six one-question-per-case
suites). `typed_decisions`: 2000 questions over 400 cases (5 mixed
choice/score/noul questions per case); latency stays per-case and `n_cases`
is disclosed beside `n_questions`.

## The pin caught a real posture gap — oc on typed_decisions

First run: the A0 identity pin FAILED on typed_decisions — arena 0.3300 vs
the PUBLISHED site row 0.4655. The published row (reflex-site `data/bench.json`,
sha `8028a10`, 2026-09-24) carries `oc_selection: selected_scale 2.0`: the
option-conditioned lane (reflex issue 038 T7b) is ARMED in the published
posture for typed_decisions — the only suite whose train rows carry
per-question gold events. The arena's knobs had oc off (its old comment said
"typed_decisions, its only armed suite upstream, is not an arena suite" —
stale the day typed_decisions joined).

Re-baseline (Issue 008 T1's law — seat at the CURRENT published posture):
`oc_select: on` in the arena knobs AND the pin's reflex `run()`. The
selection runs per suite and declines byte-identically on the other seven
(no gold events → "baseline posture holds, byte-identical" — reflex's own
decline law). The oc lane needs the `option_cond` dep feature — already
enabled (reflex issue 050's nb_ridge compile gap).

Second run: **pin 8/8 green** — arena A0 == reflex `run()` == published site
rows on every suite:

| suite | A0 (= published) | questions/cases | abstain | verdict |
|---|---|---|---|---|
| ag_news | 0.8825 | 400/400 | 0.490 | a0_stands — H2 refused (LB95 −0.0100) |
| emotion | 0.8850 | 400/400 | 0.463 | a0_stands — instrument picked A0 outright |
| sst5 | 0.3967 | 600/600 | 0.532 | a0_stands — A1 refused (LB95 −0.0129) |
| xnli_en | 0.5233 | 300/300 | 0.533 | a0_stands — instrument picked A0 outright |
| massive_intent_en | 0.7800 | 300/300 | 0.313 | **hybrid_arm — H2(1,2,8) LB95 +0.0124** |
| banking77 | 0.8260 | 500/500 | 0.468 | a0_stands — H1 refused (LB95 −0.0466) |
| typed_decisions | 0.4655 | 2000/400 | 0.602 | a0_stands — no specialist |
| prompt_injections | 0.7672 | 116/116 | 0.681 | a0_stands — no specialist |

The six reproduce Bench 004 byte-for-byte (same registered arms, same
superiority numbers) — **the sold set is still exactly massive H2**. The two
new rows' abstain rates match the published `raw_abstain` (0.602 / 0.681).

## Lane doc: three states (Issue 010 T3)

`scripts/build_hybrid_doc.py` now emits per-suite
`verdict: hybrid_arm | a0_stands`, with `hybrid` (the registered cell, today's
shape) or `measured_a0` (the A0 arm's cell) + `reason` — the bare
`skipped_suites_a0_registered` name list is gone (a name list cannot carry a
measurement; the site rendered xnli "not run" while it WAS measured — root
cause 3). Never-seated suites are simply absent. Self-test: 6 fixtures incl.
the measured-A0 known-answer arm.

## Box state

The run's own capture (RESULTS.md): power AC · powermode high · load1m 6.26
(over the quotable ceiling — `refusals: ["load 6.26 > 6 — a sibling job is on
the box"]`) · swap 1882 MB. Concurrent sibling agents were active in
riir-reflex (docs) and riir-ai during the run — the pin is internal-
consistency (same binary both sides), so accuracy rows are load-independent;
the latency cells are NOT quotable under this load and are published as
record-only. Reflex at `56e3ddd` (its plan-003 Phase 1 Thai-pins commit —
tests-only); its openthai comparison-lane flag added the `openthai: false`
RunOptions row this pin carries.

## What did NOT run

- The harness decision-point families + `code_fixtures`: the reflex seat
  refuses them (upstream companion issue reflex 049) — Issue 010 T6.
- The reflex-site half (card three-state render + republish): the checkout
  is outside this session's workspace roots — Issue 010 T4/T5's site half,
  for the session holding that checkout.
