# 008 — the 5-class signature's within-decision value structure (Issue 009 T5, companion measurement)

**Status:** CLOSED — 2026-09-27. Verdict: **CONSISTENT WITH Bench 006's
WIRE-MUST-WIDEN** — this instrument measures the within-decision value
structure the served input destroys, and its own oracle bound shows the
pooling loss is small; the wire-decoder loss Bench 006 measured is
therefore dominated by CONTEXT loss, not by within-decision pooling.
Data: `signature_spread.json`. Harness: `benches/tetris_teacher_check`
behind `tetris_goat` (`src/tetris_lane.rs`).

## Provenance (the Issue-825 class, handled)

This measurement ran CONCURRENTLY with Bench 006
(`.benchmarks/006_wire_ceiling`, katgpt-rs `tetris_10_wire_ceiling.rs` @
`6b6589370`) in another session. The two first reads DISAGREED — this
record's draft concluded "NO WIRE WIDENING" from a 95% "signature
ceiling"; Bench 006 concluded WIRE-MUST-WIDEN from 0.30 rank agreement.
Cross-reading the instruments resolved it, and the resolution went
through a Claude verdict round (REVISE, round 1): this record's 95% was
a FULL-INFORMATION oracle bound and cannot settle the wire question in
either direction; it is re-scoped here to what it measures cleanly.

## What this instrument measures

Per decision (teacher or reflex play; value oracle = the champion 1-ply
evaluator, mode at the decision root; signatures decoded through the
reflex head's own grammar):

- **Within-group spread** — for each 5-class signature group present in
  the decision, (max − min) of the true option values sharing it;
  aggregated as the per-decision MAX spread.
- **ceiling_agree** — the best possible pick by per-decision
  max-per-signature vs the eval argmax. ⚠ This is a FULL-INFORMATION
  oracle bound: it knows each decision's true per-signature maxima,
  which no wire decoder can know. It upper-bounds the cost of
  WITHIN-DECISION POOLING and nothing else — it says nothing about a
  decoder's ability to tell WHICH signature group wins in a new
  decision (that is Bench 006's question).
- **actual_pick_*** — the arm's real pick vs the eval argmax.

## Results (seeds 1..=20 + 607; teacher RNG ^ 0x05EE_D892)

| source | regime | dec | wg>0 | mean max spread | ceiling_agree (oracle) | actual_agree |
|---|---|---|---|---|---|---|
| teacher b400 | garbage 16@75 | 32 691 | **88.8%** | 28.6 pts | 95.6% | 86.3% |
| teacher b400 | garbage 18@75 | 24 145 | **89.3%** | 29.5 pts | 95.9% | 86.7% |
| teacher b400 | empty | 9 900 | **88.6%** | 28.9 pts | 95.3% | 85.5% |
| reflex head | garbage 16@75 | 2 394 | 70.9% | 25.9 pts | 95.6% | 51.3% |
| reflex head | garbage 18@75 | 1 717 | 64.9% | 17.5 pts | 94.4% | 57.5% |
| reflex head | empty | 2 987 | 86.7% | 143.3 pts | 70.2% | 28.6% |

(Mean decision value range ≈ 105–290 pts depending on cell.)

## Reading

1. **Within-decision pooling is real but CHEAP.** The 5-tuple pools
   value-different options in ~89% of teacher-play decisions (mean max
   within-group spread ≈ 29 pts ≈ 25% of the decision's range), yet the
   per-decision max-oracle still reaches the eval argmax 94.4–95.9% of
   the time — pooling costs at most ~4–6 points of pick agreement.
   **The bulk of Bench 006's decoder loss (0.30–0.55 agreement) is
   therefore CONTEXT loss: from the wire alone, a decoder cannot tell
   which signature group wins in this particular decision.** That is
   the mechanism direction Bench 006's "information, not sampling"
   finding points at; this instrument bounds the pooling share of it.
2. **Why the first draft's 95% was misread.** The max-oracle knows each
   decision's true per-signature maxima; a wire decoder knows none of
   that. A ceiling computed WITH information the serving path does not
   have is not a serving ceiling — Bench 006's saturated decoders are
   the serving-side measurement, and its cross-review addendum's
   demeaned-table class (katgpt-rs `5961e0991`, agree 0.547) is the
   strongest wire decoder measured — still ≤ the 0.85 MUST-WIDEN gate.
3. **The reflex head's actual_agree (28.6–57.5%) is NOT comparable to
   Bench 006's decoder agreements** — different position sets (its own
   short, early-dying games vs champ-play held decisions) and a
   different target (the laya oracle, not the champion eval). Read the
   head row as the deployed policy's eval-gap on its own distribution,
   never as a decoder-class ranking. Its empty-board cell (ceiling
   70.2%) does show the signature degrades fastest exactly where the
   head already fails — consistent with the context-loss story.

## Verdict

**CONSISTENT WITH WIRE-MUST-WIDEN.** The within-decision structure this
instrument destroys is real but small (≤ ~4–6 pick points); the
wire-decoder loss Bench 006 measured is dominated by context loss,
which only the widened wire (raw afterstate, Instinct-only sidecar per
Bench 006's recorded design decision) removes. This record's harness
(`src/tetris_lane.rs`) remains the lane's shared measurement substrate —
its T6 half is Bench 007.

Provenance: M3 Max, AC, load 4.9–5.9 (10 workers), bench profile,
2026-09-27. Runner:
`cargo bench --features tetris_goat --bench tetris_teacher_check`.
