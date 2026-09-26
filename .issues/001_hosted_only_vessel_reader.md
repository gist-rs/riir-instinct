# Issue 001 — the HOSTED-ONLY vessel reader (specialist weights as signed, encrypted bytes)

**Status:** OPEN — filed 2026-09-26 (Plan 001 P4). Blocked on riir-train Issue 576 T4 (first artifact).

## Why

`../riir-reflexer` ships the public vessel FORMAT (`crates/reflexer-vessel`:
68-byte header · ed25519-STRICT signature over `header ‖ payload` ·
`commitment = blake3(header ‖ payload)` · two classes, the class bit inside
the signed header). It deliberately has **no HOSTED-ONLY writer or reader**
and no decryption: it refuses the class fail-closed. The specialist weights
are HOSTED-ONLY (they are the moat), so the reader belongs in this private
repo, and the minting stays in riir-train (Research 457 moat map).

## Plan

- [ ] **T1** — depend on `reflexer-vessel` for parse/verify (a declared
      BOUNDARY row, measured); no second copy of the format.
- [ ] **T2** — HOSTED-ONLY path: verify authenticity FIRST, then decrypt
      (key from the host's secret store, never a file in a repo), then apply
      whole-snapshot (never a blend) under the monotonic version gate.
- [ ] **T3** — payload = the specialist's frozen weights + its calibration
      table; load through riir-infer's loaders; refuse a payload whose
      declared architecture does not match the loader.
- [ ] **T4** — key rotation: key-id pin table (empty until the first
      artifact; pin the first minting key in the same change that ships it);
      revocation list.
- [ ] **T5** — gates: tampered payload, wrong key, downgrade and
      wrong-class vessels are all refused (one arm each, each proven to
      fire).

## References

riir-reflexer README §Vessels + BOUNDARY:20,40,48; riir-train Research 457;
riir-clippy Proposal 009 + Bench 099 (the `rule_embed_frozen_v1.bin`
precedent).
