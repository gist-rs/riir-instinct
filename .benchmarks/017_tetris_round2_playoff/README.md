# Bench 017 — the Tetris round-2 trained critic: the tail-targeted loss lever (riir-train Issue 580 T7 / instinct Issue 009 T7, round 2)

**Date:** 2026-09-28 · **Box:** M3 Max, load 7.65/6.96/7.43 (own playoff run) ·
**Verdict: NEGATIVE — the tail-targeted loss went BACKWARDS on play. Round-1
weights (Bench 010) stand as the strongest critic; b0 stands.**

## The round-2 lever (pre-registered in Issue 009's record before any eval read)

Bench 010's diagnosis — "the residual disagreement concentrates in catastrophic
placements; a re-open needs a tail-targeted loss or critic-guided search (not
more top-1)" — named the lever this round instantiates as two additive loss
knobs (defaults 0.0 = byte-identical round-1 trainer):

1. **Gap-weighted rank** (`--rank-gap 1.0`): each (pick, o) rank pair is
   weighted `1 + (q_pick − q_o)` — pairs where misranking is catastrophic
   (large teacher gap) dominate the rank gradient; near-tie pairs shrink
   toward unit weight.
2. **Tail-weighted BCE** (`--bce-tail 4.0 --bce-tail-pow 2.0`): each BCE
   sample is weighted `1 + 4·(1 − q)²` — the low-q tail (lost positions) is
   upweighted 5×, because BCE's gradient there (o − q) is ≈0 either way and
   round-1's argmax inside the tail was arbitrary.

Everything else identical to round 1: MLP 43→128→128→1 f64 tanh, BCE +
0.3×pairwise rank (β 8), AdamW lr 1e-3 cosine 80+50, batch 2048, 10 threads,
seed 0x5EED0097, the same b1600 dataset (train 201..=280, val 281..=300).
~17 min wall.

## What measured

Trainer (`riir-train data/tetris_critic_r2/`, digest `66069784fb0fadd1`,
config recorded in its metrics.json):

| metric | round 1 | round 2 |
|---|---|---|
| val top-1 agreement | 0.7110 | **0.6270** |
| val R² | 0.2765 | 0.2102 |
| lr=0 control (Arm A law) | 0.3782 | 0.3782 (the run still clears its control) |
| t2_interesting | False | False |

Play-off (`benches/tetris_critic_goat --model`, the standing b400 protocol —
same seeds/cap, ridge refit in-run at λ×n 0.3, paired seeds 1..=20, 607
beside):

| regime | reflex | b0 | ridge | **round-2 trained** | vs b0 | vs ridge | (round-1 trained) |
|---|---|---|---|---|---|---|---|
| garbage 16@75 | 21.6 | **907.8** | 218.6 | **173.5** | Δ−736.3 lb95 −888.2 (1/0/19) | Δ−61.0 lb95 −233.6 (8/0/12) | 554.6 |
| garbage 18@75 | 13.5 | **532.5** | 117.0 | **89.5** | Δ−465.1 lb95 −672.2 (5/0/15) | Δ−28.8 lb95 −150.7 (6/2/12) | 427.3 |

T3 gate (trained > b0 AND > ridge, both regimes, paired lb95 > 0): **FAIL** —
round 2 lost to the RIDGE in both regimes, not just b0. The b0 row reproduced
Bench 010 exactly (907.8 / 532.5) — cross-bench consistency held again.

## The honest reading

The tail-weighted loss REFUTED its own hypothesis. The design assumed round-1's
top-1 disagreement concentrated in catastrophic placements was a DISCRIMINATION
failure inside the tail that upweighting would fix. Measured instead: the
upweighting over-fit the tail (where nearly every label is ≈0) at the cost of
the value surface's overall shape — val agreement fell 0.711 → 0.627 (below
the ridge's own 0.657 plateau) and play fell with it (554.6 → 173.5). Round-1's
lesson ("imitation top-1 agreement is not play strength") cuts both ways:
destroying agreement destroyed strength.

Recorded levers still standing for round 3+: **critic-guided search** (use the
round-1 critic as a prior/beam inside a shallow lookahead — the critic's value
sharpened WHERE the teacher is uncertain, never trusted alone) or **a much
larger fit** (round-1 architecture at 4–8× width; the perfect-imitation ceiling
≈ teacher ≈ 985 > b0 908 still holds). The gap-weighted-rank half was not
isolated from the BCE-tail half this round — if round 3 revisits the loss, run
the two levers in isolation first (a 2-arm ablation at b400 collection, ~70 s
each playoff).

## Records

- Weights + parity fixture: `riir-train/data/tetris_critic_r2/` (gitignored —
  the `data/tetris_critic` ignore covers the lane; weights are minted bytes).
- The playoff JSON: `tetris_critic_arm.json` (this dir), `--out
  .benchmarks/017_tetris_round2_playoff`.
- The round-1 artifacts (Bench 010, digest `a12f4a69c167b207`) are UNTOUCHED
  and remain the lane's standing strongest critic.
