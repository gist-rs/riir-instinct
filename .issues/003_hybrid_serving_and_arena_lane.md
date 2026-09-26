# Issue 003 — hybrid serving + the "Reflex · instinct" arena lane

**Status:** OPEN — filed 2026-09-26 (Plan 001 P3). Blocked on riir-train Issue 576 (a specialist to serve).

## Why

Bench 051 (riir-reflex) put the modelless lane at or above laya on 4/8
dataset suites and left xnli (0.523 vs 0.860), typed_decisions (0.319 vs
0.745) and ag_news (0.883 vs 0.950) to a trained model. The hybrid pays for
the specialist only where the modelless lane abstains, so it keeps the
µs-tier latency on easy questions.

## Plan

- [ ] **T1** — crate skeleton (declared deps: katgpt-core, riir-reflex lib,
      riir-infer; BOUNDARY rows land with the code, measured).
- [ ] **T2** — composition: serve the arm Issue 005's POC promotes (H1
      cascade / H2 prior fusion / H3 PUCT-over-options). The default until
      then is H1: modelless `nb_scope` scores → top-k prune → specialist over
      the survivors, with confident modelless answers short-circuiting (the
      fused abstain gate).
- [ ] **T3** — harness lane in riir-reflex's arena (same byte-identical
      questions; accuracy, escalation rate, p50/p99, G1 calibration). The
      lane crate lives here; riir-reflex consumes it behind an opt-in feature
      or as a subprocess lane (decide by boundary check, not preference).
- [ ] **T4** — the merged GOAT gate G0–G6 (Issue 005) vs modelless, instinct-alone AND laya, test read once; publish via
      the reflex-site lane-scoped update path (both hosts bit-identical where
      the lane is deterministic).

## References

riir-reflex Issue 038 (T4 revised → here), Bench 051; riir-ai Proposal 047;
katgpt-rs Proposal 014 (three selectable lanes).
