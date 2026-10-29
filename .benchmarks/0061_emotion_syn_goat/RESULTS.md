# Bench 0061 — emotion_syn arena GOAT read (the plan 015 bag half)

**Status: MEASURED — the bag class does NOT clear ESC-G1 on emotion_syn (LB95 −0.0213); A0 serves, no bag seat. The ENCODER class is the suite's specialist (the ESC GOAT: rethink [Bench 077](../../riir-rethink/.benchmarks/077_emotion_syn_esc_goat/RESULTS.md), all gates PASS).**

- **Lane:** reflex plan 013 T6 (claim `zed-amo-t6-emotionfit`, `59e2e93`) — the bag half of the emotion_syn re-fit, executed per rethink [plan 015](../../riir-rethink/.plans/015_emotion_syn_esc_goat_preregistration.md) §4a as amended (REVISE R1: the identity clause's premise corrected before any verdict).
- **Suite seat:** emotion_syn (the reflex NAME-label law — the first fit's int spelling joined Context unseatable, repaired `d0d7783`); the arena's A0 drift pin PASSED (arena == reflex `run()` at the published-posture knobs == 0.6925).

## The two postures of "modelless" (the R1 decomposition, both measured)

| posture | engine | emotion_syn A0 |
|---|---|---|
| pure drafter (harness defaults, selects OFF, nb_scale 0) | reflex Bench 168's floor | 0.3125 (modal-joy collapse; reproduced EXACTLY at today's HEAD) |
| selected posture (the arena/serve knobs, cal-selected nb 4.00) | this read | **0.6925** — the serving modelless arm, the GOAT floor |

The NB blend is INSIDE the engine's own scoring (`nb_scale · σ(margin/n)` per active route term) — the arena's posture is the deployed one; the floor correction is rethink plan 015 REVISE R1 (evidenced: the harness command reproduces 0.3125 exactly, same slice digests).

## The single test read (400 cases, the frozen slice)

| arm | accuracy | consult | p50/p99 µs |
|---|---|---|---|
| A0 | 0.6925 | 0.000 | 81/96 |
| A1 (bag winner: arm_a count bags, blake3 `b0cba63e6c767d99`) | 0.7000 | 1.000 | 0/1 |
| H1 | 0.7000 | 1.000 | 81/96 |
| H2(β=2,nmin=2,τ=2) — the instrument's pick, REFUSED by the superiority gate | 0.6975 | 1.000 | 0/1 |

**Paired per-question (from the frozen `predictions.json` rows, the instinct stats convention):** A1 − A0 mean +0.0075 (26W/23L/351 ties), **LB95 −0.0213**; H1 identical; H2 − A0 +0.005, LB95 −0.0168. The in-run superiority gate's refusal (H2 LB95 −0.0210) agrees with the frozen-row computation to the third decimal.

## Verdict

- The bag class's +0.75 pt point gain does not certify (LB95 < 0 at the corrected floor) — **A0 serves emotion_syn on the hybrid lane; no bag winner registers.**
- The specialist class for emotion_syn's ESC lane is the **encoder head** (rethink Bench 077: composed LB95 +0.0218, all gates PASS) — the ESC GOAT's G1 is about the encoder think leg, not this bag half; this record is the §3b/R3 evidence beside it.
- Challenger 2 (nbsvm v2) NOT RUN — its stage-1 validation barrier reads the whole test split, structurally conflicting with plan 015 §3b's train-pages-only pin + §4c's one-frozen-read law (rethink plan 015 R2).

## VOID run disclosed

Run 1 of this bench (the int-label artifact) was VOID under plan 015 §4a's then-pinned clause: the winner joined Context (unseatable — "600 of 600 case(s) present option key(s) with no artifact class row"), fell back A0-only. Root cause: the reflex seat's label law for emotion_syn is the six NAME keys (`train_row_label` → `EMOTION_KEYS`; `KeyMap::Name`, test-pinned in the reflex distill seam); the instinct trainer side carried ints. Fixed `d0d7783` (emotion_syn's `suite_row` arm maps int → name; sst5_syn stays int per the reflex FixedInt law). No verdict was drawn from the void run.

## Box state

amo (M5 Ultra, 30 cores), AC power, load 1-min ~3.15 at the run; release build; datasets `../riir-reflex/.raw/datasets` (the CANONICAL pool; emotion_syn pages per reflex plan 013 T4).
