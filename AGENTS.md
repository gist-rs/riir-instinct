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

## The arsenal laws (Proposal 001; canonical text lives THERE)

The serving lane's selection surface is the **manifest** (`arsenal.toml`,
embedded by default; `INSTINCT_ARSENAL`/`--arsenal` override) — no hard-coded
posture tables or filename conventions behind it. The short form of each law
(the proposal owns the full wording):

- **A1 bytes-are-runtime, capability-is-compile-time** (vessels only).
- **A2 never blend** — selection is monotonic atomic hot-swap of whole
  artifacts (`check_epoch_tag`: advance / idempotent / fork refused /
  downgrade refused; `LaneSlot` installs whole under the slot lock — a
  decision observes one whole server, never a torn read).
- **A3 coarse vessels, cheap routing, no dyn on the decide path** — the
  hoarding gate rides `katgpt-core::set_admission`; the centroid is the
  SIGNED simhash fold of the train corpus (`corpus_centroid` — the unsigned
  variant made every corpus a near-duplicate of every other, Bench 003's
  GOAT finding; do not revert it).
- **A5 the manifest is the only selection surface**; boot drift fails loud.
- **A6 posture rows are pinned in both media** — `tests/serve_gates.rs`
  pins the manifest's BLAKE3 digest; a TOML edit reds like a code edit.
- **A7 lazy + budgeted, evicted by wire signal** — `budget.load = "lazy"`
  rows boot `Unloaded`, the first decision loads (503 window covers it);
  `POST /arsenal/release` evicts (epoch kept); `POST /arsenal/swap` swaps
  monotonic (both loopback-only). Kill-switch `RIIR_INSTINCT_HOARD_GATE=0`
  (the exact literal) disarms the hoarding gate.
- **A8 no runtime minting** (riir-train mints); **A9 same-engine-class
  only**; **A10 moat** (PUBLIC-RELEASE carries no GAME-IP content).

The budget legs' GOAT gate is `cargo bench --bench arsenal_budget_goat
--features arsenal_goat` (Bench 003) — re-run it after any centroid, gate,
or manifest change; it refuses (exit 1) without the datasets/winners,
never a green zero.

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
cargo bench --bench arsenal_budget_goat --features arsenal_goat   # the arsenal budget GOAT (Bench 003)

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

**Benches 013+014 (2026-09-28) — the Issue 578 noul bridge LANDED:
prompt_injections serves the T2-certified A1; typed_decisions refuses
(unseatable).** Plan 003's bridge (`.plans/003_noul_bridge_578_specialists.md`,
commits `1e9f1b1`/`ed592ba`/`d644ea5`): the specialist bridge's third
presented-option arm — a noul question's fixed `[false, true]` rendering
maps onto the producer's unified `no`/`yes` pair (arena: by NAME through
the key map; serve: POSITIONAL always — presented names never reorder the
rendering); `seat_join` decides the form (Named / the positional int
spelling → the pair / **Context** — typed_decisions' 4 workflow labels are
the modelless engine's domain space, disjoint from the option-key
artifact, sentinel perm rows, identity-by-count disabled both sides);
`S2` dispatch; per-question `eval_h1`/`eval_a1_h2` (typed's 5-question
cases zip per question; the non-noul path verified byte-identical —
banking77 re-run reproduced the Bench 012 predictions with zero diffs);
a context seat's template-coverage gate (an unseatable artifact falls
back to a0_stands with the gap named, never a mid-read panic). **Bench
013** (`.benchmarks/013_prompt_injections_specialist/`): A1 0.8534 vs A0
0.7672 (+8.6 pt), **T2 PASS (paired LB95 +0.0082 > 0 at n=116) — the
second certified suite ever** (massive the first; the 578 "likely
refusal" risk inverted); G1 disclosed FAIL (Platt hurts the 2-class
sigmoid; raw ECE 0.0824 beats the floor). **Bench 014**
(`.benchmarks/014_typed_decisions_specialist/`): the Issue 578 typed
artifact is UNSEATABLE — 100/500 cases (security_incidents: 5 severity
levels + 4 action keys) present option keys the artifact has no class row
for (the train pool never carried the reflex test templates; the seat's
own corpus guard falls back to a self-doc for that workflow) → a0_stands
with the reason; A0 0.4655 re-pinned site ✓. Serving truth: the manifest
rows are SEVEN now (prompt = A1, digest `blake3:ee0b4eb4…`, the 578
winner name), `typed_decisions` asserted ABSENT; the lane doc for the
reflex-site session is merged into
`014_…/hybrid_lane_doc.json`; `deploy.yaml` ships the prompt winner +
dataset dir. Unblock path: riir-train Issue 581 (retrain over the
template key set).

**Bench 012 (2026-09-28) — the Issue 579 T3 arena bridge LANDED: banking77
serves the nbsvm v2 winner.** The bridge: `specialist::winner_bridge` is
the ONE home for the per-suite winner file + input-bag convention (the 578
coupling made structural — `check_winner_file` refuses any raw boot/swap
naming another artifact for a bridged suite). banking77 →
`banking77_nbsvm_v2.bin` (digest = Bench 612's mint, byte-verified) over
L2-normalized PRESENCE bags (`presence_bag_into`, the train-side
`norm=true` mirror); every bag site (arena, serve, hoard centroid)
dispatches through `BagConvention`; the five v1 suites stay count-bag.
The frozen read (n=500, `.benchmarks/012_banking77_nbsvm_v2_bridge/`):
A0 0.8260 (pinned == published) · A1 0.8280 · H1 0.8320 · **H2(2,8,8)
0.8540** — served under the best-measured law (manifest row +
`PINNED_MANIFEST_DIGEST` + posture table moved together; all 11 serve
gates green), T2 LB95 −0.0013 → uncertified (the ✓/✗ row; the same class
as ag_news H2 / sst5 A1), G1 face FAIL (fused readout, the ag_news
class), G3 PASS. The train-side holdout edge transferred (+2.8 pt).
Deploy rows ship v2 (`deploy.yaml`); a banking77 HOSTED-ONLY vessel must
be re-minted from v2 by riir-train before the vessel lane carries this
posture. Pin disclosure: the harness FAMILIES' site check reds under the
reflex sibling's in-flight Issue-045 tree (published bench.json predates
it) — all NINE dataset suites pinned green (arena == reflex run() ==
site) in the same session; records written under `--skip-pin-a0`.

**P1–P5 substantially DONE (2026-09-26/27) — Bench 001 is the hybrid
GOAT record; P6 landed T1–T3 (the decstat flywheel, Plan 002) with the
intake leg filed as riir-train Issue 577. The Tetris lane (Issue 009)
is measured through T8's modelless arm: T5 WIRE-MUST-WIDEN (Bench 006
+ the demeaned decoder addendum, katgpt-rs `5961e0991`; companion
instrument Bench 008 — the pooled signature costs ≤ ~4–6 pick points,
the decoder loss is context loss), T6 PASS (Bench 007 — the
no-preview/fresh-bag teacher beats reflex lb95 +969.7 / +612.2 on
paired seeds 1..=20, 607 beside), and T8's second modelless arm
NEGATIVE (Bench 009 — the closed-form KARC chebyshev-4 basis-ridge
critic, d=202, `src/tetris_critic.rs` + the `tetris_forecaster`
feature arm: beats reflex easily, loses to the b0 champion 1-ply at
every teacher budget — 377.4 vs 907.8 pieces at b1600 targets, 2/6/12;
digest byte-identical across worker counts). **b0 stands as the
modelless floor (907.8 / 532.5 pieces); the trained critic's bar is
b0 AND the ridge arm (`48b92d26b04f3ecb`).** **T7 EXECUTED 2026-09-28
(Bench 010): NEGATIVE at round-1 capacity — the trained MLP critic
(43→128→128→1 tanh, riir-train `tetris_critic_trainer`, `8dcc11b0`/
`083ddc42`) beat the ridge arm decisively in BOTH regimes (554.6 vs
218.6; 427.3 vs 117.0 — the strongest critic yet; val agreement 0.7110
cleared the ridge's 0.66 linear plateau) but LOST to b0 (lb95 −609.2 /
−359.7) → no mint, no serve wiring; round 2 stays gated. The
transferable lesson: imitation top-1 agreement ≠ play strength (the
residual disagreement concentrates in catastrophic placements — a
re-open needs a tail-targeted loss or critic-guided search). Landed
substrate: the `TrainedMlp` serve-side forward + `LanePolicy::Mlp` +
the bench `--model` arm + `tests/tetris_critic_parity` (512/512
bit-exact; serve G2 0.228 ms p50, moot while unserved). The
widened-wire sidecar (the reflex contract field + the reflex-site
sender) still gates the SERVE path (T9).** Master plan:
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
- **Bench 005** (`.benchmarks/005_hybrid_every_suite_measured/`, Issue 010
  T1/T2/T3 + T5's arena half, 2026-09-27): the arena population is now
  EVERY reflex dataset suite — the six specialist suites at the full arm
  grid + `typed_decisions` + `prompt_injections` at the A0/G0-only
  posture (no winner artifact → `run_suite_a0_only`; a present-but-
  corrupt artifact stays fatal). A0 arms are flattened PER QUESTION
  (reflex's hard-metrics convention — typed_decisions: 2000 q / 400
  cases; latency stays per-case, `n_cases` disclosed). The lane doc is
  THREE-STATE (`hybrid_arm` / `a0_stands` with its measured A0 cell +
  reason / absent = never seated; the bare `skipped_suites_a0_registered`
  list is gone). **The pin caught a real posture gap**: with oc off the
  arena read typed_decisions 0.3300 vs the PUBLISHED 0.4655 — the
  published row is the OC-ARMED posture (`oc_selection` selected_scale
  2.0, the only suite whose train rows carry per-question gold events);
  knobs re-baselined to `oc_select: on` (the selection declines
  byte-identically on the other 7), pin 8/8 green: arena == reflex
  `run()` == published site rows on every suite incl. typed_decisions
  0.4655 (abstain 0.602 == published) and prompt_injections 0.7672
  (116 q, abstain 0.681). Verdicts unchanged from 004 on the six (the
  sold set is still exactly massive H2); the two new suites are
  `a0_stands — no specialist`. Reflex at `56e3ddd` (its plan-003 Phase 1
  Thai-pins commit — tests-only); the site half (doc rebuild +
  republish + card three-state render) belongs to the session holding
  the reflex-site checkout (outside this workspace).
- **Bench 011** (`.benchmarks/011_hybrid_families_seated/`, Issue 010 T6
  + reflex Issue 049, 2026-09-28): the harness families + `code_fixtures`
  joined the arena population through reflex `03415e5`'s synthetic seat
  seam (the `Seat.synthetic` marker; `harness_cache_reuse` stays a
  disclosed refusal). All six publish `a0_stands` (A0 0.375–0.500 —
  049's scope-note expectation); the G1 disclosure rows FAIL on the tiny
  cal fronts (they register nothing); the eight dataset suites' A0 rows
  reproduced Bench 005 byte-identically — the seat refactor perturbed
  nothing. Seat gates: reflex `tests/harness_seat_gates` (5/5, incl. the
  seat-engine == manual-build bit-exact pin).
- **Bench 004** (`.benchmarks/004_rebaseline_current_reflex/`, Issue 008
  T1+T2 executed 2026-09-27, supersedes 002
  as the serving verdict): the arena re-baselined at the CURRENT
  published reflex posture (seat knobs `head+nb+ridge` select — emotion
  arms ridge@8, every other suite's ladder declines at the arming bar,
  byte-identical to off) + the **T2 product gate** (registration REFUSES
  an arm not STRICTLY above the current Reflex row — paired (pick − A0)
  LB95 > 0 on the frozen test read; `stats::PairedDiff::lb95`). The
  comparability proof is the **6/6 drift pin + site pin**: arena A0 ==
  reflex `run()` == the PUBLISHED bench.json rows, every suite (ag_news
  0.8825, emotion 0.8850, sst5 0.3967, xnli 0.5233, massive 0.7800,
  banking77 0.8260). Verdicts — **massive H2(1,2,8) 0.8267 PASSES the
  gate (LB95 +0.0124): the ONE certified arm**; ag_news H2
  (+0.0150/−0.0100), sst5 A1 (+0.0250/−0.0129), banking77 H1
  (−0.0200/−0.0466) REFUSED → A0 serves; emotion's A1 (0.8550) is
  outrun by reflex's ridge@8 base (0.8850) — the instrument picked A0
  outright; xnli A0 stands. The sold set is exactly one suite, certified.
  **The abstention contract** (found by the parity gate the same day):
  reflex's hard accuracy is the FORCED convention while the serve
  honors abstention — A0's per-case abstain flags are now recorded
  (`predictions.json` per-arm `abstained`; ag_news A0 abstains 196/400)
  and the parity gate asserts served-abstention == recorded-abstention
  before picks. Serve readiness ceilings raised 120→420 s (the ridge
  ladders are per-boot derivation work now; a frozen-derived-posture
  seam is the recorded follow-up).
- **Bench 002** (`.benchmarks/002_hybrid_052_protocol/`, the ALIGNED
  read — supersedes v2's numbers as the publishable record; SUPERSEDED
  AS THE SERVING VERDICT by Bench 004): the same
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
  lines). **The arena + sizes presence LANDED same-day** (reflex-site
  `f7a0574`): an Instinct (hybrid) lane card in every arena game, placed
  directly before the raw board — the raw board is ALWAYS the last card
  (owner rule) — rendering the registered per-suite arms from
  data/bench.json at load (`arena_hybrid_card.js`; a text-suite card, not
  a game board: game spots answer through the Reflex half), and the
  #sizes disk-footprint report gained the hosted serving posture (serve
  binary + the six t20k dataset suites as engine, the six sealed winner
  vessels as model — 42.9 MB total, recorded measurements with verbatim
  lstat commands; publish_sizes grew recorded model source kinds).
  ✅ **The serving law + the full-coverage display law (owner verdict
  2026-09-27, riir-instinct `4cc4441` + reflex-site `aeea24b`):** the
  SERVED arm is the **best measured arm** per suite over the Bench-005
  frozen read, A0 a candidate like any other — ag_news serves H2 0.8975
  (+1.5 pt), sst5 serves A1 0.4217 (+2.5 pt), massive serves H2 0.8267
  (+4.7 pt, T2-certified); emotion/banking77/xnli serve A0 because A0 IS
  the argmax there (the losing specialists are the T4/T5 backlog, not a
  refusal to serve). The T2 strict-superiority gate REMAINS as the
  advertising law (the site's ✓/✗ row), never the serving selector.
  The site publishes EVERY seated suite's cell — reflex-half suites
  included (a tie or a loss is shown, labeled, and stays visible as the
  improvement backlog; hiding a measured result reads as "can't handle
  it") — with `serves` + `gate` disclosure on each cell. Gates:
  serve_gates 11/11 (parity now resolves the expected arm from the
  MANIFEST — the record's `registered` field is T2-era data; digest
  repinned `4e63e9d9…`), publish_bench 48/48, pairing 28/28. The arena
  GAME card stays removed per Issue 009 (owner direction) until a real
  Instinct board strictly beats Reflex's.
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
  - **The serving posture is the GOAT product verdict, and since the
    Bench-004 T2 gate it is ONE verdict**: the registration itself
    refuses any arm not strictly above the current Reflex row (paired
    LB95 > 0 on the frozen test read), so the instrument pick and the
    manifest row can no longer disagree — registered == served, per
    suite. Reflex is free; a tie sells nothing. The verdict rows live
    in `arsenal.toml` (law A5 — the ONE selection surface; the
    historical `server::serving_posture()` match table was deleted by
    Proposal 001 T2); the embedded default is pinned byte-for-byte by
    the gates (law A6). The parity gate
    (`tests/serve_gates.rs::served_decisions_are_the_frozen_goat_picks`)
    replays committed test cases through `decide()` and asserts identity
    with the frozen `predictions.json` picks — the serve path IS the
    arena path — INCLUDING the abstention contract (served abstention
    == recorded abstention, per case; Bench 004's addendum).
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
