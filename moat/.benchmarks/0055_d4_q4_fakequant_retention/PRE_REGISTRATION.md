# Bench 0055 — issue 018 Lane D4: the Q4 fake-quant retention probe — PRE-REGISTRATION

**Status:** PRE-REGISTERED — margins + rules declared BEFORE any measurement
run. This file is committed before the instrument's first read; the
measurement commits cite it. Any change after the first read is recorded as
an amendment with its own commit, never an edit of the declared numbers.

Plan of record: `riir-instinct/.issues/018_rethink_encoder_lean_goat.md`
§Lane D ("Q4 retention is D1-priced separately before `LAYA_WEIGHTS_VARIANT=q4`
serves anything"; Update 6 landed the Q4 FORMAT tier and named this probe
the gate). The substrate coined the name: `fake_quant_q4_map` is documented
as "the future D4 retention probe's instrument" — D4 = Lane D step 4, the
Q4 tier's retention walk. Naming follows the substrate.

Owner trigger (2026-10-02): menu item B approved — "option (iii) re-price
+ Q4/PQ2 retention probe". This file is the probe half; the re-price is
recorded in the issue the same session. PQ2 is NOT priced here: no PQ2
grid exists (the Q4 seam's "next format row" is unlanded), so a PQ2 probe
cannot precede its format — the issue already records that ordering.

## The instrument (declared)

- **Fake-quant Q4_0** (riir-infer `WeightPosture::FakeQuantQ4` →
  `fake_quant_q4_map`, this session's posture arm over the grid the Q4
  converter already proved): every checkpoint tensor with
  `shape.ndim() >= 2` is quantize-then-dequantized in memory — per
  32-weight block, scale `d = f16(amax/7)` (the f16-rounded scale the
  real format stores), `q = roundf(w/d)` clamped [-7,+7] (the signed
  grid), dequant `w' = d·q` — and the F16 forward runs BYTE-IDENTICAL
  code over the dequantized weights. The probe measures
  WEIGHT-quantization error only; a real kernel's accumulation order is
  the D2 per-device determinism gate's subject, not this probe's.
- **1D tensors are NOT quantized** (norm weights/biases — the house GGUF
  norm law; the same 80-tensor skip set D1 named). The run's
  `fake_quant_report` names every skipped tensor; the posture is
  disclosed in every record.
- **The transform's values are byte-identical to the Q4_0 artifact's
  decode** (the converter's read-back proof, landed with the Q4 format
  tier): this probe's verdict IS the artifact tier's retention.
- **Posture:** M3 Metal (`LAYA_DEVICE=metal`, `--features
  arena-laya-metal`), the same posture every seated encoder cell and the
  D1 Q8 probe were read at. The quant-vs-F16 pairing is therefore
  device-matched by construction, and the Q4 verdict is directly
  comparable with the Q8 one (Bench 0046).
- **Determinism:** both postures are deterministic runs (bit-identical
  encode at a posture); the fresh F16 run is additionally witnessed
  against the frozen cell records.
- **Expected surface (declared, from D1's map pass — same 126 tensors):**
  421,205,504 elements · 842.4 MB F16 → **~237 MB at Q4's 0.5625 B/elt**
  (the format tier's number). The run's own report is the record.

## The cells (the heads that exist — the same coverage limit D1 declared)

| suite | head artifact (BLAKE3-sealed) | head kind | ckpt | datasets pool | n (q) |
|---|---|---|---|---|---|
| sst5 | `../riir-train/.raw/t599/t6_s0.bin` | v1 | english | t20k | 600 |
| xnli_en | `../riir-train/.raw/t599/xnli_en_encoder_v1.bin` | v1 | english | t20k | 300 |
| ag_news | `../riir-train/.raw/t599/ag_news_encoder_v1.bin` | v1 | english | t20k | 400 |
| massive_intent_en | `../riir-train/.raw/t599/massive_encoder_v1_probe.bin` | v1 | english | t20k | 300 |
| banking77 | `../riir-train/.raw/t608/banking77_encoder_v1.bin` | v1 | english | t20k | 500 |
| prompt_injections | `../riir-train/.raw/t608/prompt_encoder_v1.bin` | v1 | english | t20k | 116 |
| typed_decisions | `../riir-train/.raw/t608/typed_encoder_v2.bin` | v2 per-option | **typed** | `datasets_typed_full` | 2000 |

NOT covered (the standing limit): emotion (no head earned), the thai
suites (multilingual checkpoint DEAD BY LAW, riir-train 603), and the
typed-over-english pairing (no such head exists — C's negative, Bench
0048, stands regardless of format).

## The reads

Per suite, TWO runs of the identical arena command (same `--suite`,
`--encoder-art`, `--encoder-ckpt`, datasets, `--skip-pin-a0`, out under
`runs/<suite>_{f16,q4}/`):

1. **fresh F16** (no fake-quant flag) — the comparator, AND the identity
   witness: its per-row `picks` must EQUAL the frozen cell record's
   (where the frozen record carries rows — every suite except sst5,
   whose 029 record predates the per-row freeze and witnesses on
   accuracy alone). A witness break invalidates the suite's pairing
   (seat/box drift) and is reported, never absorbed. D1's fresh F16
   reads reproduced every frozen cell exactly; the F16 path has been
   byte-untouched since (Update 6's no-change proof), so the same holds
   is the expectation — verified per suite, never assumed.
2. **fresh fake-quant Q4** (`--fake-quant-q4`) — the probe read.

Pairing is per-row index-aligned (same seat builder, deterministic order);
the gate arithmetic is `stats::PairedDiff`'s exact form mirrored: z = 1.959963984540054,
`se = sqrt(Σd² − (Σd)²/n) / (n−1) / sqrt(n)` over the per-row differences.

## The gates (declared BEFORE measuring)

### Margins — IDENTICAL to D1's, deliberately

δ and the calibration margins are the serving bar, not a noise estimate:
they say how much per-family degradation the Q4 tier would be allowed to
serve with. Reading Q4 at different margins than Q8 would make the two
format verdicts incomparable (a "pass" at a looser bar is not a pass at
the product's bar). Declared flat, no per-suite tuning, no re-derivation
now that Q4's coarser grid is known:

| Requirement | Declared value |
|---|---|
| Margin δ | **0.02** for every suite with n ≥ 300 (sst5, xnli, ag_news, massive, banking77, typed); **0.04** for prompt_injections (n = 116 — the wide-n cell's margin, declared wide BECAUSE its n cannot resolve 0.02) |
| PASS | paired LB95 of (quant − F16) per-row accuracy diff **> −δ** |
| FAIL | paired UB95 **< −δ** |
| UNDECIDED | otherwise — never folded into PASS (the thin-family law) |
| LOW-POWER flag | `z·se > δ` (the CI can never decide; reported, the rule still applies) |

**Declared prior (never a gate):** Q4's grid is ~16× coarser than Q8's
(scale `amax/7` vs `amax/127`; max weight error bound ±d/2). If any suite
fails, the likeliest candidates on priors are the fine-grained heads —
banking77 (77-way, F16-borderline) and massive (the F16 cell that already
read UNDECIDED at Q8). This is written before the read so the record can
show which priors survived, not to pre-judge any cell.

### Calibration (same instrument as D1, instantiated on this probe's plane)

- **Quantity:** Brier, PICKED-CLASS form: `(p_pick − hit)²` per row, where
  `p_pick` is the head's recorded confidence for the picked option and
  `hit` ∈ {0,1}. Both postures share the same definition, so the paired Δ
  is well-posed.
- **Gate:** pass iff paired **UB95 of ΔBrier (quant − F16) < margin**;
  breach = FAIL-candidate (calibration), thin families UNDECIDED.
- **Margin:** **0.01** absolute for n ≥ 300 suites; **0.03** for prompt.
- **ECE:** REPORTED (10 equal-width bins over max-prob, left-open (0,1] —
  the reflex protocol convention), NOT gated.
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
  recorded and the issue's own tripwire applies — it does NOT change this
  probe's retention verdict by itself.

### Retention (reported, NOT gated)

Per-suite pick retention = Σ[pick_q4 == pick_f16]/n, plus the per-class
flip table (flips by gold class, and the flip concentration) — the
aggregate-flat-while-families-flip detector (the Orthros lesson: per-family
is the only read that counts). The Q8↔Q4 retention pairs are reported
side by side in the results.

### The lane verdict (the branch decision)

**D4 PASSES iff NO suite FAILS** (accuracy OR calibration after the
pre-registered refit path). Any FAIL ⇒ **the Q4 tier stays
measurement-only** — the format seam remains (it is landed), but
`LAYA_WEIGHTS_VARIANT=q4` may not serve anything (the issue's own law).
All-PASS/UNDECIDED ⇒ **the Q4 artifact tier is eligible to serve** (its
decode values are byte-identical to this probe's transform — the
converter's read-back proof — so no additional retention read exists to
run); the D2 law's per-device determinism re-seat still applies to the
real q4 kernels at any adoption, exactly as it did for Q8. This verdict
does NOT reopen or amend the Q8 adoption (Bench 0046's) — Q8 is adopted
and stands.

A witness break (fresh F16 ≠ frozen) invalidates that suite's pairing and
must be resolved (re-read) before any verdict is claimed — a pairing over
drifted rows is not a measurement.

## Runs + box state

- The 14 reads are sequential (GPU-exclusivity law — one compute consumer);
  each read's record carries the arena's own box-state line; the FIRST read
  of the session quotes `bench_preflight.sh` PROVENANCE beside it.
- Accuracy-only reads (no latency gate is claimed at this posture — the
  arm's latency rows ride the runs' own box disclosure).

## Declared BEFORE the first read

Commit of record: the commit adding this file (the measurement commits
reference it by hash). Nothing below this line is measured yet.
