# 007 — Tetris teacher check, serving-matched (instinct Issue 009 T6)

**Status:** CLOSED — 2026-09-27. Verdict: **PASS — T7 (expert iteration) is
unblocked.** The teacher strictly beats free Reflex's board under the
serving information rule, on held-out seeds, in separating regimes.
Data: `tetris_teacher_check.json` (garbage regimes, cap 1000) +
`tetris_teacher_check_empty_board.json` (empty-board context, cap 300).

## Setup

- **Teacher:** `katgpt_core::chance_puct` over the rulebook-champion
  evaluator (`Genome::eval_with`, mode at the decision root), under the
  issue's information rule: **NO PREVIEW, FRESH BAG** — the chance model
  after every placement is a uniform draw over all seven pieces (serving
  never sees bag state). Budgets 0/100/400/1600 (the Bench-892 regime
  ladder); c_puct 1.5, top_k 8 (the 08-goat posture — NOT Bench 205's Go
  settings). Budget 0 = the 1-ply prior argmax, the shape of T8's
  baseline.
- **Baseline (free Reflex's board):** the corpus-fitted Tetris head,
  Bench 881 decoded arm — λ=1.0, corpus 2 660 options (anchors asserted;
  fixture BLAKE3 pins asserted). Piece-blind, state-blind, first-max
  argmax — the exact served posture.
- **Context row:** the champion genome `68cae9d382014662`'s own depth-3
  beam-6 search in its served posture (preview + true bag).
- **Engine:** one shared loop — `katgpt_tetris` sim, seeded guideline
  7-bag, `FromTop` physics, guideline scoring. Every arm faces the
  identical piece stream and start board per seed.
- **Seeds:** paired set 1..=20 (the T6 eval set — T7's training seeds must
  stay disjoint from these AND from 607); seed 607 runs beside, excluded
  from the paired gate. Teacher RNG per (seed, budget): `seed ^ 0x05EE_D892`.
- **Gate:** paired per-seed pieces diff (teacher − reflex) with the house
  instrument (`stats::paired_upper_bound_f64`): strict beat = lb95 > 0.

## Results (paired seeds 1..=20; seed 607 runs beside, excluded from every
## gate column — the JSON's per_seed rows carry it; teacher RNG ^ 0x05EE_D892)

Garbage 16@75 (cap 1000):

| arm | pieces/g | lines/g | pts/g | surv | p50 ms | W/T/L | Δ pieces | lb95 |
|---|---|---|---|---|---|---|---|---|
| reflex-head (baseline) | 22.1 | 6.0 | 360 | 0/20 | 0.01 | — | — | — |
| teacher b0 (1-ply eval) | 903.1 | 367.2 | 33 668 | 18/20 | 0.01 | 20/0/0 | +881.0 | +752.1 |
| teacher b100 | 1 000.0 | 405.5 | 37 728 | 20/20 | 0.30 | 20/0/0 | +977.9 | +969.7 |
| teacher b400 | 994.9 | 404.3 | 45 291 | 19/20 | 1.29 | 20/0/0 | +972.7 | +959.3 |
| teacher b1600 | 1 000.0 | 406.6 | 50 799 | 20/20 | 5.05 | 20/0/0 | +977.9 | +969.7 |
| champion-search (context) | 1 000.0 | 408.5 | 72 754 | 20/20 | 0.72 | 20/0/0 | +977.9 | +969.7 |

Garbage 18@75 (harder; the budget ladder separates):

| arm | pieces/g | pts/g | surv | p50 ms | W/T/L | Δ | lb95 |
|---|---|---|---|---|---|---|---|
| reflex-head | 13.8 | 261 | 0/20 | 0.01 | — | — | — |
| teacher b0 | 558.6 | 20 196 | 11/20 | 0.01 | 18/1/1 | +544.8 | +327.9 |
| teacher b100 | 561.0 | 23 055 | 11/20 | 0.31 | 17/2/1 | +547.2 | +330.5 |
| teacher b400 | 753.0 | 34 496 | 15/20 | 1.19 | 20/0/0 | +739.2 | +549.1 |
| teacher b1600 | 802.0 | 43 436 | 16/20 | 5.08 | 19/1/0 | +788.2 | +612.2 |
| champion-search | 900.9 | 64 441 | 18/20 | 0.76 | 20/0/0 | +887.0 | +754.7 |

Empty-board context (cap 300; seed 607 alone tops out at 44 pieces — the
live arena shape; 20 paired seeds beside): reflex 67.4 pieces/g, 0/20 to
cap; teacher b400 20/20 at cap (16 310 pts/g); champion 20/20 (23 406
pts/g).

## Reading

1. **The check passes with maximum margin.** Every teacher budget beats
   reflex 17–20 wins to 0–1 losses on both garbage regimes; the best
   budgets carry lb95 +612 to +970 pieces. Even **b0 — the bare 1-ply
   champion evaluator with no search at all — wins 18–20/0/1**. The
   information handicap (no preview, fresh bag) costs the teacher
   little against a piece-blind baseline.
2. **The regime separates exactly as designed.** The reflex head tops out
   at 13–22 pieces under garbage starts; a saturated both-survive run
   (the tie trap) never materializes. Seed 607 sits beside as the
   empty-board context the issue asked to keep.
3. **The budget ladder is informative for T7's serving posture.** At 18@75
   the ladder climbs monotonically (532 → 535 → 717 → 764 pieces/g); at
   16@75 it saturates from b100. Search buys real strength over the 1-ply
   prior in the hard regime — the T7 student (1-ply critic) is targeting
   the b0-class posture, and its GOAT (T3) is to beat REFLEX, not the
   search; the teacher-vs-student gap is the T8 attribution question.
4. **T8's baseline shape is already priced in:** the teacher-b0 row IS the
   1-ply champion-evaluator player. The trained critic must strictly beat
   THIS (fed the widened input, per Bench 006's recorded decision), not
   merely reflex.
5. **Information rule for T7 (pinned):** this check ran the teacher on a
   FRESH BAG (uniform-7 chance model) — LESS information than the widened
   wire will carry once the sidecar ships (grid + piece + bag remainder).
   Per the issue's own rule, the T7 teacher consumes the bag remainder
   when the sidecar lands; a teacher that wins on LESS information wins
   all the more on more, so this PASS holds.
6. **Seed hygiene (the cross-review's pin):** Bench 006 trained on seeds
   1..=40 and held out 101..=140. This bench evaluated on 1..=20 (+607
   beside) — OVERLAPPING 006's training range, which is harmless today
   (006's tables never saw these arms' games and the gate is vs reflex,
   not vs 006's decoders) but MUST NOT be reused as a held-out check for
   anything 006's tables touched; T7's training seeds must avoid ALL of:
   1..=40, 101..=140, 1..=20, 607 — **suggested T7 training set:
   201..=300.**

## What this does NOT claim

- Not a distillation landing (T7) and not a serving landing (T9).
- The champion-search row is context only — its preview/bag information is
  MORE than serving sees, by design; it is the ceiling marker, never a
  candidate.
- The teacher's per-decision latencies are in-process search costs (b1600
  p50 5.3 ms); the T9 serving posture is a 1-ply critic (~µs) — G2/G4 are
  re-asserted on THAT path at T9.

Provenance: M3 Max, AC, load 4.9–5.9 (10 workers), bench profile,
2026-09-27. Runner:
`cargo bench --features tetris_goat --bench tetris_teacher_check`.
