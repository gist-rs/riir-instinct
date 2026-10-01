# Bench 038 — the emotion confusion read: why ±0.0, and whether ANY lever can move the cell

**Status:** RECORD — a records-only negative with mechanism (no training spend, no
GPU). The emotion question ("why +0 vs the board, can we do better?") answered from
FROZEN artifacts: Bench-016's `predictions.json` + the t20k emotion test gold. 2026-10-01.

## The question

Emotion serves A0 (the ridge@8-armed Reflex base, 0.8850 — the highest bar on the
board). Every measured challenger already lost (A1 0.8550, H1 0.8775, the 45-cell H2
grid, and the encoder class dead-by-screen in Bench 032 + 035: floor 0.5950, class
max-lift +14.7 pt → ceiling 0.7420 < 0.8850). What this read adds: the ERROR
STRUCTURE of the 46/400 misses, and whether the two remaining untried levers —
(1) an oracle-shaped selective fusion and (2) a Bench-0029-style synth seat — have
any purchasable headroom.

## Method + self-check

Gold join: the t20k emotion test pull (4 pages × 100 rows, page order = harness
order), `int(label)`. **Self-check: all three frozen arm accuracies reproduce
EXACTLY** (A0 0.8850 · A1 0.8550 · H1 0.8775) — the join is proven, not assumed.
A0 abstains 185/400 (the ridge-armed fused gate; the 0.8850 is the forced-pick
hard-accuracy convention). Script: `analysis.py` in this dir.

## The confusion (A0, gold × pick)

| gold | n | recall | the misses |
|---|---|---|---|
| sadness | 119 | 0.983 | 2→joy |
| joy | 118 | 0.966 | 2→surprise, 2→sadness |
| **love** | **37** | **0.568** | **15→joy** (all 15 misses) |
| anger | 64 | 0.891 | 5→sadness, 1→joy, 1→surprise |
| fear | 52 | 0.788 | 5→joy, 4→sadness, 2→anger |
| surprise | 10 | 0.400 | 4→joy, 2→fear |

**love→joy is 33% of all errors; the love/fear/surprise tail is 71%.**

## Finding 1 — the engine already knows every error it makes

46/46 errors carry conf < 0.30 (mean **0.009** vs correct-mean 0.024); 32/46 are
abstained (70% vs 43% among correct). There is no silent-failure region: the
calibrated readout flags exactly the rows it misses. Any "better" arm must beat
0.8850 inside a region the current engine has already priced as near-zero
confidence.

## Finding 2 — the love→joy errors are annotation disagreement, NOT coverage (the synth-seat lever REFUTED)

Per-error lexical verdict: each miss's signature tokens voted by the t20k TRAIN
pool itself (token → top train label, n≥3):

| signature token | train pool says | test gold on the misses |
|---|---|---|
| blessed | **joy 61/121** | love (×3) |
| honored | **joy 56/57** | love |
| generous | **joy 51/93** | love (×2) |
| impressed | **surprise 61/63** | love |
| accepted / supportive-troops / feel-liked contexts | joy | love |

**0/15 love→joy errors are recoverable from this pool** — the train pool's own
labels point AWAY from the test gold on every one. A count-table cannot learn
"blessed → love" without contradicting its own train evidence; a synth seat
teaching it would be memorizing this test split's annotation profile (the
love/joy boundary in this corpus is annotator-soft: "blessed", "honoured",
"moved", "impressed" diary text). The Bench-0029 massive synth lever does NOT
transfer to emotion — massive's gap was coverage; emotion's is annotation.

## Finding 3 — the fusion ceiling is negative on the consult subset (H1/H2 re-derived from first principles)

On A0's 185 abstentions — exactly where any cascade/fusion consults the
specialist — **A1 reads 150/185 vs A0's forced 153/185.** The specialist is
WORSE on the subset fusion would route to it; that is the measured mechanism
behind H1's 0.8775 (−0.75 pt) and it a-priori kills every β-fusion that leans
on A1 where A0 is unsure. Oracle-selective (A0 + A1's 15 rescues) = **0.9225**
— but the rescues sit inside the annotation-noise region (decorrelated noise,
not signal), so the gate that finds them without overfitting does not exist in
the measured class; both real gates (H1 fused-gate, H2 grid) measured BELOW A0.

## Verdict

**Emotion 0.8850 is the pool-consistent ceiling for every current class.** The
error mass is (a) train/test annotation disagreement (love→joy: 0/15
recoverable), (b) small-n tail classes (surprise n=10), (c) genuinely ambiguous
texts ("agitated" fear/anger, "stunned/dazed" surprise/joy). The engine flags
all of it at conf ≤ 0.30. A0 serves — correctly — and the ±0.0 cell is the
honest display of "the modelless floor won on its strongest suite."

**Remaining movers, priced:** a contextual (fine-tuned LM) model class is the
only thing that could read "blessed to know this family" as love against the
lexical prior — the wave-1 model-class re-open bar, one suite, oracle-max
+3.7 pt (0.9225), GPU-host serve posture only. Not recommended at current
priority; this record is the priced refusal.

## Box state

Records-only (frozen artifacts + CPU python). No GPU, no serve, no arena run.

## Artifacts

`analysis.py` (this dir) — the self-checking join, confusion, lexical-verdict,
and fusion-subset analysis, runnable as
`python3 analysis.py` from this directory with the sibling checkouts standing
beside the workspace.
