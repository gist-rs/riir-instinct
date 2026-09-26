# Issue 003 — hybrid serving + the "Reflex · instinct" arena lane

**Status:** OPEN — filed 2026-09-26 (Plan 001 P3). ~~Blocked on riir-train
Issue 576~~ — 576 CLOSED 2026-09-26 (Bench 608 + 609, winners
`<suite>_winner_v1.bin`). **T1 DONE 2026-09-26**: crate skeleton + the
RISP v1 artifact reader (`src/specialist.rs`), BLAKE3-seal + vocab-pin +
format fixtures; the serving forward runs the reflex tokenizer law
(`embed::hashed_tokens_into`, one lexicon) + katgpt-core `exact_sigmoid`
(never softmax); live pin `examples/load_winners` decodes all six real
winners and scores sample texts (ag_news business text → class 2,
emotion joy text → 1, banking77 card text → "get physical card"); clippy
`-D warnings` lib/examples/tests. Dep rows measured into BOUNDARY:
katgpt-core + riir-reflex (T2 consumes nb_scope for the composition).

## Why

Bench 051 (riir-reflex) put the modelless lane at or above laya on 4/8
dataset suites and left xnli (0.523 vs 0.860), typed_decisions (0.319 vs
0.745) and ag_news (0.883 vs 0.950) to a trained model. The hybrid pays for
the specialist only where the modelless lane abstains, so it keeps the
µs-tier latency on easy questions.

## Plan

- [x] **T1** — crate skeleton (declared deps: katgpt-core, riir-reflex lib,
      riir-infer; BOUNDARY rows land with the code, measured). DONE
      2026-09-26: workspace + lib; the RISP v1 reader (sealed artifact,
      i8-quantized weights, vocab pin 2^17 both directions) + the serving
      forward (bag law over the reflex tokenizer, sigmoid per class, argmax
      ties-lowest); 2 format-fixture unit tests; the live cross-repo pin
      (`examples/load_winners`, real winner bytes). riir-infer's row stays
      PLANNED (its first consumer is the hosted model lane, not the
      specialist) — declared deps without a consumer would be unused-dep
      rot.
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
