# Issue 020 — Artifact class adoption: public demo lane + serving dirs

**Status:** OPEN — Plan 623 T4 (workspace Proposal 054 / riir-ai Plan 623). **PROVISIONING UNBLOCKED 2026-10-05: the HF lane + write token are live** — org datasets `gist-rs/riir-instinct-artifacts` + `gist-rs/riir-reflex-artifacts` exist (public), the fine-grained write token is in the ops `.env` (M3 only; name `gist-rs-artifacts-write-m3`, org-scoped — see riir-ai `.docs/01_orientation/623_t8_hf_lane_record.md`). The publish half of this issue can land; the fetch half needs no token (anonymous reads).

## Why

The workspace is consolidating trained artifacts under one declared convention
(`artifacts/<kind>/<class>/` + manifest + BLAKE3 pins) so every machine can pull from remote only.
This repo's part is the PUBLIC lane plus directory alignment for serving dirs.

## Scope

- [ ] `data/demo_specialists/` → `artifacts/weights/public/` + manifest rows (BLAKE3 sidecars);
      publish to the org's public HF dataset lane (`gist-rs/riir-instinct-artifacts`)
- [ ] Add the public fetch half: pull public artifacts + verify BLAKE3 into `artifacts/cache/`
      (the `fetch_datasets.sh` digest-pin pattern), gitignored by the directory law
      (`artifacts/**`, `!artifacts/manifest.toml`)
- [ ] Serving/vessel DIRS adopt the class directory convention; **this repo's public manifest
      carries ZERO protected rows** (workspace gate law — protected artifacts consumed by public
      binaries are owned and manifested in the private repos and pulled into `cache/` at serve
      time)
- [ ] `INSTINCT_WINNERS_DIR`/`INSTINCT_VESSEL_DIR` defaults re-pointed; boot drift still fails
      loud (law A5/A6 unchanged — manifest digest re-pinned if the teaching manifest moves)

## Acceptance

- Arena + serve boots byte-identical post-move (parity gates + frozen witnesses green)
- Fresh-clone + public pull → demo lane boots
- `ci_artifact_boundary.sh` green for this repo's public tree

## Notes

Public repo: this issue deliberately carries only the public class. The A10 moat law and the
serving-side artifact classes are unchanged; no private artifact is named here or added to this
repo.
