# Bench 049 — the family full-coverage re-baseline (owner full-coverage serving, 2026-10-02)

**Status:** COMPLETE — the six harness families' A0 frozen record at the CURRENT published
posture; the serve-parity gate (`tests/serve_gates.rs::served_family_decisions_are_the_frozen_a0_picks`)
replays exactly these bytes.

## Why this record exists

Owner call 2026-10-02: "run Rethink.exe or api -> get the result" — the serve binary must
answer EVERY board suite. The six harness families joined `arsenal.toml` as ARTIFACT-LESS
A0 rows (no digest — `HybridLane::ReflexOnly`, the G0a first-class arm): the modelless
tier IS the served arm, which is the best-measured serving law's verdict here (Issue 008
T8's drop was the SPECIALIST lane's scope — n=12–16 template-shared evals, a trained win
would be unfalsifiable memorization — never a refusal to serve).

The parity gate needs a frozen record AT THE CURRENT POSTURE. The stale Bench-011 reads
(0.375–0.500) predate reflex posture work that moved the families substantially (below) —
the same re-baseline class as Bench 004 was for the nine dataset suites.

## The re-baseline finding (recorded, not hidden)

| suite | Bench 011 A0 (stale) | this record (current) |
|---|---|---|
| harness_visibility | 0.3750 | **0.5625** |
| harness_permissions | 0.4167 | **0.6667** |
| harness_tool_fit | 0.5000 | **0.9167** |
| harness_routing | 0.4375 | **0.7500** |
| harness_sensitivity | 0.4000 | **0.8000** |
| harness_cache_reuse | 0.9167 | 0.9167 |

The A0 identity pin (arena == reflex `run()`) passed on all six at the published-posture
knobs — so the SERVE (same seat + fit code) serves exactly these numbers, and the
reflex-side published board rows are STALE until reflex-side republish carries them.
The reflex-site fallback cells for the Instinct/Rethink lanes derive from THIS record
(the served product), not from the stale reflex lane rows.

## Provenance

- Arena run: `cargo run --release --bin arena -- --suite <six families> --out .benchmarks/049_family_full_coverage`
- Host: m3-max-metal (the fleet baseline box), quiet-ish box, sibling cargo builds finished.
- Seats: synthetic (reflex in-process builds) — no datasets, no winner bytes.
- pin_a0 drift pin: PASSED on all 6 (arena A0 == reflex run() byte-exact).
- Manifest: the 15-row embedded arsenal (blake3 `913fc2601d50e86110c163fa33ab5c89ff5719557fe331038a90a2349be4ce0e`).
