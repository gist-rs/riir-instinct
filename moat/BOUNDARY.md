# riir-rethink — boundary contract (the moat seed's rows, carried at the wave)

**Status:** canonical text NOW (filed at the Proposal-052 carve,
riir-instinct Plan 008 B3, 2026-10-03); the physical repo is born at the
wave — the whole-history private repo IS riir-rethink (GitHub rename or
identical-hash mirror, the owner's pick). This file ships in
`riir-instinct/moat/BOUNDARY.md` until then and becomes Rethink's root
contract at birth, verbatim.

Visibility: **private forever. Never public. No exceptions.**
(Research 003 + the 2026-10-03 owner directive: "i moat Rethink private
moat, anything need to move to the moat we must do it".)

## Owns

- **The encoder arm** — the L3 serving rung and its measurement lane:
  the sealed NLEH v1/v2 heads, the live laya-english/typed/multilingual
  encode, the shared-worker residency (one worker per checkpoint), the
  fake-quant probe postures (D1 Q8 storage tier adopted; D4 Q4 measured
  out), the arena record-only faces + the auto-emitted lane docs
  (issue 017 T5's no-hand-typed-numbers law).
- **The HOSTED-ONLY vessel reader** — blake3-XOF confidentiality
  envelope, class gate, monotonic apply, the minting helper. A10
  structural: this code never compiles into a public repo, and
  HOSTED-ONLY artifacts never touch uncontrolled hardware.
- **The economy plane** — the decstat capture lane (consent-gated,
  `RIIR_INSTINCT_STATS`: Unset never pushes), the receipt verifier
  (Plan 043 C1), the receipt-floor runner (the D5 arming gate).
- **The production serving posture** — the arsenal manifest (the
  best-measured-arm verdict, raw-winner digests; its law-A6 byte pin is
  carried inline in the open repo's `tests/serve_gates.rs` and must stay
  in lockstep), the deploy.yaml cf-container shape, the sealed winner
  vessels, synth corpora, production seats.
- **The moat records** — the encoder/decstat/quant/distill bench records
  (029–048, 050/054/055/056: the D1/D2a/D4 know-how), the design docs
  (.plans/002/006/007, .proposals/001, .issues/016), the A/B instruments.
- **The trainer half** (deferred — riir-instinct Plan 008 B4, until
  riir-train Issue 607 lands + both boxes rebase): the NLEH trainer
  modules (`instinct_encoder_lane.rs`, `instinct_laya_head.rs`) + the
  moat training recipes/examples.

## Does not own

| Concern | Correct home |
|---|---|
| The open L1 stack (specialist/hybrid/arsenal grammar/server edge/receipts/stats/tetris) | **`riir-instinct`** (public) — Rethink depends on it as a crate; the seam is `server::{LaneBackend, ExtBoots, install_ext_boots}` + the `AnySuiteServer::Ext` seat |
| Training loops and GPU training runs | `../riir-train` (Rethink consumes sealed bytes, like Instinct does) |
| Model forward substrate | `../riir-infer` (public) |
| Vessel FORMAT | `../riir-reflexer` (public; Rethink enables the HOSTED-ONLY reader capability) |
| Settlement/ledger semantics | `../riir-dapps` (Rethink is the riir-kat CLIENT) |
| Modelless engine | `../riir-reflex` (public) |

## May depend on

| Crate | Condition |
|---|---|
| riir-instinct | path dep; the extension point is the ONLY coupling — never a fork of the open bins (the 6.3k-line verdict). The bin hunks in `moat/bin_hunks/` re-home as Rethink's own bin over the open lib; the HTTP edge is re-shared, never copied |
| riir-kat | `kat_transport` (the decstat wire client) |
| reflexer-vessel | **`vessel_hosted_read`** (the capability feature the public repo never enables) |
| ed25519-dalek, papaya | the seal primitive + the lock-free capture counters |
| riir-reflex | `laya-riir` (+ `-metal`/`-cuda`) — the encoder lane's substrate |

## Standing invariants

- **A10 — the moat law:** HOSTED-ONLY artifacts and this source never
  reach a public repo or uncontrolled hardware. The public Instinct build
  refuses the class structurally; the fence's `^moat/` history arm keeps
  it that way.
- **The pin lockstep:** the production manifest's byte pin lives in the
  OPEN repo's serve_gates (so the teaching gate replays the production
  verdict); any manifest edit lands in BOTH repos in one family.
- **Consent:** `RIIR_INSTINCT_STATS` — Unset never pushes; `--unmine`
  semantics are riir-refine's, honored here.
- **Weights never enter any repo.**

## Drift ledger (target vs actual)

**None.** (Born with the carve; the trainer half's deferral is Plan 008
B4's recorded row, not drift.)
