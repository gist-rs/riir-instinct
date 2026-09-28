# Bench 021 — the Tetris trained-critic round-4 playoff (512×512, Issue 009 T7)

**Status: MEASURED — T7 BAR NOT CLEARED; b0 stands.** The negative is the
record (Bench 009's bar governs). Round-4's weights are NOT minted, the
serve path stays unwired (T9's gate).

## The lever (pre-registered in issue 009)

Single-knob capacity: `--hidden 512,512` (~287k params), everything else
identical to round 3 (same loss/schedule/seed/dataset; riir-train
`data/tetris_critic_r4/`, trainer epochs 130, final val agree 0.7697 —
the trainer's own T2 gate INTERESTING, above round-3's 0.7574).

## The playoff (the frozen eval seeds 1..=20 paired + 607 beside)

| regime | b0 | ridge arm | R4 trained | trained−b0 (LB95) | W/T/L | trained−ridge (LB95) |
|---|---|---|---|---|---|---|
| garbage 16@75 | 907.8 | 218.6 | **804.8** | −59.4 (−214.2) | 2/16/2 | +615.9 (+411.7) ✓ |
| garbage 18@75 | 532.5 | 117.0 | **462.7** | −73.2 (−338.5) | 7/5/8 | +363.1 (+149.6) ✓ |

G2: trained per-spot p50 **3.760 ms / 3.422 ms** — 3.4–3.8× OVER the
1 ms serve bar (the width caveat priced in the round-3 record, now
measured).

## The reading

- The width→play trend is MONOTONE across rounds 1/2/3/4
  (554.6 → 743.9 → 804.8 at 16@75; 427.3 → 455.7 → 462.7 at 18@75) and
  agreement too (0.7110 → 0.7574 → 0.7697) — but the returns are
  DECAYING (+189.3, then +60.9) while the G2 cost grows ~linearly with
  width. The b0 gap (−59.4) closed by less than a third of round-3's
  move; W/T/L moved 1/14/5 → 2/16/2.
- The ridge half passes in both regimes, comfortably. The trained
  critic's remaining deficit is the same ~10% catastrophic tail.
- **The capacity lever is EXHAUSTED by the trend + the G2 breach**: the
  next width rung (1024×1024, ~1.1M params) prices at ~8–15 ms/spot —
  8–15× the serve bar — for a projected return well under round-4's
  +60.9. The recorded round-5 levers stand: critic-guided search (needs
  the budget re-derivation; at 3.76 ms/critic the ~240-eval
  next-piece expectation is ~0.9 s/decision — 900× the G2 bar) or a
  different loss/data shape. Both are NEW pre-registrations, never
  defaults.

Eval seeds read FIVE times now (009/010/017/018/021), every read a
pre-registered lever, every negative recorded.

Box state: load ~9.5 (the openthai T4 teacher dump + its server + the
R4 trainer's own tail ran concurrently) — the playoff is
deterministic-per-seed; the piece counts are the claim, the ms columns
are inflated by contention and NOT a serve-bar measurement (the G2
breach reading above is the RATIO to the bar, conservative even so:
an unloaded run would read FASTER, i.e. the breach is bounded below
by 3.4×, not above).

Record: `tetris_critic_arm.json` (the machine record).
