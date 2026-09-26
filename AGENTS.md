# AGENTS.md — riir-instinct (private)

The global `~/.agents/` rules apply; this file documents repo-local context.

## Boundary contract — read `BOUNDARY.md` first

[`BOUNDARY.md`](BOUNDARY.md) is authoritative. **Domain test:** is this the
model-based or hybrid decision lane (specialist serving, modelless-drafter
composition, HOSTED-ONLY vessel reading, instinct arena rows, hosted
deploy)? NO → it belongs in another repo; file there. Boundary checks run via
the `boundary-guard` skill (`../riir-ai/scripts/ci_boundary_contract.sh`).

## Role

The trained sibling of the modelless engine. The naming follows the L0–L5
adaptive stack (riir-ai Proposal 047): **Reflex** = hard-wired response
(`../riir-reflex`, modelless, public), **Instinct** = learned fast response
(trained weights, private). The flow is riir-clippy's, end to end:

```
arena/serve (here) ─ decisions ─▶ decstat rows (consent-gated) ─▶ riir-kat wire
      ▲                                                           │
      │ HOSTED-ONLY vessel (bytes)                                 ▼
riir-deployer (cf-container) ◀─ vessel minting ◀─ riir-train ◀─ riir-dapps settle → corpus
```

## Laws

1. **riir-train trains, this repo consumes bytes** (riir-clippy Proposal 009).
2. **Modelless first inside the hybrid**: the modelless lane answers when it
   is confident; the specialist is paid for only where it abstains.
3. **Promotion is GOAT-gated** against BOTH the modelless and laya lanes on
   the same test split, read once; demote the loser.
4. **Weights never committed**; HOSTED-ONLY never leaves controlled hardware.

## Sibling layout

```
/git/riir-instinct   ← this repo
/git/riir-reflex     ← modelless engine + arena harness (public)
/git/riir-reflexer   ← rulebook engine + vessel FORMAT (public)
/git/riir-infer      ← model forward substrate (public)
/git/riir-train      ← specialists + vessel minting (private)
/git/riir-deployer   ← deploy orchestration (private)
/git/riir-dapps      ← settlement (private)
```

## Branch

`develop` is the working branch. No feature branches.

## Current state

**P1–P3a substantially DONE (2026-09-26/27) — Bench 001 is the hybrid
GOAT record.** Master plan:
[`.plans/001_instinct_lane_clippy_flow.md`](.plans/001_instinct_lane_clippy_flow.md).

- **The hybrid composition** (`src/hybrid.rs`): H1 cascade (reflex
  fused-gate pass-through → top-k prune → specialist over survivors) +
  H2 prior fusion (p′ᵢ ∝ pᵢ·exp(g·β·mᵢ), the Proposal 013 shape) + the
  G0 kill-switch arm; allocation-free hot path (tests/g4_alloc.rs); the
  label join is a name-injection asserted both directions.
- **The arena** (`src/bin/arena.rs`): the GOAT runner over reflex's
  ONE-WAY `harness::runner::seat` seam (byte-identical questions + the
  deployed Bench 051 posture through the same fit code reflex's runner
  uses; the A0 drift pin asserts arena A0 == reflex `run()` byte-exact).
- **Bench 001 v2** (`.benchmarks/001_hybrid_goat/`): the single frozen
  test read under the CORRECTED instrument (Issue 006 — v1 scored the
  label permutation by position and compared label-space picks against
  presented-option gold; only massive's 20-of-59 sampled options could
  expose it, and v1's registration also double-indexed
  `rank0_sorted[select_arm(..)]`). Registered arms — **ag_news
  H2(β=0.25,nmin=2,τ=2) 0.9000** (A0 0.8625 / A1 0.8875; G1+G5 PASS);
  **emotion A1 0.8550**, **sst5 A1 0.4217**, **banking77 A1 0.7960**
  (G1+G3 PASS); **massive H2(β=0.25,nmin=4,τ=4) 0.8300** (A1 0.8167 vs
  A0 0.4200 — the suite flips from "A0 stands" to a hybrid win; G1+G3
  PASS); **xnli A0 stands** (G5 refused honestly). The laya paired face
  PASS ×6 (lane sub-µs p50 vs laya 118–650 ms). G2's H1 fusion-only
  overhead 8–20 ns/q narrow, honest fails on wide (the O(n·k) prune —
  the O(n) survivor heap is the named remedy). Open face: the
  reflex-site publish.
- **Stats** (`src/stats.rs`): Wilson bounds, paired non-inferiority
  (δ = max(1.0pp, 2.5·SE)), Pareto rank-0, Beta-LCB selection
  (katgpt-core best_belief) — the pre-registration instrument.
