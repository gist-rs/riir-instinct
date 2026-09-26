# Issue 006 — the massive_intent_en specialist reads chance-level on the reflex seat

**Status:** OPEN — filed 2026-09-27 (Bench 001 finding). The GOAT run's
instrument surfaced it; A0 stands on massive (registered), so the suite's
verdict is unaffected — but A1 at 0.0267 test / 0.0150 cal (~chance over
59 options) contradicts riir-train Bench 609's massive .670 and must be
explained before any distilled specialist is trusted on a seat.

## The anomaly

Bench 001 (`../.benchmarks/001_hybrid_goat/RESULTS.md`), massive row:

- A1 (instinct alone, the sealed `massive_intent_en_winner_v1.bin`):
  **0.0267** test, 0.0150 cal.
- The join pin verified every seat label (59) matched an artifact label
  (60) by NAME — the mapping is not the defect (riir-train trains over
  the full 60-intent set; the seat's test option union is 59; the extra
  class is never a survivor).
- Contrast: the same arena loads banking77's artifact at **0.7960** and
  emotion's at 0.8550 — the pipeline is not broken wholesale.

## What the 0.027 is NOT

1. Not the label join (verified by name, both directions).
2. Not the engine arity (59-way scoring over matched class rows).
3. Not a seat-shape change (A0 reflex reads 0.42 on the same questions —
   and the reflex engine trains on the same train_docs the seat builds).

## Working hypotheses (diagnose in this order)

1. **Teacher KeyMap::Name mismatch specific to massive.** The 576 T3
   teacher writer maps laya's per-label probabilities into the student's
   class order by option-key NAME for massive (`KeyMap::Name`) — if the
   laya checkpoint's label strings differ from the seat option keys
   (e.g. description-vs-name), the teacher vector is a PERMUTED prior and
   the distilled student inherits a shuffled prior. banking77/ag_news use
   `FixedInt` (positional) — those artifacts work.
2. **Train-vs-seat text shape.** The seat states are pyjson-serialized
   (`serialize_state`); the student trained on riir-train's own rows
   (t20k mirror, raw text). A constant JSON envelope dilutes but should
   not collapse; check whether t20k's massive `text` field is the
   utterance or already a serialized state.
3. **Dataset drift.** The reflex `.raw/datasets` massive train pull on
   this box starves 29 of 59 labels (the Issue-039 self-doc guard fired
   during the run) — the pool differs from 051's bytes. Does riir-train's
   t20k massive mirror share the drift?

## Plan

- [ ] T1 — reproduce on ONE case: bag the seat state bytes vs the
      t20k training text for the same utterance; diff the feature bytes.
- [ ] T2 — replay the teacher's `KeyMap::Name` mapping for massive: do
      the laya answer keys equal the seat option keys exactly?
- [ ] T3 — depending on T1/T2, re-distill massive's teacher dump or fix
      the seat-side text; re-run the arena's massive row only.
- [ ] T4 — record the verdict; cross-ref 576 (the artifact stays sealed
      either way — the fix is a RETRAIN or a READER fix, never a
      post-hoc byte patch).
