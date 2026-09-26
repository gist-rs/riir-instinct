# Issue 003 — hybrid serving + the "Reflex · instinct" arena lane

**Status:** OPEN — T1–T4 DONE; **T4's verdict updated 2026-09-27 to the
corrected Bench 001 v2** (Issue 006's instrument fix — the row below is
superseded). The site publish remains.

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
- [x] **T4** — the merged GOAT gate G0–G6 MEASURED 2026-09-27 (Bench
      001; **v2 same day** — Issue 006's corrected instrument:
      `.benchmarks/001_hybrid_goat/`): per-suite registered arms —
      **ag_news H2(β=0.25,nmin=2,τ=2) 0.9000** (A0 0.8625, A1 0.8875;
      G1+G5 PASS); **emotion A1 0.8550**, **sst5 A1 0.4217**,
      **banking77 A1 0.7960** (G1+G3 PASS); **massive
      H2(β=0.25,nmin=4,τ=4) 0.8300** (A1 0.8167 vs A0 0.4200 — the
      suite flips to a hybrid win); **xnli A0 stands** (G5 refused
      honestly). G2's H1 fusion-only overhead passes narrow suites
      (8–20 ns/q); the wide-suite breach got its remedy 2026-09-27 —
      the running-threshold prune (output-identical, sort-reference +
      tie pins; `examples/prune_bench`: massive 135→109 ns,
      banking 163→122, n=400 441→290) — still ~10–30 ns above the
      100 ns bar at n=59–77 (the remaining cost is the compare floor +
      the sorted-array inserts, i.e. the O(n) work itself); the arena's
      G2 row refreshes at its next run. The laya paired face measured
      same day: **PASS ×6** (lane sub-µs p50 vs laya 118–650 ms). The
      site publish remains open — **blocked on comparability, not
      wiring**: the site's published modelless/laya lanes are the
      Bench 052 protocol runs (reflex-site `9839f5a`), while the v2
      arena numbers come from THIS box's current `.raw/datasets`, which
      the record itself flags as differing from earlier pulls
      ("internally consistent, never comparable"). Publishing the
      hybrid lane beside them without a same-protocol, same-bytes
      re-measurement would put non-comparable numbers on the public
      tables — the publish needs one aligned run (the reflex harness
      protocol + the arena seat on the same bytes) first.

## References

riir-reflex Issue 038 (T4 revised → here), Bench 051; riir-ai Proposal 047;
katgpt-rs Proposal 014 (three selectable lanes).
