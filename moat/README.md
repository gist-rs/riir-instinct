# moat/ — the Rethink SEED (the private lane's designated home)

**This directory is the moat.** Riir-ai Proposal 052 (the owner directive
2026-10-03: *"i moat Rethink private moat, anything need to move to the moat
we must do it"*) moved everything sellable out of the open Instinct surface
and into this seed. The private repo that IS Rethink is this whole-history
repository (renamed on GitHub, or mirrored with identical hashes — the
owner's pick at the wave); `moat/` is the part of its tree that never
ships publicly. The Phase-C fresh-root export is manifest-driven
(`scripts/open_export_manifest.txt`) — **`moat/` is not in the manifest, so
the public tree never contains it** — and `scripts/fence_gate.sh --history`
learns `^moat/` as a moat path, so an export that ever carried it REDS.

## What lives here

| Path | What |
|---|---|
| `src/encoder_serve.rs` | the serve-side encoder lane (issue 016 T2): the sealed NLEH head + the laya agent resident from boot + the shared-worker registry (issue 018 Lane B) |
| `src/encoder_arm.rs` | the arena-side encoder arm (issue 014 C1): the NLEH v1/v2 codec + the live laya encode replay (record-only measurement) |
| `src/vessel.rs` | the HOSTED-ONLY vessel reader (Plan 001 P4): blake3-XOF envelope, class gate, monotonic apply, the mint helper |
| `src/decstat.rs` | the economy capture lane (Plan 002 / Issue 004): consent-gated decision-outcome stats over the riir-kat wire |
| `src/decstat_verify.rs` | the receipt VERIFIER (Plan 043 C1): boots the seat engine, resolves lease items, compares decision hashes |
| `tests/*.rs` | the moat gates: `serve_encoder_parity`, `encoder_shared_parity`, `vessel_gates`, `decstat_gates` |
| `examples/decstat_floor.rs` | the receipt-floor runner (Plan 043 C2, the D5 arming gate) |
| `bin_hunks/` | the BIN-side hunks (serve preflight/vessel/decstat wiring, arena encoder face/lane-doc) deleted from the open bins at B2 — verbatim, with origin maps |
| `arsenal.toml` | the PRODUCTION manifest (the best-measured-arm serving verdict, raw-winner digests) — the open build's law-A6 pin carries it INLINE in `tests/serve_gates.rs`, byte-verified |
| `deploy.yaml` | the cf-container deploy shape (6 sealed winners + 6 t20k suites) |
| `.deploy/` | deployer build/stage trees (untracked) |
| `scripts/encoder_load_ab.py` | the encoder load A/B instrument (issue 018 Lane D) |
| `.benchmarks/029,030–048,050*` | the encoder/decstat/quant/distill records (029 C1, 031 T2 parity, 032/035 screens, 042–048 cells + D1/D2a, 050 Lane B + bekko, 054 load A/B, 055 D4 Q4, 056 bekko-400m) |
| `.plans/002,006,007` | the decstat flywheel, the 051 rename (superseded), the access-plane E2E |
| `.proposals/001` | the arsenal cognition-vessel protocol (A1/A10 two-class law) |
| `.issues/016` | the encoder class GPU-host seating record |

Deliberately NOT here (content over the ledger's range): `.benchmarks/0051–0053`
stay in the open tree — the harness-families retirement record and the hybrid
quotable ladder + its synth seat are the OPEN product's published evidence.

## The Rethink lift (when the private repo is born)

1. Rethink depends on `riir_instinct` as a crate; the extension point is
   `riir_instinct::server::{LaneBackend, ExtBoots, install_ext_boots}` +
   the `AnySuiteServer::Ext` seat. `enc_boot_bytes` / `enc_boot_vessel`
   are the installer halves (they move with `encoder_serve.rs`).
2. `Cargo.toml` here is the manifest TEMPLATE (not a workspace member —
   never built in this repo): deps `riir-kat` (decstat wire),
   `reflexer-vessel` with `vessel_hosted_read` (the HOSTED-ONLY reader
   capability), `ed25519-dalek`, `papaya`, plus the reflex laya tree for
   the encoder lane.
3. The open build keeps `vessel_public_read` ONLY — the HOSTED-ONLY class
   refuses structurally in every open boot path (verified by
   `fence_gate.sh --post-split` GREEN).
4. Bin hunks: `bin_hunks/README.md` has the lift order and the re-share
   rule (the HTTP edge is re-shared, never copied).
5. The moat tests re-point their record paths at this tree
   (`moat/.benchmarks/`) and re-run as Rethink's gates.

## Laws that bind this directory

- **A10 (the moat law):** nothing here ever rides a PUBLIC-RELEASE vessel
  or a public repo. The HOSTED-ONLY reader never compiles into the open
  build.
- **Consent (Plan 002):** `RIIR_INSTINCT_STATS` — Unset never pushes.
- **The owners' gates:** the fresh-root export + the visibility flip are
  the owner's personal acts (Phase C); this seed changes nothing about
  that timing.
