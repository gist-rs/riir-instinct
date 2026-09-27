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
/git/riir-kat        ← the decstat wire client (opt-in `decstat` feature, Plan 002 / Issue 004:
                        `kat_protocol_decstat` composer + `push_decstat`; the ledger/settlement
                        semantics stay in riir-dapps — this is a client-only consumption)
```

## Build commands

```sh
cargo check
cargo clippy --all-targets -- -D warnings
cargo test                                   # the gate suite (serve gates skip loud without data)
cargo run --release --bin arena              # the GOAT run (writes .benchmarks/<out>/)

# The hosted serving lane (P5, Issue 002):
cargo run --release --bin serve -- --bind 127.0.0.1:8091 --suites ag_news,massive_intent_en
#   datasets default ../riir-reflex/.raw/datasets_t20k · winners
#   ../riir-train/data/instinct_specialists · INSTINCT_DATASETS_DIR /
#   INSTINCT_WINNERS_DIR / INSTINCT_BIND env overrides · CORS via
#   RIIR_INSTINCT_ALLOWED_ORIGIN (comma list, closed by default)
curl -s -X POST localhost:8091/decide \
  -d '{"suite":"ag_news","state":"Wall Street rallies as the Fed signals a rate cut"}'

# Deploy: plan → cross-build (zigbuild x86_64) → stage (no CF creds needed);
# the real `wrangler containers build` + deploy is the owner-adjacent step
../riir-deployer/target/debug/riir-deploy -m deploy.yaml plan
RIIR_DEPLOY_STAGE_ONLY=1 \
  ../riir-deployer/target/debug/riir-deploy -m deploy.yaml deploy --dest local
file .deploy/local/stage/decisions/app-bin   # → ELF 64-bit x86-64
```

## Branch

`develop` is the working branch. No feature branches.

## Current state

**P1–P5 substantially DONE (2026-09-26/27) — Bench 001 is the hybrid
GOAT record; P6 landed T1–T3 (the decstat flywheel, Plan 002) with the
intake leg filed as riir-train Issue 577.** Master plan:
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
- **Bench 001 v2** (`.benchmarks/001_hybrid_goat/`, the corrected
  instrument — Issue 006): v1 scored the label permutation by position
  and compared label-space picks against presented-option gold; only
  massive's 20-of-59 sampled options could expose it, and v1's
  registration also double-indexed `rank0_sorted[select_arm(..)]`. The
  prune remedy for the wide-suite G2 breach (the O(n·k) prune →
  running-threshold `Cascade::prune`, output-identical, sort-reference
  + tie pins) landed 2026-09-27 (`27e9257`). **Superseded as the
  publishable record by Bench 002 below** — its bytes (`.raw/datasets`,
  the old pull) were never comparable with the published lanes.
- **Bench 002** (`.benchmarks/002_hybrid_052_protocol/`, the ALIGNED
  read — supersedes v2's numbers as the publishable record): the same
  arena at the Bench-052 protocol on the SAME `datasets_t20k` bytes the
  published lanes + the 4090 T5 re-run carry (977/977 byte-verified);
  the comparability proof is **A0 == the published 052 modelless rows
  6/6** (ag_news 0.8825, emotion 0.7375, sst5 0.3967, xnli 0.5233,
  massive 0.7800, banking77 0.8260) + the in-run xnli drift pin.
  Registered arms — **ag_news H2(β=0.25,nmin=2,τ=2) 0.8975** (G1 FAIL:
  the raw fused readout is already calibrated at 0.0133, Platt hurt;
  G5 PASS); **emotion A1 0.8550**, **sst5 A1 0.4217** (G1+G3 PASS);
  **massive H2(β=1,nmin=2,τ=8) 0.8267** (A0 0.7800 → +4.7 pt — the
  v2 "flips from A0 0.42" story was the old first-N sample's
  unrepresentative A0; G1+G3 PASS; the t20k test split lacks
  `cooking_query` so the cal front presents an artifact-known,
  seat-unknown option — the bridge gained the NaN-no-evidence
  extension: margin muted to 0, never a rival, pinned by
  `h2_nan_evidence_mutes_the_margin_and_is_never_a_rival`);
  **banking77 H1 0.8060 @ 46.8% consult** (G3 FAIL — H1 pays up to
  ~4.7 pt at 95% confidence; no promotable hybrid arm, A0 stands);
  **xnli A0 stands**. G2 fusion-only 9–119 ns/q; the laya paired face
  PASS (lane sub-2 µs vs laya metal 118–187 ms p50). **The reflex-site
  publish LANDED 2026-09-27**: the `hybrid` lane class joined
  publish_bench (carry/rename/inventory + `LANE_DISPLAY` →
  `Instinct (hybrid)` — cap-case rename 2026-09-27, old spelling kept as
  a LANE_DISPLAY alias so re-publishing the prior bench.json lands it),
  the bench page (table + filter + charts + the
  LANES palette slot + the lane explainer), and the lane published from
  `hybrid_lane_doc.json` — the registered arm per suite, xnli honestly
  absent (test_publish_bench 22/22 incl. the hybrid case; page + chart
  smokes PASS; `scripts/build_hybrid_doc.py` re-derives the reflex
  metric laws exactly and cross-checks against the arena's G1 ECE
  lines).
- **Stats** (`src/stats.rs`): Wilson bounds, paired non-inferiority
  (δ = max(1.0pp, 2.5·SE)), Pareto rank-0, Beta-LCB selection
  (katgpt-core best_belief) — the pre-registration instrument.
- **P4 — the HOSTED-ONLY vessel reader** (`src/vessel.rs`, opt-in
  `vessel` feature, 2026-09-27): authenticates with reflexer-vessel's
  own exported primitives (peek, PinTable key resolution, strict
  ed25519, blake3 commitment), applies the blake3-XOF confidentiality
  envelope, decodes the RISP artifact, enforces the monotonic apply
  gate; gate taxonomy 7/7 in `tests/vessel_gates.rs` against real
  fixtures. `encrypt_payload` is the published minting contract
  for riir-train; the first vessels ARE minted — the minter is
  riir-train's `vessel-mint` bin (`vessel_mint` feature, riir-train
  `495e9b7d`; the class-aware cap is reflexer `049a583`, the cap-pin
  gate instinct `f2fd12a`), and the serve lane boots vessel-if-present
  (`INSTINCT_VESSEL_DIR` + KEY + PINS, monotonic applied-state files,
  live-verified: banking77 as one 10 MB vessel, `source: Vessel`).
- **P5 — the hosted serving lane** (`src/server.rs` +
  `src/bin/serve.rs` + `deploy.yaml`, 2026-09-27): the servable binary
  and the cf-container shape (Issue 002, katgpt-rs Proposal 014 §4
  Tier-2(b)).
  - **The serving posture table is the GOAT product verdict, not the
    registration instrument's pick** (`server::serving_posture`): they
    disagree exactly once — banking77's cal front registered H1 and G3
    FAILED it, so **A0 serves** there and the H1 row stays a published
    site measurement. The parity gate
    (`tests/serve_gates.rs::served_decisions_are_the_frozen_bench_002_picks`)
    replays committed test cases through `decide()` and asserts identity
    with the frozen `predictions.json` picks — the serve path IS the
    arena path.
  - The edge is std-only HTTP (`/decide`, `/healthz`, `/`); lanes boot
    on 64 MiB-stack threads with the listener bound FIRST (healthz live
    during the seat boot; loading/failed lanes answer 503 with the state
    named, never a silent fallback). Every response carries the
    **decision receipt** (Proposal 014 §4): build fingerprint (blake3
    over rustc release/commit/host + the compiled feature set, generated
    by `build.rs`), BLAKE3(input), BLAKE3(canonical decision), lane id.
    Refusals carry machine-readable `code`s (`unknown_suite`, `loading`,
    `bridge_undefined`, `too_large`, …). CORS allow-list env
    `RIIR_INSTINCT_ALLOWED_ORIGIN`, closed by default.
  - The container: `deploy.yaml` (domain `ops`, cf-container,
    `standard-2`) cross-builds the serve bin via cargo-zigbuild (the
    deployer's ELF gate) and rides the deployer's **`files:` rows**
    (riir-deployer `75b8240`) to stage the 6 sealed winners + 6 t20k
    dataset suites (per-file BLAKE3 rows) beside the binary; env points
    the lanes at `/data/{winners,datasets}` (the generated Dockerfile
    has no WORKDIR — relative `to` paths land at the image root).
    Secrets never enter the manifest (cf-container env keys are
    plan-time refused when secret-shaped — the cf-worker T1d rule).
  - Verified locally end to end: plan → zigbuild → 948-row BLAKE3
    manifest → stage → docker run (x86-64 under Rosetta, HEALTHCHECK
    healthy) → all six lanes ready ≤ 14.7 s → live decisions 0.2–5 ms
    with receipts. The CF push itself is owner-adjacent (creds);
    mainnet is the owner ceremony (T4).
