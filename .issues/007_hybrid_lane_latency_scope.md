# Issue 007 — the hybrid lane's latency scope: A1/H2 cells publish arm-only time under an "end-to-end" label

**Status:** IN PROGRESS — scope law landing same day (arena field + doc builder + site disclosure).

## Symptom

On reflex.gist.rs `/bench`, the `instinct (hybrid)` lane reads
**0.6–1.6 µs** p50 on ag_news/emotion/sst5/massive_intent_en while the
reflex modelless lane it is composed OVER reads **88–539 µs** — the hybrid
appearing 100×+ faster than its own first stage. `banking77` reads
0.3795 ms — correctly ABOVE modelless (0.353 ms).

## Root cause (read off the code, not inferred)

`riir-instinct/src/bin/arena.rs`:

- **H1** (banking77) is the true cascade: `total_durs_us = dt_us +
  se.durs_us[ci]` — reflex seat solve **+** fusion. Honest.
- **A1 / H2** (`eval_a1_h2`) run **no reflex solve at all** — the
  docstring says it: *"no reflex solve — the specialist forward + the
  frozen count tables only"* — and set `total_durs_us = fwd_us` (the
  specialist forward alone, ~1 µs).

`scripts/build_hybrid_doc.py` then publishes `total_durs_us` for every
cell as *"the composed end-to-end per-question latency"* — true only for
H1. One lane label, two measurement scopes, 1 of 5 cells honest.

## The scope law (the guard)

*A latency cell must name what it timed. "End-to-end" is a claim only a
seat-composing arm may make.*

- [x] **T1 — typed scope in the arena.** `ArmOut.contains_seat_solve: bool`
      (A0/H1 → true; A1/H2 → false). Serialized into `predictions.json`;
      absent in old artifacts (serde default) — the doc builder then infers
      by arm name (A0/H1 → seat+arm, else arm-only) so the Bench-002 record
      stays readable.
- [x] **T2 — the doc builder discloses.** `build_hybrid_doc.py` emits
      `latency_scope: "seat+arm" | "arm-only"` per cell and stops calling
      arm-only time end-to-end.
- [x] **T3 — the site renders it.** `publish_bench.py` passes the field
      through; the bench page shows the scope beside the latency cell for
      `arm-only` rows.
- [x] **T4 — pinned.** A doc built from a fixture whose registered arm is
      A1 must carry `arm-only` (builder self-test).

Deliberately NOT a fix: measuring A1/H2 as seat+specialist. They do not
run reflex; composing the numbers would fabricate a counterfactual cascade
cost — that composition already exists and is called H1. Label, don't
invent.
