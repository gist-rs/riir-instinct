# Bench 018 — the Tetris round-3 trained critic: the larger-fit lever, single knob (riir-train Issue 580 T7 / instinct Issue 009 T7, round 3)

**Date:** 2026-09-28 · **Box:** M3 Max, load 5.5–8.8 (own runs) ·
**Verdict: NEGATIVE on the T3 bar (b0 stands), POSITIVE on the capacity
lever — the strongest trained critic yet, and the width→play trend is now
monotone across three rounds.**

## The round-3 lever (single knob, everything else round-1)

Round 2 (Bench 017) confounded two loss knobs and went backwards. Round 3
changes exactly ONE variable from round 1: width. `--hidden 256,256`
(77,313 params vs round-1's 22,273), same b1600 dataset (train 201..=280,
val 281..=300), same loss (BCE + 0.3×uniform pairwise rank, β 8), same
schedule (130 epochs, 80 warm, batch 2048, lr 1e-3 cosine, wd 1e-5), same
seed 0x5EED0097, 10 threads, ~62 min wall. Trainer:
`riir-train/data/tetris_critic_r3/`, digest `bf9464925be910d8`.

| metric | round 1 (128×128) | round 2 (tail loss) | round 3 (256×256) |
|---|---|---|---|
| val top-1 agreement | 0.7110 | 0.6270 | **0.7574** |
| val R² | 0.2765 | 0.2102 | **0.3361** |
| trainer T2 gate | False | False | **INTERESTING** (cleared the ~0.75 bar) |
| lr=0 control | 0.3782 | 0.3782 | 0.1576 (cleared) |

## The play-off (standing b400 protocol; same seeds/cap; paired 1..=20, 607 beside)

| regime | reflex | b0 | ridge | round-1 | **round-3** | vs b0 | vs ridge |
|---|---|---|---|---|---|---|---|
| garbage 16@75 | 21.6 | **907.8** | 218.6 | 554.6 | **743.9** | Δ−172.1 lb95 −326.2 (1/**14**/5) | **Δ+503.2 lb95 +312.7** (15/2/3) ✅ |
| garbage 18@75 | 13.5 | **532.5** | 117.0 | 427.3 | **455.7** | Δ−80.6 lb95 −306.5 (6/7/7) | **Δ+355.7 lb95 +117.1** (12/1/7) ✅ |

T3 gate (trained > b0 AND > ridge, both regimes, paired lb95 > 0): the
**ridge half now passes in both regimes** (round 1 passed it too; round 2
failed it); the **b0 half still fails** — but the W/T/L shape is the
informative change: at 16@75 the round-3 critic TIES b0 in 14 of 20 paired
games (survives as long as b0) and loses 5. The deficit concentrates in a
few games where its argmax dies early — consistent with Bench 010's
catastrophic-placement diagnosis, now down to ~25% of games from ~95%.

Per-spot p50 0.840 ms (serve G2 bar 1 ms — within it, but 3.7× round-1's
0.228 ms; noted for T9's serve budget).

## The honest reading

The capacity lever MOVED play: 554.6 (round 1) → 743.9 (round 3) at 16@75,
and val agreement 0.7110 → 0.7574 — the width→play trend is monotone across
rounds 1/2/3 (round 2's regression was the loss, not the width). Round 1's
"agreement ≠ strength" lesson stands, but its converse now has three data
points: every agreement gain above the 0.71 plateau moved play up too. The
perfect-imitation ceiling ≈ teacher ≈ 985 > b0 908 still holds, and the
round-3 critic is at 82% of b0's mean (743.9/907.8).

**Not cleared: no mint, no serve wiring; b0 stands.** The eval seeds have now
been read four times (009/010/017/018) — each read was a pre-registered lever
from the issue's own record, and every negative was recorded as such; the
multiple-look cost is disclosed here rather than hidden.

## Standing round-4 levers (both priced)

1. **A much larger fit**: 512×512 (~300k params) projects ~2 min/epoch →
   **~4–5 h wall** — the next width rung, mechanically identical to this
   round. Whether the width curve bends before reaching b0 is unknown;
   round 3's marginal gain (+189 pieces for 3.5× params) suggests it bends
   but may not be exhausted.
2. **Critic-guided search**: the round-3 critic as a prior/beam inside a
   shallow lookahead (the serve-time expectation over the 7 next pieces,
   Issue 009's recorded ~240 evals ≈ <1 ms at this critic's 0.84 ms —
   TIGHT against G2; needs the budget re-derived before building).

## Records

- Weights + parity fixture: `riir-train/data/tetris_critic_r3/`
  (gitignored; minted bytes).
- Playoff JSON: `tetris_critic_arm.json` (this dir).
- Round-1 artifacts (`a12f4a69c167b207`) and round-2's negative record
  (Bench 017) untouched.
