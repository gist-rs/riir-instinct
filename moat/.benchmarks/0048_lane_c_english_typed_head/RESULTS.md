# Bench 0048 — issue 018 Lane C: the typed head over the adopted (Q8, english) encode — NEGATIVE (the english substrate does not carry the typed signal)

**Status:** RECORD — Lane C executed on the D1-pass branch and took the
issue's NEGATIVE branch: the typed cell keeps its incumbent
(`typed_encoder_v2.bin`, typed checkpoint) exactly as seated; no rollback
(there was none to roll back). Pre-registration: commit `8a3b721` BEFORE
any read. No frozen test read was spent (the trainer's reserve law — see
R2). No site change (no re-seat). Box: m3-max Metal, AC, powermode 2(high)
— `PROVENANCE: power=AC Power load=2.37 swap=2643.25M canary=skipped
powermode=2(high)` (preflight PASSED before the first Metal run).

## The executed chain

| step | tool | result |
|---|---|---|
| R1a train cache | `dump_encoder_states --checkpoint english`, `LAYA_WEIGHTS_VARIANT=q8`, Metal | 4000 rows → `typed_train_lenc_english_q8.bin` (4.67 GB) · 296.2 s |
| R1b test cache | same | 2000 rows → `typed_test_lenc_english_q8.bin` (2.33 GB) · 169.7 s |
| R2 train | `instinct_typed_head_trainer` (Bench 040 discipline verbatim: holdout 800 stratified, 15 epochs, hidden 128, Adam b32, seed 0x599599, plain CE) | **EARN NO — READ RESERVED** (below) |
| R3 frozen read | `instinct_encoder_eval` | **NOT TAKEN** — the earn bar refused, the read stays reserved (the trainer's no-cheat law) |
| gate | — | **NOT RUN** — the re-seat gate requires a candidate head; none exists |
| witness | — | not applicable (no WIN; pre-reg scopes it to a WIN) |

## The numbers (why the earn refused)

The class-relative earn bar is: head holdout acc > reference-logits holdout
acc on the SAME cache. Both sides measured:

| quantity | adopted-encoding (q8, english) | typed-checkpoint twin (Bench 040/039) |
|---|---|---|
| reference, train pool | **0.3698** | 0.8225 |
| reference, holdout | **0.3600** | 0.7762 |
| **v2 head, holdout** | **0.3113 — BELOW its own reference** | 0.7975 (earned) |
| reference, test floor (ref-only screen, never a winner read) | **0.3595** (719/2000) | 0.7445 |

The head trained FLAT: train-acc(2k) 0.31–0.37 across all 15 epochs, never
above its own reference — there is no learnable signal in the english
representation for typed questions. Probe exported per the reserve law:
`typed_encoder_v2_q8en_probe.bin` · blake3
`62d88fc68aaaa2710539e69e9991c78d9b7ecc930133948f235c96776f3d9ea5` (the
READ stays reserved — the test split was never touched by this head).

The per-width ref-only floor closes the loop at the representation level —
the english checkpoint is AT OR BELOW chance on every width:

| width | q8-english floor | chance |
|---|---|---|
| 2-wide (600 rows) | 0.4733 | 0.5 |
| 4-wide (1100 rows) | 0.3182 | 0.25 |
| 5-wide (300 rows) | 0.2833 | 0.2 |
| **all** | **0.3595** | ≈0.32 |

## Instrument corroboration (the chain is witnessed, the deficit is real)

- Bench 039 (line 17) already measured the F16 english floor on the typed
  TEST pool: **0.3575** (715/2000) — itself a wire-fidelity witness against
  the published laya-english typed base (reflex `.issues/029`), recorded
  there as "the substrate the lane would NOT ride".
- This bench's adopted-encoding floor: **0.3595** — Δ +0.0020 vs the F16
  twin. The adopted Q8 encoding preserves the (floor-level) read exactly as
  D1's retention table predicted, and the dumper × q8-artifact × english
  combination is corroborated end to end by an independently published
  number.
- The typed checkpoint's floors on the same rows (0.8225 train / 0.7445
  test) are the ~45-point representation gap the head cannot cross.

## The tripwire was NOT triggered — and why the landing pad was not spent

The pre-registration instantiated branch (b) as: the new head's TEST acc
below the reference on the same cache. No test read exists (reserved), so
the branch did not fire on its declared read — the holdout floor-failure is
the same failure CLASS, recorded here as the evidence. The CE+λ·Brier
landing pad (issue 018, the 562/576 tripwire) was deliberately NOT built
for this arm, reasoning on record: the pad is a calibration-shaped fix for
a head that TRAINED; this head never learned (flat at its own reference's
floor across every epoch — a loss reweight changes gradient emphasis, not
information content, and the deficit is representational: at-or-below-chance
per width). A future owner who disagrees can spend the one-loss-option
refit against the kept train cache (regenerable in ~5 min either way) — the
holdout plane is re-runnable freely; only the test read is one-shot.

## Consequences (recorded, none surprising)

1. **The typed cell keeps `typed_encoder_v2.bin` (typed ckpt, 0.7550
   seated record-only)** — the issue's negative branch, verbatim.
2. **The one-checkpoint (english) claim is REFUTED for typed at BOTH the
   floor and the head level.** It is measured, never retried without a new
   substrate — e.g. an english checkpoint fine-tune that carries
   typed-decision-shaped data would be riir-train work with its own screen
   (the 039 screen-first law), not a re-run of this lane.
3. **Lane B's value is RAISED, not lowered:** with typed un-servable from
   the english checkpoint, the two-resident-checkpoint posture (english +
   typed) is the standing shape — exactly the RAM/latency concern Lane B's
   one-worker-per-(checkpoint, device) plumbing addresses. Lane B stays the
   daylight session.
4. **Lane E is NOT triggered** — its trigger is a D1/D2 retention failure;
   neither fired (the encoder checkpoints work; the english one just cannot
   serve typed).
5. reflex-site: untouched — no board change (no re-seat).

## What remains (the amended order, post-C)

D2b (device-resident Q8 + dequant-fused kernels — the per-device
determinism gate's real subject, the multi-session kernel job) → Q4/PQ2
second; Lane B/A daylight.

## Reproduction

```sh
# R1 (riir-infer cwd; Metal; the adopted encoding)
LAYA_DEVICE=metal LAYA_WEIGHTS_VARIANT=q8 \
  cargo run --release -p riir-infer-laya --features laya-riir-metal \
  --example dump_encoder_states -- --checkpoint english \
  --cases ../riir-train/.raw/t608/typed_train_cases.jsonl \
  --out ../riir-train/.raw/t608/typed_train_lenc_english_q8.bin
# (…and the test half against typed_test_cases.jsonl)

# R2 (riir-train cwd; deterministic — SplitMix64 0x599599)
CARGO_TARGET_DIR=/tmp/c_lane_c_train cargo run --release -p riir-train-engine \
  --example instinct_typed_head_trainer -- \
  --cache .raw/t608/typed_train_lenc_english_q8.bin \
  --cases .raw/t608/typed_train_cases.jsonl \
  --holdout 800 --epochs 15 --hidden 128 \
  --out .raw/t608 --out-name typed_encoder_v2_q8en     # → EARN NO, probe exported

# the ref-only floor (never a winner read)
/tmp/c_lane_c_train/release/examples/instinct_encoder_eval --ref-only \
  --cache .raw/t608/typed_test_lenc_english_q8.bin \
  --cases .raw/t608/typed_test_cases.jsonl             # → 0.3595
```
