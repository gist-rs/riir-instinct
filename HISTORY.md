# HISTORY — riir-instinct

## 2026-09-27 — Bench 002: the aligned Bench-052-protocol read + the reflex-site hybrid lane publish (Issue 003 T4 closed)

The site publish's comparability blocker is discharged with a measurement,
not an argument. The arena re-ran at the **Bench-052 protocol on the same
`datasets_t20k` bytes** the published lanes carry (the 4090 T5 run verified
977/977), and the comparability proof is **A0 == the published 052 modelless
rows 6/6** (ag_news 0.8825, emotion 0.7375, sst5 0.3967, xnli 0.5233,
massive 0.7800, banking77 0.8260) plus the in-run xnli drift pin. Record:
`.benchmarks/002_hybrid_052_protocol/` (ALIGNMENT.md carries the table,
the box state, and the gate notes).

Registered arms at the aligned protocol: ag_news H2(0.25,2,2) 0.8975
(G1 FAIL — the raw fused readout is already calibrated at 0.0133, the
Platt refit hurt; the raw readout stands), emotion A1 0.8550, sst5 A1
0.4217 (both G1+G3 PASS), massive H2(1,2,8) 0.8267 vs A0 0.7800 (+4.7
pt — the v2 "flips from A0 0.42" story was the old first-N sample's
unrepresentative 30-of-60 label prefix), banking77 H1 0.8060 @ 46.8%
consult (G3 FAIL — H1 pays up to ~4.7 pt at 95% confidence; no
promotable hybrid arm, A0 stands), xnli A0 stands. The v2 numbers are
superseded as the publishable record.

**The bridge gap the aligned protocol surfaced** (and v2 could not, on
the old bytes): the t20k massive test split carries 59 of the artifact's
60 intents (`cooking_query` has zero test rows), so the train-derived
cal front legitimately presents an artifact-known, seat-unknown option —
the Issue-006 bridge refused that shape and the run died at case 0. The
`cooking_query` panic's first diagnosis was WRONG (the seat's test cases
are clean; a python replication of the sampler "proved" it) — the
offending case was CAL-side, invisible to any test-split probe. Resolution:
`fill_positions`/`key_map` admit artifact-only labels (sentinel seat
index), and `prior_fusion_pick` gained the NaN-no-evidence mark — the
margin term mutes to 0 (the prior stands) and the option is never a
rival in the best/second scan; pinned by
`h2_nan_evidence_mutes_the_margin_and_is_never_a_rival` (the first test
draft's expectation was also wrong twice — the second-known rival is
the rival, and f32 `exp` rounding put the honest tolerance at 1e-6).

**The reflex-site publish** (`instinct (hybrid)` lane): publish_bench
gained the `hybrid` lane class (merge carry with the device-variant
skip, LANE_DISPLAY, the wholesale-replace inventory, both rename
surfaces), the bench page gained the lane in table/filter/charts + the
explainer + a magenta palette slot, and the det cell went three-state
(a lane that does not claim a repeat check renders "—", never a lying
✗). `scripts/build_hybrid_doc.py` packages the frozen read (registered
arm per suite; A0-registered suites honestly absent — xnli carries no
hybrid lane) with the reflex metric laws re-derived exactly, including
the `cooking_query`-class cal-front extension in the arena. Site tests
22/22 incl. the new hybrid case; bench_page + chart_render smokes PASS.
Published from `hybrid_lane_doc.json`; `lane_sources.git_sha` names the
landing commit.

## 2026-09-27 — Issue 006 RESOLVED: the arena's position-vs-index instrument defect (Bench 001 v2)

Fix `3a12a70`. The v1 massive anomaly (A1 0.0267 vs Bench 609's .670) was
the ARENA, never the specialist. Two instrument bugs, one class:

1. **Space mismatch**: A1/H1/H2 scored and picked in the seat's LABEL
   space (`perm` is label→class) while A0's probs and every `gold.idx`
   speak the question's PRESENTED-option space; the arena compared them
   directly. Expected ≈ 1/59 ≈ .017 — the observed .0150/.0267. Masked on
   every full-universe suite (identity); only massive's 20-of-59 sampled
   + shuffled options could expose it.
2. **Double indirect**: `cands[rank0_sorted[select_arm(..)]]` —
   `select_arm` already returns the candidate index. v1's banking77
   "A0 stands" registration was an artifact of this; v2 it crashed loud
   (rank0 [0,2,16]) and is fixed.

All three filed hypotheses refuted by measurement (the probe:
`examples/massive_anomaly_probe.rs`): the teacher dump join is asserted
at train (KeyMap permutation cannot pass); the pyjson state envelope
costs ~0.7 pp (state 0.7559 vs raw 0.7626 on the seat's own rows);
t20k and datasets massive test rows are identical, and the artifact
(seal `7bc3ee385f81abc0` = Bench 609's recorded winner) reads 0.86
train / 0.76 test through the SERVING reader.

The fix: `h1_decide` takes a per-case `class_of_pos` bridge;
`fill_positions` mirrors the engine's own two resolution rules (by name
when every presented key is a seat label — massive/banking77; identity
under k == N — the fixed-criteria suites, score-array included; else
refuse loud). A1/H2 pick among presented positions; the H2 margin's
rivals are the presented options. Pinned by
`h1_scores_the_class_the_position_denotes`.

**Bench 001 v2 supersedes v1 wholesale** (corrected frozen read):
massive A1 **0.8167**, registered H2(β=0.25,nmin=4,τ=4) **0.8300** vs A0
0.4200 — the suite flips from A0-stands to a hybrid win; banking77
registers **A1 0.7960** (v1's A0 was the bug); ag_news H2(0.25,2,2)
0.9000; emotion/sst5 A1 (0.8550/0.4217); xnli A0 stands. Laya paired
face PASS ×6. The lesson generalizes: **a pick/gold pair is a SPACE
contract — when a seat samples its option set, every scorer must speak
the presented-option space, and an instrument index that survives on
coincidence is an OOB panic waiting for the first suite that breaks the
coincidence.**

## 2026-09-27 — Bench 001: the hybrid GOAT run (P3 + P3a through the single test read)

The Reflex · instinct hybrid measured end to end. Commits: reflex seat
`979dd90`+`8d725d3`, hybrid `d4caaa6`, arena `dea91d5`, join-contract fix
`96fe890`, micro-assert fix (pre-run). The run: `.benchmarks/001_hybrid_goat/`
(RESULTS.md + predictions.json + registration.json).

- The seat seam lives in REFLEX (one-way): `harness::runner::seat` —
  `prepare_seat` / `fit_posture` (the deployed Bench 051 posture through
  the SAME code reflex's runner uses) / `eval_seat` (+
  `laya_escalation_latency_us` behind `laya-riir`). The boundary check
  forced the shape: reflex consuming instinct would be a cycle. The A0
  drift pin (arena A0 xnli_en == reflex `run()` hard accuracy,
  byte-exact 0.5167) proves the seat path on every run.
- Verdict: ag_news promotes **H2(β=1,nmin=2,τ=4)** — 0.8975 vs A0
  0.8625 / A1 0.8875, p50 2 µs vs A0's 150 µs, G1+G5 PASS. emotion +
  sst5 promote **A1** (the specialist alone: 0.8550 / 0.4217 vs A0's
  0.5750 / 0.2017; H1's reflex half DRAGS the cascade down — the fused
  gate passes reflex answers that are wrong more often than the
  specialist's). xnli + massive + banking77 keep **A0** (G5 refused
  honestly). H3 excluded with reason (no chain-shaped specialist
  suite).
- Honest fails on record: G2's H1 fusion-only bar (100 ns/q) is
  breached on wide suites by the O(n·k) prune (massive 136 ns/q,
  banking77 181 ns/q) — the O(n) survivor heap is the named remedy;
  this box's `.raw/datasets` differ from
  Bench 051's bytes (the A0 rows shifted accordingly — the arena is
  internally consistent per the drift pin; 051 cross-references are
  indicative, never comparable numbers). ⚠ SUPERSEDED the same day by
  Issue 006's resolution (the entry above): the massive chance reading
  and the xnli/massive/banking77 A0 registrations were the instrument's
  position-vs-index defect — read Bench 001 v2 for the corrected
  numbers.
- Instrument lessons paid for en route: the sigmoid-gate calibrator's
  `apply()` is the identity until `refit()` (the first run read Platt ==
  raw to 4 decimals — G1 false-failed everywhere until the face called
  refit; now pinned by tests/calibrator_probe.rs); the G5 budget face
  must REFUSE (not pass) when the registered arm is A0/A1; a micro-loop
  liveness assert on accumulated picks is wrong-headed (picks can all
  be label 0 — black_box is the liveness guarantee); the label join's
  contract is seat⊆artifact, not equality (massive: 60 artifact labels
  vs the seat's 59 offered options).

## 2026-09-26 — birth

Created per riir-ai Proposal 047 (owner decision: the model-based/hybrid
lane lives in a PRIVATE repo, riir-clippy-shaped, because hybrid wiring and
weights are product moat and `riir-reflex` is public). Trainer:
riir-train Issue 576. Motivation: riir-reflex Issue 038 / Bench 051 — the
modelless lane plateaus below laya on xnli / typed_decisions / ag_news.
