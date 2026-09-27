# Bench 003 — `arsenal_budget_goat`: the arsenal budget legs' GOAT gate (Proposal 001 T8)

**Status:** COMPLETE 2026-09-27 — GOAT **PASS** 7/7 after one real
defect the gate caught and fixed (the unsigned fold, below). Run:
`cargo bench --bench arsenal_budget_goat --features arsenal_goat`
(release; the run REFUSES — exit 1 — when the datasets/winners are
absent, naming `INSTINCT_DATASETS_DIR` / `INSTINCT_WINNERS_DIR`).

**Box state (both runs, from the bench's own PROVENANCE lines):** M3 Max
16 cores, AC power, battery 100%, load averages ~4.8–5.7 (concurrent
agent sessions on the box — the bars carry ≥140× headroom at p99, so the
load class cannot reach a verdict).

## The four legs (the proposal's GOAT line, made measurable)

| gate | bar | final run |
|---|---|---|
| init validate p99 (parse + validate incl. six artifact digest reads) | ≤ 1000 ms | 12.7 ms p50 / 13.9 ms p99 |
| lazy registry (6 slots) p99 | ≤ 1 ms | 0 µs |
| hoard_check k=5 (real six centroids) p99 + admits | ≤ 100 µs, 6/6 admit | 3.25 µs / 4.3 µs, 600/600 |
| hoard_check k=64 (synthetic) p99 | ≤ 100 µs | 5.6 µs / 7.4 µs |
| slot Ready-read p99 (decision-path lock+match) | ≤ 10 µs | 0 ns p50 / 42 ns p99 |
| swap hammer: torn reads / decision errors / served | 0 / 0 / ≥1000 | **0 / 0 / 335,063** over 12 installs |
| armed-off identity | literal pin + 6/6 armed admits, bit-identical ×2 | PASS (vendi 2.878 vs floor 1.2 per suite) |

Disclosed, never gated:

- **Eager six-lane boot total 19.6 s** (ag_news 1041 ms, emotion 719,
  sst5 522, massive 2888, **banking77 13,176**, xnli 1260) — the cost the
  lazy posture defers; the dominant lane is banking77's seat fit. The
  lazy boot pays NONE of this (registry 0 µs) and the first-decision
  503 window covers the per-lane price.
- **Swap install p50 2.5 s / p99 21 s** — the honest production swap
  price: a fresh `prepare_seat` (posture fit) + artifact decode +
  whole-snapshot install. Swaps are operator/curator acts (A7), never
  per-decision work.
- **Contended decide** (3 readers under swap churn) p50 146 µs / p99
  174 µs; **banking77 quiescent** (77 labels, the served ceiling) p50
  347 µs / p99 366 µs. Bench 002's 9–119 ns gated the FUSION axis only;
  this is the full served path (eval_seat synthesis + specialist scoring
  + fusion) — the served ceiling, disclosed for capacity planning.
- 400k decisions/12 installs in the hammer with zero torn reads and
  monotone epochs — A2's atomicity under real concurrent load, not just
  the unit hammer.

## The finding the gate caught: the unsigned fold made every corpus a near-duplicate

The FIRST full run **GOAT FAILED** on the identity leg — and the failure
was the gate doing its job. With the original unsigned `fold8`
construction, the armed hoarding gate admitted **0/6 real suites**
(each refused `NearDuplicate` at the first loaded neighbor, cos > 0.95 —
visible as an 83 ns check: the colinearity early-exit).

Root cause: `fold8` sums unsigned bucket proportions. Every English
corpus's token-hash histogram reads ≈ uniform over the 8 admission
buckets (the hash is content-blind at bucket granularity), so every
suite's centroid pointed within 0.95 cos of every other's — mirrors and
strangers were indistinguishable, and the gate's own contract ("two
suites serving near-identical corpora produce colinear centroids") could
not hold: ALL corpora read colinear. The unit tests passed because they
used synthetic orthogonal vectors; no test ever fed the gate REAL
corpora — the bench was the first instrument to do so.

Fix (same landing, `src/arsenal_ops.rs::corpus_centroid`): a **signed
simhash fold** — each bag entry's weight is multiplied by a balanced ±1
derived from a multiplicative mix of its own bucket id (token-level
sign; per-axis signs would cancel out of the cosine and are useless).
Same corpus (a mirror) reproduces the identical token set → identical
signed sum → cos → 1 (still refused); distinct corpora produce
independent ±1 sums → cos ≈ 0 (admitted; the six real suites now read
vendi 2.878 vs floor 1.2). Anti-correlated directions (cos < −0.95) are
maximal distinctness, not colinearity — the gate only caps positive cos,
correctly.

Scope note: the centroid feeds ONLY the hoarding gate (boot/swap
policy); served decisions never touch it — the frozen-predictions
serving parity gate (`tests/serve_gates.rs`, 11/11) passes unchanged.
All 45 lib tests (default) and 54 (all-features) green; clippy clean at
both postures.

## Validation state

- Validated against **riir-reflex@9dc2e02** (their last consumer-green
  commit — develop@63a4f9d carries the sibling's ungated
  `set_blend_scales` writing `option_cond`/`nb_ridge`-gated fields, the
  consumer-posture break recorded in Proposal 001's T5+T6 row; a re-run
  against the fixed sibling is owed when they land the fix). The bench
  consumes only the serving APIs (`prepare_seat` / `boot_from_seat` /
  `boot_bytes` / `decide` / `centroid`), unchanged across that range.
- Full log: `goat_pass.log` (stdout only — the seat-fit progress spam
  rides stderr, 371 lines, kept out of the record by construction).
- Promotion verdict: the budget/swap legs **stand as shipped** (default
  posture, kill-switches in place); the GOAT gate is now the standing
  re-run instrument for any centroid/gate/manifest change.
