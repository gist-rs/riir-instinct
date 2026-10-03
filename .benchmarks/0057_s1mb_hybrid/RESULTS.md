# Bench 0057 — the Reflex · instinct hybrid GOAT run

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome off — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power Some("AC Power") · powermode Some("high") · load1m Some(6.82) · swap_mb Some(31393.75) · quotable Some(false) · refusals ["load 6.82 > 6 — a sibling job is on the box"]

## s1mb_choice

Verdict: **a0_stands** — the instrument registered A0 outright.

Posture: cap 16 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.010/0.576 · specialist bag count. Questions: 4329 over 4329 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.1550 | 0.1183 | 0.000 | 3687 |
| ✓ | A1 | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.25,nmin=2,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.25,nmin=2,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.25,nmin=2,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.25,nmin=4,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.25,nmin=4,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.25,nmin=4,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.25,nmin=8,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.25,nmin=8,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.25,nmin=8,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.5,nmin=2,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.5,nmin=4,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.5,nmin=4,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.5,nmin=4,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.5,nmin=8,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.5,nmin=8,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=0.5,nmin=8,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=1,nmin=2,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=1,nmin=4,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=1,nmin=8,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=1,nmin=8,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=2,nmin=2,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=2,nmin=2,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=2,nmin=4,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=2,nmin=4,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=2,nmin=4,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=2,nmin=8,τ=2) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=2,nmin=8,τ=4) | 0.1250 | 0.0922 | 1.000 | 41 |
| ✓ | H2(β=2,nmin=8,τ=8) | 0.1250 | 0.0922 | 1.000 | 41 |

Served arm: **A0** (rank-0 47/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.2296 | 0.000 | 414 | 1954 |
| A1 | 0.3024 | 1.000 | 4 | 144 |
| H1 | 0.2885 | 0.780 | 414 | 1955 |

H1 decomposition: escalation 0.780 · decision p50 0 µs · total p50 414 µs (reflex 414 µs) · total p99 1955 µs.

### Gates

- **G1 calibration:** raw ECE 0.2060 · Platt 0.2060 (fit engaged: true) · conformal floor 0.4061 → FAIL
- **G2 fusion overhead:** H1 fusion-only 461 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 497 ns/q) · H2 fusion 1.17 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## s1mb_noul

Verdict: **a0_stands** — instrument pick A1 refused — paired LB95 -0.0169 (Issue 008 T2).

Posture: cap 64 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.455/0.601 · specialist bag count. Questions: 6173 over 6173 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5000 | 0.4423 | 0.000 | 437 |
| ✓ | A1 | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.25,nmin=2,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.25,nmin=2,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.25,nmin=2,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.25,nmin=4,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.25,nmin=4,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.25,nmin=4,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.25,nmin=8,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.25,nmin=8,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.25,nmin=8,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.5,nmin=2,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.5,nmin=4,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.5,nmin=4,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.5,nmin=4,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.5,nmin=8,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.5,nmin=8,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=0.5,nmin=8,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=1,nmin=2,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=1,nmin=4,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=1,nmin=8,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=1,nmin=8,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=2,nmin=2,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=2,nmin=2,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=2,nmin=4,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=2,nmin=4,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=2,nmin=4,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=2,nmin=8,τ=2) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=2,nmin=8,τ=4) | 0.5200 | 0.4620 | 1.000 | 26 |
| ✓ | H2(β=2,nmin=8,τ=8) | 0.5200 | 0.4620 | 1.000 | 26 |

Served arm: **A0** (rank-0 47/48 candidates; the full table rides `registration.json`). The instrument's pick **A1** was REFUSED by the superiority gate (paired LB95 -0.0169 ≤ 0) — A0 serves and the suite is not sold (Issue 008 T2).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.7055 | 0.000 | 311 | 494 |
| A1 | 0.7024 | 1.000 | 8 | 63 |
| H1 | 0.7024 | 1.000 | 311 | 495 |

H1 decomposition: escalation 1.000 · decision p50 0 µs · total p50 311 µs (reflex 311 µs) · total p99 495 µs.

### Gates

- **G7 product gate (Issue 008 T2):** A1 − A0 mean -0.0031 · paired LB95 -0.0169 → REFUSED — A1 is not sold; A0 serves
- **G1 calibration:** raw ECE 0.0846 · Platt 0.1680 (fit engaged: true) · conformal floor 0.2077 → FAIL
- **G2 fusion overhead:** H1 fusion-only 8 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 16 ns/q) · H2 fusion 4.63 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## s1mb_score

Verdict: **a0_stands** — no specialist artifact (Issue 010 T2).

Posture: cap 16 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.230/0.415 · specialist bag count. Questions: 2571 over 2571 cases. A0/G0-only posture — no specialist artifact (Issue 010 T2).

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.2300 | 0.1854 | 0.000 | 283 |

Served arm: **A0** (rank-0 1/1 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.4967 | 0.000 | 266 | 403 |

### Gates

- **G1 calibration:** raw ECE 0.0064 · Platt 0.2665 (fit engaged: true) · conformal floor 0.4305 → FAIL
- **G2 fusion overhead:** N/A — no hybrid lane exists (A0/G0-only posture); A0's absolute latency is in the table.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

