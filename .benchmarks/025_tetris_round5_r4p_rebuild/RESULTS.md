# Bench 025 — the round-5 r4' rebuild (issue 009 teacher-blend lane, component qualification)

**Status:** MEASURED — attempt 1 recorded as an INVALID BUILD (schedule drift, caught by
verdict before any A/B spend); attempt 2 (valid config) **QUALIFIED at 0.7679 ≥ floor
0.7647**. The blend component is qualified; the teacher A/B (026) is the next gate.

## The lane (pre-registered in issue 009, commits 37dffa1 + 26fe411)

Round 5 blends the r4 Tetris critic's logit into the chance_puct teacher eval at w=0.5
(teacher-side lever — the teacher runs offline; the serve path stays the 1-ply student).
Before any A/B spend, the blend component ("the r4 critic") must be rebuilt on THIS box
(the M3 weights are unreachable from here; the M3-copy route remains the cleanest unblock)
at the EXACT r4 config and qualify at **0.7647 = r4's own M3 val-agreement 0.7697 − 0.005**.
Below the floor: provenance defect, lane stops.

## Attempt 1 — INVALID BUILD (discarded; digest `7df0615d5c11a9de`)

Measured best val agreement **0.7333 @ epoch 127** (curve 0.6544@22 → 0.6921@44 →
0.7200@72 → 0.7319@105 → 0.7333@127; control 0.2263 is the untrained-init number for the
512-wide arch and is NOT comparable with round-1's 0.3782 128-wide control — not cited
either way; q-ceiling 0.9786). Full metrics:
`attempt1_invalid_build_metrics.json` (copied from riir-train
`data/tetris_critic_r4p/metrics.json`; the parity fixture rides beside it).

**The cause is NOT the dataset and NOT a floor event: the run omitted `--warm-epochs 80`.**
The flag defaults to **0** (`tetris_critic_trainer.rs:724`), so attempt 1 trained a full
130-epoch cosine where rounds 1/3/4 trained 80 constant-LR epochs then cosine over the
last 50 (Bench 018 README:14: "130 epochs, 80 warm, batch 2048, lr 1e-3 cosine"; round 4
was "identical to round 3"). The attempt-1 log's own lr column decays from epoch 2
(9.99e-4 @ep3, 9.86e-4 @ep11) where a warm schedule holds 1.00e-3 through epoch 81 —
the diagnostic the verdict reviewer named, verified against the code and the prior
records before adoption. A schedule that starts decaying 80 epochs early is a first-order
training change and fully accounts for a −0.036 agreement gap; a ~0.3% dataset-count
delta never did.

Two holes the attempt exposed, both closed:
1. `warm_epochs` was missing from the pre-reg's "EXACT r4 config" list (now amended in
   issue 009) AND from the trainer's `metrics.json` config block (fixed in riir-train
   `2b90664a`; the launch script `scripts/r5_launch_train.ps1` is committed beside it as
   attempt 1's provenance — the missing flag is visible in its argument list).
2. `.benchmarks/.highwater` was stale (read 0023 while disk had `024_sst5_nbsvm_v2`);
   repaired to 0025 at this record's allocation. The `.raw/` dataset is gitignored, so
   the dataset digests are recorded HERE (below).

**Attempt 1 is discarded, not a floor verdict:** no A/B ran, no r5 dataset was built,
the eval seeds (1..=20 + 607) remain at 5 reads. The floor (0.7647) applies UNCHANGED
to attempt 2.

## The dataset (both attempts; the pinned input)

Regenerated deterministically on THIS box from the pinned export recipe (b1600 teacher,
seeds 201..=280 train / 281..=300 val, regimes 16@75+18@75, cap 1000, threads 10,
rng salt 0x5eed892, champion `68cae9d382014662`). Counts 1,065,793 train / 279,454 val
(Bench 010's M3 originals: 1,068,934 / 280,261 — ~0.3% fewer; mechanism UNATTRIBUTED,
see the open question below). Per-file BLAKE3-16 (the manifest lives in gitignored
`.raw/`, so the digests are recorded here):

| file | blake3_16 |
|---|---|
| train-16x75.bin | `3c54586fbdc16fe3` |
| val-16x75.bin | `3751428593d3ef1d` |
| train-18x75.bin | `5fe86d2f3aad4589` |
| val-18x75.bin | `f93cd599d50a4c95` |

**Plain-path digest-clear (measured while adjudicating attempt 1):** the dataset was
regenerated a second time at the PRE-BLEND commit `67b18e4` (detached worktree, isolated
target dir) — **all four file digests byte-identical** to the `60bd324`-era regen on this
box. The blend commit did not change the plain teacher; the "plain path is byte-identical"
design claim is now a measurement.

## Attempt 2 — the valid rebuild: **QUALIFIED (0.7679 ≥ floor 0.7647)**

Same command + `--warm-epochs 80`, out `riir-train data/tetris_critic_r4p2/`:

- Best val agreement **0.7679 @ epoch 130** (curve saturated at the checkpoint:
  0.7635@104 → 0.7679@126..130; val R² 0.3598; q-ceiling 0.9786; t2_interesting
  TRUE). **Floor 0.7647 CLEARED (+0.0032)**; the r4-M3 reference is 0.7697, so the
  rebuild gap is −0.0018 — within the floor's −0.005 tolerance and consistent with
  the same-critic premise. The schedule-drift explanation is CONFIRMED by the
  intervention: the ONLY change from attempt 1 was the warm-epochs flag, and the
  agreement moved +0.0346 (0.7333 → 0.7679).
- Weights `data/tetris_critic_r4p2/tetris_mlp_v1.bin`, digest **`5c7eb6ca4ae3bb3e`**
  (TETMLP1 v1, 285,697 params), wall 13,866 s. The fixed trainer recorded
  `warm_epochs: 80` in the metrics config block (the completeness fix verified).
- Schedule mechanics verified end to end: lr 1.00e-3 held through epoch 80 (log),
  cosine from epoch 81 (6.28e-4 @ep102 → 5.67e-4 @ep104), matching the
  80-constant-then-cosine-50 shape of rounds 1/3/4.

**The blend component is qualified. Next pre-registered gate: the teacher A/B
(026) — plain b1600 vs blended b1600, seeds 401..=420, three cells, no-op guards
before cells, GO iff paired lb95 > 0 on ≥ 1 cell AND no cell ub95 < 0.**

## Open question recorded (not attributed, not acted on)

The ~0.3% sample-count delta vs the M3 counts is an observation, not a finding: the M3
per-file digests are not on this box (Bench 010 recorded counts only), so "the dataset
differs cross-arch" rests on counts alone and its mechanism (x86 vs Apple Silicon ulp
divergence in the teacher's search rollouts) is a HYPOTHESIS. It only becomes load-bearing
if attempt 2 also misses the floor. The clean disolver either way: the owner copies the
M3 dataset files (and/or the r4 weights, digest of record `7e319da89b7f5a7c`) — bytes
cross boxes; rollouts do not.

## Box state

Windows 11 / i7-13700K 16C / 31.8 GiB, AC power. Attempt 1 ran 17:03–21:05 alongside two
sibling riir-infer bench sessions (GPU-bound, ended ~18:40); epochs ~102–122 s. Attempt 2
launched on a quieter box (one sibling GPU bench continuing at reduced count).
