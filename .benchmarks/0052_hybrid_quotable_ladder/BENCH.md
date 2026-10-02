# Bench 0052 — the hybrid lane's stamped latency (the ladder's middle rung)

**Status:** COMPLETE — published lane-scoped on reflex.gist.rs (reflex-site
`bd8dc6d`); the frontier plots Instinct at 58.5% cc / 3.5 µs p50 geomean
(9/9 quotable).

## Why

The /bench/ efficiency frontier's product ladder (Reflex → Instinct →
Rethink, the 2026-10-02 user ask) rendered Reflex and Rethink but never
Instinct: the hybrid lane's published cells carry timing but no Issue-021
verdict — `backfill_latency_verdicts.py` measured it: ZERO instinct docs
carry box_state, so every hybrid timing cell stayed unjudged and the
publisher refused the geomean (`n_unjudged 9`).

## What landed (this repo)

- **arena:** `write_box_state` — the structured span the run already
  captures (start/end, the encoder-doc's shape) now also lands as
  `box_state.json` in the out dir.
- **build_hybrid_doc.py:** `--box-state <file>` embeds the span into the
  doc's `meta.box_state` (validated start/end); publish_bench stamps
  per-cell verdicts from it — the exact path reflex's own docs ride.
  Self-test PASS.

## The run

- PROVENANCE: power=AC Power · powermode=2(high) · load 3.73 → 4.82 ·
  **latency QUOTABLE** both ends.
- Suites: the nine index suites at the arena seat. A0 leg byte-exact on
  all nine (0.8825 / 0.7800 / 0.396667 / 0.885 / 0.523333 / 0.767241 /
  0.5725 / 0.375 / ag_news).
- git sha `eef77d1`; host m3 (→ m3-max-metal at publish).

## Disclosures (recorded, not hidden)

1. **The arena pin's site half is posture-stale on banking77 ONLY** —
   reflex `run()` reads 0.826 at the arena's seat knobs (head-select ON)
   while the PUBLISHED modelless row is 0.842 (the 10-01 head-OFF
   full-pool restore, reflex 058/d750175, postdating the last full-arena
   site check — Bench 012's run 1 was the last one-suite site ✓).
   The other eight suites match the site byte-exactly. Run under
   `--skip-pin-a0` per the Bench-016/0051 precedent; leg 1 (arena ==
   reflex `run()`) HELD on all nine. The main-board republish reconciles.
2. **The massive cell refreshes 0.8267 → 0.8400** — the manifest's
   serving arm is the SYNTH-SEAT posture (0029: H2(β=1,nmin=2,τ=4) over
   the vetoed synth corpus); the prior published doc predated it. Run
   0053 seats the synth corpus exactly per 0029's command; every other
   suite byte-identical.

## Artifacts

- `0052_hybrid_quotable_ladder/` — predictions/registration/RESULTS +
  the built `hybrid_lane_doc.json` (box_state embedded).
- `0053_massive_synth_0052/` — the massive synth-seat record merged
  per-suite into the doc (build_doc_merged argv order).
