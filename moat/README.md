# moat/ — BORN as `../riir-rethink` (a pointer, not a seed any more)

The Rethink seed's job is done: the private product repo **`riir-rethink`**
(gist-rs/riir-rethink, birth commit `e263686`) was born 2026-10-03 from
THIS directory's tracked contents, verbatim (riir-instinct Plan 008 B3 +
the owner's birth directive: "move any code to riir-rethink before
public; history is fine").

- The whole git history STAYS HERE (this repo keeps its pre-split
  lineage; the birth is a fresh root whose first commits cite these
  hashes — `4ffd08f` seeded, `e263686` born).
- The moat SOURCE (`src/encoder_arm.rs`, `src/encoder_serve.rs`,
  `src/vessel.rs`, `src/decstat*.rs`), the gates, the records, the
  production manifest, and the deploy shape now live in `../riir-rethink`.
  The trainer half joined it 2026-10-03 (Plan 008 B4:
  `src/instinct_encoder_lane.rs`, `src/instinct_laya_head.rs` + the five
  `examples/instinct_encoder_*` / teacher / static-surrogate recipes);
  the L1 trainers went PUBLIC into this repo instead (`src/instinct_*.rs`
  + `examples/instinct_arm_*` etc.).
- The open crate's coupling to Rethink is the lane-backend seam only:
  `server::{LaneBackend, ExtBoots, install_ext_boots}` + the
  `AnySuiteServer::Ext` seat (Proposal 052).
- The fence's `^moat/` history arm still applies: the fresh-root public
  export (Phase C, owner-gated) must never carry this directory's paths.

Contract files: `../riir-rethink/BOUNDARY.md` (carried verbatim from
`moat/BOUNDARY.md`), `../riir-rethink/README.md`, `../riir-rethink/AGENTS.md`.
