# Issue 026: `LaneSlot` LRU evict/reload hysteresis — the small churn guard

**Status:** Open — small, standalone; env-gated with `0` = current behavior; NOT a step_noise consumer (right-sized tool, see Fix shape)  
**Claimed:** zed-m3-i026, 1793430322, expires 2026-11-02T07:05:22+00:00

**Provenance:** follow-up consumer-routing pass after katgpt-rs Issue 975 closed (close commit `878526076`; PoC G1–G4 PASS at `48fd0be4d`; consumers (a)–(d) filed per katgpt-rs `HISTORY.md` §Issue 975). Motivated by riir-train Research 485's diagnosis — "update too aggressive relative to evidence" — during the 2026-10-31 routing pass. Honesty note recorded up front: the FIX here is plain hysteresis, not the `StepNoiseMonitor` — pulling a regime monitor to do one idle-time comparison fails the right-sizing bar (substrate-first / SOLID). The paper motivates; it is not consumed. (This issue survives verdict round 1 unchanged in shape; the code citations, alloc claim, and memory bound below are the round-1 corrections.)

## What ships today (verified in `src/serve_edge.rs`, 2026-10-31)

- `fn lru_make_room` at `serve_edge.rs:762` — the R5 policy: loop { collect candidates `Vec<(u64, usize)>` of Ready lazy slots (not the incoming suite), evict least-recently-used while residency is at/over capacity }; `fn lru_capacity` at `:744` reads `INSTINCT_LRU_CAPACITY` **live at every trigger** (`0`/unset = off). Called from `decide_edge` before a lazy load (`:1483–1491`), best-effort, never a refusal.
- `LaneSlot` + `last_used_ms()` live in `src/arsenal_ops.rs` (the slot registry + monotonic `touch()` stamps; reload of an evicted lane takes `InstallOutcome::Idempotent`).
- The 503 window: a lazy row boots `Unloaded`; the first decision triggers a load, and loading lanes answer 503 until Ready.

## The pathology

Under capacity oscillation (working set ≈ capacity), a lane can be evicted, immediately re-demanded, reloaded (paying the load + serving 503s in the window), and evicted again — churn with no residency gain. That is Research 485's signature in miniature: the eviction act is more aggressive than the usage-pattern evidence justifies.

## Fix shape — hysteresis floor on eviction eligibility

A Ready lazy lane is eviction-eligible only when idle for ≥ `INSTINCT_LRU_MIN_IDLE_SECS` (new env, **default 0 = byte-identical current policy** — the `INSTINCT_LRU_CAPACITY` precedent; 0 never means "aggressive", it means "this guard off"). **Read live at every trigger, the same way `lru_capacity` reads its env** (consistent with the shipped pattern; no config struct, no boot pin). Tie-break unchanged: least-recently-used first among eligible lanes (slot order breaks recency ties, as today). Scope note: the OVERSHOOT_CAP ceiling below only arms when `min_idle > 0` AND `capacity > 0` — with either knob off, today's exact policy runs (G1's flag-off byte-identity already implies this; written down so no reader has to derive it).

**The memory cost, named (verdict round 1):** capacity is already soft — when nothing is evictable, the load proceeds. A min-idle floor widens that softness: a burst touching N lanes inside the window pushes residency toward capacity + N, routinely. Two counters, both in scope:

1. **Overshoot ceiling:** when resident lazy lanes exceed `capacity + OVERSHOOT_CAP` (const 16 — a named const, not a second env), the idle floor is IGNORED for the oldest lanes (plain LRU resumes above the ceiling). Residency is thereby bounded: `≤ capacity + OVERSHOOT_CAP` for lazy lanes, always.
2. **G-row for the bound:** the oscillation fixture asserts peak resident lazy lanes ≤ capacity + OVERSHOOT_CAP alongside the reload-reduction figure — the churn win is only a win if the memory price is pinned.

If NO lane is eligible at eviction time (below the ceiling), evict nothing this trigger (best-effort stays best-effort — a skipped eviction is counted and visible in the policy log line, never silent).

Explicitly out of scope: touching the release wire (`POST /arsenal/release` — an explicit operator act always evicts), eager rows (exempt today, exempt here), the epoch/monotonic-swap machinery (`check_epoch_tag` untouched), and any StepNoiseMonitor wiring (deliberately not consumed — see the honesty note).

## Tasks

- [ ] T1 `min_idle` in `lru_make_room` over the existing `last_used_ms()` epoch stamps (process-relative millis — order only, no wall-clock meaning); env parse + clamp via a `lru_min_idle()` twin of `lru_capacity()`; 0 = off; OVERSHOOT_CAP const beside it
- [ ] T2 G1: flag-off eviction decision sequence byte-identical — a deterministic fixture (fixed lane sequence + capacity) pins the eviction decisions with and without the env
- [ ] T3 G2: seeded capacity-oscillation fixture — with the floor armed, evict→reload cycles drop materially (pre-registered: **≥ 50% fewer reloads** on the fixture — a materiality bar against a synthetic fixture, no production churn data exists yet; stated as such) at equal capacity and equal served-decision set (answers unchanged — latency/residency axis only)
- [ ] T4 G-row (memory): peak resident lazy lanes ≤ capacity + OVERSHOOT_CAP on the same fixture
- [ ] T5 G4: **zero alloc DELTA vs current** — the current policy already `collect()`s a `Vec<(u64, usize)>` per loop pass (`serve_edge.rs:762–787`), so "alloc-free" is not the incumbent claim; this issue must not make it worse. (Replacing the Vec with a fixed-capacity min-scan is a separate cleanup, deliberately out of scope here.)
- [ ] T6 Policy log line names skipped eviction counts (loud-zero discipline: `skipped_not_idle=N` visible when it fires)
- [ ] T7 Record: `.benchmarks/` row + a one-line cross-ref from riir-train Research 485's consumer list (as motivated-by, not consuming)

## Non-goals

- No answer-path change: served decisions stay the frozen-GOAT-picks replay (the parity gate pins it) — eviction order affects latency, never answers.
- No interaction with the 503 contract or the load/swap FSM — this only reorders WHO is eligible for eviction.
