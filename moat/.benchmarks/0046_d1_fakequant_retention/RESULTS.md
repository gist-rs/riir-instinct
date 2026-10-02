# Bench 0046 — issue 018 Lane D1: the Q8 fake-quant retention probe — MEASURED, LANE PASSES

**Status:** MEASURED — 14 reads (7 suites × {fresh F16, fake-quant Q8_0}),
paired per pre-registration (`PRE_REGISTRATION.md`, committed `33d02ed`
BEFORE any read). **LANE D1 VERDICT: PASS** — no suite FAILED; 5 PASS,
2 UNDECIDED (named below). Per the pre-registered lane rule and the issue's
branch table: **D1 pass ⇒ D2 (full Q8 adoption in riir-infer) before C.**

**Preflight (the Issue-021 law):** `PROVENANCE: power=AC Power load=2.85
swap=2643.25M canary=skipped powermode=2(high)` — the first read's posture;
every run's own box line rides `runs_log.txt` (load 2.85–5.5 across the
session, AC throughout, determinism witnessed per suite anyway).

## The instrument

`riir-infer` `294999e` (fake-quant module + `WeightPosture` agent seam) +
`riir-instinct` `2a61105` (`--fake-quant`). Q8_0 block-32 (f16-rounded
scale = the storage grid, roundf convention, [−127,127]) over every
ndim≥2 tensor; 1D norms/biases skipped (80 named in each report — the
house GGUF norm law). Forward code byte-identical both postures; only the
loaded weights differ. Quantized surface (both checkpoints, same shape):
**126 tensors · 421,205,504 elements · 842.4 MB F16 → ~424 MB Q8** — the
issue's D2 estimate (848→424 MB) confirmed by construction. Measured
weight error: max |Δ| 0.0163 · mean |Δ| 3.0e-4.

## The reads (fresh F16 first — the identity witness — then fake-quant)

| suite | n | F16 acc (frozen == fresh) | Q8 acc | Δacc | retention | flips |
|---|---|---|---|---|---|---|
| sst5 | 600 | 0.5267 == 0.5267 ✓ᵃ | 0.5333 | +0.0067 | 0.9850 | 9 |
| xnli_en | 300 | 0.8600 == 0.8600 ✓ | 0.8600 | +0.0000 | 1.0000 | 0 |
| ag_news | 400 | 0.9475 == 0.9475 ✓ | 0.9500 | +0.0025 | 0.9975 | 1 |
| massive_intent_en | 300 | 0.6567 == 0.6567 ✓ | 0.6467 | −0.0100 | 0.9767 | 7 |
| banking77 | 500 | 0.4420 == 0.4420 ✓ | 0.4380 | −0.0040 | 0.9560 | 22 |
| prompt_injections | 116 | 0.8017 == 0.8017 ✓ | 0.8017 | +0.0000 | 1.0000 | 0 |
| typed_decisions | 2000 | 0.7550 == 0.7550 ✓ | 0.7570 | +0.0020 | 0.9905 | 19 |

ᵃ accuracy-only witness (the 029 record predates the per-row freeze;
every other suite witnessed on PER-ROW PICKS — all six held exactly, the
determinism law at every suite).

## The gates (pre-registered arithmetic, `pair_and_gate.py`)

| suite | δ | LB95(Δacc) | acc verdict | cal margin | UB95(ΔBrier) | cal verdict | SUITE |
|---|---|---|---|---|---|---|---|
| sst5 | 0.02 | −0.0013 | PASS | 0.01 | +0.0022 | PASS | **PASS** |
| xnli_en | 0.02 | +0.0000 | PASS | 0.01 | +0.0004 | PASS | **PASS** |
| ag_news | 0.02 | −0.0024 | PASS | 0.01 | +0.0065 | PASS | **PASS** |
| massive_intent_en | 0.02 | −0.0213 | UNDECIDED | 0.01 | +0.0018 | PASS | **UNDECIDED** |
| banking77 | 0.02 | −0.0151 | PASS | 0.01 | +0.0190 | UNDECIDED | **UNDECIDED** |
| prompt_injections | 0.04 | +0.0000 | PASS | 0.03 | +0.0063 | PASS | **PASS** |
| typed_decisions | 0.02 | −0.0022 | PASS | 0.01 | +0.0002 | PASS | **PASS** |

- **The two UNDECIDED rows are named, never folded** (the thin-family law
  cuts both ways — they are not PASSes and not FAILs):
  - **massive**: mean Δacc −1.0pt with LB95 −0.0213, δ 0.02 — the CI
    straddles the margin by 0.0013; mean retention 97.7% (7 flips / 300).
    A one-row difference flips this cell's verdict — it is the probe's
    honest resolution limit at n=300, not a retention concern.
  - **banking77**: calibration UNDECIDED — UB95(ΔBrier) 0.0190 vs margin
    0.01, on a head whose F16 calibration is ALREADY gross (Brier 0.488,
    ECE 0.494 at F16; the cell itself is the screened-DEAD class,
    Bench 0043). The Δ noise rides a head that barely calibrates at F16;
    the pre-registered temperature refit is the BREACH path and this row
    did not breach (LB95 < margin < UB95), so no refit ran. If the lane
    re-opens for banking77's own retrain (C/D2 territory), the refit
    question re-runs on the new head.
- **ECE (reported, not gated)**: ΔECE ≤ +0.0051 on every suite
  (banking77 +0.0051, massive +0.0050, prompt +0.0031, ag_news +0.0006,
  typed −0.0003, sst5 −0.0008, xnli +0.0011) — quant did not move
  confidence materially anywhere.
- **Aggregate-flat-while-families-flip check (the Orthrus guard)**: the
  aggregate reads flat (+0.09pt mean over 4216 rows) and the per-family
  table above carries the full spread — no family hides a sign flip
  behind it (max −1.0pt, min +0.67pt). The per-class flip table rides
  `d1_verdict.json`; flips concentrate nowhere (max 22/500 banking77,
  consistent with its F16 borderline head).

## Verdict + branch

**D1 PASSES** (no suite FAILED). **Branch ⇒ D2**: real Q8 adoption in
riir-infer (the house dequant machinery — the probe's block format IS the
format D2 stores), THEN C (the typed head retrain) on the adopted Q8
encode — C never trains on F16 in this branch (the double-spend rule).
D2's own gates (per the issue): the real kernels' per-device determinism
re-seat, the adoption re-seat of every cell (frozen reads invalidated by
the numerics change — by design), the forced C retrain.

**Absolute-floor scope note (declared)**: the conformal-naive floor
instrument lives in the cells' G1 faces (the hybrid arms' confidence
plane), not in this probe's picked-class Brier plane; the floor tripwire
for any NEW gradient-trained head applies when C trains (its pre-declared
loss option is C's landing pad). For the EXISTING heads, quant moved
nothing beyond the Δ margins above — no new floor breach was created, and
banking77's F16 uncalibrated state (ECE 0.494) is the cell's own recorded
disclosure, not a quant effect.

## Reproduction

`runs_all.sh` (the 14 commands verbatim — the seated cells' commands ±
`--fake-quant`), `runs_log.txt` (full stdout incl. every box line),
`d1_verdict.json` (the gate arithmetic's full output), `pair_and_gate.py`
(the exact pre-registered arithmetic, stdlib only). Fresh F16 reads are
bit-identical to the frozen cells — the whole probe re-runs to the same
verdict.
