# AGENTS.md — riir-instinct (the open lane; the moat moved to `moat/`)

The global `~/.agents/` rules apply; this file documents repo-local context.

## ⚠ THE SPLIT (riir-ai Proposal 052, 2026-10-03) — read before anything else

**Instinct OPENS** as the public teaching lane — and is SINGLE-SOURCE-OF-TRUTH
since 2026-10-03: the owner retired the two-repo arrangement ("git history
leak is fine… i dont want 2 confusing repo i want single source of truth");
the FULL git history lives on the public repo, `gist-rs/riir-instinct-internal`
is DELETED-owner-side, and the 4090 box's remote must point at
`gist-rs/riir-instinct.git`. Moat content in git HISTORY is owner-sanctioned;
the moat law still binds the working TREE and the fence
(`scripts/fence_gate.sh --post-split`) stays.
**Rethink is the PRIVATE moat**: `../riir-rethink` (gist-rs/riir-rethink,
born `e263686`, seeded verbatim from `moat/` at `4ffd08f`; fresh root — the
whole git history stays HERE, `moat/README.md` is the pointer). The seam
(`server::install_ext_boots` + `LaneBackend` + the `Ext` seat) is the
plug-in path Rethink uses; `../riir-rethink/bin_hunks/` archives what left
the bins. The seam's RERANK contract (options in → pick-or-abstain out,
options echoed exactly as presented) is gated by
`tests/ext_seat_rerank_gates.rs` — refine Plan 202 R3a's first-customer
traffic; the gate is its OWN test binary because the install is
once-per-process and `serve_gates` pins the no-backend refusal of the
same build.
Split feature changes: `arena-laya` → **`laya-face`** (the G2 paired face vs
public reflex's laya lane); `serve-encoder*` and `decstat` are GONE from
this tree (Rethink-only). Winners default to `artifacts/cache` (env
`INSTINCT_WINNERS_DIR` overrides): `scripts/fetch_artifacts.sh` pulls the
public rows of `artifacts/manifest.toml` from the org HF lane and
hash-verifies them (public rows only — a protected row is refused loud).
The embedded manifest is the teaching default (`data/arsenal.toml`); the
production verdict rides inline byte-pinned in `tests/serve_gates.rs`.
Sections below that describe encoder/decstat/vessel work predate the split —
those surfaces live in `moat/` now.

## Naming law — SUPERSEDED by the split (kept for the record until C1)

Decided 2026-09-30 (owner call), SUPERSEDED 2026-10-03 by riir-ai Proposal 052's
split: Instinct does not retire as a brand; the repo does not rename. The
disambiguation table's third axis lives in
`../riir-game-sdk/.docs/10_multiplayer_topology/tick_tier_model.md` §(a);
the hero-strategy `rethinks` counter is game-side vocabulary — greps must
not cross the streams. Site lanes: `hybrid → Instinct` (the open product),
`encoder → Rethink` (the private moat).

## Boundary contract — read `BOUNDARY.md` first

[`BOUNDARY.md`](BOUNDARY.md) is authoritative. **Domain test:** is this the
model-based or hybrid decision lane (specialist serving, modelless-drafter
composition, instinct arena rows)? NO → it belongs in another repo; file
there. (HOSTED-ONLY vessel reading is Rethink's now.) Boundary checks run
via the `boundary-guard` skill (`../riir-ai/scripts/ci_boundary_contract.sh`).

## Role

Per riir-ai Proposal 051, the **all-tier adaptive decision-serving family** — rungs
L1 bags/hybrids through L3 encoder thinks (L4/L5 gated on 048's). **Reflex**
= hard-wired modelless response (`../riir-reflex`, public, the free floor);
**Rethink** = the trained/adaptive private product (`../riir-rethink`). The
flow is riir-refine's, end to end:

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
embedded by default; `INSTINCT_ARSENAL`/`--arsenal` override) — no
hard-coded posture tables or filename conventions behind it.

- **A1** bytes-are-runtime, capability-is-compile-time (vessels only).
- **A2** never blend — selection is monotonic atomic hot-swap of whole
  artifacts (`check_epoch_tag`: advance/idempotent, fork+downgrade refused;
  `LaneSlot` installs whole under the slot lock — never a torn read).
- **A3** coarse vessels, cheap routing, no dyn on the decide path — the
  hoarding gate rides `katgpt-core::set_admission`; the centroid is the
  SIGNED simhash fold of the train corpus (`corpus_centroid` — the unsigned
  variant made every corpus a near-duplicate of every other, Bench 003's
  GOAT finding; do not revert it).
- **A5** the manifest is the only selection surface; boot drift fails loud.
- **A6** posture rows are pinned in both media — `tests/serve_gates.rs`
  pins the manifest's BLAKE3 digest; a TOML edit reds like a code edit.
- **A7** lazy + budgeted, evicted by wire signal or by the R5 LRU
  policy (`INSTINCT_LRU_CAPACITY`, 0 = off) — `budget.load = "lazy"`
  rows boot `Unloaded` (the 503 window covers the first load);
  `POST /arsenal/release` evicts, `POST /arsenal/swap` swaps monotonic
  (both loopback-only); the LRU policy releases the least-recently-used
  Ready lazy lane at a lazy trigger when residency is at/over capacity
  (best-effort, eager rows exempt — the same release machinery, the tag
  kept, the reload `Idempotent`). Kill-switch `RIIR_INSTINCT_HOARD_GATE=0`
  (the exact literal) disarms the hoarding gate.
- **A8** no runtime minting (riir-train mints); **A9** same-engine-class
  only; **A10** moat (PUBLIC-RELEASE carries no GAME-IP content).

Budget GOAT gate: `cargo bench --bench arsenal_budget_goat --features
arsenal_goat` (Bench 003) — re-run after any centroid, gate, or manifest
change; it refuses (exit 1) without the datasets/winners, never a green zero.

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
#   arena datasets default = reflex CANONICAL ../riir-reflex/.raw/datasets at the
#   pinned engine baseline REFLEX_BASELINE_SHA (Issue 013 re-baseline a; t20k stays
#   on disk as the ARCHIVED pool — published hybrid rows are t20k-seated references,
#   reproduce via --datasets-dir ../riir-reflex/.raw/datasets_t20k)
cargo bench --bench arsenal_budget_goat --features arsenal_goat   # the arsenal budget GOAT (Bench 003)

# The public artifact fetch lane (instinct issue 020 / riir-ai Plan 623 T4): pulls
# the public-class rows of artifacts/manifest.toml from the org HF dataset
# lane (gist-rs/riir-instinct-artifacts) into artifacts/cache/, BLAKE3 +
# exact-size verified against the manifest BEFORE use (public rows only —
# a protected row is refused loud); CHECK=1 verifies cached bytes only, no
# network. With no manifest the lane is an honest no-op:
scripts/fetch_artifacts.sh

# The hosted serving lane (P5, Issue 002) — serve keeps its OWN t20k default
# (winners are t20k-trained; the arena re-baseline must NOT move it):
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

## Numbering discipline

Issue, plan, doc, benchmark, and research numbers are monotonic and never
reused: read the target dir's `.highwater`, use value + 1, write the new
value back (`.issues/`, `.plans/`, `.docs/`, `.benchmarks/`, `.research/`).

## Current state

### Serving board (the hybrid lane — the open product)

Serving law (owner verdict 2026-09-27, `4cc4441`): the SERVED arm is the
best measured arm per suite, A0 a candidate like any other; the T2
strict-superiority gate (paired (pick − A0) LB95 > 0 on the frozen read) is
the ADVERTISING law (the site's ✓/✗ row), never the serving selector —
registered == served. The verdict rows live in `arsenal.toml` (law A5); the
embedded default is pinned byte-for-byte (law A6); the parity gate
(`tests/serve_gates.rs::served_decisions_are_the_frozen_goat_picks`) replays
the frozen picks through `decide()` — the serve path IS the arena path,
including the abstention contract (served-abstention == recorded-abstention,
per case).

| suite | serves | Rethink encoder (record-only, `serve: ✗`) |
|---|---|---|
| massive_intent_en | H2(β=1,nmin=2,τ=4) **0.8400** (Bench 0029 synth seat; was 0.8267) | 0.675 — not seated (better-only law) |
| banking77 | H2(β=2,nmin=8,τ=8) **0.8540** (nbsvm v2 PRESENCE bags, Bench 012; T2 uncertified — reads LB95 −0.0163 at the 0059 published posture) | DEAD (Bench 039 screen) |
| typed_decisions | H2(β=0.5,nmin=2,τ=2) **0.6475** (Benches 019+020, T2-certified; re-certified 0058+0059 at the engine + posture moves — LB95 +0.0598) | v2 head **0.7550** (Bench 041) |
| prompt_injections | A1 **0.8534** (Bench 013, T2-certified) | DEAD (Bench 039 screen) |
| code_fixtures | A1 **0.5625** (Bench 028, T2 LB95 +0.0021; unsold vs paw 0.6250) | — |
| sst5 | A1 **0.4217** | ENC **0.5267** (Bench 029, T2-certified) |
| ag_news | H2 **0.8975** | ENC **0.9475** (Bench 037) |
| xnli_en | A0 **0.5233** | ENC **0.8600** (Bench 036) |
| emotion | A0 **0.8850** (the board's highest bar — every challenger measured out, Bench 038) | DEAD (Bench 032+035 screen) |

The six home-made harness families are RETIRED (owner call 2026-10-02,
reflex `31b11d2`) — board 15 → 9.

**Bench 0059 seat re-baseline (2026-10-08):** the arena/server seat = the
CURRENT published posture — head-select RETIRED (the site's rows are head-off
since the 10-01 full-pool re-basis, reflex issue 058; outcome-load-bearing only
on banking77) + the issue-079 `per_byte` drafter lever armed (the `d3aeeae`
site posture). The A0 identity pin is GREEN on all nine suites BOTH halves
(arena == run() == the published site row) — the first full-pin run since the
re-basis caught both drifts. Served cells digit-hold (registered == served,
no manifest edit); sst5's T2 reads REFUSED at the canonical population
disclosed in the bench record.

**Bench 0060 s1mb A0-pin close (2026-10-09, `60f373c`):** the trio's dataset
dirs wired into the canonical pool (`.raw/datasets_s1mb`, Plan 010's fetch —
m3 symlinks + 4090 copies), closing 0059's standing disclosure. **A0 pin
GREEN 3/3 both halves** — the first s1mb verification since the 650202c
section. s1mb_score's site row was genuinely stale (`0.4967 → 0.4982`, the
issue-079 `per_byte` vintage; +4 questions) — republished via the sanctioned
`publish_bench.py` with BOTH hosts refreshed in one publish after the
cross-host gate correctly REFUSED the m3-only update (4090 leg re-measured,
bit-identity 3/3). choice/noul bit-stable. Two fixes rode the bench: the
choice-pin red was a serde_json 1-ULP parse artifact (issue #505 class — the
17-digit literal parses to `…d63f` vs computed `…d640`; `float_roundtrip`
feature added; run-1's WIP attribution corrected), and the arena's
`RunOptions` gained `pplx: false` (the reflex-082 field add broke this
arena's build). Verdicts: choice a0_stands (A0 registered at the instrument),
noul's A1 T2-refused (LB95 −0.0169), score no-winner (Issue 010 T2). The site
rows' hybrid/encoder aux seat-readout fields remain b6425bf-era —
stale-but-unrendered, disclosed in the bench record.

- Composition (`src/hybrid.rs`): H1 cascade (reflex fused-gate pass-through
  → top-k prune → specialist over survivors) + H2 prior fusion
  (p′ᵢ ∝ pᵢ·exp(g·β·mᵢ), the katgpt-rs Proposal 013 shape) + the G0 kill-switch arm;
  allocation-free hot path (`tests/g4_alloc.rs`); the label join is a
  name-injection asserted both directions (seat⊆artifact).
- Arena (`src/bin/arena.rs`): the GOAT runner over reflex's ONE-WAY
  `harness::runner::seat` seam; the A0 drift pin asserts arena A0 == reflex
  `run()` byte-exact; auto-emits `hybrid_lane_doc.json` (measured numbers
  never hand-typed).
- Multi-question serve: `decide_multi` + the name-first noul law + the
  presentation-width guard (a noul presentation must be the pair); **S8**
  covers 8-label arity; the single-question form refuses 422
  `bridge_undefined`.
- `specialist::winner_bridge` = the ONE home for the per-suite winner file
  + input-bag convention (`check_winner_file` refuses a mismatched artifact
  for a bridged suite); `BagConvention` dispatches every bag site.
- `src/staleness.rs` (+ `examples/staleness_probe.rs`): the frozen-artifact
  staleness probe — REPORT-ONLY pre-swap impact read (fire rule
  `flips>0 OR mean|Δgold|≥0.02`), fixture
  `tests/fixtures/staleness_probe_set.json`, gates `tests/staleness_gates.rs`.
- Stats (`src/stats.rs`): Wilson bounds, paired non-inferiority
  (δ = max(1.0pp, 2.5·SE)), Pareto rank-0, Beta-LCB — the pre-registration
  instrument.
- Serve edge: std-only HTTP (`/decide`, `/healthz`, `/`); lanes boot on
  64 MiB-stack threads with the listener bound FIRST (loading/failed lanes
  answer 503 with the state named); every response carries the decision
  receipt (blake3 build fingerprint + BLAKE3(input) + BLAKE3(canonical
  decision) + lane id); machine-readable refusal `code`s; CORS via
  `RIIR_INSTINCT_ALLOWED_ORIGIN`, closed by default. The `/decide`
  envelope is the **v1 decision_wire contract** (the R7 freeze — reflex
  issue 074): `contract_version` on the request (absent = 1, unknown →
  400 fail-closed naming the supported set, echoed on every answer), the
  response field set frozen additive-only, and `<2` presentations refuse
  422 at the edge before any lane (a malformed body must never poison a
  lane's slot lock). The contract record:
  `../riir-reflex/.docs/03_decision_flow/wire_contract_v1.md`; the
  in-source law: `src/serve_edge.rs` `CONTRACT_VERSION`.
- Tetris lane (Issue 009, closed out of levers): the b0 champion stands as
  the modelless floor (907.8 / 532.5 pieces); every priced critic lever
  measured or closed; a re-open needs a lever bringing new information
  representable WITHIN the 1 ms serve bar (`src/tetris_critic.rs`, feature
  `tetris_forecaster`, bench `--model`).

### The encoder lane (pre-split records; the serving surfaces live in Rethink now)

- `posture ENC` served GPU hosts only (feature `serve-encoder`, which
  implies `serve-encoder-shared` — ONE encode worker per checkpoint, RAM
  2.5–3.1×, p99 0.37×; runtime demote switch
  `RIIR_INSTINCT_ENCODER_SHARED=0`). CPU-only deploy shapes carry the
  class-wide serve refusal (Issue 014) — every ENC cell publishes
  `serve: ✗`.
- **Q8 ADOPTED end-to-end** (Bench 0046: 14 paired reads, 5 PASS / 2
  UNDECIDED / 0 FAIL, retention 0.956–1.000; Bench 0047 read-back bit-exact
  — 842.6 → 447.7 MB = 53.1%): serve with `LAYA_WEIGHTS_VARIANT=q8`
  (env-only, sidecar BLAKE3-verified, never auto-derived).
- **Q4 FAILED** its probe (Bench 0055: banking77 Δacc −10.0pt UB95 −0.1410,
  retention 0.568) — `LAYA_WEIGHTS_VARIANT=q4` may not serve ANYTHING;
  measurement-only. Re-open bar: a head-class change (QAT/GPTQ-class, in
  riir-train).
- The DEFAULT teaching manifest is UNTOUCHED by the encoder arc (ENC rows
  are a GPU-host deployment surface; a build without the feature refuses an
  ENC row loud naming `--features serve-encoder`). D1 (an ENC cell
  publishing as served) stays trigger-blocked until a real GPU serving
  deploy exists; thai suites wait on the multilingual checkpoint
  (riir-train 603, 4090-queued).
- Arena fake-quant probes: `--fake-quant` (Q8) / `--fake-quant-q4` (Q4).
