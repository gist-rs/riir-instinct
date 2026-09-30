# Bench 028 — the Reflex · instinct hybrid GOAT run

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome off — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power None · powermode None · load1m None · swap_mb None · quotable None · refusals ["power source unreadable — UNJUDGED"]

## code_fixtures

Verdict: **hybrid_arm**.

Posture: cap 18446744073709551615 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.461/0.337 · specialist bag presence. Questions: 32 over 16 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.4688 | 0.3977 | 0.000 | 684 |
| ✓ | A1 | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.25,nmin=2,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.25,nmin=2,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.25,nmin=2,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.25,nmin=4,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.25,nmin=4,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.25,nmin=4,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.25,nmin=8,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.25,nmin=8,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.25,nmin=8,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.5,nmin=2,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.5,nmin=4,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.5,nmin=4,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.5,nmin=4,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.5,nmin=8,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.5,nmin=8,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=0.5,nmin=8,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=1,nmin=2,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=1,nmin=4,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=1,nmin=8,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=1,nmin=8,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=2,nmin=2,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=2,nmin=2,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=2,nmin=4,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=2,nmin=4,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=2,nmin=4,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=2,nmin=8,τ=2) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=2,nmin=8,τ=4) | 0.8203 | 0.7570 | 1.000 | 23 |
| ✓ | H2(β=2,nmin=8,τ=8) | 0.8203 | 0.7570 | 1.000 | 23 |

Served arm: **A1** (rank-0 47/48 candidates; the full table rides `registration.json`). The superiority gate certified it strictly above Reflex (paired LB95 +0.0021 > 0).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.3750 | 0.000 | 290 | 1570 |
| A1 | 0.5625 | 1.000 | 2 | 22 |
| H1 | 0.5625 | 1.000 | 290 | 1575 |

H1 decomposition: escalation 1.000 · decision p50 0 µs · total p50 290 µs (reflex 290 µs) · total p99 1575 µs.

### Gates

- **G7 product gate (Issue 008 T2):** A1 − A0 mean +0.1875 · paired LB95 +0.0021 → PASS (strictly above Reflex)
- **G1 calibration:** raw ECE 0.2524 · Platt 0.1519 (fit engaged: true) · conformal floor 0.3430 → PASS
- **G2 fusion overhead:** H1 fusion-only 50 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 123 ns/q) · H2 fusion 5.48 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

