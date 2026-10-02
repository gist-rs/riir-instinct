# moat/bin_hunks — the BIN-side moat hunks archived at the carve


Proposal 052's carve refuses to FORK `src/bin/serve.rs` / `src/bin/arena.rs`
(the 6.3k-line entanglement verdict) — the bins stay single-homed in the
open tree, stripped to the L1 lanes + the extension points. These files are
the moat hunks the B2 move family deleted from them, extracted verbatim at
the B1 seam commit `22ae291` (the parity-gated state, pre-deletion). Each
file's first line names its origin + line range.

## The Rethink lift (when the private repo is born)

1. Rethink depends on `riir_instinct` as a crate (the riir-refine shape).
2. `serve_enc_preflight.rs` + `serve_decstat_*.rs` + the vessel hunks
   become Rethink's own serve bin over `riir_instinct::server` (the
   `LaneBackend` + `install_ext_boots` seam is the plug-in path). The
   HTTP edge is RE-SHARED, never copied: lift the edge into the open
   crate FIRST if Rethink needs it parameterized (the recorded follow-up),
   then consume it.
3. `arena_encoder_*` hunks re-home as Rethink's own measurement harness
   importing the public lib pieces (`prepare_seat`, the paired-stats
   helpers). The lane-doc emitter's laws (auto-emission, issue 017 T5)
   carry over verbatim.
4. The parity gates in `moat/tests/` re-point at the Rethink crate root
   (`CARGO_MANIFEST_DIR`-relative records live in `moat/.benchmarks/`).

NOT compiled here — `moat/` is the seed, not a workspace member.
