# Bench 004 — the re-baseline: A0 at the CURRENT published Reflex posture + the strict-superiority product gate

**Status:** MEASURED — Issue 008 T1 + T2 executed 2026-09-27 on the M3 (AC,
powermode 2, sibling agent sessions active — the GUI-exempt class; the arena's
own box-state line carries the reading). One frozen test read
(`predictions.json`); the full runner log is `run.log`.

## What changed and why

Reflex is free; Instinct is the paid lane, so it must strictly beat the
Reflex it actually competes with — not the Reflex of last week. Two defects
made the old record unquotable (Issue 008 root causes 1+2):

1. **Stale baseline.** Bench 002 registered every arm against the Bench-052
   protocol's A0; Reflex has since armed its cal-selected heads (Bench 057:
   emotion's NBSVM ridge@8 → 0.8850). The arena now seats Reflex at the
   CURRENT published posture: `head_select + nb_select + ridge_select`,
   registry caps, genome off (reflex's published rows predate that lane).
   The ridge ladder arms only where the selection slice clears the house
   bar — emotion selects 8, every other arena suite selects 0 (reflex
   measured the full-workspace delta at 0.0000) — so the published
   per-suite posture is reproduced by ONE knob set, not per-suite
   hard-coding.
2. **The tie sold as an arm.** The T2 product gate is now the registration:
   the registered arm must be STRICTLY above the current Reflex row —
   paired (pick − A0) 95% lower bound > 0 on the frozen test read — else
   the registration refuses and A0 serves. `stats::PairedDiff` gained the
   `lb95` bound (known-answer-tested both directions).

## The comparability proof — 6/6 + site ✓

The re-pinned drift pin asserts, for EVERY arena suite: arena A0 == reflex
`run()` hard accuracy at the same knobs — and, when the reflex-site
checkout stands beside the workspace, reflex's rows == the PUBLISHED
`bench.json` numbers:

```
A0 pin · ag_news:            arena == reflex run() == 0.882500 · site ✓
A0 pin · emotion:            arena == reflex run() == 0.885000 · site ✓
A0 pin · sst5:               arena == reflex run() == 0.396667 · site ✓
A0 pin · xnli_en:            arena == reflex run() == 0.523333 · site ✓
A0 pin · massive_intent_en:  arena == reflex run() == 0.780000 · site ✓
A0 pin · banking77:          arena == reflex run() == 0.826000 · site ✓
```

The seat posture per suite (the published derivation, disclosed):
ag_news cap 64 / head 0 / nb 4 / ridge 0; emotion cap 64 / head 0 / nb 4 /
**ridge 8**; sst5 cap 64 / head 0 / nb 16; xnli cap 64 / head 1 / nb 16
(pair view); massive cap 48 / head 0 / nb 4; banking77 cap 40 / head 1 /
nb 1. Banking77's ridge ladder ran to scale 8 and declined (sel-slice
0.8250 < base + bar) — the honest arm of the selection protocol, ~65 s of
the suite's 74 s.

## The verdicts (the frozen test read)

| suite | instrument pick | pick acc | A0 acc | paired mean | LB95 | gate | serves |
|---|---|---|---|---|---|---|---|
| massive_intent_en | H2(β=1,nmin=2,τ=8) | 0.8267 | 0.7800 | +0.0467 | **+0.0124** | **PASS** | **H2(1,2,8)** |
| ag_news | H2(β=0.25,nmin=2,τ=2) | 0.8975 | 0.8825 | +0.0150 | −0.0100 | REFUSED | A0 |
| sst5 | A1 | 0.4217 | 0.3967 | +0.0250 | −0.0129 | REFUSED | A0 |
| banking77 | H1 | 0.8060 | 0.8260 | −0.0200 | −0.0466 | REFUSED | A0 |
| emotion | A0 (the instrument's own pick) | 0.8850 | 0.8850 | — | — | — | A0 |
| xnli_en | A0 | 0.5233 | 0.5233 | — | — | — | A0 |

The product consequence, stated plainly: **Instinct sells exactly one
suite — massive_intent_en, +4.7 pt, certified strictly above Reflex at
95% confidence.** Every other suite serves A0 (== free Reflex) until a
specialist certifies. Emotion's specialist no longer even survives the
instrument: reflex's ridge@8 base (0.8850) outruns A1 (0.8550) — root
cause 1 measured end to end. The site's "Instinct vs Reflex" row law
(strictly ahead on every suite it sells) now reads ✓ on one certified
suite instead of ✗ on four unsellable ones. Growing the sold set is
Issue 008 T4/T5 (better specialists, coverage) — the gate is the
discipline that keeps the ✓ honest.

G3 (non-inferiority) stays a reported face judged on the instrument pick,
so a refused arm's regression disclosure survives the refusal
(banking77's G3 FAIL is visible in RESULTS.md beside its refusal).

## Serving side (the re-pin)

- `arsenal.toml`: ag_news/emotion/sst5/banking77/xnli rows → `A0` with the
  refusal numbers in the row comments; massive unchanged. Digest re-pinned
  in `tests/serve_gates.rs`.
- `tests/serve_gates.rs`: face-2 rows = the Bench-004 verdicts; the parity
  gate reads THIS record's `predictions.json`; the HTTP happy-path test
  asserts ag_news's A0 shape (full-arity probabilities, no specialist
  scores, escalated false).
- Boot-cost disclosure: every serve/seat boot now derives the published
  posture INCLUDING the ridge ladders — ≈ +5 s (emotion) and ≈ +60 s
  (banking77, 77 classes × 5 ladder rungs) on an eager six-lane boot.
  Deterministic derivation of the published posture, never a fork from
  the arena path. A frozen-derived-posture seam (skip the ladder at
  serve time) is the recorded follow-up if boot cost ever binds.

## Latency faces (box state in the log; sibling sessions active)

Fusion overhead holds the G2 bars: H1 fusion-only 9–126 ns/q, H2
0.98–3.08 ns/option, all suites. Modelless lane p50: ag_news 157 µs ·
emotion 122 µs · sst5 98 µs · xnli 95 µs · massive 117 µs · banking77
387 µs (the A0/seat solve — the µs-class tier the hybrid must pay only on
escalation).

## Addendum — the abstention contract (found by the parity gate)

The first serve_gates run after the re-baseline reded the frozen-picks
parity gate on ag_news case 0: the serve answered `None` (abstained)
where the record carried pick 2. Root cause: **two conventions that had
never collided**. Reflex's *hard* accuracy (the published convention,
and the arena's A0 record) counts FORCED picks — abstains are forced to
their argmax pick (`forced_rows`) — while the serve path honors the
fused gate's abstention as a first-class answer (`pick: None`). At
Bench 002 the parity gate's suites served H1/H2 (never abstain), so the
gap was invisible; ag_news now serves A0 and its gate abstains on
**196/400 test questions (49%)**.

Repair (this bench, same day): the arena's `ArmOut` records the
`abstained` flags (A0 = the seat's `qo.abstained`; H1 = the passthrough
half — reflex abstained AND the gate did not escalate; A1/H2 never),
`predictions.json` carries them per arm, and the parity gate asserts the
abstention contract FIRST (served abstention == recorded abstention,
per case) before comparing picks. The HTTP happy-path test now decides
on a frozen-record case the gate ANSWERS, so its served-pick shape
assertions are contract-true.

Record stability: the arena was re-run at reflex `7b5f0ba` (three
sibling commits after the first run) — every registration, gate
outcome, and the 6/6+site pin reproduced byte-identically; the sibling
commits added measurement-only arms (`nli_feature_ab`,
`cascade_worthiness_lcb` — both pinned off in the drift pin, the
flag-off = baseline contract).

Boot-cost disclosure: the raised serve readiness ceilings (120 s →
420 s) reflect the re-baselined boot cost — the ridge ladders are
derivation work every boot now pays (banking77 ≈ 65 s standalone;
more under test contention). A frozen-derived-posture seam (skip the
ladder at serve time, the selected values pinned as data) remains the
recorded follow-up if boot cost ever binds.
