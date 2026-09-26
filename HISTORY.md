# HISTORY — riir-instinct

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
  massive's distilled specialist reads CHANCE on the seat (Issue 006 —
  A0 unaffected; banking77's artifact works at 0.796, so the pipeline
  is not wholesale broken); this box's `.raw/datasets` differ from
  Bench 051's bytes (the A0 rows shifted accordingly — the arena is
  internally consistent per the drift pin; 051 cross-references are
  indicative, never comparable numbers).
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
