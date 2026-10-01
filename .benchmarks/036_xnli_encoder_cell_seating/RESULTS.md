# Bench 036 — the Reflex · instinct hybrid GOAT run

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome off — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power Some("AC Power") · powermode Some("high") · load1m Some(2.38) · swap_mb Some(3519.0) · quotable Some(true) · refusals []

## xnli_en

Verdict: **a0_stands** — the instrument registered A0 outright.

Posture: cap 64 · head 1.00 · nb 16.00 · ridge 0.00 (pair) · fused-gate thresholds \n0.552/0.511 · specialist bag count. Questions: 300 over 300 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5900 | 0.5318 | 0.000 | 157 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.5150 | 0.4570 | 1.000 | 5 |
| ✓ | H2(β=2,nmin=4,τ=2) | 0.5150 | 0.4570 | 1.000 | 5 |
| ✓ | H2(β=2,nmin=8,τ=2) | 0.5150 | 0.4570 | 1.000 | 5 |

Served arm: **A0** (rank-0 4/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.5233 | 0.000 | 124 | 145 |
| A1 | 0.4100 | 1.000 | 1 | 3 |
| H1 | 0.4100 | 1.000 | 124 | 145 |

H1 decomposition: escalation 1.000 · decision p50 0 µs · total p50 124 µs (reflex 124 µs) · total p99 145 µs.

### Gates

- **G1 calibration:** raw ECE 0.5027 · Platt 0.1071 (fit engaged: true) · conformal floor 0.1351 → PASS
- **G5 pre-registered (gap suite):** no hybrid arm registered (A0 stands) — the gate is refused, never a pass → FAIL
- **G2 fusion overhead:** H1 fusion-only 12 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 29 ns/q) · H2 fusion 4.35 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (300 escalated):** lane p50 0 µs vs laya p50 203546 µs · lane−laya UB95 -174734 µs ≤ 0 → PASS
- **C1 encoder arm (RECORD-ONLY — serve: ✗, the encoder class is refused at serve; issue 014 decision 1):** NLEH head `../riir-train/.raw/t599/xnli_en_encoder_v1.bin` (3 classes · feat 4096) over the live laya-english encode (device metal) · accuracy **0.8600** (300) · vs the incumbent A1: paired mean +0.4500 · LB95 +0.3832 (strictly above — the T2 form, A1 the comparator) · per-row p50 231709 µs / p99 428943 µs (the full arm cost: encode + head; the provisional text-lane bar is ≤ ~300 µs — the arm reads ms-class, the recorded ground of the refusal). No serve change: A1 keeps serving.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

