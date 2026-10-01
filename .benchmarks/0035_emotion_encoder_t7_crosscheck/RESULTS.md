# Bench 035 — Issue 016 T7 cross-check: the Bench-032 screen NEGATIVE reproduced EXACTLY on the 4090 (independent box, CUDA device, fresh seeds)

**Status:** CONFIRMATION — the Issue-825 discipline (two sessions reached T7
independently hours apart; before landing the second negative, the first was
cross-run). Every load-bearing Bench-032 cell reproduces EXACTLY here, so the
two records are one verdict wearing two postures: **the emotion encoder lane
is dead — emotion stays A0-served (0.8850), the posture-gap stands, no frozen
read was ever spent in either session.**

## What happened (the 825 shape, measured)

Two sessions picked up 016 T7 the same evening from the same summary handoff:

- **The sibling session (Bench 032, `a7dad63`, M3 Metal)** ran the
  screen-first law: the reference floor on the test split (0.5950) vs the
  0.8850 bar + the +14.7 pt max-lift ceiling → dead lane, the train never
  earned.
- **This session (4090 box)** took T7 as its open item and ran the FULL
  pre-registered sweep before fetching: 5-head gold-only sweep (fresh seeds
  0x016E016..A, quarantine + wire-faithful cases) → **all five heads refused
  the trainer gate on the holdout** (best 0.4835 vs the reference's 0.4975,
  floor 0.75; train-acc ~0.71 vs holdout ~0.47 = overfit). The frozen read
  was never spent here either.

Both negatives were recorded independently before either read the other. Per
Issue 825 ("before recording a negative, check whether a sibling shipped the
same primitive; cross-run the fixtures"), the cross-run follows — and it is
the strong form: **the two sessions' independent lanes AGREE, and the cells
reproduce byte-exactly.**

## The cross-check (this box, 2026-10-01 06:23, GPU quiet after the dq614 matrix)

Fresh `dump_encoder_states --checkpoint english` at `LAYA_DEVICE=cuda` over
the sibling session's OWN case bytes (`riir-train/.raw/t599/emotion_test_cases.jsonl`
— this session's wire-faithful maker; the sibling's t607 artifacts were never
synced to this box) → 400 rows in **8.6 s** (98,593,550 bytes — the same size
as the M3-Metal cache). Then `instinct_encoder_eval` over the fresh CUDA
cache with **this session's seed-4 head** (`.raw/t599/emotion_encoder_t7_s4_probe.bin`,
BLAKE3-verified at load — trained on this box's CPU cache, holdout-refused).

| cell | Bench 032 (M3 Metal) | this run (4090 CUDA) | verdict |
|---|---|---|---|
| reference floor | **0.5950** (238/400) | **0.5950** (238/400) | **EXACT** |
| recall sadness / joy | 0.73 / 0.74 | 0.7311 / 0.7373 | match |
| recall love / anger | 0.38 / 0.47 | 0.3784 / 0.4688 | match |
| recall fear / surprise | 0.37 / 0.10 | 0.3654 / 0.1000 | match |
| gold dist | — | [119, 118, 37, 64, 52, 10] | the 400-row seat ✓ |
| cache size | 98.6 MB | 98,593,550 B | match |

Metal-vs-CUDA feature drift flips ZERO cells — the third device-independence
witness for this lane (the 014 replay was the first two).

**Post-run re-witness (2026-10-01 13:58, GPU compute-clear again):** a THIRD
CUDA encode (`emotion_test_lenc_cuda2.bin`, same 98,593,550 bytes) + read
reproduces both cells exactly — floor 0.5950 (238/400), winner 0.6200
(248/400) — now under post-dq614 conditions (the sibling's matrix re-run
finished without incident). Three encodes, three identical readout pairs,
two devices: the record's numbers are stable, not a box artifact.

**The independent-seeds lift figure:** this session's best sweep head (s4,
holdout 0.4835, trained on the quarantined 15,997-row pool) reads **0.6200**
(248/400) on the test split — **+2.5 pt over the reference, 26.5 pts under
the 0.8850 bar.** Even the head the sibling's screen never trained lands
nowhere near the bar; the "+14.7 pt max lift" ceiling was optimistic for
emotion and the verdict holds with 4× the margin. Pick agreement
head-vs-reference: 323/400 (0.8075).

## This session's own sweep (the fuller negative, train-side — new data, same verdict)

| arm | holdout acc (2000, stratified) | gate |
|---|---|---|
| laya reference (no distill) | **0.4975** (995/2000) | the class's forecast |
| s0 / s1 / s2 / s3 | 0.4590 / 0.4745 / 0.4685 / 0.4730 | REFUSED ×4 |
| **s4 (best)** | **0.4835** | REFUSED (floor + class-relative) |

Reference train-pool read 0.5961; train-acc(2k) at epoch 15:
0.7210/0.7115/0.7165/0.7195/0.7060 — the heads overfit (~0.71 train vs ~0.47
holdout). The refusal is floor-robust: even `--min-holdout 0.0` refuses on
the non-waivable class-relative bar alone. Quarantine disclosure: 3
label-conflicting cross-split duplicate texts dropped train-side (the
bench-615 precedent; test split byte-identical, comparability preserved);
30 within-train conflicting-label texts kept (0.19% dataset noise, disclosed).

## Box state (the Issue-021 law)

Windows 11 Pro, i7-13700K, RTX 4090 24 GB, AC. The sweep + the train-cache
encode ran CPU-side CONCURRENTLY with the sibling session's pre-registered
dq614 Plan-614 T5 phase-matrix GPU run (93–100% util, 13–23 GB VRAM,
21:32–06:10 — no contention: the encode is CPU; the CPU posture is the lane's
G5 reference posture). The CUDA cross-check encode ran AFTER the matrix
exited (GPU compute-clear; 8.6 s). The dq run finished without incident; its
own close-out is its session's to land.

## Standing (unchanged from Bench 032, now double-measured)

Emotion stays A0-served (0.8850); the fragile-row posture-gap stands — THIS
class cannot secure the row; the row's security waits on a different model
class or the D1 posture. The frozen read budget stays RESERVED in both
sessions' registrations (neither spent it). Artifacts (gitignored,
regenerable): `riir-train/.raw/t599/emotion_*` (this session's cases,
caches, 5 probe heads + logs) — the sibling's `t607/` set is the M3's.
