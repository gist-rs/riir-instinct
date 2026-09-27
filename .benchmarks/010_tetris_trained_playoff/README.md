# Bench 010 — the Tetris trained-critic play-off (riir-train Issue 580 T3 / instinct Issue 009 T7)

**Date:** 2026-09-28 · **Box:** M3 Max, load 13.5/15.3/12.8 (sibling training active), 88% mem free · **Verdict: T7 BAR NOT CLEARED — b0 stands; the trained arm is the strongest critic yet (>> ridge) but loses to the champion 1-ply in both regimes.**

## What ran

The full T7 expert-iteration loop, first round:

1. **Dataset** (Bench 009's export, BLAKE3-verified): teacher b1600 search-root
   Q over seeds 201..=280 (train) / 281..=300 (val), garbage 16@75 + 18@75,
   1,068,934 train / 280,261 val samples, 288 B fixed-width samples.
2. **Trainer** (`riir-train crates/riir-train-engine/examples/tetris_critic_trainer.rs`):
   MLP 43 → 128 → 128 → 1, f64, hidden **tanh**, output sigmoid, BCE(soft
   target q) + 0.3 × pairwise logistic rank term (β=8) against the pick.
   AdamW lr 1e-3, batch ≥2048 samples (whole-group batches), 80 constant-lr
   epochs + 50 cosine-decay, 10 threads, seed 0x5EED0097, deterministic
   (fixed init/shuffle/reduction; fp reduction order is thread-count
   dependent — recorded, fixed at 10).
3. **Play-off** (`benches/tetris_critic_goat.rs --model …`): the Trained arm
   joins Reflex / champion-b0 / the in-bench ridge (b400 refit, the same
   seeds/cap — the same protocol), paired seeds 1..=20, seed 607 beside,
   cap 1000, both regimes.

## The play-off (the T3 gate)

| regime | reflex | b0 | ridge (b400 refit) | **trained** | trained vs ridge | trained vs b0 |
|---|---|---|---|---|---|---|
| garbage 16@75 | 21.6 | **907.8** | 218.6 | **554.6** | Δ+304.4 lb95 **+96.5** (11/2/7) ✅ | Δ−370.9 lb95 −609.2 ❌ |
| garbage 18@75 | 13.5 | **532.5** | 117.0 | **427.3** | Δ+325.9 lb95 **+104.4** (11/0/9) ✅ | Δ−110.4 lb95 −359.7 ❌ |

Gate (trained vs b0 AND vs ridge, paired lb95 > 0, both regimes): **FAIL** —
trained beats the ridge decisively in both regimes but loses to b0 in both.
b0 reproduced Bench 009/T6 exactly (907.8 / 532.5).

Cross-check vs the ridge's own b1600-target cell (Bench 009: 377.4 @
16@75): the trained head (b1600 data) beats it too — "trained > ridge at
equal teacher budget" holds independent of the in-run refit budget.

## Offline gates (T2, val 281..=300)

| metric | lr=0 control | trained | ridge (b1600, Bench 009) |
|---|---|---|---|
| top-1 agreement | 0.3782 | **0.7110** | 0.657 / 0.661 |
| q R² | −7.93 | **0.2765** | 0.32 / 0.37 |
| q-argmax ceiling | — | 0.9789 | — |

The Arm A law held: training beat the control (0.711 > 0.378). Agreement
cleared the ridge plateau (the linear-in-basis hypothesis CONFIRMED — the
plateau was a capacity limit, not a data limit) and R² approached the
ridge's. But the ceiling diagnostic says 0.711 still leaves a 0.27
imitation gap, and the play-off shows what it costs.

## The reading (why the negative is informative)

- **Imitation agreement ≠ play strength.** The trained critic matches the
  teacher's pick more often than the ridge does, and plays 2.5-3.6×
  stronger — yet still 350 pieces under b0. The residual disagreement is
  concentrated in catastrophic placements (the lossy tail), not the
  average decision. Any re-open must target the tail (tail-weighted loss,
  or critic-guided search rather than pure 1-ply argmax), not top-1
  agreement.
- **The prize is real but distant:** perfect q imitation would play ≈ the
  teacher (≈985 train-side pieces > b0's 908). Closing from R² 0.28 to the
  ~0.9-class fit that would flip the play-off is a bigger model/data
  effort — round 2 of the recipe is explicitly gated on round 1 clearing,
  which did not happen.
- **G2 (serve latency):** trained p50 0.228 ms/decision — ~1.3% of the
  1 ms bar (the ridge: 0.006 ms). Moot while the arm does not serve.

## T5's parity contract (landed + green)

`tests/tetris_critic_parity.rs`: the serve-side forward
(`TrainedMlp::score`) replays the trainer's scores **bit-exactly — 512 of
512 fixture rows** (f64, documented op order, format-v1 weights with the
activation id in the header). The parity fixture pins the exact shipped
digest `a12f4a69c167b207`. Skips loud without `TET_CRITIC_MODEL`;
`TET_CRITIC_PARITY_REQUIRE=1` makes the skip a failure.

## Traps paid for (all measured, all fixed same-session)

1. **`softplus` transcription NaN** — wrote `(−z.abs()).ln_1p()` (=
   ln(1 − |z|): NaN for |z| > 1) instead of `(−z.abs()).exp().ln_1p()` in
   the numerically-stable softplus. The loss display went NaN from epoch 1
   while the GRADIENT path (sigmoid-only) stayed correct — weights trained
   fine under a NaN loss readout (probe C's R² improved smoothly). A NaN
   number next to healthy curves is a display-path bug, not necessarily a
   model bug.
2. **Rank-term sign error (probe A)** — `dL/dz_o = +β·σ(−βΔ)`, not
   negative: the wrong sign pushes non-pick logits UP → saturation →
   ±inf logits → `inf − inf = NaN` in the delta → whole-model NaN. Fixed
   with the per-group rank-grad precompute (the pick's dz3 accumulates one
   term per pair).
3. **Reader w3 offset** — the format's `w3` is h2-long (Glorot fan_out=1),
   not 1; the first reader read 1 f64 and parsed `b3` from garbage.
4. **`x.iter_mut()` over 43 slots indexing `num[k]`** (len 33) — the
   one-hot loop must not sweep the numeric block.
5. **Training conditioning** (probes 1/B): sigmoid-hidden crawls into the
   constant-marginal solution at every lr tried (R² ≤ 0 after 23 epochs);
   the schedule that annealed lr while the fit was still descending (probe
   E) was the other half. tanh hidden (same bounded-squash family; the
   binding law is never-softmax) + 80 constant-lr epochs fixed both — a
   measured divergence from the issue's "sigmoid activations" wording.

## Artifacts

- `tetris_critic_arm.json` — the play-off (this dir).
- `trainer_metrics.json` — the trainer's config + curve + best checkpoint.
- Weights: `riir-train data/tetris_critic/tetris_mlp_v1.bin`
  (`a12f4a69c167b207`, untracked data; regenerate with the recorded
  config — deterministic at fixed `--threads 10`).
- Parity fixture: beside the weights (committed nowhere; regenerated by
  the trainer; the test consumes it via env).
