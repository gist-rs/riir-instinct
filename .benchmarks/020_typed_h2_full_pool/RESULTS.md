# Bench 020 — the Reflex · instinct hybrid GOAT run

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome off — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power Some("AC Power") · powermode Some("high") · load1m Some(16.8) · swap_mb Some(1690.19) · quotable Some(false) · refusals ["load 16.80 > 6 — a sibling job is on the box"]

## typed_decisions

Verdict: **hybrid_arm**.

Posture: cap 48 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.068/0.912 · specialist bag count. Questions: 2000 over 400 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5720 | 0.5353 | 0.000 | 33715 |
| ✓ | H1 | 0.6300 | 0.5938 | 0.714 | 33715 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=0.5,nmin=2,τ=8) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=0.5,nmin=4,τ=2) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=0.5,nmin=4,τ=4) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=0.5,nmin=4,τ=8) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=0.5,nmin=8,τ=2) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=0.5,nmin=8,τ=4) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=0.5,nmin=8,τ=8) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=1,nmin=2,τ=2) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=1,nmin=4,τ=2) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=1,nmin=8,τ=4) | 0.6380 | 0.6019 | 1.000 | 15 |
| ✓ | H2(β=1,nmin=8,τ=8) | 0.6380 | 0.6019 | 1.000 | 15 |

Served arm: **H2(β=0.5,nmin=2,τ=2)** (rank-0 20/48 candidates; the full table rides `registration.json`). The superiority gate certified it strictly above Reflex (paired LB95 +0.0580 > 0).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.5725 | 0.000 | 1071 | 44832 |
| A1 | 0.6300 | 1.000 | 5 | 41 |
| H1 | 0.6335 | 0.717 | 1071 | 44833 |
| H2(β=0.5,nmin=2,τ=2) ★ | 0.6475 | 1.000 | 5 | 41 |

H1 decomposition: escalation 0.717 · decision p50 0 µs · total p50 1071 µs (reflex 1071 µs) · total p99 44833 µs.

### Gates

- **G7 product gate (Issue 008 T2):** H2(β=0.5,nmin=2,τ=2) − A0 mean +0.0750 · paired LB95 +0.0580 → PASS (strictly above Reflex)
- **G1 calibration:** raw ECE 0.1899 · Platt 0.0095 (fit engaged: true) · conformal floor 0.1701 → PASS
- **G2 fusion overhead:** H1 fusion-only 11 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 276 ns/q) · H2 fusion 3.17 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.


## Landing note (2026-09-28, the serving decision)

**typed_decisions serves H2(β=0.5,nmin=2,τ=2) 0.6475** — the manifest row
moved A1 → H2 (`arsenal.toml`), PINNED_MANIFEST_DIGEST re-pinned, and the
serve-gate parity now replays THESE H2 picks through `decide_multi`
(`typed_decisions_serves_the_frozen_h2_picks`, 14/14 green).

Certified against EVERY leg — the first arm in repo history to clear all
three:
- vs A0' 0.5725 (the full-pool reflex row, reflex Bench 078): +7.5 pt,
  paired LB95 +0.0580 — T2 PASS.
- vs A1 0.6300 (the certified specialist, Bench 015): +1.75 pt, paired
  LB95 +0.0062 (84W/49L at n=2000) — the Issue-005 attribution standard
  MET (the hybrid beats the specialist it composes, not just reflex).
- vs H1 0.6335: +1.4 pt, paired LB95 +0.0034.
- G1 PASS (platt 0.0095 vs the conformal floor 0.1701).

Margin source: the option-conditioned (qid, option) count tables
(reflex issue 038 T7b) exposed to the seat through reflex `b5cf4b0`'s
`oc()` accessor + `seen_count` evidence bitmap — Issue 005's H2/H3
precondition ("typed's options are state-field values, the nb_scope
margin never activates") resolved. The fusion itself is the unchanged
`prior_fusion_pick`; only the margin's table family changed.

Serve-path defect caught by the parity gate at its first H2 replay: the
server's seat knobs carried `oc_select: false` (an upstream-gap note —
"option_cond rides the dep for the nb_ridge compile only"), so the
serve engine never built the oc tables and the H2 arm would have served
a pure-A1 fusion wearing the H2 name (drift at question 5, case
agent_trace_observability). The knobs now mirror the arena's
byte-identically; the noul lookup keys translate to the event spellings
("no"/"yes") against the presented fixed rendering (["false","true"]),
position-aligned.

Box state: load 16.8 (the Tetris round-4 trainer + sibling agents) —
latency NOT QUOTABLE, the accuracy is the claim (deterministic compute;
the arena lines disclose the refusal).
