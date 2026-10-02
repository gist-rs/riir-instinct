# Bench record (prompt_injections_q4) — the Reflex · instinct hybrid GOAT run

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome off — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power Some("AC Power") · powermode Some("high") · load1m Some(9.29) · swap_mb Some(2499.25) · quotable Some(false) · refusals ["load 9.29 > 6 — a sibling job is on the box"]

## prompt_injections

Verdict: **hybrid_arm**.

Posture: cap 64 · head 0.00 · nb 1.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.799/0.462 · specialist bag count. Questions: 116 over 116 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.8000 | 0.7254 | 0.000 | 147 |
| ✓ | A1 | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=2,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=2,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=2,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=4,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=4,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=4,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=8,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=8,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=8,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.5,nmin=2,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.5,nmin=4,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.5,nmin=4,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.5,nmin=4,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.5,nmin=8,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.5,nmin=8,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=0.5,nmin=8,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=2,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=4,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=8,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=8,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=2,nmin=2,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=2,nmin=2,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=2,nmin=4,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=2,nmin=4,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=2,nmin=4,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=2,nmin=8,τ=2) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=2,nmin=8,τ=4) | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H2(β=2,nmin=8,τ=8) | 0.9600 | 0.9117 | 1.000 | 3 |

Served arm: **A1** (rank-0 47/48 candidates; the full table rides `registration.json`). The superiority gate certified it strictly above Reflex (paired LB95 +0.0082 > 0).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.7672 | 0.000 | 74 | 97 |
| A1 | 0.8534 | 1.000 | 0 | 3 |
| H1 | 0.8534 | 1.000 | 74 | 97 |

H1 decomposition: escalation 1.000 · decision p50 0 µs · total p50 74 µs (reflex 74 µs) · total p99 97 µs.

### Gates

- **G7 product gate (Issue 008 T2):** A1 − A0 mean +0.0862 · paired LB95 +0.0082 → PASS (strictly above Reflex)
- **G1 calibration:** raw ECE 0.0824 · Platt 0.1017 (fit engaged: true) · conformal floor 0.3769 → FAIL
- **G2 fusion overhead:** H1 fusion-only 8 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 14 ns/q) · H2 fusion 4.48 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (116 escalated):** lane p50 0 µs vs laya p50 20633 µs · lane−laya UB95 -20221 µs ≤ 0 → PASS
- **C1 encoder arm (RECORD-ONLY — serve: ✗, the encoder class is refused at serve; issue 014 decision 1):** NLEH v1 head `../riir-train/.raw/t608/prompt_encoder_v1.bin` (2 classes · feat 3072) over the live laya-english encode (device metal) · accuracy **0.7759** (116 rows) · vs the incumbent A1: paired mean -0.0776 · LB95 -0.1710 (not certified above A1) · per-row p50 15954 µs / p99 34919 µs (the full arm cost per QUESTION row: encode + head; the provisional text-lane bar is ≤ ~300 µs — the arm reads ms-class, the recorded ground of the refusal). No serve change: A1 keeps serving.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

