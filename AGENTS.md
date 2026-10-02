# AGENTS.md — riir-instinct (private)

The global `~/.agents/` rules apply; this file documents repo-local context.

## Naming law — this repo becomes `riir-rethink` (riir-ai Proposal 051)

**Decided 2026-09-30 (owner call); the mechanical rename is Phase 1, not yet
landed** — until it is, the on-disk directory, git remote, and workspace
registration still read `riir-instinct`. The law, in force now:

- **The product is `riir-rethink`** — the all-tier adaptive decision-serving
  family (L1–L5 rungs inside ONE product; tiers are rungs, never per-tier
  lanes — the reserved `riir-director` is retired, L5 curation is a job).
- **"Instinct" retires from product/brand naming** and returns to meaning
  exactly one thing: the game-side L1 tier word (riir-ai Proposal 048's map,
  `Instinct (basic)` / `Instinct (vessel)`). Site lanes rebrand `Instinct
  (hybrid)` → **`Rethink (hybrid)`**, `Instinct (encoder)` → **`Rethink
  (encoder)`** (aliases keep history landing).
- **Three registries, one table** — game tier / repo / product lane. The
  disambiguation table's third axis (incl. the `rethinks`/`think_every`/
  `since_rethink` row) lives in ONE home:
  `../riir-game-sdk/.docs/10_multiplayer_topology/tick_tier_model.md` §(a).
  The hero-strategy `rethinks` counter (`riir-games-mmorpg/src/hero_strategy_moe.rs`)
  is game-side cadence vocabulary — the aligned collision this product name
  adopts, never a reference to this repo. Greps must not cross the streams.
- **Recorded fallback:** `riir-cogito` (zero grep hits, same meaning) if the
  collision tax ever compounds (051 caveat 1).

## Boundary contract — read `BOUNDARY.md` first

[`BOUNDARY.md`](BOUNDARY.md) is authoritative. **Domain test:** is this the
model-based or hybrid decision lane (specialist serving, modelless-drafter
composition, HOSTED-ONLY vessel reading, instinct arena rows, hosted
deploy)? NO → it belongs in another repo; file there. Boundary checks run via
the `boundary-guard` skill (`../riir-ai/scripts/ci_boundary_contract.sh`).

## Role

The trained sibling of the modelless engine — per Proposal 051, the
**all-tier adaptive decision-serving family** (not just the L1 "learned fast
response" of the original 047 naming; the rungs span L1 bags/hybrids through
L3 encoder thinks, with L4/L5 rungs gated on 048's). **Reflex** = hard-wired
response (`../riir-reflex`, modelless, public, the free floor — NOT a rung in
this product); **Rethink** = this repo (trained/adaptive, private). The flow
is riir-refine's, end to end:

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

**Issue 016 T1–T4 LANDED (2026-09-30, Bench 031 + instinct `82011af` + riir-ai
`0c39b4f30f` + riir-train `f3ce8973`): the encoder class now SERVES — `posture
ENC`, feature `serve-encoder`, GPU hosts only — and the client + vessel halves
ship beside it. T7 (the emotion screen) EXECUTED 2026-09-30 — DEAD BY THE SCREEN
(Bench 032: the reference floor 0.5950 vs the 0.8850 bar; the class's max lift
cannot close it — the train run never earned; emotion stays A0, the posture-gap
risk stands) and CROSS-CHECKED 2026-10-01 (Bench 035, the Issue-825 double-run:
an independent 5-head sweep refused every head — holdout 0.459–0.4835 vs the
reference's 0.4975; the best head reads 0.6200 on test, 26.5 under the bar; the
Bench-032 cells reproduce EXACTLY at the 4090 CUDA posture — dead by screen AND
by sweep, the frozen read never spent in either session).**
`src/encoder_serve.rs`: the sealed NLEH head + the laya-english agent
resident from boot (the boot warmup encode pays the pipeline compile, L9);
the agent on a dedicated worker thread (the reflex !Send law — clients get
a Send+Sync channel handle, forwards serialized); the C1 canonical-
presentation contract enforced (template criteria order is the presented
space — NOT `seat.labels`, which for a score suite is index strings; and
the suite's serialized-state form parsed back into the envelope Value so
`serialize_state` re-derives the arena's exact prompt bytes — both caught
by the gate's first red run at 198/600). Routing: `Arm::Enc` in
`server.rs` (bag server refuses ENC by construction; vessel boots refuse
ENC until T4's mint); grammar in `arsenal.rs` (ENC rows: explicit `file`,
eager-only — a lazy ENC row is an L9 violation). **Issue 018 Update 10
(2026-10-02, owner-approved): `serve-encoder` IMPLIES `serve-encoder-shared`
— ONE encode worker per checkpoint is the ENC default** (RAM 2.5–3.1×, p99
0.37× under contention, byte-identical decisions; Benches 0050/0054 + the
0054 addendum). The runtime demote switch **`RIIR_INSTINCT_ENCODER_SHARED=0`**
(the exact literal) bit-restores per-lane keys; `encoder_topology_label()`
is the one topology-label home. The arena's fake-quant probes:
`--fake-quant` (Q8, Lane D1 — Bench 0046 PASS, the Q8 tier adopted) /
`--fake-quant-q4` (Q4, Lane D4 — Bench 0055, Update 11: **FAILED** — banking77
Δacc −10.0pt UB95 −0.1410, retention 0.568; `LAYA_WEIGHTS_VARIANT=q4` may not
serve anything, the Q4 tier stays measurement-only, re-open bar = a head-class
change).
**The DEFAULT manifest is
UNTOUCHED** (the byte pin holds; sst5 keeps serving A1) — ENC rows are a
GPU-host deployment surface; a build without the feature refuses an ENC
row loud naming `--features serve-encoder` (014 still governs every CPU-
only deploy shape). Gates: `tests/serve_encoder_parity.rs` (the frozen
Bench-029 read replayed **316/600 EXACT** through the serve surface at the
CPU posture + arena-runner pick parity 32/32 in-process + determinism +
the 1 s L3 slot bound) + ENC grammar/refusal arms in `serve_gates.rs` +
e2e HTTP smoke (healthz `arm: "ENC"` / decide 200 + the score vector
crossing, the embedding never does). T1's derivation (the G2 GPU-s/s unit
+ the admission-cap formula + the stand-in cells) is in the same record.
D1 (the sst5 cell publishes as served) stays trigger-blocked until a real
GPU serving deploy exists. **T3 (riir-ai `riir-agents/src/decision_client.rs`,
feature `decision_client`): the decision-wire thin client — one std TCP POST
under a per-call L3 deadline, every failure path collapsing into
`DecisionClass::BagFallback` (the record names WHICH class answered;
late-but-good responses dropped on arrival; the connect phase capped at
min(remaining, 250 ms) for the Windows refusal-latency fact); 9 module
tests incl. the G1b deadline-miss injection + guard Layer 1.33. WIRE-ONLY
per the boundary law (zero deps on the serving repos). **T4 (instinct
`82011af` + train `f3ce8973`): the HOSTED-ONLY head vessel —
`load_hosted_head_bytes` (same walk, raw NLEH payload out),
`boot_vessel/_bytes` ENC routes on the arsenal's existing monotonic apply
(no new lineage code), `vessel-mint` accepts NLEH; the round-trip gate
proves vessel-lane == raw-lane + the downgrade refusal; the production
mint is the owner's key ceremony.**

**Issue 016 T8+T9 COMPLETE (2026-10-01, Benches 039/040/041; instinct `73cd6d6`
+ `10f6b8e`): the typed_decisions head run was EARNED, trained, and the cell
SEATED record-only — `serve: ✗` (class-wide refusal stands; D1 trigger-blocked).**
T8 screens (Bench 039): banking77 DEAD (floor 0.4980 + max lift 0.645 < 0.8540),
prompt_injections DEAD (0.6983 + 0.147 = 0.845 < 0.8534), typed ALIVE — the
typed-checkpoint floor 0.7445 reads +9.7 pt over the 0.6475 seating bar before
any lift; the three wire-fidelity witnesses reproduce published board reads
byte-exactly. T9 train half (Bench 040): the **NLEH v2 per-option head** (the
scoring shape the v1 codec cannot express) trained EARNED (holdout 0.7975 vs
the reference's 0.7762) + the single frozen read **0.7550** — +10.75 pt over the
0.6475 displayed arm (`../riir-train/.raw/t608/typed_encoder_v2.bin`, blake3
`c2e9f346…`). T9 seat half (Bench 041): `encoder_arm.rs` reads v2 — the law-copy
mirror of riir-train's t608 codec, per-QUESTION flattening (typed's 5 q/case →
2000 rows, the A0 arms' convention), `--encoder-ckpt typed` (parse-time
validated; v1 keeps the C1 one-question law byte-identically) — and the
**cell-identity witness HOLDS: 0.7550 (1510/2000) EXACT on the live m3-Metal
typed encode**, reproduced on both runs (the third-posture law). En-route: a
PRE-EXISTING latent `laya_face` index bug fixed (per-question escalation indices
mapped into `cases[]` — valid only at 1 q/case; multi-question suites now SKIP
the face loudly, never mis-measure). The lane doc is AUTO-EMITTED (see 017 T5).
Board: the Rethink lane grew 3/9 → **4/9** (sst5 · xnli · ag_news · typed — the
strongest cell vs the displayed arm, +10.75 pt; reflex-site `245bf34`, CF
`8abd1add`, the Issue-021 latency ack recorded: ambient load, the 364 ms figure
is the 3-orders-over-bar magnitude evidence). REMAINS T5–T6 (G2/G4 land with
the consumer; T6 = D1, trigger-blocked); thai suites wait on the multilingual
checkpoint (riir-train 603, 4090-queued); the 6 harness families are law-excluded.

**Issue 017 CLOSED (2026-10-01, Bench 036 + 037; instinct `cdc92d1` + `b03fded`
+ `2960701`, reflex-site `70d9e80`, CF `eeee1b19`): the owner's progress-display
call — seat the measured xnli cell as-is, record-only — reversing the
014-round-3 "no separate xnli cell" decision ("no progress at all" reads worse
than a seated −4.0).** xnli_en: **ENC 0.8600 (258/300)** cell-identical to 599
T5a (the third-posture witness), vs A1 mean +0.4500 · LB95 +0.3832; p50
231,709 µs (ms-class — the recorded ground of the 014 refusal, now QUOTABLE
under the preflight-cleared box state); the head ties the laya-english reference
300/300 picks, carried as the cell's reference-identified disclosure. ag_news
seated (Bench 037): **ENC 0.9475 (379/400)** cell-identical to 600 T1, +5.0 pt
over the serving H2 0.8975 (LB95 +0.0320), −0.25 pt under its own laya-english
reference; NOT seated anywhere the encoder reads WORSE (massive 0.675 < 0.827,
emotion ~0.62–0.74 < 0.885 — the better-only law). The owner's two follow-ups
folded: **the arena AUTO-EMITS `hybrid_lane_doc.json`** on every `--encoder-art`
run (`write_encoder_lane_doc`, `--encoder-note` carries the train-side
attribution prose — measured numbers never hand-typed), and **the vs-best card
counts the FAMILY arm** (best measured cell of Instinct-hybrid · Rethink-encoder
per suite; encoder rows violet + record-only tag; the coverage literals are
DATA-DERIVED in both smokes, the page prose count-free). Attribution honesty
KEPT as disclosure, not concealment; 014's class-wide serve refusal and 599's
record STAND — 603's multilingual screen cannot unseat these cells either way.

**Bench 0029 + Plan 426 T6 (2026-09-30): the massive SYNTH SEAT serves —
H2(β=1,nmin=2,τ=4) 0.8400 (was 0.8267), the modelless A0 0.7800 → 0.8133
(== reflex bench 091's V5 anchor EXACTLY), every arm rose.** The synth
corpus (2048 openthai-vetoed template×slot rows, blake3 `8eed806a…`, extra
cap 128/label) seats via reflex `prepare_seat_with_synth` (blake3-verified,
suite-header pinned, in-universe drop counted) applied at
`build_seat_engine` — the exact V5 arm-B construction, SHARED with the
corpus-ab lane via `specs_corpus_extended` (the measured build and the
served build are the same code). The manifest row moved τ=8 → τ=4 (the
certified gold-seat pick did not survive the stronger floor: T2 LB95
−0.0039, mean +0.0267 — the banking77 law, serves under best-measured);
A6 digest re-pinned `cf851e27…`; the massive serve gates boot the synth
seat against 0029's frozen picks (17/17). Serve-side env
`INSTINCT_SYNTH_CORPUS_DIR` (absent = gold, byte-identical). Vs-best:
−9.3 → **−8.0** (openthai 0.9200 holds; 008 T7 massive open). Owner-side
remainders: the deploy files row + env at the next deploy; the site cell
at the next republish.

**Issue 014 CLOSED (2026-09-30): C1 EXECUTED POSITIVE (Bench 029 — the
sst5 encoder-arm lane ON RECORD at ENC 0.5267, T2-certified +10.50 over
A1, published `serve: ✗`; the owner's class-wide staged GO refused
per-request encoder inference at serve) · C2 EXECUTED NEGATIVE (riir-train
Issue 602 / Bench 615 — the static-vector surrogate reads 0.3917 vs the
0.4383 gliner bar, paired LB95 −0.0691 vs A1; latency PASSES at p99
21.4 µs but accuracy binds). The failure path executed: **sst5 stays on
A1 (0.4217)**; the extraction substrate (tokenize_question + TOKN
sidecars + TABL/TSFT/NSUR + the µs-class allocation-free serve path)
stands reusable for any future C2-class rung on a bar-lower or
order-insensitive suite.**
`src/encoder_arm.rs` + the arena's `--encoder-art` flag (features
`arena-laya`/`arena-laya-metal`): the sealed NLEH v1 head
(`.raw/t599/t6_s0.bin`, BLAKE3-verified) replayed over a LIVE M3-Metal
laya-english encode of the seat's own test cases — a byte-lawful
consumer-side re-implementation of riir-train's codec + feature law
(never a trainer dep; the question wire via reflex's `case_questions`,
made `pub` for this — the ONE render-law home). **THE FROZEN READ: ENC
0.5267 (316/600)** — cell-identical to the trainer's read for the THIRD
posture (4090 cache-witness → M3-Metal dump → live arena encode); **T2
CERTIFIED vs the incumbent A1: mean +0.1050, paired LB95 +0.0535** (A1
0.4217 serves UNCHANGED — no manifest change; A0 0.3967 == reflex
run()). Per-row latency p50 14,709 µs / p99 17,452 µs on Metal — ~49×
the ~300 µs provisional text-lane bar, on a device the deploy shape
(CPU-only standard-2 cf-container) does not carry. LIVE on reflex.gist.rs
as the `Instinct (encoder)` lane (reflex-site `b9b2ee1`): its own filter
chip + the sst5 cell beside the unchanged serving cell; the instinct.js
verdict bars deliberately EXCLUDE the serve-refused lane. En-route: caught
+ fixed a REAL T8 serve regression — the name-first noul law (`2d20397`)
dropped the presentation-width guard (a hostile 3-option noul presentation
silently answered over the fixed pair); the width guard restored on the
pair_named branch, serve_gates 15/15. Record
`.benchmarks/029_sst5_encoder_c1/`; full lane history: Issue 014.

**Issue 008 T8 RESOLVED (2026-09-30): the seven 0.0-edge ties are gone —
code_fixtures BROKEN by a real specialist, the six harness families DROPPED.**
The T8 tie-break lane ran the pre-registered protocol end to end: trainer
floor 0.45 → the wave-1 per-head NBSVM v2 minted at holdout **0.5536**
(reproducing wave-1's number exactly — a determinism witness; blake3
`264714b9e7518e33…`); **THE FROZEN READ: A1 0.5625 vs A0 0.3750 — T2 PASS**
(mean +0.1875, paired LB95 **+0.0021** — the pre-registered marginality, +6
net questions is the whole n=32 margin; G1 PASS; A1 at 2 µs p50 vs A0's
290 µs). Record `.benchmarks/028_code_fixtures_tie_break/`. The suite serves
A1 (the 9th manifest row; still −6.3 under the paw bar → unsold under the
amended law, untied). Serving-path landings that mint required: the
winner_bridge entry (Presence + the **MultiQuestion** contract — every case
carries TWO questions; the SingleQuestion shape guard refuses that by
construction), **S8** in AnySuiteServer (the serve path had no 8-label
arity), and the **name-first noul law** in `decide_multi`'s Named join
(NOUL_PAIR through the key map first — the arena's fill_positions law; the
pair may live in the ARTIFACT while the seat offers only the 8 module
labels; prompt_injections' positional law byte-preserved as the fallback).
The families' drop reason is measured (n=12–16 template-shared eval — a win
would be unfalsifiable memorization; recorded in 008's T8 block); the lane
re-opens only with a larger template-disjoint reflex-side eval. The site
half (code_fixtures cell serves 0.5625; the six family cells leave the
card) LANDED LIVE 2026-09-30 (reflex-site `7eeb936` + the instinct lane-doc
`0d044e8`: cell serves A1 certified LB95 +0.0021, the six harness rows
dropped, smoke floors now data-derived — board gap −25.0 → −6.3 vs the paw
bar).

**Issue 008 T7 wave 1 (2026-09-29, owner GO): six vs-best specialist lanes
measured, all NEGATIVE — the sold set is unchanged (the seven serving rows of
Bench 019/020).** sst5's holdout gate PASS (the 579 v2 lever, +3.10 pt LB95
+0.0076) was REFUSED at the single frozen arena read (Bench 024: A1 0.4217 ==
the v1 row, T2 LB95 −0.0151, gliner's 0.4383 bar unmet) — A0 serves, the
bridge row reverted same-session. code_fixtures' flat-union collapse was found
+ fixed (per-head training + merge) and still read 0.5536 vs the 0.72 floor.
massive/ag_news/xnli holdout negatives; typed rides the 581 ceiling. The
re-open bar is a model-CLASS upgrade, never another bag sweep. Full record:
HISTORY.md 2026-09-29. Issue 005 H3 closed premise-refuted (typed questions
are INDEPENDENT-GIVEN-CASE — no tree). **Issue 009 T7 CLOSED-OUT OF LEVERS
2026-09-30: round 5's teacher A/B (Bench 026, z-form) measured NO-GO beside
the residual pilot's null (Bench 027), and round 6 (the label-blend, the
last priced lever) was analytically closed under verdict — self-distillation
fixed point + the decisive ground that a win could not ship (the r4 config
runs 3.4–3.8× over the 1 ms serve bar). Every recorded lever measured or
closed (capacity 010/018/021, loss 017, teacher blend 026+027, label blend
round-6); b0 stands; the issue stays OPEN owner-visible with T1/T2/T4/T9
gated on a future lever that brings new information representable WITHIN
the 1 ms bar (the T5 widened-wire side, or a different teacher signal);
eval seeds at 5 reads.**

**Bench 022 + Plan 005 (2026-09-29) — the frozen-artifact staleness probe
LANDED (Issue 012, resolved + removed): the soft early-warning readout beside
the hard pick-parity gate — arXiv:2609.30652's fixed-trace probe, modelless
form, at the ARTIFACT plane (the specialist sigmoid score is the surface
serving inherits). `src/staleness.rs` (pure, ungated: ProbeSet + BLAKE3
digest, per-item gold Δ / pick-flip / margin-Δ divergence, the pre-registered
fire rule `flips>0 OR mean|Δgold|≥0.02`, `compare_pair` = the probe AND the
pre-swap impact report — same measurement, different intent) +
`examples/staleness_probe.rs` (build/run) + the committed fixture
`tests/fixtures/staleness_probe_set.json` (192 items, label-stratified
round-robin — first-N is wrong on the clustered banking77 mirror; digest
`98a683969c285f157a3d81de7ffbdabebfac77873ce57eaa4b33ffe298d20f93`; probe
time reads NO dataset dir) + `tests/staleness_gates.rs` (6 count-pinned
tests, skip-loud without material). MEASURED (`.benchmarks/022`): **NOT
dead-by-domination** — the real Bench-012 bridge drift (banking77 v1 count →
v2 presence) reads 13/64 pick flips, mean |Δgold| 0.240, max 0.852 → FIRED;
byte-identical pairs (v2-vs-self canary + the emotion armA==winner control)
read EXACTLY 0.0 everywhere; the ag_news candidate pair reads divergent too
(1/64) — a swap's blast radius is measurable BEFORE the monotonic apply.
Each artifact scored under its OWN bag convention (v1=count, v2=presence —
scoring v1 under presence would read a model that never existed). REPORT-ONLY
by law; deferred (gated on a real refresh decision): the arsenal_ops swap-path
hook + the overlay-refresh policy arm (issue part 3; the plan's T6).
`.benchmarks/.highwater` repaired 0015→0022 (it was STALE — dirs 016..021
landed without a bump); en-route took the pre-existing-at-HEAD
`server.rs` needless_borrow one-liner in passing (would have red the
`-D warnings` gate).

**Bench 019+020 (2026-09-28) — the Issue-005 option-conditioned margin
ARMED and SERVING: typed_decisions serves the certified H2(β=0.5,
nmin=2,τ=2) 0.6475** (was A1 0.6300). The margin source is reflex
issue 038 T7b's `(qid, option)` count tables, exposed to the seat via
reflex `b5cf4b0` (`oc()` + `seen_count` — Issue 005's H2/H3
precondition). Bench 020 (full pool): certified against EVERY leg —
vs A0' 0.5725 (T2 LB95 +0.0580), vs A1 +1.75 pt (LB95 +0.0062 — the
first arm to beat its own specialist at LB95 > 0), vs H1 +1.4 pt
(LB95 +0.0034); G1 PASS. Bench 019 (the 15-suite regression): every
other published row byte-identical. Serving landed: the manifest row
A1 → H2, digest re-pinned, the serve-gate parity replays the frozen
H2 picks (14/14) — and the parity gate CAUGHT the serve engine
silently carrying `oc_select: false` (a pure-A1 fusion would have
served wearing the H2 name; the knobs now mirror the arena's).

**Bench 016 (2026-09-28) — `harness_cache_reuse` SEATED (Issue 010's OWED
item): the arena population is 15 suites, every reflex dataset suite now
carries a measured row.** A0 0.9167 == reflex `run()` == Bench 072,
verdict `a0_stands` (no specialist artifact); the noul polarity armed
through the seat's synthetic cal-front selection fallback. The other 14
suites reproduced their published rows byte-identically, serving arms
included (massive H2 0.8267 T2-certified, banking77 H2 nbsvm-v2 0.8540,
typed A1 0.6300, prompt A1 0.8534). Records under `--skip-pin-a0` — the SITE half VERIFIED DONE 2026-09-28
(reflex-site `74b49e4`: all 15 hybrid rows seated and live on
reflex.gist.rs, curl-verified, chart_render_smoke + bench_page_smoke
PASS); `--skip-pin-a0` remains the documented posture for the OWNER-GATED
residual — the cross-lane pool/engine divergence (site modelless rows are
reflex HEAD on the canonical pool; hybrid rows are the t20k seat), now
recorded in the owner-gate pickup (Issue 013). Issue 010 closed + removed
(HISTORY.md hygiene section, 2026-09-29). Two instrument facts written down: the arena's default datasets
dir is the FROZEN Bench-005 re-baseline pool (`datasets_t20k`) —
re-pointing it is a re-baseline decision, never a cleanup (one run of
this bench against reflex's canonical `.raw/datasets` pool matched no
published record and was discarded); and `typed_decisions` in that pool
still reads the stale 800-row pull (0.4655 == the site row) — the
full-pool lane is Bench 015's scratch-dir read until the canonical pool
is re-baselined and republished. Lane doc rebuilt at
`016_…/hybrid_lane_doc.json` (the reflex-site session's input).

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
sender) still gates the SERVE path (T9; the reflex contract half
LANDED — reflex `0c9fcab`'s named optional sidecar field,
parse-and-ignore byte-identical).** **T7 rounds 2–3 EXECUTED 2026-09-28
(Benches 017 + 018, both single-lever reads on the same seeds): round 2
— the tail-targeted loss (`--rank-gap 1.0 --bce-tail 4.0`, digest
`66069784fb0fadd1`) went BACKWARDS (play 554.6→173.5, agreement
0.711→0.627; the pair over-fit the low-q tail; the levers were
confounded — isolation first if revisited). Round 3 — the larger fit
(`--hidden 256,256`, digest `bf9464925be910d8`, everything else
round-1) is the STRONGEST critic yet: agreement 0.7574 (trainer T2
gate INTERESTING for the first time), play 743.9/455.7 vs b0
907.8/532.5 — the ridge half passes in both regimes, the b0 half
still fails but the W/T/L moved to 1/14/5 (ties where it survives as
long as b0; the catastrophic-pick deficit is down to ~25% of games).
Width→play is monotone across rounds 1/2/3; b0 stands, no mint.
Round-4 levers priced: 512×512 (~4–5 h) or critic-guided search (the
~240-eval next-piece expectation is TIGHT against G2 at 0.84 ms —
budget re-derivation before building). The eval seeds have been read
four times (009/010/017/018), every read a pre-registered lever,
every negative recorded.** Master plan:
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
