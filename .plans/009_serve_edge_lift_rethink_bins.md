# The serve-edge lift + the Rethink bin reassembly (the next split unit)

**Status:** OPEN — filed 2026-10-03 at the Phase C landing. The next
recorded lift step (riir-rethink BOUNDARY drift ledger "the serve bin
reassembles from `bin_hunks/` over the open lib next"). B4 (the trainer
half) stays separate — deferred on riir-train Issue 607.

## Why this plan exists

The public flip landed (gist-rs/riir-instinct, `382b6bd`, CI green both
jobs). Rethink's bins cannot consume the HTTP edge today because the
edge lives IN the open bin: `src/bin/serve.rs` is 1,523 lines with
env/arg parsing + lane boot + the entire HTTP loop in `main()` —
`server.rs` holds the lane machinery only. `bin_hunks/README.md`'s law:
**the HTTP edge is RE-SHARED, never copied** — lift it into the open
crate FIRST, then Rethink consumes it. Forking the bin is refused
(052's 6.3k-line entanglement verdict).

## The dual-landing law — SUPERSEDED same day (2026-10-03): land ONCE

> The owner retired the two-repo arrangement hours after Phase C ("git
> history leak is fine… i dont want 2 confusing repo i want single source
> of truth"). The full history now lives ON the public repo
> (gist-rs/riir-instinct; `develop` is the working branch, `main` the same
> tip for the landing page) and this checkout's origin points there.
> **Open-crate src changes land ONCE** — commit on `develop`, push; no
> replay step, no second repo. The fence `--post-split` still asserts the
> delta stays open-shaped, and Rethink's pin-lockstep law (its AGENTS.md
> law 2) still covers the manifest. The text below is the superseded
> Phase-C posture, kept as the record of what it was.

Any open-crate src change now lands TWICE, one family: the private
whole-history tree (develop, gist-rs/riir-instinct-internal) AND the
public export repo (main, gist-rs/riir-instinct). Mechanics: land +
gate on develop, then replay the same tree delta as a fresh commit on
the public repo (the public repo accepts new commits; it is a fresh
root that grows), re-run the fence both arms + the public CI. The
fence `--post-split` is the assertion that the delta stays open-shaped.
Rethink's pin-lockstep law (its AGENTS.md law 2) already covers the
manifest; this plan extends the discipline to src.

## T1 — the edge lift (open side, both repos)

- [ ] T1.1 extract the HTTP edge from `src/bin/serve.rs` into a lib
      module (`src/serve_edge.rs`): the listener loop, routing
      (/decide, /healthz, /, /arsenal/swap, /arsenal/release), the
      response/error helpers, `SrvState`, `artifact_path_for`,
      `swap_ok`, `hex_digest`. Parameterize by a `ServeConfig`
      (bind, datasets_dir, winners_dir, arsenal_path, suite_filter,
      synth_corpus_dir, cors_origin) — the exact fields the bin's env/
      arg parsing produces today. Zero behavior change (the bin's
      output byte-identical; `serve_gates` + the live smoke are the
      proof).
- [ ] T1.2 `src/bin/serve.rs` shrinks to: env/arg parsing →
      `ServeConfig` → `riir_instinct::serve_edge::run(cfg)`. The
      receipts/lane boot stay where they are (lib already).
- [ ] T1.3 gates: clippy `-D` ×postures, lib 56/56, serve_gates 18/18,
      calibrator 6/6, staleness 2/2, g4 1/1, tetris parity 1/1 + the
      live boot smoke (healthz + /decide with receipts). On BOTH repos,
      plus the public CI green.
- [ ] T1.4 the fence both arms on both repos post-landing.

## T2 — the Rethink bins (rethink side)

- [ ] T2.1 Rethink's `src/bin/serve.rs`: its own env/arg surface
      (re-adding the moat flags from the `serve_*` hunks:
      `serve_enc_preflight`, `serve_decstat_boot`, `serve_vessel_config`,
      `serve_vessel_state_machinery`) → installs the ENC lane backend +
      the HOSTED vessel boot via `riir_instinct::server::install_ext_boots`
      → calls the SHARED edge. The decstat client boots beside the seat
      (`serve_decstat_capture` hooks the answer path).
- [ ] T2.2 Rethink's measurement harness from the `arena_encoder_*`
      hunks (6 files) over the public lib pieces (`prepare_seat`, the
      paired-stats helpers); the lane-doc emitter's laws carry verbatim
      (issue 017 T5 — instinct numbering).
- [ ] T2.3 the moat parity gates (`moat/tests/` → rethink `tests/`)
      re-point at the rethink crate root; the frozen Bench-029/599/600
      replays must stay EXACT (the cell-identity witnesses).
- [ ] T2.4 the hunks' origin maps verified: every hunk either landed,
      or is recorded as intentionally dropped with a reason (the
      export-manifest discipline applied to the bins).

## Sequencing + risks

1. T1 is the blocker; T2 starts only after T1 lands both repos.
2. The 1,400-line move is mechanical but visibility-sensitive
   (`crate::` → lib paths, `pub(crate)` boundaries) — the gates are the
   proof, never "it compiles".
3. A sibling agent is active in riir-rethink (untracked issue 017) —
   coordinate the rethink-side landing at a commit boundary, their
   uncommitted files untouched.
4. The public CI runs on every push now — keep pushes gated (local
   gates green first), the fence lane is the fast red.

## References

- `../riir-rethink/bin_hunks/README.md` — the lift design (the four
  numbered steps this plan decomposes).
- `.plans/008` Phase C — the landed flip + the C1 export deltas.
- riir-ai Proposal 052 — the seam-not-fork verdict.
