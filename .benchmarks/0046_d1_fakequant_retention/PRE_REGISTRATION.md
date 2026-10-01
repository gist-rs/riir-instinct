# Bench 0046 — issue 018 Lane D1: the Q8 fake-quant retention probe — PRE-REGISTRATION

**Status:** PRE-REGISTERED — margins + rules declared BEFORE any measurement
run. This file is committed before the instrument's first read; the
measurement commits cite it. Any change after the first read is recorded as
an amendment with its own commit, never an edit of the declared numbers.

Plan of record: `riir-instinct/.issues/018_rethink_encoder_lean_goat.md`
§Lane D (D1 — "the retention probe"); branch consequences §Order of
execution (D1 pass → D2 → C; D1 fail → C on F16 → E).

## The instrument (declared)

- **Fake-quant Q8_0** (riir-infer `riir-infer-laya/src/laya/riir/fake_quant.rs`,
  this session): every checkpoint tensor with `shape.ndim() >= 2` is
  quantize-then-dequantized in memory — per 32-weight block, scale
  `d = f16(amax/127)` (the f16-rounded scale the real format stores),
  `q = roundf(w/d)` clamped [-127,127], dequant `w' = d·q` — and the F16
  forward runs BYTE-IDENTICAL code over the dequantized weights. The probe
  measures WEIGHT-quantization error only; a real kernel's accumulation
  order is D2's per-device determinism gate's subject, not this probe's.
- **1D tensors are NOT quantized** (norm weights/biases — the house GGUF
  norm law; <1% of checkpoint bytes). The run's `fake_quant_report`
  names every skipped tensor; the posture is disclosed in every record.
- **Posture:** M3 Metal (`LAYA_DEVICE=metal`, `--features arena-laya-metal`),
  the same posture every seated encoder cell was read at. The quant-vs-F16
  pairing is therefore device-matched by construction.
- **Determinism:** both postures are deterministic runs (bit-identical
  encode at a posture); the fresh F16 run is additionally witnessed
  against the frozen cell record below.

## The cells (the heads that exist — the issue's stated coverage limit)

| suite | head artifact (BLAKE3-sealed) | head kind | ckpt | datasets pool | n (q) |
|---|---|---|---|---|---|
| sst5 | `../riir-train/.raw/t599/t6_s0.bin` | v1 | english | t20k | 600 |
| xnli_en | `../riir-train/.raw/t599/xnli_en_encoder_v1.bin` | v1 | english | t20k | 300 |
| ag_news | `../riir-train/.raw/t599/ag_news_encoder_v1.bin` | v1 | english | t20k | 400 |
| massive_intent_en | `../riir-train/.raw/t599/massive_encoder_v1_probe.bin` | v1 | english | t20k | 300 |
| banking77 | `../riir-train/.raw/t608/banking77_encoder_v1.bin` | v1 | english | t20k | 500 |
| prompt_injections | `../riir-train/.raw/t608/prompt_encoder_v1.bin` | v1 | english | t20k | 116 |
| typed_decisions | `../riir-train/.raw/t608/typed_encoder_v2.bin` | v2 per-option | **typed** | `datasets_typed_full` | 2000 |

NOT covered (the limit the issue states): emotion (no head earned — the
disclosure state), the thai suites (multilingual reference reads, no heads;
the multilingual checkpoint is DEAD BY LAW per riir-train 603), and the
typed-over-english pairing (no such head exists — C trains it, on the
encoding this probe's verdict selects).

## The reads

Per suite, TWO runs of the identical arena command (same `--suite`,
`--encoder-art`, `--encoder-ckpt`, datasets, `--skip-pin-a0`, out under
`runs/<suite>_{f16,q8}/`):

1. **fresh F16** (no `--fake-quant`) — the comparator, AND the identity
   witness: its per-row `picks` must EQUAL the frozen cell record's (where
   the frozen record carries rows — every suite except sst5, whose 029
   record predates the per-row freeze and witnesses on accuracy alone).
   A witness break invalidates the suite's pairing (seat/box drift) and is
   reported, never absorbed.
2. **fresh fake-quant** (`--fake-quant`) — the probe read.

Pairing is per-row index-aligned (same seat builder, deterministic order);
the gate arithmetic is `stats::PairedDiff`'s exact form mirrored: z = 1.959963984540054,
`se = sqrt(Σd² − (Σd)²/n) / (n−1) / sqrt(n)` over the per-row differences.

## The gates (declared BEFORE measuring)

### Accuracy non-inferiority (the issue's table, instantiated)

| Requirement | Declared value |
|---|---|
| Margin δ | **0.02** for every suite with n ≥ 300 (sst5, xnli, ag_news, massive, banking77, typed); **0.04** for prompt_injections (n = 116 — the wide-n cell's margin, declared wide BECAUSE its n cannot resolve 0.02) |
| PASS | paired LB95 of (quant − F16) per-row accuracy diff **> −δ** |
| FAIL | paired UB95 **< −δ** |
| UNDECIDED | otherwise — never folded into PASS (the issue's thin-family law) |
| LOW-POWER flag | `z·se > δ` (the CI can never decide; reported, the rule still applies) |

δ = 0.02 is two accuracy points on a scale where Q8 weight noise is
expected sub-point; it is the smallest round margin the n ≥ 300 cells can
actually resolve under a realistic flip rate, and it is FLAT (no
per-suite tuning). The lane verdict consumes the suite verdicts verbatim.

### Calibration (the issue's gate, instantiated on the record's confidence plane)

- **Quantity:** Brier, PICKED-CLASS form: `(p_pick − hit)²` per row, where
  `p_pick` is the head's recorded confidence for the picked option (the
  per-class/per-option sigmoid) and `hit` ∈ {0,1}. The full multiclass
  form is NOT reconstructible from the frozen records (they carry the
  picked option's confidence only); both postures share the same
  definition, so the paired Δ is well-posed. Declared here, not chosen
  after the fact.
- **Gate:** pass iff paired **UB95 of ΔBrier (quant − F16) < margin**;
  breach = FAIL-candidate (calibration), thin families UNDECIDED.
- **Margin:** **0.01** absolute for n ≥ 300 suites; **0.03** for prompt.
- **ECE:** REPORTED (10 equal-width bins over max-prob, left-open (0,1] —
  the reflex protocol convention), NOT gated: ECE on n ≤ 500 is bin-noisy,
  and the issue names the paired ΔBrier upper bound as the gate.
- **Δ-retention breach response (pre-registered, the issue's branch (a)):**
  a breached suite may run the ONE pre-registered per-family temperature
  refit — fit scalar T on the suite's CAL split under the quant posture
  (argmax-preserving; applied as p' = sigmoid-class scaling of the head's
  logits — v1: logit(p)/(T) re-sigmoided; v2 per-option identically), then
  recompute the gate. Clears the margins ⇒ **PASS-with-refit** (recorded);
  does not clear ⇒ that suite FAILS the lane. One refit per breached
  family — no sweep.
- **Absolute-floor tripwire (the issue's branch (b)):** if ANY head reads
  BELOW the conformal-naive floor on the quant posture (or on the fresh
  F16 witness — either way it is the standing 562/576 class), it is
  recorded and the issue's own tripwire applies (narrow riir-train
  calibration-aware-loss filing) — it does NOT change D1's retention
  verdict by itself.

### Retention (reported, NOT gated)

Per-suite pick retention = Σ[pick_q == pick_f16]/n, plus the per-class
flip table (flips by gold class, and top (from→to) pick pairs) — the
aggregate-flat-while-families-flip detector the issue demands (the Orthrus
lesson: per-family is the only read that counts).

### The lane verdict (the branch decision)

**D1 PASSES iff NO suite FAILS** (accuracy OR calibration after the
pre-registered refit path). Any FAIL ⇒ the D1-fail branch (C on F16, then
E distill). All-PASS/UNDECIDED ⇒ the pass branch (D2 before C), with every
UNDECIDED suite named in the record. A witness break (fresh F16 ≠ frozen)
invalidates that suite's pairing and must be resolved (re-read) before any
verdict is claimed — a pairing over drifted rows is not a measurement.

## Runs + box state

- The 14 runs are sequential (GPU-exclusivity law — one compute consumer);
  each run's record carries the arena's own box-state line; the FIRST run
  of the session quotes `bench_preflight.sh` PROVENANCE beside it.
- Accuracy-only reads (no latency gate is claimed at this posture — the
  arm's latency rows ride the runs' own box disclosure).

## Declared BEFORE the first read

Commit of record: the commit adding this file (the measurement commits
reference it by hash). Nothing below this line is measured yet.
