# Bench 002 — the Reflex · instinct hybrid GOAT run

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the Bench 051 protocol (`--head-select --nb-select`, registry caps), fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 xnli_en accuracy equals reflex's own `run()` row byte for byte. H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. The v1 read scored the label permutation by position and compared label-space picks against position-space gold — invisible wherever the presented set is the full universe, chance-level on massive (20 of 59 + shuffle); its registration also double-indexed `rank0_sorted[select_arm(..)]` (select_arm already returns the candidate index).Box state: power Some("AC Power") · powermode Some("high") · load1m Some(2.96) · swap_mb Some(2390.75) · quotable Some(true) · refusals []

## ag_news

Posture: cap 64 · head 0.00 · nb 4.00 (bag) · fused-gate thresholds 0.011/0.411. Questions: 400.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.8550 | 0.8085 | 0.000 | 277 |
| ✓ | H2(β=0.25,nmin=2,τ=2) | 0.8700 | 0.8251 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=2,τ=4) | 0.8700 | 0.8251 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=2,τ=8) | 0.8700 | 0.8251 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=4,τ=2) | 0.8700 | 0.8251 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=4,τ=4) | 0.8700 | 0.8251 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=4,τ=8) | 0.8700 | 0.8251 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=8,τ=2) | 0.8700 | 0.8251 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=8,τ=4) | 0.8700 | 0.8251 | 1.000 | 3 |
| ✓ | H2(β=0.25,nmin=8,τ=8) | 0.8700 | 0.8251 | 1.000 | 3 |

Registered arm: **H2(β=0.25,nmin=2,τ=2)** (rank-0 10/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.8825 | 0.000 | 153 | 299 |
| A1 | 0.8875 | 1.000 | 2 | 4 |
| H1 | 0.8950 | 0.490 | 153 | 299 |
| H2(β=0.25,nmin=2,τ=2) ★ | 0.8975 | 1.000 | 2 | 4 |

H1 decomposition: escalation 0.490 · decision p50 0 µs · total p50 153 µs (reflex 153 µs) · total p99 299 µs.

### Gates

- **G1 calibration:** raw ECE 0.0133 · Platt 0.0324 (fit engaged: true) · conformal floor 0.3066 → FAIL
- **G5 pre-registered (gap suite):** budget face: A1−arm UB95 +0.0020 ≤ δ 0.0175 (quality face missed: LB 0.8639 vs 0.8875) → PASS
- **G2 fusion overhead:** H1 fusion-only 12 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 24 ns/q) · H2 fusion 2.55 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (196 escalated):** lane p50 0 µs vs laya p50 187498 µs · lane−laya UB95 -187181 µs ≤ 0 → PASS
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## emotion

Posture: cap 64 · head 0.00 · nb 4.00 (bag) · fused-gate thresholds 0.004/0.427. Questions: 400.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5100 | 0.4521 | 0.000 | 126 |
| ✓ | A1 | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H1 | 0.6350 | 0.5773 | 0.545 | 126 |
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
| A0 | 0.7375 | 0.000 | 119 | 153 |
| A1 | 0.8550 | 1.000 | 1 | 2 |
| H1 | 0.7975 | 0.430 | 119 | 153 |

H1 decomposition: escalation 0.430 · decision p50 0 µs · total p50 119 µs (reflex 119 µs) · total p99 153 µs.

### Gates

- **G1 calibration:** raw ECE 0.1777 · Platt 0.0796 (fit engaged: true) · conformal floor 0.2276 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.1175 · UB95 -0.0790 vs δ 0.0548 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 16 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 45 ns/q) · H2 fusion 1.84 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (172 escalated):** lane p50 0 µs vs laya p50 117974 µs · lane−laya UB95 -120267 µs ≤ 0 → PASS
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## sst5

Posture: cap 64 · head 0.00 · nb 16.00 (bag) · fused-gate thresholds 0.005/0.417. Questions: 600.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.2900 | 0.2408 | 0.000 | 106 |
| ✓ | A1 | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H1 | 0.3000 | 0.2501 | 0.510 | 106 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.3400 | 0.2877 | 1.000 | 1 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.3400 | 0.2877 | 1.000 | 1 |

Registered arm: **A1** (rank-0 12/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.3967 | 0.000 | 98 | 128 |
| A1 | 0.4217 | 1.000 | 1 | 1 |
| H1 | 0.4133 | 0.532 | 98 | 128 |

H1 decomposition: escalation 0.532 · decision p50 0 µs · total p50 98 µs (reflex 98 µs) · total p99 128 µs.

### Gates

- **G1 calibration:** raw ECE 0.1008 · Platt 0.0835 (fit engaged: true) · conformal floor 0.2121 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.0250 · UB95 +0.0129 vs δ 0.0392 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 12 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 35 ns/q) · H2 fusion 2.19 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (319 escalated):** lane p50 0 µs vs laya p50 151163 µs · lane−laya UB95 -150576 µs ≤ 0 → PASS
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## xnli_en

Posture: cap 64 · head 1.00 · nb 16.00 (pair) · fused-gate thresholds 0.003/0.511. Questions: 300.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5900 | 0.5318 | 0.000 | 110 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.5150 | 0.4570 | 1.000 | 3 |
| ✓ | H2(β=2,nmin=4,τ=2) | 0.5150 | 0.4570 | 1.000 | 3 |
| ✓ | H2(β=2,nmin=8,τ=2) | 0.5150 | 0.4570 | 1.000 | 3 |

Registered arm: **A0** (rank-0 4/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.5233 | 0.000 | 90 | 109 |
| A1 | 0.4100 | 1.000 | 1 | 2 |
| H1 | 0.5000 | 0.533 | 90 | 109 |

H1 decomposition: escalation 0.533 · decision p50 0 µs · total p50 90 µs (reflex 90 µs) · total p99 109 µs.

### Gates

- **G1 calibration:** raw ECE 0.5027 · Platt 0.0067 (fit engaged: true) · conformal floor 0.1351 → PASS
- **G5 pre-registered (gap suite):** no hybrid arm registered (A0 stands) — the gate is refused, never a pass → FAIL
- **G2 fusion overhead:** H1 fusion-only 9 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 20 ns/q) · H2 fusion 3.00 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (160 escalated):** lane p50 0 µs vs laya p50 186120 µs · lane−laya UB95 -185346 µs ≤ 0 → PASS
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## massive_intent_en

Posture: cap 48 · head 0.00 · nb 4.00 (bag) · fused-gate thresholds 0.058/0.526. Questions: 300.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5700 | 0.5117 | 0.000 | 131 |
| ✓ | H1 | 0.6900 | 0.6336 | 0.480 | 131 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.8300 | 0.7813 | 1.000 | 3 |

Registered arm: **H2(β=1,nmin=2,τ=8)** (rank-0 3/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.7800 | 0.000 | 112 | 135 |
| A1 | 0.8167 | 1.000 | 1 | 2 |
| H1 | 0.7900 | 0.313 | 112 | 135 |
| H2(β=1,nmin=2,τ=8) ★ | 0.8267 | 1.000 | 1 | 2 |

H1 decomposition: escalation 0.313 · decision p50 0 µs · total p50 112 µs (reflex 112 µs) · total p99 135 µs.

### Gates

- **G1 calibration:** raw ECE 0.0607 · Platt 0.0556 (fit engaged: true) · conformal floor 0.2943 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.0467 · UB95 -0.0124 vs δ 0.0673 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 99 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 129 ns/q) · H2 fusion 0.90 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (94 escalated):** lane p50 0 µs vs laya p50 323139 µs · lane−laya UB95 -328032 µs ≤ 0 → PASS
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## banking77

Posture: cap 40 · head 1.00 · nb 1.00 (bag) · fused-gate thresholds 0.021/0.555. Questions: 500.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.8100 | 0.7597 | 0.000 | 706 |
| ✓ | H1 | 0.8200 | 0.7704 | 0.475 | 706 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.8150 | 0.7651 | 1.000 | 16 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.8150 | 0.7651 | 1.000 | 16 |
| ✓ | H2(β=2,nmin=2,τ=8) | 0.8150 | 0.7651 | 1.000 | 16 |
| ✓ | H2(β=2,nmin=8,τ=4) | 0.8150 | 0.7651 | 1.000 | 16 |

Registered arm: **H1** (rank-0 6/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.8260 | 0.000 | 379 | 686 |
| A1 | 0.7960 | 1.000 | 6 | 20 |
| H1 | 0.8060 | 0.468 | 380 | 687 |

H1 decomposition: escalation 0.468 · decision p50 0 µs · total p50 380 µs (reflex 379 µs) · total p99 687 µs.

### Gates

- **G1 calibration:** raw ECE 0.7059 · Platt 0.0060 (fit engaged: true) · conformal floor 0.3603 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean +0.0200 · UB95 +0.0466 vs δ 0.0320 (from the cal discordant rate) → FAIL
- **G2 fusion overhead:** H1 fusion-only 117 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 149 ns/q) · H2 fusion 0.92 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (234 escalated):** lane p50 1 µs vs laya p50 650295 µs · lane−laya UB95 -651805 µs ≤ 0 → PASS
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

