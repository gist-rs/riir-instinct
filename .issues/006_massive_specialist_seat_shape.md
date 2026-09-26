# Issue 006 — the massive_intent_en specialist reads chance-level on the reflex seat

**Status:** RESOLVED 2026-09-27 — the specialist was never broken. The ARENA
compared two different index spaces: A0's probs and every `gold.idx` speak
the question's PRESENTED-option space (positions among the shuffled 20), while
A1/H1/H2 scored and picked in the seat's LABEL space (the 59-label
permutation) and the arena compared the two directly. Only massive could
expose it — every other suite presents its full label universe, where the two
spaces coincide. Fixed; the corrected instrument's frozen read is
`.benchmarks/001_hybrid_goat` (v2): massive A1 **0.8167** (was "0.0267"),
registered **H2(β=0.25, nmin=4, τ=4) 0.8300** vs A0 0.4200 — the suite flips
from "A0 stands" to a hybrid win, G1+G3 PASS.

## The anomaly (as filed)

Bench 001 v1, massive row: A1 0.0267 test / 0.0150 cal — below the 0.05
chance of a 20-option seat, against riir-train Bench 609's winner holdout
.670. banking77's artifact read 0.796 through the same path.

## What the 0.027 was NOT (all three filed hypotheses refuted by measurement)

1. **Teacher KeyMap::Name mismatch** — refuted: the teacher writer fails
   loud on any presented key not in the student universe and pins every
   per-row gold (`map_targets`), and the student asserts dump classes ==
   sorted universe (order included). A permuted prior cannot get through.
2. **Train-vs-seat text shape** — refuted: the seat's state string is
   `{'utterance': …}` (pyjson of a one-key state — the options live in the
   question criteria, NOT the state), and scoring the seat's own test rows
   through the serving path reads 0.7559 (state form) vs 0.7626 (raw form):
   the envelope costs ~0.7 pp.
3. **Dataset drift** — refuted: t20k and datasets massive test rows are the
   same rows (identical scores to the 4th decimal, 2268/2974 both). The
   artifact (BLAKE3 `7bc3ee385f81abc0`, matching Bench 609's recorded seal)
   reads 0.86 t20k-train / 0.76 both mirrors' test splits through the
   SERVING reader — the model is healthy everywhere.

## The actual defect (two instrument bugs, one class)

**Position-vs-index confusion**, the same class twice:

1. `eval_a1_h2`/`eval_a0_h1`/the H2 grid: `SpecialistLane::join`'s `perm`
   maps SEAT LABEL → artifact class, but the arena indexed `perm[li]` with
   A0 survivor POSITIONS (0..19 into the presented options) and compared
   `pick_alone`'s label-space pick against `case.gold[0].idx` (a position).
   Expected accuracy ≈ P(pick lands in 0..19)·P(coincides with gold's
   position) ≈ 1/59 ≈ .017 — exactly the observed .0150/.0267. Masked on
   every suite whose presented set is the full universe (ag_news/emotion/
   sst5/xnli: identity by construction; banking77: 77 keys are the labels),
   exposed only by massive's 20-of-59 sampled + shuffled options.
2. The registration: `cands[rank0_sorted[select_arm(..)]]` — `select_arm`
   already returns the CANDIDATE index (`rank0[best_pos]`, per its
   docstring); re-indexing `rank0_sorted` by it double-indirects. Masked
   while the winning rank-0 element happened to equal its position (v1:
   ag_news H2(1,2,4)→0.8975, banking A0 — v1's banking/massive/ag_news
   registrations were artifacts of this); crashed loud in v2 once massive's
   corrected H2 arms changed its rank-0 set to [0, 2, 16].

## The fix (this commit)

- `src/hybrid.rs`: `h1_decide` takes a per-case `class_of_pos: &[usize]`
  (position → artifact class row) — the bridge is per-case seat data, never
  lane data; the pick is a POSITION. New `scores_classes_into` scores
  arbitrary class rows; new test `h1_scores_the_class_the_position_denotes`
  pins the contract on a perm-vs-position discriminating toy.
- `src/bin/arena.rs`: per-suite `key_map` (criteria key → (label idx, class
  row)) + per-case `fill_positions` mirroring the ENGINE's own two
  resolution rules (`solve_into`): all keys name seat labels → by name
  (massive/banking77); else count == label count → identity by index (the
  fixed-criteria suites; score-array criteria included — sst5); else REFUSE
  loud. A1/H2 score and pick among the presented positions; the H2 margin's
  rivals become the presented options (the decision's true alternatives).
- Registration: `select_arm(&rank0, ..)` used directly as the candidate
  index (the redundant sort dropped).
- `examples/massive_anomaly_probe.rs` (new, T1's instrument): scores one
  winner through the serving path over every source × form — the
  differential that localized the collapse to the arena path in one run.

## Verdict for the record

The v1 massive anomaly was an instrument defect, not a model defect — and
the instrument's other readings were MORE damaged than the anomaly showed:
banking77's v1 "A0 stands" (0.3960) hid a registered-A1 truth (0.7960), and
massive now registers H2 at 0.8300 (+41 pp over A0). The v2 frozen read
supersedes Bench 001 v1 wholesale; the seat/artifact/training pipeline
needed no change. Cross-ref: riir-train Issue 576 (the producer, healthy),
riir-reflex `harness::runner::engine_request` + `solve_into` (the space
laws the bridge mirrors).
