# Bench 040 — the earned typed_decisions run: the NLEH v2 per-option head — frozen read 0.7550 (+10.75 over the displayed arm)

**Status:** RECORD — the train-side half of 016 T9 is COMPLETE (the v2 scoring
shape + the trainer + the earned holdout + the SINGLE frozen test read). The
seat half (arena replay + lane-doc emit + site publish) is the follow-up: the
C1 encoder arm currently scores ONE question per case with the ENGLISH
checkpoint — typed needs the multi-question-per-case flattening and the TYPED
checkpoint, a deliberate widening with its own gates.

## The run (every number from this session, m3-max, AC, loadavg ~4.5-5)

- **The screen (Bench 039)**: typed-checkpoint reference floor **0.7445** on
  the wire-faithful t20k test read — +9.7 pt OVER the 0.6475 seating bar →
  the head run EARNED (pre-registered rule).
- **The trainer** (`instinct_typed_head_trainer`, NLEH v2): train pool 4000
  question rows (800 cases; widths 2/4/5), holdout 800 stratified over
  (width, gold) cells, gold-only arm (no teacher exists for typed — not a T3
  suite), 15 epochs, hidden 128, Adam batch-32 — the v1 discipline verbatim.
  - reference logits on the TRAIN pool: 0.8225; **on holdout: 0.7762**
  - **v2 head holdout: 0.7975** (+2.13 pt over the reference holdout) →
    EARN YES (the issue-600 class-relative bar).
- **The artifact**: `.raw/t608/typed_encoder_v2.bin` (riir-train, this box)
  · blake3 `c2e9f3468eb18ba26a93ff14b07debb5a01b4cde71f6dbd48e6fc493ebc89cd3`
  · NLEH v2 · d 1024 · hidden 128 · feat/option 2048.

## The single frozen test read (nothing tuned against it)

```
winner .raw/t608/typed_encoder_v2.bin · NLEH v2 per-option · d 1024 · hidden 128
  winner  test acc 0.7550  (1510/2000)
    2-wide: 600 rows · 487 hits · 0.8117
    4-wide: 1100 rows · 791 hits · 0.7191
    5-wide: 300 rows · 232 hits · 0.7733
  pick agreement head-vs-reference: 1864/2000 (0.9320) · both-right 1432
  · head-only-right 78 · ref-only-right 57
```

- vs the reference floor 0.7445: **+1.05 pt** (78 head-only-right vs 57
  ref-only-right — the head adds real lift, not noise).
- vs the DISPLAYED family arm (hybrid H2 **0.6475**): **+10.75 pt**.
- vs the board's best (AgentJev **0.7715**): −1.65 pt (their stack stays
  ahead on typed — recorded honestly).

## What this would seat

The **fourth Rethink encoder cell** (sst5 0.5267 · xnli 0.8600 · ag_news
0.9475 · typed 0.7550) and the STRONGEST vs the displayed arm (+10.75 pt —
sst5 was +10.5). Expected board effect: vs-Reflex 9/9 ahead; the typed row of
the vs-best card moves from the hybrid's 0.6475 to 0.7550.

## The follow-up (seat half — the 017 pattern, its own session)

1. `encoder_arm.rs` v2: `read_nleh_v2` + per-option forward + the
   MULTI-QUESTION flattening (typed carries 5 questions/case — the C1
   one-question-per-case refusal must widen to the per-question form the A0
   arms already use) + the **TYPED checkpoint** selection (the artifact rides
   `Checkpoint::TypedDecisions`, not english).
2. The arena `--encoder-art` replay on m3-Metal over the seat's own test
   rows → the cell-identity witness (**0.7550 must reproduce**, the
   third-posture law) + the auto-emitted lane doc (017 T5's
   `write_encoder_lane_doc`).
3. Lane-scoped site publish (`PUBLISH_BENCH_LANES=encoder`) + smoke updates
   (the Rethink lane grows 3/9 → 4/9 — coverage literals are data-derived,
   prose is count-free).

## Reproduction

```sh
# the artifact is sealed in riir-train/.raw/t608/ on this box; the trainer:
CARGO_TARGET_DIR=/tmp/t608_train_build cargo run --release -p riir-train-engine \
    --example instinct_typed_head_trainer -- \
    --cache .raw/t608/typed_train_lenc_typed.bin --cases .raw/t608/typed_train_cases.jsonl \
    --holdout 800 --epochs 15 --hidden 128        # deterministic (SplitMix64 seed 0x599_599)
# the frozen read:
/tmp/t608_train_build/release/examples/instinct_encoder_eval \
    --winner .raw/t608/typed_encoder_v2.bin \
    --cache .raw/t608/typed_test_lenc_typed.bin --cases .raw/t608/typed_test_cases.jsonl
```
