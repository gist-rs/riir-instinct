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

**P1 — BORN 2026-09-26 (md-only).** Master plan:
[`.plans/001_instinct_lane_clippy_flow.md`](.plans/001_instinct_lane_clippy_flow.md).
