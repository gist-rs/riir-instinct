# Bench 006 — the tetris spot-wire information ceiling (Issue 009 T5)

**Status:** MEASURED — 2026-09-27, M3 Max (AC, charged); deterministic
(modelless, seeded — byte-identical verdict on repeat runs).

## What ran

`katgpt-rs examples/tetris_10_wire_ceiling.rs` (repo root example,
`required-features = ["template_decode"]`): the Issue 009 T5 instrument.
One serving-matched full-feature reference vs four wire-constrained
decoders, all 1-ply, no preview, hold OFF, fresh 7-bag:

- **champ1ply** — the rulebook champion `68cae9d382014662`
  (`Genome::champion_hybrid`) with `NextPreview` OFF + `depth = 1`.
  ⚠ Two traps found and fixed en route: `plies()` clamps to ≥2 while the
  preview rule is on (so a bare `depth = 1` is silently depth 2), and the
  depth-2 recursion injects `TOPOUT` (−1e12) values whenever the known
  next piece has no landing — the first quick run's LOO MSE read 8.7e19
  (λ-flat, the outlier signature) before the rule was killed. With the
  rule off, per-option values are pure `leaf_value` over the afterstate —
  exactly T8's baseline shape.
- **wire-lin / wire-t1 / wire-t2 / wire-t3** — trained to predict
  champ1ply's per-option value FROM THE WIRE ONLY: standardized ridge over
  the five ordinals (the reflex head's own shape); saturated cell-mean
  tables t1 (5-tuple, 1500 cells) → t2 (⊗ piece, 10 500) → t3 (⊗ the full
  v2 state sentence: hw, holes2, tallest/lowest region — 1 575 000), each
  arm falling down its own chain to the linear model below support 4.
  t3 is the MAXIMAL decoder over everything the current text wire carries.

Training: champ1ply-driven games (the teacher's own state distribution),
seeds 1..=40 × regimes {empty, 16:75, 18:75 garbage}, cap 300 — 29 253
states / 660 935 rows. λ selected by leave-one-GAME-out MSE via
sufficient-statistics subtraction (λ=0.001 won; LOO MSE ≈ 1.506e5, RMS
residual ≈ 388 on values spanning ≈ ±2e3 — the five ordinals already miss
the champion's value by ~⅕ of its range at the FIT, before any play).
Held evaluation: seeds 101..=140 × same regimes, cap 5000 — 120 games per
arm. Seed 607 never used.

## Results (full run, 2.2 s wall — accuracy probe, no latency claim)

| arm | mean pts | mean lines | tetrises | pieces | topouts/120 |
|---|---|---|---|---|---|
| champ1ply | **107 638** | **1256.2** | **44.8** | **3153** | 72 |
| wire-t3 (max wire decoder) | 90 | 1.8 | 0.00 | 19 | 120 |
| wire-t2 | 65 | 1.3 | 0.00 | 16 | 120 |
| wire-t1 | 71 | 1.4 | 0.00 | 17 | 120 |
| wire-lin | 66 | 1.4 | 0.00 | 18 | 120 |

Paired per-seed (champ1ply − t3): meanΔ 107 548 pts / 1254 lines;
115 W / 1 L / 4 T; sign test p < 0.0001. Every wire game topped out.

**Rank agreement on champ1ply's own held decisions (n = 378 301):**

| decoder | argmax agreement | mean rank of champ pick | champ in top-3 |
|---|---|---|---|
| wire-lin | 0.1227 | 10.1 | 0.211 |
| wire-t1 | 0.1470 | 6.8 | 0.320 |
| wire-t2 | 0.1586 | 7.6 | 0.310 |
| **wire-t3** | **0.3002** | 5.0 | 0.550 |

Occupancy is NOT the explanation: t1 (the option-only table) is dense
(592/1500 cells ≥ 4 covering the eval stream — 99.7% of t1-arm decisions
answered from a ≥4-support cell), yet t1 agreement is 0.147. The v2 state
sentence's clauses (piece, hw, holes2, region) lift t3 to 0.300 — real but
far from champion-level. The information, not the sampling, is the bound.

## Verdict (the pre-declared rules, judged on wire-t3)

**WIRE-MUST-WIDEN.** Agreement 0.3002 ≤ 0.85 and points ratio 0.001 with
sign p < 0.05 — both MUST-WIDEN gates fire independently. The five coarse
ordinals (holes / side / surface / height / clears) plus the entire v2
state sentence cannot represent the champion 1-ply evaluator's ranking:
the evaluator's discriminating features (exact hole counts, wells,
row/col transitions, bumpiness shape) are destroyed by the projection,
and T7's expert iteration distills THE SEARCH — a student that cannot
represent the teacher's ranking cannot beat the lane it must strictly
beat (Issue 009 T3). This confirms Issue 009's own binding-constraint
hypothesis quantitatively ("distilling into it would likely TIE" —
measured: the wire ceiling is ~0.30 rank agreement and a 0.001 board
ratio, far below tie).

This does NOT measure the reflex head itself — the GOAT bar (T3) stays
Instinct-vs-Reflex on boards; what T5 pins is that the CURRENT wire
cannot carry any champion-level decoder, so the widened wire is a
precondition of the whole Tetris lane, not an optimization.

## The T5 design decision (recorded per the issue)

The wire widens **Instinct-only** (deliberate owner posture recorded
here per the issue's own instruction): the modelless lane keeps the
5-class text sentence byte-identical — v0.2.x serving, the reflex-site
boards, and every existing fixture/pin are untouched. The Instinct lane
receives the raw afterstate for the spot question as a sidecar payload
(the 10×20 grid rows + current piece + bag remainder) beside the
sentences, over its own `/decide` request — the reflex-site game client
already holds the board it renders, so the site half is a sender change,
not an engine change. Consequences:

- The T7/T8 critic consumes raw afterstate features (the T8 baseline =
  the 1-ply champion evaluator on the SAME widened input — now actually
  expressible).
- The reflex binary contract change (an optional sidecar field on the
  game decision payload) + the reflex-site sender/board-card half are
  the two cross-repo lands T5 still owes — filed, not silently open:
  reflex-site T4 is the same session's board card (Issue 009 T4).
- T1's "byte-identical to Reflex ⇒ record a negative" check stays armed:
  if the widened-input student cannot beat the 1-ply champion baseline
  (T8), the lane dies there, as the issue orders.

## Instrument provenance

- `katgpt-rs` example: `examples/tetris_10_wire_ceiling.rs` at
  `6b6589370` (rebased from `483f2cf6e` — the citation follows the
  landed SHA) — run with
  `cargo run --release --example tetris_10_wire_ceiling
  --features template_decode` (the `[[example]]` row carries
  `required-features = ["template_decode"]`).
- Determinism: seeded 7-bag + seeded garbage boards + fixed seed split;
  repeat run byte-identical (verdict line re-verified after the clippy
  refactor).
- Drift pins inside the probe: champion genome id; the state-key mirror
  (clause-substring checks of `render_state_sentence` over all 21
  board×piece combos); pack round-trips incl. max keys.
