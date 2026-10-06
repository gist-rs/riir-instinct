# Issue 020 — Artifact class adoption: public demo lane + serving dirs

**Status:** LANDED 2026-10-06 at `f78a45f` (Plan 623 T4, workspace Proposal 054 / riir-ai Plan 623). Verdict: the public demo lane is adopted — the two demo winners are manifest-pinned public weights published to the org HF lane under content-addressed keys, the fetch lane round-trips anonymously, every serving/example/bench default reads `artifacts/cache`, and the enforcement gates are green (`artifact-sync lint` zero findings, workspace sweep `riir-instinct: migrated`, pre-push scan over the landed range zero findings). **PROVISIONING UNBLOCKED 2026-10-05: the HF lane + write token are live** — org datasets `gist-rs/riir-instinct-artifacts` + `gist-rs/riir-reflex-artifacts` exist (public), the fine-grained write token is in the ops `.env` (M3 only; name `gist-rs-artifacts-write-m3`, org-scoped — see riir-ai `.docs/01_orientation/623_t8_hf_lane_record.md`). The publish half of this issue can land; the fetch half needs no token (anonymous reads).

## Why

The workspace is consolidating trained artifacts under one declared convention
(`artifacts/<kind>/<class>/` + manifest + BLAKE3 pins) so every machine can pull from remote only.
This repo's part is the PUBLIC lane plus directory alignment for serving dirs.

## Scope

- [x] `data/demo_specialists/` → `artifacts/weights/public/` + manifest rows (BLAKE3 pins in the manifest rows — see the deviations note);
      publish to the org's public HF dataset lane (`gist-rs/riir-instinct-artifacts`)
- [x] Add the public fetch half: pull public artifacts + verify BLAKE3 into `artifacts/cache/`
      (the `fetch_datasets.sh` digest-pin pattern), gitignored by the directory law
      (`artifacts/**`, `!artifacts/manifest.toml`)
- [x] Serving/vessel DIRS adopt the class directory convention; **this repo's public manifest
      carries ZERO protected rows** (workspace gate law — protected artifacts consumed by public
      binaries are owned and manifested in the private repos and pulled into `cache/` at serve
      time)
- [x] `INSTINCT_WINNERS_DIR`/`INSTINCT_VESSEL_DIR` defaults re-pointed; boot drift still fails
      loud (law A5/A6 unchanged — the teaching manifest `data/arsenal.toml` did NOT move, so its
      A6 BLAKE3 digest pin in `tests/serve_gates.rs` is byte-unchanged — verified by the green
      `arsenal_manifest_bytes_are_pinned_byte_for_byte` gate)

## Landed record

- **Publish evidence** (lane `gist-rs/riir-instinct-artifacts`, dataset commits
  `1799a755` choice / `976548f5` noul; remote key = content address per spec law):
  | row | blake3_plain (= blake3_cipher) | bytes |
  |---|---|---|
  | `s1mb_choice_winner_v1.bin` | `f5c583e29f4b11617c433845b1346f4bf76757f7d2093b81f9f1624d4ed288b9` | 119,817,785 |
  | `s1mb_noul_winner_v1.bin` | `f24566cf9c2d8969b9f4dd155a8ac78314271fbe027ea761b95f87cd47230f6a` | 262,222 |
  Hashes + sizes measured from the placed bytes (`b3sum` + `stat`), never carried from the
  issue text. Upload token rode env only (`set -a; . ../riir-refine/.env; set +a`), never
  printed.
- **Anonymous fetch round-trip**: `scripts/fetch_artifacts.sh` →
  `clean — 2 public row(s) verified, 2 fetched`, exit 0; `artifacts/cache/` bytes
  re-hashed byte-identical to the manifest pins AND to the canonical placed bytes.
  `CHECK=1` posture → `clean — 2 public row(s) verified, 0 fetched` (verify-only, no network).
- **Design notes.**
  - **Winners default = `artifacts/cache`** (not the canonical placement dir): the acceptance
    "fresh-clone + public pull → demo lane boots" can only hold if the default IS where pulls
    land; the fetch lane writes `artifacts/cache/<name>`.
  - **Manifest `name` carries the loadable filename** (`s1mb_choice_winner_v1.bin`, not the
    stem): the consumer's winner bridge constructs `<suite>_winner_v1.bin`
    (`src/specialist.rs::winner_bridge`) and the fetch lands `cache/<name>` — a stem name
    would pull a file the demo lane cannot load. This deviates from the issue brief's
    "name = file stem" sketch, deliberately, for that acceptance.
  - **Canonical placement keeps the class infix** (`artifacts/weights/public/<name>.public.bin`):
    the enforcement walks (artifact-sync T7 `placement_walk` + the gate's vendored fallback)
    require `<name>.<class>.<ext>`, and the ORPHAN walk refuses any file under
    `artifacts/<kind>/<class>/` answering to no row. Canonical (producer/publish staging) and
    cache (consumer/pull) coexist by design.
  - **No physical `.blake3` sidecars** (deviation from the brief's step 2, evidence-based):
    a sidecar beside the file under `artifacts/weights/public/` is an ORPHAN finding
    (answers to no row), a sidecar named to answer (`<name>.public.blake3`) makes the
    placement AMBIGUOUS (two matches), and a sidecar in `cache/` is a CACHE finding
    (hashes to no row). The manifest rows carry `blake3_plain`/`blake3_cipher` — the spec's
    own trust root — and the riir-train manifest precedent keeps hashes in rows, not sidecars.
  - **Public rows carry `blake3_cipher == blake3_plain` and `cipher_bytes == plain_bytes`**:
    no age envelope on the public class — the cipher fields are the schema's required
    identity twins, not a second object.
  - **`INSTINCT_VESSEL_DIR` absent — nothing to re-point**: grep-verified absent from the open
    tree; the vessel class reads via manifest rows (Rethink hosts the encoder lane), so vessel
    paths arrive via manifest rows, not a dir default.
- **Gates** (all run at the landed tree):
  - `artifact-sync lint --repo <this>` (T7 tool walk): PASSED — zero findings, 2 artifact
    row(s), 0 source pin(s).
  - `artifact_boundary.py --sweep`: PASSED — `✓ riir-instinct: migrated (manifest present)`
    (zero findings, 9 lane repos).
  - `artifact_boundary.py --scan <repo>:f78a45f~1..f78a45f`: PASSED — 0 size-prefiltered,
    0 hashed (the commit carries only the manifest + code; `git status`/`git diff --stat`
    never listed any `artifacts/weights/*` or `artifacts/cache/*` payload).
  - `cargo clippy --all-targets -- -D warnings`: clean.
  - `cargo test --lib`: 86 passed / 0 failed.
  - `cargo test --test serve_gates`: 24/24, both manifest byte pins green.
  - `cargo test --test staleness_gates`: 6/6 (the data-gated bodies — banking77/massive
    winner pairs — skip by design: those files are not part of the public demo pair; the
    cache carries only the two s1mb rows, so the tests read their absence the same way the
    old `data/demo_specialists` default did).
  - `fence_gate.sh --post-split`: **RED, PRE-EXISTING at HEAD `c617805`** (verified by
    stash-and-rerun) — `tests/serve_gates.rs: EncoderLane` (a Bench-020 comment naming
    `EncoderLane::from_parts`) + `tests/serve_g4_alloc.rs: instinct_specialists` (the
    `../riir-train/data/instinct_specialists` default). Both files are outside this issue's
    write scope (serve_gates pins/tests + serve_g4_alloc are do-not-touch); the landed diff
    contains neither term. Filed here as an honest disclosure, not fixed in this issue.

## Acceptance

- **Arena + serve boots byte-identical post-move (parity gates + frozen witnesses green)** —
  held by construction + measured: the teaching manifest (`data/arsenal.toml`) is artifact-less
  A0, so serve boots never touch winners; `serve_gates` 24/24 including both manifest byte
  pins and the frozen-witness faces; `serve.rs` boot smoke prints
  `winners: artifacts/cache` with the same 9-row arsenal digest
  `blake3:1eb91da47976fc5d`. The arena's s1mb suite seat run is data-blocked on this box
  (no `s1mb_*` dataset dirs in either reflex pool here) — the demo-lane boot proof ran
  through the same consumer path instead: `load_winners` against `artifacts/cache` decoded
  both winners through the winner bridge + serving reader (914-label choice space,
  2-label noul, seal verified) — `PASSED — every artifact decoded through the serving reader`.
- **Fresh-clone + public pull → demo lane boots** — proven: the fetch lane round-trips
  anonymously (both rows fetched + hash-verified into `artifacts/cache/`), the defaults
  (serve/arena/examples/bench, `INSTINCT_WINNERS_DIR` unset) read exactly that dir, and the
  pulled bytes load through the serving reader (above).
- **`ci_artifact_boundary.sh` green for this repo's public tree** — held: `artifact-sync lint`
  zero findings (the T7 file walk IS the boundary's workstation half for this repo; the gate's
  own `--workstation` mode adjudicates the script's home repo, riir-ai, and passes), the
  workspace sweep reads `riir-instinct: migrated (manifest present)`, and the pre-push leak
  scan over the landed range is green. The "hook tracked but core.hooksPath not set (T8
  enrolls)" line is the expected UNSEEN disclosure (enrollment is T8's act), not a finding.

## Notes

Public repo: this issue deliberately carries only the public class. The A10 moat law and the
serving-side artifact classes are unchanged; no private artifact is named here or added to this
repo.
