# Issue 001 — the HOSTED-ONLY vessel reader (specialist weights as signed, encrypted bytes)

**Status:** CLOSED 2026-09-27 — T1–T5 landed (`src/vessel.rs` +
`tests/vessel_gates.rs`, feature-gated `vessel`; gate arms 7/7 green,
clippy clean at all three feature postures). Minting remains open in
riir-train (the first real artifact pins its first key-id in the change
that ships it).

~~Blocked on riir-train Issue 576 T4 (first artifact)~~ — 576 CLOSED with
the RISP winners; the reader ships against the documented format + gate
fixtures so the first real mint has a verified loader waiting for it.

## Why

`../riir-reflexer` ships the public vessel FORMAT (`crates/reflexer-vessel`:
68-byte header · ed25519-STRICT signature over `header ‖ payload` ·
`commitment = blake3(header ‖ payload)` · two classes, the class bit inside
the signed header). It deliberately has **no HOSTED-ONLY writer or reader**
and no decryption: it refuses the class fail-closed. The specialist weights
are HOSTED-ONLY (they are the moat), so the reader belongs in this private
repo, and the minting stays in riir-train (Research 457 moat map).

## Plan

- [x] **T1** — depend on `reflexer-vessel` for parse/verify (a declared
      BOUNDARY row, measured); no second copy of the format.
- [x] **T2** — HOSTED-ONLY path: verify authenticity FIRST, then decrypt
      (key from the host's secret store, never a file in a repo), then apply
      whole-snapshot (never a blend) under the monotonic version gate.
- [x] **T3** — payload = the specialist's frozen weights + its calibration
      table; load through riir-infer's loaders; refuse a payload whose
      declared architecture does not match the loader.
- [x] **T4** — key rotation: key-id pin table (empty until the first
      artifact; pin the first minting key in the same change that ships it);
      revocation list.
- [x] **T5** — gates: tampered payload, wrong key, downgrade and
      wrong-class vessels are all refused (one arm each, each proven to
      fire).

## References

riir-reflexer README §Vessels + BOUNDARY:20,40,48; riir-train Research 457;
riir-clippy Proposal 009 + Bench 099 (the `rule_embed_frozen_v1.bin`
precedent).

## Landing record (2026-09-27)

- **T1** — `reflexer-vessel` is the optional dep behind the `vessel`
  feature (default builds never resolve it); BOUNDARY row landed.
  The reader consumes the format crate's own primitives (`peek`,
  `PinTable::resolve_key`, `commitment_of`, the HEADER/SIG consts) —
  nothing about the seal is re-implemented. `decode` itself refuses
  HOSTED-ONLY by its own law, so the hosted reader authenticates via
  peek + pin resolution + ed25519-STRICT and takes the verified
  payload directly (the signature's message = header ‖ payload is
  the documented wire contract; the 3-line mirror is attributed in
  `src/vessel.rs`).
- **T2** — verify-then-decrypt in that order; the key is a parameter
  (host secret store's business, never a repo file); the whole
  [`Specialist`] swaps in or the load fails — no blend path exists.
- **T3** — payload = the RISP v1 artifact bytes (BLAKE3-sealed,
  decoded by the existing reader); the confidentiality envelope is
  BLAKE3-XOF keystream XOR under `(key, context ‖ nonce16)` — the
  riir-neuron-db `local_kv::crypto` pattern, zero new crypto deps;
  integrity is the OUTER signature over the ciphertext. `encrypt_
  payload` is exported as the minter's contract (riir-train).
  A declared-architecture mismatch is the artifact reader's own
  version/magic refusal (RISP v1 only).
- **T4** — rotation = `PinTable` (key-ids + revoke + the operator
  wildcard). DEFAULT pins stay empty until the first real mint; that
  change pins the first key-id (the reflexer P3 law, carried).
- **T5** — the gate taxonomy, one arm each, all against REAL minted
  vessels (the fixture minter assembles the documented wire format
  directly; the format crate's HOSTED-ONLY writer is deliberately
  private): good-vessel-loads-whole · tampered-payload (outer seal
  refuses) · wrong-key (payload refused, never garbage scores) ·
  downgrade/replay (refused with versions named, monotonic gate
  before any decryption) · PUBLIC-RELEASE-in-the-hosted-lane
  (WrongClass — a config error, refused loud) · unknown/revoked key
  (fail-closed) · disk-seam == memory-seam. 7/7 green.
