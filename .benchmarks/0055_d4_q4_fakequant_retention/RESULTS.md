# Bench 0055 — issue 018 Lane D4: the Q4 fake-quant retention probe — MEASURED, LANE FAILS

**Status:** MEASURED — 14 reads (7 suites × {fresh F16, fake-quant Q4_0}),
paired per pre-registration (`PRE_REGISTRATION.md`, committed before any
read). **LANE D4 VERDICT: FAIL** — banking77 FAILED both gates decisively;
5 suites UNDECIDED; xnli_en the one clean PASS. Per the pre-registered lane
rule: **the Q4 tier stays MEASUREMENT-ONLY — `LAYA_WEIGHTS_VARIANT=q4` may
not serve anything.** The Q8 adoption (Bench 0046 PASS) is untouched.

**Preflight (the Issue-021 law):** the first read's posture; every run's own
box line rides `runs_log.txt` (AC throughout, sibling sessions active — the
same disclosed class as D1's session; these are accuracy reads, no latency
gate is claimed).

## The instrument

`WeightPosture::FakeQuantQ4` (riir-infer — the agent arm over
`fake_quant_q4_map`, the grid the Q4 converter already proved) + the arena's
`--fake-quant-q4` (mutually exclusive with `--fake-quant`). Q4_0 block-32
(f16-rounded scale = `amax/7`, roundf convention, signed grid [-7,+7]) over
every ndim≥2 tensor; 1D norms/biases skipped. Forward code byte-identical
both postures; only the loaded weights differ. Quantized surface (same 126
tensors as D1): **421,205,504 elements · 842.4 MB F16 → 236.9 MB at Q4**
(0.5625 B/elt — the format tier's declared number, confirmed by
construction). Measured weight error: **max |Δ| 0.2932 · mean |Δ| 5.5e-3**
(vs Q8's 0.0163 / 3.0e-4 — the ~18× coarser grid, as declared in the
prior).

## The identity witnesses (all held — the pairings are valid measurements)

Fresh F16 == frozen cell on every suite: per-row picks exact on the six
suites whose records carry rows (xnli, ag_news, massive, banking77, prompt,
typed) · sst5 accuracy-only (0.5267 == 0.5267, the 029 record predates the
per-row freeze) — the same witness pattern D1 read. The F16 path was
byte-untouched since Update 6's no-change proof; it held again.

## The reads + the gates (pre-registered arithmetic, `pair_and_gate.py`)

| suite | n | F16 acc | Q4 acc | Δacc | LB95 | acc verdict | retention | flips | Δacc vs Q8 | ret vs Q8 | SUITE |
|---|---|---|---|---|---|---|---|---|---|---|---|
| sst5 | 600 | 0.5267 | 0.5183 | −0.0083 | −0.0370 | UNDECIDED | 0.8233 | 106 | −0.0150 | −0.1617 | **UNDECIDED** |
| xnli_en | 300 | 0.8600 | 0.8733 | +0.0133 | −0.0144 | PASS | 0.9400 | 18 | +0.0133 | −0.0600 | **PASS** |
| ag_news | 400 | 0.9475 | 0.9450 | −0.0025 | −0.0135 | PASS | 0.9875 | 5 | −0.0050 | −0.0100 | **PASS** |
| massive_intent_en | 300 | 0.6567 | 0.6267 | −0.0300 | −0.0651 | UNDECIDED | 0.8433 | 47 | −0.0200 | −0.1334 | **UNDECIDED** |
| banking77 | 500 | 0.4420 | 0.3420 | **−0.1000** | **−0.1410** | **FAIL** | **0.5680** | **216** | −0.0960 | −0.3880 | **FAIL** |
| prompt_injections | 116 | 0.8017 | 0.7759 | −0.0259 | −0.0869 | UNDECIDED | 0.8879 | 13 | −0.0259 | −0.1121 | **UNDECIDED** |
| typed_decisions | 2000 | 0.7550 | 0.7460 | −0.0090 | −0.0203 | UNDECIDED | 0.9280 | 144 | −0.0110 | −0.0625 | **UNDECIDED** |

Calibration (picked-class Brier; margins 0.01 / 0.03 as pre-registered):

| suite | UB95(ΔBrier) | cal verdict | ECE F16 → Q4 |
|---|---|---|---|
| sst5 | +0.0124 | UNDECIDED | 0.0173 → 0.0219 |
| xnli_en | −0.0046 | PASS | 0.0937 → 0.0831 |
| ag_news | +0.0095 | PASS | 0.0302 → 0.0302 |
| massive_intent_en | +0.0204 | UNDECIDED | 0.0440 → 0.0570 |
| banking77 | **+0.1330** | **FAIL** | 0.4936 → 0.5540 |
| prompt_injections | +0.0458 | UNDECIDED | 0.0468 → 0.0667 |
| typed_decisions | +0.0011 | PASS | 0.0409 → 0.0419 |

**The lane verdict consumes the suite verdicts verbatim: banking77 FAILED
both gates ⇒ D4 FAILS.** No suite is folded, no margin re-read, no
post-hoc rescue.

## The failure anatomy (banking77)

- Accuracy: Δ −10.0pt with UB95 −0.1410 — the CI is ENTIRELY below the
  −0.02 margin; this is a resolved degradation, not a resolution limit.
- Retention 56.8% (216/500 picks moved) — the Q4 weight noise (max err
  0.29) scrambles a 77-way head that already sits at the F16 borderline
  (the 0043 screened-DEAD class; its ECE 0.494 at F16). Exactly the
  declared prior's likeliest-failure candidate.
- Calibration moved the same direction (ΔBrier UB95 +0.1330 ≥ margin 0.01).

**The refit path is MOOT, recorded not run:** the pre-registered per-family
temperature refit is ARGMAX-PRESERVING — it rescales confidence, never
moves picks — so it cannot touch an ACCURACY fail, and the suite verdict
is FAIL if either axis fails. Running it would be a decorative computation
wearing a gate's name; the lane verdict is decided by the accuracy leg
alone. (For the record: even the calibration leg alone breached.)

## The honest read

- **Q8's adoption stands untouched** — 0046 PASSed at retention 95.6–100%
  with every gate table green-or-named; nothing in this probe re-reads it.
- **Q4 is not a serving format for these heads.** One suite failed on
  evidence no margin choice could absorb; three more read UNDECIDED on
  accuracy and four on calibration — at a coarser grid the direction is
  consistently negative (mean Δacc −2.5pt across 4216 rows vs Q8's +0.09pt).
- The Q4 FORMAT seam stays landed (it is the D2 format tier's machinery and
  the converter's read-back proof is real); what fails is the RETENTION —
  the values the format would serve are, for this head class, not good
  enough. `LAYA_WEIGHTS_VARIANT=q4` remains measurement-only by the issue's
  own law ("Q4 retention is D1-priced separately before q4 serves
  anything").
- **PQ2 inherits the lesson, not a verdict:** no PQ2 grid exists, so no PQ2
  read is possible; when its format row lands, its probe is D4-shaped with
  THIS record as the prior — coarse grids fail on fine-grained heads first,
  and a format whose per-family walk cannot clear banking77 cannot serve
  the seated set wholesale.
- The low-power flags: 5/7 suites carry `z·se > δ` (the CI could never
  decide them at these n) — reported per the pre-registration; the rule
  stands (they are UNDECIDED, never PASSes), and banking77's FAIL is
  resolved, not low-power (UB95 −0.1410 is 7× past the margin).

## Verdict + branch

**D4 FAILS ⇒ the Q4 tier stays measurement-only** (the pre-registered
branch). No re-seat, no adoption, no serving change: the seated F16/q8
cells stand; the /#sizes row stands at the adopted q8 posture; reflex-site
untouched. The re-open bar is a HEAD-class change (a finer Q4 grid variant,
a per-head quantization mix, or a head retrained at Q4 — riir-train
territory), priced as its own lane if ever wanted; nothing in this record
requests it.

## Reproduction

`run_all.sh` (the 14 commands verbatim — the seated cells' commands ±
`--fake-quant-q4`, arena at `/tmp/q4probe-instinct`), `runs_log.txt` (full
stdout incl. every box line), `d4_verdict.json` (the gate arithmetic's full
output), `pair_and_gate.py` (the pre-registered arithmetic — D1's margins
verbatim, stdlib only). Fresh F16 reads are bit-identical to the frozen
cells — the whole probe re-runs to the same verdict.
