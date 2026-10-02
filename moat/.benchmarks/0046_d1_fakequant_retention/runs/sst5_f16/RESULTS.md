# Bench record (sst5_f16) — the Reflex · instinct hybrid GOAT run

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome off — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power Some("AC Power") · powermode Some("high") · load1m Some(3.56) · swap_mb Some(2643.25) · quotable Some(true) · refusals []

## sst5

Verdict: **a0_stands** — instrument pick A1 refused — paired LB95 -0.0129 (Issue 008 T2).

Posture: cap 64 · head 0.00 · nb 16.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.281/0.417 · specialist bag count. Questions: 600 over 600 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.2900 | 0.2408 | 0.000 | 100 |
| ✓ | A1 | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.3400 | 0.2877 | 1.000 | 1 |

Served arm: **A0** (rank-0 11/48 candidates; the full table rides `registration.json`). The instrument's pick **A1** was REFUSED by the superiority gate (paired LB95 -0.0129 ≤ 0) — A0 serves and the suite is not sold (Issue 008 T2).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.3967 | 0.000 | 91 | 102 |
| A1 | 0.4217 | 1.000 | 1 | 1 |
| H1 | 0.4217 | 1.000 | 91 | 102 |

H1 decomposition: escalation 1.000 · decision p50 0 µs · total p50 91 µs (reflex 91 µs) · total p99 102 µs.

### Gates

- **G7 product gate (Issue 008 T2):** A1 − A0 mean +0.0250 · paired LB95 -0.0129 → REFUSED — A1 is not sold; A0 serves
- **G1 calibration:** raw ECE 0.1008 · Platt 0.0835 (fit engaged: true) · conformal floor 0.2121 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.0250 · UB95 +0.0129 vs δ 0.0392 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 13 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 40 ns/q) · H2 fusion 2.18 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (600 escalated):** lane p50 0 µs vs laya p50 13961 µs · lane−laya UB95 -13750 µs ≤ 0 → PASS
- **C1 encoder arm (RECORD-ONLY — serve: ✗, the encoder class is refused at serve; issue 014 decision 1):** NLEH v1 head `../riir-train/.raw/t599/t6_s0.bin` (5 classes · feat 6144) over the live laya-english encode (device metal) · accuracy **0.5267** (600 rows) · vs the incumbent A1: paired mean +0.1050 · LB95 +0.0535 (strictly above — the T2 form, A1 the comparator) · per-row p50 14250 µs / p99 17042 µs (the full arm cost per QUESTION row: encode + head; the provisional text-lane bar is ≤ ~300 µs — the arm reads ms-class, the recorded ground of the refusal). No serve change: A1 keeps serving.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

