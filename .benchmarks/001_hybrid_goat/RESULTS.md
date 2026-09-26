# Bench 001 — the Reflex · instinct hybrid GOAT run

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the Bench 051 protocol (`--head-select --nb-select`, registry caps), fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 xnli_en accuracy equals reflex's own `run()` row byte for byte. H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. The v1 read scored the label permutation by position and compared label-space picks against position-space gold — invisible wherever the presented set is the full universe, chance-level on massive (20 of 59 + shuffle); its registration also double-indexed `rank0_sorted[select_arm(..)]` (select_arm already returns the candidate index).

Box state: power Some("AC Power") · powermode Some("high") · load1m Some(4.27) · swap_mb Some(2430.75) · quotable Some(true) · refusals []

## ag_news

Posture: cap 64 · head 0.00 · nb 16.00 (bag) · fused-gate thresholds 0.022/0.411. Questions: 400.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.8100 | 0.7597 | 0.000 | 258 |
| ✓ | H1 | 0.8400 | 0.7921 | 0.475 | 258 |
| ✓ | H2(β=0.25,nmin=2,τ=2) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=2,τ=4) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=2,τ=8) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=4,τ=2) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=4,τ=4) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=4,τ=8) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=8,τ=2) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=8,τ=4) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=8,τ=8) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=2,τ=2) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=4,τ=2) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=8,τ=4) | 0.8600 | 0.8140 | 1.000 | 3 |
| ✓ | H2(β=1,nmin=8,τ=8) | 0.8600 | 0.8140 | 1.000 | 3 |

Registered arm: **H2(β=0.25,nmin=2,τ=2)** (rank-0 20/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.8625 | 0.000 | 153 | 258 |
| A1 | 0.8875 | 1.000 | 2 | 4 |
| H1 | 0.8875 | 0.525 | 153 | 258 |
| H2(β=0.25,nmin=2,τ=2) ★ | 0.9000 | 1.000 | 2 | 4 |

H1 decomposition: escalation 0.525 · decision p50 0 µs · total p50 153 µs (reflex 153 µs) · total p99 258 µs.

### Gates

- **G1 calibration:** raw ECE 0.0251 · Platt 0.0197 (fit engaged: true) · conformal floor 0.3121 → PASS
- **G5 pre-registered (gap suite):** budget face: A1−arm UB95 -0.0016 ≤ δ 0.0175 (quality face missed: LB 0.8667 vs 0.8875) → PASS
- **G2 fusion overhead:** H1 fusion-only 11 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 27 ns/q) · H2 fusion 2.27 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (210 escalated):** lane p50 0 µs vs laya p50 187112 µs · lane−laya UB95 -185832 µs ≤ 0 → PASS
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## emotion

Posture: cap 64 · head 0.00 · nb 4.00 (bag) · fused-gate thresholds 0.006/0.427. Questions: 400.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.3750 | 0.3210 | 0.000 | 126 |
| ✓ | A1 | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H1 | 0.5350 | 0.4769 | 0.510 | 126 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.6900 | 0.6336 | 1.000 | 2 |

Registered arm: **A1** (rank-0 12/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.5750 | 0.000 | 115 | 127 |
| A1 | 0.8550 | 1.000 | 1 | 2 |
| H1 | 0.7325 | 0.468 | 115 | 127 |

H1 decomposition: escalation 0.468 · decision p50 0 µs · total p50 115 µs (reflex 115 µs) · total p99 127 µs.

### Gates

- **G1 calibration:** raw ECE 0.1777 · Platt 0.0796 (fit engaged: true) · conformal floor 0.2276 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.2800 · UB95 -0.2328 vs δ 0.0594 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 20 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 44 ns/q) · H2 fusion 1.78 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (187 escalated):** lane p50 0 µs vs laya p50 118560 µs · lane−laya UB95 -121018 µs ≤ 0 → PASS
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## sst5

Posture: cap 64 · head 0.00 · nb 0.00 (bag) · fused-gate thresholds 0.000/0.417. Questions: 600.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.2450 | 0.1991 | 0.000 | 123 |
| ✓ | A1 | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H1 | 0.2750 | 0.2268 | 0.520 | 123 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.25,nmin=2,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.25,nmin=2,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.25,nmin=2,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.25,nmin=4,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.25,nmin=4,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.25,nmin=4,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.25,nmin=8,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.25,nmin=8,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.25,nmin=8,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.5,nmin=2,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.5,nmin=4,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.5,nmin=4,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.5,nmin=4,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.5,nmin=8,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.5,nmin=8,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0.5,nmin=8,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=1,nmin=2,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=1,nmin=4,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=1,nmin=8,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=1,nmin=8,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=2,nmin=2,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=2,nmin=2,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=2,nmin=4,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=2,nmin=4,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=2,nmin=4,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=2,nmin=8,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=2,nmin=8,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=2,nmin=8,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |

Registered arm: **A1** (rank-0 48/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.2017 | 0.000 | 96 | 125 |
| A1 | 0.4217 | 1.000 | 1 | 2 |
| H1 | 0.3317 | 0.582 | 96 | 125 |

H1 decomposition: escalation 0.582 · decision p50 0 µs · total p50 96 µs (reflex 96 µs) · total p99 125 µs.

### Gates

- **G1 calibration:** raw ECE 0.1008 · Platt 0.0835 (fit engaged: true) · conformal floor 0.2121 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.2200 · UB95 -0.1686 vs δ 0.0501 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 18 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 35 ns/q) · H2 fusion 1.93 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (349 escalated):** lane p50 0 µs vs laya p50 151347 µs · lane−laya UB95 -150036 µs ≤ 0 → PASS
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## xnli_en

Posture: cap 64 · head 1.00 · nb 4.00 (pair) · fused-gate thresholds 0.002/0.511. Questions: 300.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5250 | 0.4669 | 0.000 | 103 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.4850 | 0.4275 | 1.000 | 4 |
| ✓ | H2(β=2,nmin=2,τ=4) | 0.4850 | 0.4275 | 1.000 | 4 |
| ✓ | H2(β=2,nmin=4,τ=2) | 0.4850 | 0.4275 | 1.000 | 4 |
| ✓ | H2(β=2,nmin=4,τ=4) | 0.4850 | 0.4275 | 1.000 | 4 |

Registered arm: **A0** (rank-0 5/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.5167 | 0.000 | 89 | 98 |
| A1 | 0.4100 | 1.000 | 1 | 2 |
| H1 | 0.5167 | 0.537 | 89 | 98 |

H1 decomposition: escalation 0.537 · decision p50 0 µs · total p50 89 µs (reflex 89 µs) · total p99 98 µs.

### Gates

- **G1 calibration:** raw ECE 0.5070 · Platt 0.0084 (fit engaged: true) · conformal floor 0.1452 → PASS
- **G5 pre-registered (gap suite):** no hybrid arm registered (A0 stands) — the gate is refused, never a pass → FAIL
- **G2 fusion overhead:** H1 fusion-only 9 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 22 ns/q) · H2 fusion 2.80 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (161 escalated):** lane p50 0 µs vs laya p50 178833 µs · lane−laya UB95 -179619 µs ≤ 0 → PASS
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## massive_intent_en

Posture: cap 48 · head 1.00 · nb 1.00 (bag) · fused-gate thresholds 0.081/0.628. Questions: 300.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.6900 | 0.6336 | 0.000 | 120 |
| ✓ | H1 | 0.7450 | 0.6907 | 0.470 | 120 |
| ✓ | H2(β=0.25,nmin=4,τ=4) | 0.8500 | 0.8030 | 1.000 | 2 |

Registered arm: **H2(β=0.25,nmin=4,τ=4)** (rank-0 3/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.4200 | 0.000 | 92 | 114 |
| A1 | 0.8167 | 1.000 | 1 | 2 |
| H1 | 0.4800 | 0.627 | 92 | 114 |
| H2(β=0.25,nmin=4,τ=4) ★ | 0.8300 | 1.000 | 1 | 2 |

H1 decomposition: escalation 0.627 · decision p50 0 µs · total p50 92 µs (reflex 92 µs) · total p99 114 µs.

### Gates

- **G1 calibration:** raw ECE 0.1043 · Platt 0.0616 (fit engaged: true) · conformal floor 0.3337 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.4100 · UB95 -0.3484 vs δ 0.0616 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 138 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 165 ns/q) · H2 fusion 0.90 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (188 escalated):** lane p50 0 µs vs laya p50 324914 µs · lane−laya UB95 -328746 µs ≤ 0 → PASS
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## banking77

Posture: cap 40 · head 1.00 · nb 1.00 (bag) · fused-gate thresholds 0.056/0.493. Questions: 500.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.8550 | 0.8085 | 0.000 | 304 |
| ✓ | A1 | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H1 | 0.9150 | 0.8758 | 0.480 | 304 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=0.5,nmin=4,τ=2) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=0.5,nmin=4,τ=4) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=0.5,nmin=8,τ=2) | 0.9500 | 0.9171 | 1.000 | 7 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.9500 | 0.9171 | 1.000 | 7 |

Registered arm: **A1** (rank-0 18/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.3960 | 0.000 | 349 | 671 |
| A1 | 0.7960 | 1.000 | 5 | 20 |
| H1 | 0.4760 | 1.000 | 350 | 671 |

H1 decomposition: escalation 1.000 · decision p50 1 µs · total p50 350 µs (reflex 349 µs) · total p99 671 µs.

### Gates

- **G1 calibration:** raw ECE 0.4084 · Platt 0.1532 (fit engaged: true) · conformal floor 0.3966 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.4000 · UB95 -0.3456 vs δ 0.0370 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 176 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 211 ns/q) · H2 fusion 0.92 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (500 escalated):** lane p50 1 µs vs laya p50 650245 µs · lane−laya UB95 -648727 µs ≤ 0 → PASS
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

