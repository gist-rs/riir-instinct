# Bench 0050 — the bekko distill T2 reads (riir-train Issue 608 T3): massive WASH at the serving arm (10/10/280), xnli decisively below — both negatives recorded, nothing served changes

**Status: MASSIVE — T2 FAIL (the bekko specialist is a wash at the serving arm: bekko-H2 vs incumbent-H2 mean +0.0000, 10 wins / 10 losses / 280 ties) · XNLI — T2 FAIL (bekko-A1 0.4000 vs the served A0 0.5233, the instrument registered A0 outright) · the honest-outcome rule applies: negatives recorded, no mint into the serving dir, the manifest is byte-untouched.**

## Question (pre-registered, Issue 608 T3)

Do bekko-68m-distilled specialists clear the T2 gate vs the CURRENT SERVED rows —
massive H2(β=1,nmin=2,τ=4) 0.8400 (synth seat), xnli A0 0.5233? The bekko teacher
measured strong (Bench 103: massive test 0.8667, xnli test 0.6767 — xnli the board's
largest gap), and the owner's distill lane (Issue 608) names both suites as targets.

## The pipeline (all train-side until the single test read)

1. **Teacher dumps** (reflex `--distill --distill-teacher bekko`, the T1 seam landed
   for this — reflex `20527e6`): RIDT v1 + BLAKE3 over the FULL train splits of
   `.raw/datasets_t20k` — massive 11,514 rows / 60 classes, teacher(train) acc
   **0.9244** (p50 63 ms, 739 s) · xnli 20,000 rows / 3 classes, teacher(train) acc
   **0.6668** (p50 42 ms, 851 s). The RIDT headers carry `license: MIT
   (hotchpotch/bekko-system-one — Copyright (c) 2026 Yuichi Tatsumi; verified
   2026-10-02)` — the attribution law riding the artifact (Issue 608 T5). Seals:
   massive `7b2a9ad88bc047bf…`, xnli `bb29de6f3aa7f36d…`.
2. **Student retrain** (riir-train `instinct_arm_b`, gold_mix ∈ {0.0, 0.5} +
   Arm A): both suites' winners are **B(gold_mix=0) — the pure-distillation arm
   BEAT the gold-trained arm** (massive 0.6850 vs A 0.6600; xnli 0.3700 vs A
   0.3650, holdout 200) — the distillation WORKS train-side; every winner beat its
   lr=0 control. Artifacts minted to `data/instinct_specialists_bekko68m/`
   (massive blake3 `a0310b4bb44747b1…`, xnli `6aff5eb78ba8f173…`) — a SEPARATE
   dir, never the serving dir.
3. **The single test reads** (this bench, the pre-registered decision): the 0029
   protocol per suite, `--winners-dir` pointed at the bekko dir — the ONLY change
   vs the serving posture.

## The massive read (synth seat, `--skip-pin-a0` — the 0029 overlay posture)

```
target/release/arena --suite massive_intent_en \
  --winners-dir ../riir-train/data/instinct_specialists_bekko68m \
  --synth-corpus ../riir-reflex/.raw/corpus_synth/massive_intent_en_synth.jsonl \
  --skip-pin-a0 --out .benchmarks/0050_bekko_distill_t2/massive
```

| arm | bekko specialist (this read) | incumbent openthai specialist (0029) |
|---|---|---|
| A0 | **0.8133** (244/300 — the synth-seat anchor reproduced EXACTLY) | 0.8133 |
| A1 | 0.8267 | 0.8167 |
| H1 | 0.8300 | 0.8333 |
| H2 best | 0.8400 (β=1,nmin=2,**τ=2**) | 0.8400 (β=1,nmin=2,**τ=4**) |

**The paired verdicts** (per-row `correct` vectors, both predictions.json frozen):

| comparison | mean | LB95 | wins/losses/ties |
|---|---|---|---|
| bekko-H2 vs incumbent-H2 (**the serving arm**) | **+0.0000** | −0.0246 | **10/10/280** |
| bekko-A1 vs incumbent-A1 | +0.0100 | −0.0163 | 13/10/277 |
| bekko-H1 vs incumbent-H1 | −0.0033 | −0.0273 | 9/10/281 |

**Reading:** the bekko specialist is a **wash at the serving arm** — 280 of 300
identical picks, 10 flips each way, net zero. The +1.0 pt A1 edge (uncertified,
LB95 ≤ 0) is absorbed by the H2 fusion: the modelless engine dominates the fused
ordering, so a different-but-comparable specialist changes nothing the serving arm
serves. In-run T2-vs-A0: H2 0.8400 − A0 0.8133, mean +0.0267, LB95 −0.0039 →
REFUSED (byte-identical stats to 0029's incumbent gate — same discordant
structure). **T2 vs the SERVED row: FAIL (mean 0) — no swap.** The winner artifact
stays the openthai-trained one (digest 7bc3ee38…); the manifest is untouched.

## The xnli read (gold seat, A0 pin ON — the pin's reflex-run() leg reads the gold corpus)

```
target/release/arena --suite xnli_en \
  --winners-dir ../riir-train/data/instinct_specialists_bekko68m \
  --out .benchmarks/0050_bekko_distill_t2/xnli
```

- **A0 pin held: arena == reflex run() == 0.5233 (site ✓)** — the read is valid
  (reflex at `20527e6`; the distill/jsonl-lane changes do not touch the seat path).
- A1 (the bekko specialist): **0.4000** — 12.3 pt BELOW the served A0 0.5233.
- H1 0.4000; the best H2 cal-side read 0.5650 but the instrument registered
  **A0 outright** (rank-0 5/48 candidates).
- **T2: FAIL decisively.** The teacher's 0.6767 test accuracy does not survive the
  bag-student distillation (0.3700 train-holdout predicted this; the seat's cal
  machinery cannot close a 12-pt specialist deficit).

## The adjudication (the honest-outcome rule, Issue 608's own law)

1. **Both T2 gates MISS — negatives recorded, nothing served changes.** The bekko
   distill path does not produce a promotable specialist for either target suite.
   The board's massive/xnli cells stay exactly as they are.
2. **What the negative MEANS:** the bekko teacher's per-suite strength (Bench 103)
   does not transfer through the one-vs-all bag student — massive's 60-class
   wash shows the H2 serving arm is engine-dominated (the specialist is nearly
   irrelevant to what serves), and xnli's specialist is simply far below the
   modelless lane. The distill MACHINERY is the landed value: `--distill-teacher
   bekko` (reflex `20527e6`), the RIDT license field, and `--synth-teacher bekko`
   (Path 2's machinery landed with the same seam).
3. **Path 2 stays open on its own merits** (the synth-corpus bekko-veto operates
   on the ENGINE axis — the axis that actually moves the serving arm, per 0029's
   +3.33 A0 corpus lift): the corpus the openthai veto produced is the current
   lift; a bekko-veto corpus is a different acceptance profile at comparable
   teacher strength (massive train acc 0.9244 vs openthai's test 0.9200). Not
   decided here — its own pre-registered read when run.
4. **The artifacts stay on disk, unregistered:** `data/instinct_specialists_bekko68m/`
   (train-side winners, decoded-OK) — they are the negative's evidence, not
   candidates for serving. Issue 608's "On PASS" task (arsenal row + serve parity
   + deploy.yaml) is NOT exercised — its precondition failed.

## Box state

`PROVENANCE: power=AC Power load=3.66 swap=2611.25M canary=129.4us/best5
powermode=2(high)` (the distill dump's preflight; the arena runs carried the same
box state — the arena's own box-state line: power AC Power · powermode high ·
load 3.66 · swap ~2603 MB · quotable true). Teacher dumps: massive 739 s + xnli
851 s CPU FP32 (the author's reference posture). The paired stats are
deterministic integer arithmetic on frozen picks — box-independent.

## Artifacts

- `massive/` + `xnli/` — the arena runs (RESULTS.md, registration.json,
  predictions.json each)
- reflex `.raw/distill_teacher_bekko68m/` — the RIDT dumps (DATA on disk,
  gitignored by law) + `DISTILL.md`/`distill.json`
- riir-train `data/instinct_specialists_bekko68m/` — the train-side winner
  artifacts (gitignored by law)
- reflex `20527e6` — the T1 seam (`--distill-teacher bekko`, JsonlOracle, the
  license header field, `--synth-teacher bekko`)

## Cross-refs

- riir-train Issue 608 (the distill lane; T1+T2 done here, T3's gate decided
  here, T5's attribution landed in the T1 commit)
- Bench 103 (`../riir-reflex/.benchmarks/103_bekko_v0_17m_gate/`) — the teacher's
  measured cells
- Bench 0029 — the synth-seat certification this read reproduces (A0 anchor exact)
- reflex `.issues/042` standing negative — the distill replaces the SERVED
  ARTIFACT, never adds a runtime hop (unaffected: nothing served changed)
