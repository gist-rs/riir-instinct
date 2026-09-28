# Bench 015 — the typed_decisions retrain seats: A1 T2-certified against BOTH A0 legs (the serving row waits on the multi-question serve shape)

**Status:** MEASURED — 2026-09-28. Train-side lane: riir-train Issue 581
/ Bench 614 (the retrain; commit `d59d3984`, artifact BLAKE3
`7f7a39e1935f8665beaf61106a84a65a0066a73fe3eee4ad7638fda0f776f2f0`,
epochs 3, 49 classes). This record is the consumer verdict: the 014
refusal is LIFTED — every presented key now has a class row — and the
specialist clears the product gate against the stronger of the two A0
postures. **No serving row in this landing**: the serve path's
one-question-per-case shape guard refuses typed's 5-question sets at
boot (`server.rs::from_parts`), and `--suites` defaults to ALL manifest
rows, so a row today would break the default container boot. The row
lands with the multi-question serve extension — **Issue 011** (the
upstream `decision_wire` law is "one state, ALL questions answered in
one call", so the shape is a per-question-sets decide, not a squeeze).
Records: `RESULTS.md` / `predictions.json` / `registration.json`
(arena-written).

## The control (run 1 — nothing had moved yet)

The canonical datasets dir + the OLD (014) artifact, full A0 pin:
**A0 0.4655 == reflex `run()` == published site ✓**, verdict a0_stands —
Bench 014 reproduced byte-identically on today's tree before anything
changed (the `data/tetris_critic/` dir and the reflex sibling's in-flight
tree touched nothing this lane reads).

## The owner-gated shape call (adjudicated by verdict)

Issue 581 T1 offered (a) synthesized security_incidents rows vs (b) real
rows. The deciding fact was measured: the 014 "template drift" was a
**fetch-cap artifact** — reflex's fetch took 800 of 1200 train rows and
all 400 test rows; the train split's 300 security_incidents rows sit at
offsets 900–1199, behind the cap. (b)'s own precondition resolved TRUE
and real rows dominate synthesis on data honesty; the verdict round also
added the gates this run honours: snapshot identity, disjointness, the
A0′ arm, and the max(A0, A0′) registration bar.

## The extended-dir read (run 2)

`--datasets-dir ../riir-train/.raw/datasets_typed_full` (train-000..007
byte-copies of reflex's canonical pages + the fetched 008..011; test
bytes identical; cal front byte-identical — the first train rows are the
same pages). `--skip-pin-a0` is deliberate (the Bench-012 precedent):
the pin's site half compares against the PUBLISHED envelope's row, and
this scratch dir diverges from it BY DESIGN — that divergence is the
A0′ measurement.

| arm | acc | note |
|---|---|---|
| A0′ | **0.5725** | reflex's own lane on the FULL pool — the corpus guard's self-doc fallback for security_incidents was costing the published A0 ~11 pt |
| **A1** | **0.6300** | instrument pick (rank-0, tied-max cal LCB 0.5958 with every H2 variant); registered |
| H1 | 0.6335 | fused gate; test point estimate edges A1 by noise — paired LB95 **−0.0048** (54 wins / 47 losses / 1899 ties; the arms differ only on the 101 non-escalated questions). A1 serves-class at 12 µs p99 vs H1's 1971 µs |

Gates (the max(A0, A0′) bar — both legs required):

- **vs A0′** (the arena's internal T2): mean +0.0575, LB95 **+0.0373 > 0** → PASS.
- **vs the PUBLISHED A0 0.4655** (offline paired read off run 1's frozen
  picks; same 2000 questions, snapshot-verified): mean +0.1645, LB95
  **+0.1432 > 0** (526 wins / 197 losses) → PASS.
- **G1 calibration:** raw ECE 0.4345 · Platt 0.0020 · conformal floor
  0.2117 → PASS.
- Latency scope: A1 p99 11.96 µs in-process. ⚠ Box state (the arena's
  own capture): `load1m 8.06 · quotable false — a sibling job is on the
  box` — accuracy rows are deterministic and stand; the LATENCY rows are
  directional only (the serving decision rests on the accuracy pairing,
  not on latency).

## The honest breakdown (a pooled LB95 hides this)

Per-workflow test accuracy (reconstructed from the frozen predictions ×
the dataset rows; 500 q/workflow):

| workflow | A0 pub | A0′ | A1 |
|---|---|---|---|
| agent_trace_observability | 0.4320 | 0.4300 | **0.6520** |
| customer_service | 0.6600 | 0.6260 | 0.6640 |
| invoice_processing | 0.5280 | **0.6140** | 0.5700 |
| security_incidents | 0.2420 | 0.6200 | **0.6340** |

- **A1's pooled win is NOT mainly the new workflow**: on
  security_incidents A0′ (0.6200) nearly matches A1 (0.6340); the
  biggest swing is agent_trace_observability (+22 pt over A0′), and on
  invoice_processing A0′ BEATS A1 by 4.4 pt.
- **Rare keys collapse to the majority class** (per-key recall on the 9
  newly-covered keys): investigate 72/74 (0.973) · High 37/49 (0.755) ·
  Moderate 16/31 (0.516) — but close_benign 0/3 · contain 1/15 ·
  monitor 0/8 · Low 0/9 · Negligible 0/1 · Critical 1/10. The train
  pool's own imbalance (investigate 214 vs monitor 21; Negligible 6)
  reproduces in predictions. Class-rebalancing is the improvement
  backlog, not a serving blocker.

Cal-prefix note (the verdict's disclosure ask): the trainer reads the
FULL train envelope, cal rows included — the lane's established protocol
(Bench 610–612; reflex's own corpus law treats cal rows as corpus
members). The T2 product gate reads the TEST split only. If a future
hybrid arm choice keys on cal-slice accuracy, A1 has seen those rows —
inherited precedent, disclosed here so it cannot be misread later.

## Serving truth

- The manifest carries **no typed row** in this landing (the boot-shape
  blocker above; an inert-but-boot-breaking row is the 014 lesson's
  mirror — the refusal must be loud, not deferred into a red boot).
- When Issue 011 lands: the row is `A1` (the registered arm; H1's edge
  is measured noise), digest `blake3:7f7a39e1…`, and the lane doc cell
  flips to serves-A1 with this record's numbers.
- reflex's own cap fix (the +10.7 pt modelless A0 gain) is filed as
  **riir-reflex Issue 052** — worth landing whichever way the specialist
  went.

## The lane doc

`.benchmarks/014_typed_decisions_specialist/hybrid_lane_doc.json` — the
typed cell moves from a0_stands to the measured-certified state with the
serve-pending disclosure (the site's display law keeps the not-yet-served
reason visible, never hidden).
