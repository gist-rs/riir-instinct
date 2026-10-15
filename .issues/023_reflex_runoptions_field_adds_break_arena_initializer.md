# Issue 023 — reflex RunOptions field adds keep breaking the arena pin initializer (3rd occurrence: pplx → d3 → jeva)

**Status:** OPEN — the 3rd occurrence (jeva, E0063) repaired in the same commit
that files this (compile restore, posture-identical `jeva: false`); the CLASS
stays open for a structural fix. Recurrence window trend: 1.5 d (d3) → 1.5 h
(jeva) — caught faster only because a clippy sweep happened to run.

## The class

Reflex's `harness::runner::RunOptions` grows a new comparison-lane field with
every lane landing (pplx `4ad25df`-era → d3 `46ddc75` 2026-10-14 → jeva
`83031d2` 2026-10-16). This repo's arena carries the ONLY out-of-reflex
struct-literal initializer (`pin_a0_identity`, `src/bin/arena.rs`) — an E0063
compile break at every field add, live on develop until someone repairs it.
The rider grid / arm_b runs do NOT compile the arena bin, so the break hides
from the lanes that run daily.

| occurrence | reflex commit | field | broken on develop | repaired |
|---|---|---|---|---|
| 1 (Bench 0060 class) | pplx lane | `pplx` | ~days | `5eccd45` predecessor |
| 2 | `46ddc75` (629 T2) | `d3` | ~1.5 d | `5eccd45` (2026-10-15) |
| 3 | `83031d2` (090 B1+B2) | `jeva` | ~1.5 h | this issue's commit (2026-10-16) |

## Why it keeps happening

The initializer deliberately lists every lane's posture (documentation value:
the A0 drift pin's posture is explicit per lane). `..Default::default()` is not
available (`RunOptions` has required fields, no `Default`), and listing means
every upstream field add is a break by construction. Cross-repo: reflex cannot
see this consumer.

## Candidate structural fixes (decision owed, reflex-side coordination)

1. **Reflex-side constructor** — `RunOptions::lanes_off()` (or `Default` +
  builder): every comparison lane `false`, required fields set by the caller.
  The arena pins one line; future field adds never break it. Cost: a reflex
  API addition consumed here (one-line change at the pin).
2. **Instinct-side compile canary** — a cheap `cargo check --bin arena` step
  in this repo's routine (or a documented "run after any reflex lane landing"
  rule in both repos' AGENTS). Cost: process discipline, no code; catches the
  class the same day but does not prevent it.

Option 1 removes the class; option 2 only shortens the window. The reflex-side
owner call decides (this repo cannot add reflex API surface unilaterally —
boundary law).

## Repair record (3rd occurrence)

`jeva: false` added to `pin_a0_identity` beside `d3` (same comment pattern).
Validated: `cargo clippy --all-targets -- -D warnings` exit 0 (the failing
posture; the "generated 1 warning" lines are the known incremental-cache
hardlink notices, not lints). No other RunOptions initializer exists in this
repo (grep-verified at the repair).
