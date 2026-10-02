# Bench 0051 — the families wide-eval re-baseline (reflex Plan 009 REVISED-2 / issue 059)

**Status:** COMPLETE — the six harness families' A0 frozen record on the WIDE
(template-disjoint, 96–100-case) eval populations; the serve-parity gate
(`tests/serve_gates.rs::served_family_decisions_are_the_frozen_a0_picks`) re-pinned to
replay exactly these bytes.

## Why this record exists

reflex `e78c0e6` widened the five families' eval slices under Plan 009 REVISED-2 (the
owner's display-only call: the wide eval feeds a quarantined reflex-site section, our
lanes only, behind the honesty caveat — the certification purpose stays dead). The swap
invalidates Bench 049's serve-parity record BY CONSTRUCTION: 049's frozen picks were
recorded against the n=12–16 eval texts, which no longer exist in `prepare_seat`'s
population. The gate re-pins here.

## The frozen read (instinct A0 = the seat posture, wide evals)

| suite | wide n | A0 acc | verdict | p50 µs |
|---|---|---|---|---|
| harness_visibility | 96 | 0.4896 | a0_stands | 10 |
| harness_permissions | 96 | 0.4896 | a0_stands | 10 |
| harness_tool_fit | 96 | 0.4896 | a0_stands | 10 |
| harness_routing | 96 | 0.4479 | a0_stands | 10 |
| harness_sensitivity | 100 | 0.3400 | a0_stands | 10 |
| harness_cache_reuse | 12 | 0.9167 | a0_stands | 10 |

Reflex modelless baseline posture (`nb_scale 0`, reflex `.benchmarks/105`): 0.3125 /
0.3125 / 0.3021 / 0.2812 / 0.2200 / 0.5000 — the seat posture's NB-count-table lift
over the raw engine is real on the wide evals (the corpus teaches class vocabulary,
not case texts), most of all on cache_reuse's noul polarity (Some(1), cal 0.75 → test
0.9167 on its unchanged 12).

## The A0 drift pin (run WITHOUT --skip-pin-a0 first — what it said)

Leg 1 (arena A0 == reflex `run()` in-process at the arena's knobs): **HELD on all
six** — 0.489583 / 0.489583 / 0.489583 / 0.447917 / 0.340000 / 0.916667, the exact
records above. Leg 2 (reflex run() == the PUBLISHED reflex-site rows): **FAILED on
the five** — the published family rows are the Bench-049-era (old-eval) numbers
(0.5625 / 0.6667 / 0.9167 / 0.7500 / 0.8000); the pin's canned remedy ("re-baseline
the knobs") does NOT apply — the knobs match; the EVAL moved. The published rows
refresh at the next full board republish (a main-board decision, deliberately out of
the quarantined-section landing's scope). This record is written under
`--skip-pin-a0` (the Bench-016 precedent) with this note as the reason; leg 1's
evidence is the first run's output, quoted above.

## Provenance

- Arena run: `CARGO_TARGET_DIR=/tmp/famwide-instinct-target cargo run --release --bin
  arena -- --skip-pin-a0 --suite <six> --out .benchmarks/0051_families_wide_eval`
- reflex dep: `e78c0e6` (the wide-eval landing; the arena built against it).
- Host: m3-max-metal. Box state: power AC · powermode high · load 4.18 · swap 2603 MB
  · quotable TRUE.
- ⚠ Tree disclosure: instinct's working tree carried an in-flight SIBLING session's
  encoder-serve WIP (`src/encoder_pool.rs`, `encoder_serve.rs`, `lib.rs`,
  `Cargo.toml` — uncommitted at record time); the family A0 path does not ride the
  encoder lane, and HEAD was `d8a3b9f`. Disclosed for the record, never hidden.
- Seats: synthetic (reflex in-process builds); no datasets, no winner bytes.
- Manifest: the embedded arsenal (family rows artifact-less — `HybridLane::ReflexOnly`,
  no re-mint needed).

## Downstream

- reflex-site's quarantined families section (`data/families.json`) consumes THIS
  record for the Rethink (hybrid) cells and reflex `.benchmarks/105` for the Reflex
  (modelless) cells; the encoder lane renders `not run` (owner call pending on
  riir-train heads — absence is honest, not failure).
- Bench 049's numbers remain the historical record of the n=12–16 era; the
  full-coverage SERVING posture continues to answer the families at the seat engine
  (the serve binary rebuilds its seats from reflex HEAD, so served answers now ride
  the wide evals' engine population with the same posture).
