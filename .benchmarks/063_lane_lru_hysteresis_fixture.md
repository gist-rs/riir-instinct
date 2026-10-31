# Bench 063 — Issue 026 LaneSlot LRU evict/reload hysteresis (the fixture record)

**Status:** LANDED 2026-10-31 (zed-m3-i026) — a SYNTHETIC-FIXTURE record, stated as such: no
production churn data exists yet, so the numbers are the seeded-drive figures the pre-registered
bars were written against (issue 026 T3's own honesty note). Production validation rides the
standing serve posture; the fixture is the regression pin.

## What shipped

- `INSTINCT_LRU_MIN_IDLE_SECS` (seconds, 0/unset = off — the `INSTINCT_LRU_CAPACITY` posture):
  a Ready lazy lane is eviction-eligible only when idle ≥ the floor. Read live at every trigger.
- `OVERSHOOT_CAP = 16` (named const, not a second env): at/over `capacity + OVERSHOOT_CAP`
  residents the floor is WAIVED — plain LRU resumes — bounding residency.
- `skipped_not_idle=N` in the policy log line when the floor skips a trigger (loud-zero).
- The decision is a PURE function (`lru_victim`) over the candidate set + knobs — the fixtures
  drive it sequentially (release's only effect on the next pass's candidate set is removal), so
  G1/G2/G4 are hermetic: no env, no clock, no live edge.

## Fixture results (the drive: suites arriving round-robin, clock ticks per arrival)

| fixture | posture | reloads | peak resident | verdict |
|---|---|---|---|---|
| G1 flag-off identity | capacity 2, 3 suites × 20 rounds, floor 0 | 60 (every arrival — the thrash signature: working set > capacity) | 2 (= capacity) | eviction sequence == the legacy algorithm inlined verbatim (byte-identity oracle) |
| G2 oscillation, floor ON | same schedule, floor 60 s | **3** (initial installs only; the held working set never re-loads) | 3 | **94.9% fewer reloads** — pre-registered bar ≥ 50%: PASS |
| G4 residency bound | 24 distinct suites × 1 arrival, capacity 2, floor 60 s | 24 (each distinct suite once) | **18 = capacity + OVERSHOOT_CAP** | ≤ capacity + 16: PASS; the waive actually fired (evictions non-empty) |

Boundary edges pinned: idle == min_idle is ELIGIBLE (`>=`); a never-used stamp (0) reads as
idle-since-boot; a fully-fresh resident set returns None (the skip path) below the ceiling.

## Memory cost, named (issue 026's counter)

The floor widens the incumbent's soft residency (a burst over budget loads anyway); the ceiling
caps the widening: post-install residency ≤ capacity + 16 always (G4). The momentary
pre-make-room population can read ceiling + 1 only if a load lands between the trigger and its
own eviction check — not a state the drive (or the edge: the incoming lane is not Ready until
after make-room) can produce.

## Live-wire

`tests/ext_seat_rerank_gates.rs::lru_capacity_evicts_the_least_recently_used_lane` (floor-off)
passes unchanged end-to-end — the env-off posture is byte-identical through the real edge.
A live min-idle test would need ≥ 1 s real sleeps for a seconds-granularity env; the hermetic
fixtures carry the behavioral pin instead (deliberate, stated here).

## Validation

`cargo test --lib` 109/109 (4 new hysteresis fixtures incl.); clippy `-D warnings` clean;
`cargo test --test ext_seat_rerank_gates` 8/8; rustfmt clean.

## Deferred

- The one-line cross-ref from riir-train Research 485's consumer list (issue 026 T7's second
  half): the routing restriction gives riir-train edits to their owning sessions — this lane
  records the motivated-by here instead. `[-]` in the issue close.
