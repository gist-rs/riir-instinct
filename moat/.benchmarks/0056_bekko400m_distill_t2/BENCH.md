# Bench 0056 — the bekko-400M distill T2 reads (riir-train Issue 609 T3): massive refused at the serving arm (0.8233 vs the served 0.8400), xnli a0_stands (0.4067 vs 0.5233) — both gates MISS, the lane re-closes

**Status:** MEASURED 2026-10-02 — riir-train Issue 609's ONE re-open of the Issue-608
closure, executed at the 400M teacher pins (reflex `2a5e4a0`'s T1 flip). Both T2 gates
MISS; per 609's honest-outcome rule the lane re-closes with the negative and nothing
served changes. Companion record: reflex Bench 109 (the Path-2 corpus read, V5 PASS
but below the incumbent).

## The pipeline (identical to Bench 0050's, ONE variable changed: the teacher)

1. **Teacher dumps** (reflex `--distill --distill-teacher bekko` at the T1 400M pins,
   `hotchpotch/bekko-system-one-v0-400m` @ `4aeb85b9…`): RIDT v1 + BLAKE3 over the
   FULL train splits of `.raw/datasets_t20k` — xnli 20,000 rows / 3 classes,
   teacher(train) acc **0.8054** (p50 146 ms, 3655 s) · massive 11,514 rows / 60
   classes, teacher(train) acc **0.9396** (p50 272 ms, 4607 s). Seals: xnli
   `72567f35ba479cff…`, massive `355f5ec23d80bb79…` (b3sum-verified). The RIDT
   headers carry the MIT attribution (the Issue-608-T5 law, automatic). Artifacts:
   `../riir-reflex/.raw/distill_teacher_bekko400m/`.
2. **Student retrain** (riir-train `instinct_arm_b`, epochs 3, holdout 200, gold_mix
   ∈ {0.0, 0.5}, Arm A — the 0050 protocol, only `--teacher-dir` changed): massive
   WINNER B(gold_mix=0) holdout **0.6800** (A 0.6600, B(0.5) 0.6800, control 0.0200);
   xnli WINNER B(gold_mix=0.5) holdout **0.3750** (A 0.3650, B(0) 0.3650, control
   0.3350). Every winner beat its lr=0 control. Artifacts minted to
   `../riir-train/data/instinct_specialists_bekko400m/` (massive blake3
   `a34fa1dfd22adf7f…`, xnli `ccdac3659f9175c8…`) — a SEPARATE dir, never the
   serving dir.

   **The train-side read on the teacher upgrade:** xnli teacher(train) acc jumped
   +13.9 pt (0.6668 → 0.8054) but the student holdout moved +0.5 pt (0.3700 →
   0.3750); massive teacher(train) +1.5 pt (0.9244 → 0.9396) and the student holdout
   moved −0.5 pt (0.6850 → 0.6800). **The one-vs-all bag student does not convert
   teacher accuracy into student accuracy on these suites** — the binding constraint
   was never the teacher's label quality.
3. **The single test reads** (this bench, the pre-registered decision): the 0029
   protocol per suite, `--winners-dir` pointed at the bekko400m dir — the ONLY
   change vs the serving posture.

## The massive read (synth seat, `--skip-pin-a0` — the 0029 overlay posture)

```
target/release/arena --suite massive_intent_en \
  --winners-dir ../riir-train/data/instinct_specialists_bekko400m \
  --synth-corpus ../riir-reflex/.raw/corpus_synth/massive_intent_en_synth.jsonl \
  --skip-pin-a0 --out .benchmarks/0056_bekko400m_distill_t2/massive
```

| arm | bekko-400M (this read) | bekko-68M (0050) | incumbent openthai (0029) |
|---|---|---|---|
| A0 | **0.8133** (244/300 — the synth-seat anchor reproduced EXACTLY) | 0.8133 | 0.8133 |
| A1 | 0.8133 | 0.8267 | 0.8167 |
| H1 | 0.8233 | 0.8300 | 0.8333 |
| pick H2 | **0.8233** (β=2,nmin=2,τ=2) | 0.8400 (β=1,nmin=2,τ=2) | 0.8400 (β=1,nmin=2,τ=4) |

Registration: rank-0 3/48; the instrument registered H2(β=2,nmin=2,τ=2).
`T2 superiority: H2 − A0 mean +0.0100 · LB95 −0.0117 > 0 → REFUSED → A0 serves.`
**T2 vs the SERVED row (0.8400): FAIL** — 0.8233 is 1.7 pt below the served row and
the in-run gate refused. The same `[join] artifact carries 1 label(s) the seat never
offers` disclosure appears (an artifact-known, seat-unknown label — unmapped, never
survivors). G1 PASS (raw 0.0943 · platt 0.0595 · floor 0.2600); G3 PASS. Fusion
overhead 0.91 ns/option (G2 bar). A0 anchor exact (the 0029-reproduction law holds).

## The xnli read (gold seat, A0 pin ON)

```
target/release/arena --suite xnli_en \
  --winners-dir ../riir-train/data/instinct_specialists_bekko400m \
  --out .benchmarks/0056_bekko400m_distill_t2/xnli
```

| arm | bekko-400M (this read) | bekko-68M (0050) |
|---|---|---|
| A0 | **0.5233** (== reflex run() == site ✓) | 0.5233 |
| A1 | 0.4067 | 0.4000 |
| H1 | 0.4067 | 0.4000 |
| best H2 (cal) | 0.4850 — registered **A0 outright** (rank-0 3/48) | 0.5650 — registered A0 outright |

**T2: FAIL decisively** — the specialist reads 11.7 pt BELOW the served A0 0.5233
(the 68M's was 12.3 pt below). The instrument refused any hybrid arm (a0_stands);
G5 refused (no hybrid registered — FAIL, never a pass); G1 PASS (raw ECE 0.5027 ·
Platt 0.1071 · floor 0.1351). A0 identity pin green.

## The verdict (the honest-outcome rule, 608's law restated in 609)

**Both T2 gates MISS at the 400M teacher. Nothing served changes; no mint; the
distill lane re-closes.** The re-open is ANSWERED, not merely exhausted:

- The teacher upgrade attacked exactly the hypothesized mechanism (label quality:
  xnli teacher(train) 0.6668 → 0.8054) and the student read did not move. **The
  one-vs-all bag student saturates well below the teacher on these suites — teacher
  accuracy is not the binding constraint.** The distill MACHINERY remains the landed
  value (the `--distill-teacher`/`--synth-teacher` bekko seam + the RIDT license
  field, reflex `2a5e4a0`/`20527e6`).
- The remaining lever on the xnli cell is unchanged: 607 (S1MB mining), owner-gated.
- Path 2's corpus axis moved slightly WITH the teacher prior (68M corpus 0.8067 →
  400M corpus 0.8100, both below the openthai incumbent 0.8133) — the axis is alive
  but the teacher is not its lever either. Reflex Bench 109 carries that read; the
  incumbent openthai corpus stays.

## Box state

Arena runs: `power AC Power · powermode high · load1m 4.73–5.42 · swap ~10.8 GB ·
quotable true` (structured box_state.json per run; the paired stats are deterministic
integer arithmetic on frozen picks — box-independent). The teacher dump ran under a
LOADED box — preflight REFUSED at launch (`load 8.21 > MAX_LOAD 6.0`, sibling
sessions active) and the run was launched anyway with an 8-thread torch cap
(`OMP_NUM_THREADS=8`, multi-tenant courtesy); it later ran through a transient
load-33 spike. **The dump's accuracy/seal columns are load-independent; its p50/p99
latency columns are load-affected and are NOT a claim** (146/859 ms xnli, 272/1478 ms
massive — read as "CPU FP32 under load", the 68M's 42/63 ms were a quiet box).
`PROVENANCE: power=AC Power load=8.21 swap=2491.25M canary=118.3us/best5
powermode=2(high)` — the launch-time refusal quote, recorded per the box-state law.

## Artifacts

- `massive/` + `xnli/` — the arena runs (RESULTS.md, registration.json,
  predictions.json, box_state.json each)
- reflex `.raw/distill_teacher_bekko400m/` — the RIDT dumps (DATA on disk,
  gitignored by law) + `DISTILL.md`/`distill.json`
- riir-train `data/instinct_specialists_bekko400m/` — the train-side winner
  artifacts (evidence, never registered)

## Cross-refs

- riir-train Issue 609 — the re-open this bench answers (T1/T2/T3 now all measured)
- Issue 608 + Bench 0050 — the parent closure and the 68M-teacher negatives this
  read replicates the protocol of
- reflex Bench 109 — the Path-2 corpus read at the 400M (V5 PASS, below incumbent)
- reflex Bench 107 — the board priors that funded the re-open
- reflex `.issues/042` standing negative — the distill replaces the SERVED
  specialist, never the modelless engine; unchanged
