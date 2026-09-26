# Issue 003 — hybrid serving + the "Reflex · instinct" arena lane

**Status:** OPEN — T1–T4 DONE; **T4's site publish UNBLOCKED 2026-09-27**:
the aligned Bench-052-protocol re-run landed (Bench 002,
`.benchmarks/002_hybrid_052_protocol/` — A0 == the published 052
modelless rows 6/6, same `datasets_t20k` bytes the 4090 T5 run verified
977/977), the reflex-site hybrid lane carry + render shipped
(test_publish_bench 22/22, page + chart smokes PASS), and the lane is
published. The v2 (Bench 001, old-bytes) numbers are superseded by
Bench 002 — the v2 "massive flips from A0 0.42" narrative was the old
first-N sample's unrepresentative A0; the honest stratified story is
A0 0.7800 vs hybrid 0.8267.

~~Blocked on riir-train
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
**T2 DONE 2026-09-27** (`src/hybrid.rs`): H1 cascade (reflex fused-gate
pass-through, else the specialist over the deterministic top-k prune
survivors) + H2 prior fusion (p′ᵢ ∝ pᵢ·exp(g·β·mᵢ), the Proposal 013
shape; per-token nb margin via the engine's `nb_scope()` accessor); G0
kill switch a first-class arm (ReflexOnly → byte-identical A0); the
label join is a name-injection asserted both directions (and it earned
its keep twice — see the 60-vs-59 refinement); G4 alloc-free pinned by
tests/g4_alloc.rs. **T3 DONE 2026-09-27** (`src/bin/arena.rs` + the
reflex `harness::runner::seat` seam): the lane lives HERE (the boundary
check was forced — reflex consuming instinct would be a cycle; the seat
is reflex's ONE-WAY public surface), byte-identical questions via the
seat, the deployed Bench 051 posture via the same fit code reflex's
runner uses, and the A0 DRIFT PIN (arena A0 xnli_en == reflex `run()`
hard accuracy, byte-exact) proving the seat path.

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
- [x] **T2** — composition: the arena measured all arms 2026-09-27
      (Bench 001). H1 is the default posture (top_k 8); H2 registered on
      ag_news (the one promotable hybrid — see Issue 005 T5).
- [x] **T3** — DONE 2026-09-27: the arena lane (this repo, `src/bin/arena.rs`)
      over the reflex `seat` seam — accuracy, escalation rate, p50/p99,
      G1 calibration, the A0 drift pin. The lane-wiring boundary check:
      a reflex→instinct dep is a cycle, so the lane lives here and
      reflex exposes the one-way seat (`prepare_seat` / `fit_posture` /
      `eval_seat` + `laya_escalation_latency_us` behind `laya-riir`).
- [x] **T4** — DONE 2026-09-27 (v2 + the **aligned Bench-052 re-run,
      Bench 002**): per-suite registered arms at the 052 protocol
      (`datasets_t20k` bytes, stratified split, A0 == the published 052
      rows 6/6 — `.benchmarks/002_hybrid_052_protocol/ALIGNMENT.md`):
      **ag_news H2(β=0.25,nmin=2,τ=2) 0.8975** (A0 0.8825; G1 FAIL —
      the raw fused readout is already calibrated, Platt hurt; G5
      PASS); **emotion A1 0.8550** (G1+G3 PASS); **sst5 A1 0.4217**
      (G1+G3 PASS); **massive H2(β=1,nmin=2,τ=8) 0.8267** (A0 0.7800,
      +4.7 pt — the honest stratified gap; G1+G3 PASS; the t20k test
      split lacks `cooking_query`, so the cal front presents an
      artifact-known seat-unknown option — the bridge gained the
      NaN-no-evidence extension, margin muted to 0, never a rival);
      **banking77 H1 0.8060** (A0 0.8260; **G3 FAIL** — H1 pays up to
      ~4.7 pt at 95% confidence; no promotable hybrid arm, A0 stands);
      **xnli A0 stands** (G5 refused honestly — no hybrid lane
      published). G2: fusion-only 9–119 ns/q (the 100 ns bar holds on
      the narrow suites; massive/banking sit 2–19 ns over — the compare
      floor + sorted inserts are the O(n) work). The laya paired face
      measured: **PASS ×5** (lane sub-2 µs vs laya metal 118–187 ms
      p50). **Site publish LANDED 2026-09-27**: reflex-site gained the
      `hybrid` lane class (publish_bench carry + rename + inventory,
      LANE_DISPLAY `instinct (hybrid)`, page table + filter + charts +
      LANES palette slot; test_publish_bench 22/22 with the new hybrid
      case; bench_page + chart_render smokes PASS) and the lane was
      published from `hybrid_lane_doc.json` (the registered arm per
      suite, xnli honestly absent; `lane_sources.git_sha` names the
      landing commit). The doc builder is `scripts/build_hybrid_doc.py`
      (the reflex metric laws re-derived exactly — ece_of's 15-bin law,
      stable-argsort acc@50cov, nearest-rank p50/p99 — cross-checked
      against the arena's own G1 ECE lines).

## References

riir-reflex Issue 038 (T4 revised → here), Bench 051; riir-ai Proposal 047;
katgpt-rs Proposal 014 (three selectable lanes).
