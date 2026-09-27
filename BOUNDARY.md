# riir-instinct — boundary contract

Visibility: private

> The single source of truth for what may live in and depend on this repo.
> Audited by the `boundary-guard` skill and
> `../riir-ai/scripts/ci_boundary_contract.sh` (workspace dep graph +
> contract honesty). Cross-repo rules LINK to their one canonical home —
> never copied. Born 2026-09-26 per `../riir-ai/.proposals/047_riir_instinct_hybrid_decision_engine.md`.

## Owns

**The model-based and hybrid decision lanes** ("Reflex · instinct") — the
private, trained sibling of the public modelless engine `../riir-reflex`,
shaped like `../riir-clippy` (private product engine over public katgpt-rs):

- specialist serving: load a sealed specialist artifact (bytes, never a path
  dep) and run its forward through `../riir-infer` loaders
- the hybrid composition: the modelless lane (riir-reflex `nb_scope` count
  tables) prunes options to top-k / answers confident questions; the
  specialist decides the rest (escalation policy + calibrated confidence)
- the HOSTED-ONLY vessel READER (the class `../riir-reflexer` deliberately
  refuses): verify → decrypt → apply, whole-snapshot, monotonic
- the instinct arena rows (harness lane beside modelless / laya) and the
  hosted-lane deploy manifest (`deploy.yaml`, via `../riir-deployer`)
- the **arsenal selection protocol** (Proposal 001 T1/T2): the
  `arsenal.toml` manifest — the ONE suite → artifact digest → class →
  serving posture → pins → budget surface for the hosted serving lane,
  digest-validated at boot (drift fails loud); the hard-coded posture
  match table and suite const it replaced are deleted, and the embedded
  default is pinned byte-for-byte by `tests/serve_gates.rs` (law A6)

## Does not own

| Concern | Correct home |
|---|---|
| Training, distillation, the self-evolve loop, vessel MINTING + lineage store | `../riir-train` (Issue 576; Research 457 moat map) |
| Model forward kernels, loaders, tokenizers | `../riir-infer` (public; ships loaders, not weights) |
| The modelless engine + arena harness | `../riir-reflex` (public) |
| The vessel FORMAT crate (public read/verify for PUBLIC-RELEASE) | `../riir-reflexer` (`reflexer-vessel`) |
| Game runtime / NPC cognition / the L0–L5 layer stack | `../riir-ai` (Proposal 047 §L0–L5) |
| Deploy orchestration | `../riir-deployer` |
| Settlement, decstat contribution rows, pricing | `../riir-dapps` (+ `../riir-kat` wire) |

## May depend on

| Crate | Location | Condition |
|---|---|---|
| katgpt-core | `../katgpt-rs/crates/katgpt-core` | `default-features = false`, features `best_belief` + `sigmoid_calibration`. Consumed: `exact_sigmoid` (the score readout, never softmax — `src/specialist.rs`), `best_belief_score` (the ε-quantile Beta LCB — the pre-registration instrument's selector, `src/stats.rs`), `SigmoidGateCalibrator` (the G1 Platt face, the arena) — measured at P3 T1/T2 (2026-09-26/27) |
| riir-reflex | `../riir-reflex` | lib dep, features `modelless,nb_scope`; consumed: `embed::hashed_tokens_into` (the ONE tokenizer law — the specialist's training-side `events_into` is the same bytes) + the `nb_scope` tables for the hybrid composition + `harness::{suites, metrics, runner::seat}` (the seat — byte-identical questions + the deployed posture, Issue 003 T3) — measured at P3 T1/T3. **Never a dep on the harness's lane runners** (the arena calls `run()` only for the A0 drift pin) |
| `reflexer-vessel` | `../riir-reflexer/crates/reflexer-vessel` | default features; the vessel FORMAT crate — `peek`/`PinTable`/`commitment_of` only (the reader assembles authenticity from the format crate's own primitives; its `decode` refuses HOSTED-ONLY by its own law, which is exactly why the hosted reader lives here) — measured at P4 T1–T5 (2026-09-27, `tests/vessel_gates.rs`, 7 arms). (Backtick form: the parser's convention for a non-`riir`/`katgpt`-family name.) |
| riir-kat | `../riir-kat` | `default-features = false`, feature `kat_transport` (implies `client`); the decstat wire client (Plan 002 / Issue 004 T1–T2, lane `dec`): `kat_protocol_decstat` composer + `push_decstat` transport. Opt-in `decstat` cargo feature — default builds never resolve it. Consumed as a CLIENT: no ledger/settlement semantics here (riir-dapps owns those) — measured 2026-09-27 (`tests/decstat_gates.rs`); its `client` tier implies `account_key` → riir-auth → riir-neuron-db, which is why the root manifest carries the katgpt-rs `[patch]` table (the C7 double-resolve law — a git-reached katgpt dep compiles twice without it at this build root) |
| katgpt-dec / katgpt-device-verify / katgpt-hla / katgpt-micro-belief / katgpt-personality / katgpt-sense / katgpt-types / katgpt-attn-match | `../katgpt-rs/crates/*` | `[patch]`-section only (the riir-auth 5856d61 convention, landed at Plan 002 T1) — unified-pin routing of the neuron-db git deps onto this working tree (the boundary gate's C7 law); NOT direct imports: no instinct module reaches for them beyond what the riir-kat → riir-auth → riir-neuron-db surface compiles |
| katgpt-tetris | `../katgpt-rs/crates/katgpt-tetris` | the katgpt-rs substrate module the leaf law admits (the same crate public `../riir-reflexer` consumes): board sim + seeded 7-bag + garbage starts + the frozen Bench-892 champion genome — the SHARED ENGINE of the Tetris lane (Issue 009 T5/T6; never a fourth engine). NOT a game runtime — the "no game crates" row targets riir-ai's game stack. Opt-in `tetris` cargo feature — default builds never resolve it. Consumed: `sim`, `lookahead::{Bag, apply, garbage_board}`, `rulebook::{Genome, Leaf, View, decide}` — measured 2026-09-27 (Bench 007/008, `src/tetris_lane.rs`) |
| fastrand | crates.io | version-matched `"2"` (the katgpt-core/katgpt-tetris pin — one copy compiles); the teacher-search RNG (`tetris_lane`), seeded per (seed, budget). Opt-in behind the same `tetris` feature — measured 2026-09-27 |

Next planned rows (land WITH their first consumer, each measured):
`riir-infer` (model forward loaders, HOSTED-ONLY lane). **Never riir-ai**
(this repo sits beside riir-ai, not downstream of it). No game crates, no
Python, no candle.

## Landed rows

| Crate | Location | Condition |
|---|---|---|
| ed25519-dalek | crates.io, default features (the format crate pins the same line) | the signature/seal primitive behind reflexer-vessel's own ed25519-STRICT verification + the gate fixtures' minter; no other crypto dep (confidentiality is BLAKE3-XOF keystream, already in-tree) — measured at P4 (2026-09-27) |
| papaya | crates.io | the decstat sink's lock-free counters (version-matched to the riir-auth pin already in this graph); opt-in behind the same `decstat` feature — measured 2026-09-27 |
| toml | crates.io, `default-features = false`, feature `parse` | the arsenal manifest parser (Proposal 001 T1): schema types + boot validation in `src/arsenal.rs`; parse-only (the manifest is read, never written, by this crate); non-optional (the serving lane always resolves its selection surface) — measured 2026-09-27 |
| katgpt-tetris | `../katgpt-rs/crates/katgpt-tetris` | the Tetris lane's shared engine (Issue 009 T5/T6, 2026-09-27): sim + lookahead + champion genome, opt-in `tetris` feature — see the May-depend-on row; the boundary contract row and the manifest landed together |

## Standing invariants

- **Source secrecy**: private; distribution is prebuilt binaries / hosted
  lanes only. Never `cargo publish`.
- **Weights never enter any repo** (`.gitignore` refuses `*.vessel`, `*.bin`,
  `*.safetensors`, `*.gguf`). HOSTED-ONLY artifacts never reach
  uncontrolled hardware; encrypted at rest.
- **No-cheat protocol** (riir-reflex Issue 038): train rows only for any
  corpus/weights; select on held-out train; arena test split read once.
- **No training code here** — riir-train trains, this repo consumes bytes.

## Inherited boundaries (links)

- Dep direction: `../riir-ai/BOUNDARY.md`
- Public/private split: `../riir-ai/.research/003_Commercial_Open_Source_Strategy_Verdict.md`

## Drift ledger (target vs actual)

**None.**
