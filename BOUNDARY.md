# riir-instinct — boundary contract

Visibility: private TODAY; **OPENS as the public teaching product at
Phase C** (riir-ai Proposal 052 — the owner's visibility flip is a
personal act; `publish = false` until the fence is GREEN on the
fresh-root export, which it is on this tree: `scripts/fence_gate.sh
--post-split`).

> The single source of truth for what may live in and depend on this repo.
> Audited by the `boundary-guard` skill and
> `../riir-ai/scripts/ci_boundary_contract.sh` (workspace dep graph +
> contract honesty). Cross-repo rules LINK to their one canonical home —
> never copied. Born 2026-09-26 per `../riir-ai/.proposals/047_riir_instinct_hybrid_decision_engine.md`;
> rewritten 2026-10-03 at the Proposal-052 split carve (riir-instinct Plan 008).

## Owns

**The open trained-specialists decision lane** ("Reflex · instinct") — the
teaching/product sibling of the public modelless engine `../riir-reflex`:

- specialist serving: load a sealed specialist artifact (bytes, never a
  path dep) and run its forward over the PUBLIC `../riir-reflex` embed
- the hybrid composition: the modelless lane (riir-reflex `nb_scope` count
  tables) prunes options to top-k / answers confident questions; the
  specialist decides the rest (escalation policy + calibrated confidence)
- the **arsenal selection protocol** (the manifest grammar + laws A5–A7):
  suite → artifact digest → class → serving posture → pins → budget,
  digest-validated at boot (drift fails loud); the embedded default is the
  TEACHING manifest (`data/arsenal.toml`, artifact-less A0 rows), pinned
  byte-for-byte by `tests/serve_gates.rs` (law A6) — and the PRODUCTION
  verdict is carried inline in the same test file, byte-pinned separately
- the **PUBLIC-RELEASE vessel boot**: reflexer's public decode/open over
  the operator pin table + the monotonic gate (the teaching arc's mint →
  manifest → serve path); the HOSTED-ONLY class refuses here
  structurally (fail-closed — the reader capability is not compiled)
- the **lane-backend extension point** (`server::LaneBackend` +
  `install_ext_boots` + the `AnySuiteServer::Ext` seat): the contract a
  downstream lane class implements to seat itself; the open build ships
  none — an ENC row refuses loud naming the seam
- the Tetris lane (public-by-design demo heads; the trained-critic record
  is measured-negative teaching gold)
- the measurement-honesty instruments (`staleness.rs`, `stats.rs`) and
  the arena/bench/serve gate suite
- the **L1 training lane** (Plan 008 B4, lifted from `../riir-train`
  2026-10-03): the Arm-A one-vs-all logistic specialist trainer + the
  NBSVM ridge mirror + the typed option-conditioned variant + the
  banking77 teacher-case law (`src/instinct_specialist*.rs`,
  `instinct_nbsvm`, `instinct_bank77_cases`) and the teaching recipes
  (`examples/instinct_arm_a{,_typed,_b}`, `instinct_bank77_cases`,
  `instinct_v2_gate`, `instinct_emotion_v2`) — the train → holdout +
  winner law → seal half of the teaching arc, runnable from a clone

## Does not own — the moat (born as the private `../riir-rethink` repo, 2026-10-03)

| Concern | Correct home |
|---|---|
| **The encoder arm** (the sealed NLEH head + live laya encode, serving + arena faces) | **`../riir-rethink`** (`src/encoder_serve.rs` + `src/encoder_arm.rs`; depends on this crate; plugs in via `install_ext_boots`) |
| **The HOSTED-ONLY vessel reader** (blake3-XOF, class gate, mint helper) | **`../riir-rethink`** (`src/vessel.rs`; this repo keeps the PUBLIC-RELEASE reader only) |
| **The economy plane** (decstat capture + receipt verify + floor runner) | **`../riir-rethink`** (`src/decstat*.rs`, `examples/decstat_floor.rs`; riir-dapps keeps settlement semantics) |
| The moat records/docs/artifacts (encoder/quant/distill benches 029–048·050–056 subset, deploy.yaml, the production arsenal.toml, .plans/002·006·007, .proposals/001, .issues/016) | **`../riir-rethink`** (born 2026-10-03 from the `moat/` seed at `4ffd08f`; this repo keeps `moat/README.md` as the pointer — the whole git history stays HERE) |
| Training, distillation, the self-evolve loop, vessel MINTING + lineage store | `../riir-train` (Issue 576; Research 457 moat map). Amended 2026-10-03 (Plan 008 B4): the L1 CPU trainers + teaching recipes moved HERE (the training lane above); riir-train keeps GPU training, the distill teachers, and the HOSTED vessel minter |
| Model forward kernels, loaders, tokenizers | `../riir-infer` (public; ships loaders, not weights) |
| The modelless engine + arena harness | `../riir-reflex` (public) |
| The vessel FORMAT crate (both classes' wire format) | `../riir-reflexer` (`reflexer-vessel`; capability features split the readers) |
| Game runtime / NPC cognition / the L0–L5 layer stack | `../riir-ai` (Proposal 047 §L0–L5) |
| Deploy orchestration (the open repo has no deploy.yaml) | `../riir-deployer`; Rethink's deploy shape is `../riir-rethink/deploy.yaml` |
| Settlement, decstat contribution rows, pricing | `../riir-dapps` (+ `../riir-kat` wire — a Rethink-only dep now) |

## May depend on

| Crate | Location | Condition |
|---|---|---|
| katgpt-core | `../katgpt-rs/crates/katgpt-core` | `default-features = false`, features `best_belief` + `sigmoid_calibration`. Consumed: `exact_sigmoid` (the score readout, never softmax — `src/specialist.rs`), `best_belief_score` (the ε-quantile Beta LCB — the pre-registration instrument's selector, `src/stats.rs`), `SigmoidGateCalibrator` (the G1 Platt face, the arena) — measured at P3 T1/T2 (2026-09-26/27) |
| riir-reflex | `../riir-reflex` | lib dep, features `modelless,nb_scope,nb_ridge,option_cond`; consumed: `embed::hashed_tokens_into` (the ONE tokenizer law) + the `nb_scope` tables for the hybrid composition + `harness::{suites, metrics, runner::seat}` (the seat — byte-identical questions) — measured at P3 T1/T3. **Never a dep on the harness's lane runners.** The opt-in `laya-face`/`laya-face-metal` features forward `laya-riir`/`laya-riir-metal` for the arena's G2 paired face (the rename of the retired `arena-laya*` spellings) |
| `reflexer-vessel` | `../riir-reflexer/crates/reflexer-vessel` | optional (the `vessel` feature); **public read only** — `decode`/`open`/`peek`/`PinTable`/`ApplyState`/`check_monotonic`; the HOSTED-ONLY class refuses by the format crate's own law, which is exactly why the hosted reader lives in the moat — the public boot measured at the B2 move (2026-10-03) |
| katgpt-tetris | `../katgpt-rs/crates/katgpt-tetris` | the Tetris lane's shared engine (Issue 009 T5/T6): sim + lookahead + champion genome, opt-in `tetris` feature — never a fourth engine; NOT a game runtime. Consumed: `sim`, `lookahead`, `rulebook` — measured 2026-09-27 |
| katgpt-core `karc_forecaster` | `../katgpt-rs/crates/katgpt-core` | in the `tetris` feature set (Issue 009 T8's modelless arm): the Plan-308 KARC fit substrate. Opt-in with the lane — measured Bench 009 |
| fastrand | crates.io | version-matched `"2"` (one copy compiles); the teacher-search RNG. Opt-in behind `tetris` |
| ed25519-dalek | crates.io | **dev-dependencies only** (2026-10-03, the carve): test-side vessel MINTING (`arsenal_ops`'s identity tests); the lib surface carries zero ed25519 |
| toml | crates.io, `default-features = false`, feature `parse` | the arsenal manifest parser: schema types + boot validation in `src/arsenal.rs`; parse-only; non-optional |

**Removed at the carve (2026-10-03), now moat-only deps: `riir-kat`,
`papaya`, `ed25519-dalek` (lib), the hosted `reflexer-vessel` reader use.**
The `[patch."https://github.com/katopz/katgpt-rs"]` table left with them
(it existed for the riir-kat → riir-auth → riir-neuron-db git-reached
tree; no git-URL katgpt dep remains in this graph).

Next planned rows (land WITH their first consumer, each measured):
none today. **Never riir-ai** (this repo sits beside riir-ai, not
downstream of it). No game crates, no Python, no candle.

## Standing invariants

- **The fence** (Research-003 amendment #3 shape): `scripts/fence_gate.sh
  --post-split` GREEN is the open tree's precondition — no moat files, no
  moat code references, allowlist deliberately empty. `--history` GREEN
  (single root, zero moat paths) is the owner's pre-flip read on the
  Phase-C export. `publish = false` until then.
- **The moat is downstream**: Rethink depends on THIS crate and plugs lanes
  in via `install_ext_boots` — never a fork of `server.rs`/`serve.rs`/
  `arena.rs` (the 6.3k-line entanglement verdict).
- **Weights never enter any repo** (`.gitignore` refuses `*.vessel`,
  `*.bin`, `*.safetensors`, `*.gguf`; `/data/demo_specialists/` joins them
  at the carve — Phase C mints demo winners into it).
- **No-cheat protocol** (riir-reflex Issue 038): train rows only for any
  corpus/weights; select on held-out train; arena test split read once.
- **Training scope** (amended 2026-10-03, Plan 008 B4): the L1 training
  lane IS in-tree as teaching code — the CPU, modelless, public-data
  trainers (`src/instinct_specialist*.rs`, `instinct_nbsvm`,
  `instinct_bank77_cases` + the `examples/instinct_*` recipes), so the
  teaching arc (fetch public data → train → holdout + winner law → seal
  → mint → serve) runs from a clone. What stays OUT: GPU training,
  distill and the self-evolve loop (`../riir-train`), and the moat
  encoder trainers (`../riir-rethink`). Heavy/artifact-producing runs
  write to `data/trained_specialists/` (gitignored), never into the
  serving default.
- **Demo ≠ production**: the embedded manifest serves the modelless tier
  from a fresh clone; the production verdict is Rethink's. Teaching
  numbers never join the board.

## Inherited boundaries (links)

- Dep direction: `../riir-ai/BOUNDARY.md`
- Public/private split: `../riir-ai/.research/003_Commercial_Open_Source_Strategy_Verdict.md`
  (+ the 2026-10-03 amendment #3: Instinct opens, Rethink stays private)
- The moat seed's own contract: `moat/BOUNDARY.md` (the Rethink-side rows,
  carried at the wave)

## Drift ledger (target vs actual)

**None.** (The carve landed with the registration one-commit-family law
honored in-repo: BOUNDARY.md rewrite + moat/BOUNDARY.md + the manifest
finalize are ONE push; the workspace-wide repo_set/pin-file registration
is wave-gated — a registered-but-absent repo is an UNSEEN red in every
population check, so the contract stays honest by registering at birth.)
