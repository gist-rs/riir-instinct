# Bench 016 — the Reflex · instinct hybrid GOAT run

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome off — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power Some("AC Power") · powermode Some("high") · load1m Some(4.07) · swap_mb Some(1698.19) · quotable Some(true) · refusals []

## ag_news

Verdict: **a0_stands** — instrument pick H2(β=0.25,nmin=2,τ=2) refused — paired LB95 -0.0100 (Issue 008 T2).

Posture: cap 64 · head 0.00 · nb 4.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.011/0.411 · specialist bag count. Questions: 400 over 400 cases.

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

Served arm: **A0** (rank-0 10/48 candidates; the full table rides `registration.json`). The instrument's pick **H2(β=0.25,nmin=2,τ=2)** was REFUSED by the superiority gate (paired LB95 -0.0100 ≤ 0) — A0 serves and the suite is not sold (Issue 008 T2).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.8825 | 0.000 | 162 | 289 |
| A1 | 0.8875 | 1.000 | 2 | 4 |
| H1 | 0.8950 | 0.490 | 162 | 289 |
| H2(β=0.25,nmin=2,τ=2) ✗ | 0.8975 | 1.000 | 2 | 4 |

H1 decomposition: escalation 0.490 · decision p50 0 µs · total p50 162 µs (reflex 162 µs) · total p99 289 µs.

### Gates

- **G7 product gate (Issue 008 T2):** H2(β=0.25,nmin=2,τ=2) − A0 mean +0.0150 · paired LB95 -0.0100 → REFUSED — H2(β=0.25,nmin=2,τ=2) is not sold; A0 serves
- **G1 calibration:** raw ECE 0.0133 · Platt 0.0324 (fit engaged: true) · conformal floor 0.3066 → FAIL
- **G5 pre-registered (gap suite):** budget face: A1−arm UB95 +0.0020 ≤ δ 0.0175 (quality face missed: LB 0.8639 vs 0.8875) → PASS
- **G2 fusion overhead:** H1 fusion-only 12 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 28 ns/q) · H2 fusion 2.88 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## emotion

Verdict: **a0_stands** — the instrument registered A0 outright.

Posture: cap 64 · head 0.00 · nb 4.00 · ridge 8.00 (bag) · fused-gate thresholds \n0.008/0.427 · specialist bag count. Questions: 400 over 400 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.7750 | 0.7224 | 0.000 | 144 |
| ✓ | A1 | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.6900 | 0.6336 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.6900 | 0.6336 | 1.000 | 2 |

Served arm: **A0** (rank-0 11/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.8850 | 0.000 | 127 | 157 |
| A1 | 0.8550 | 1.000 | 1 | 2 |
| H1 | 0.8775 | 0.463 | 127 | 157 |

H1 decomposition: escalation 0.463 · decision p50 0 µs · total p50 127 µs (reflex 127 µs) · total p99 157 µs.

### Gates

- **G1 calibration:** raw ECE 0.8622 · Platt 0.0000 (fit engaged: true) · conformal floor 0.2780 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean +0.0000 · UB95 +0.0000 vs δ 0.0100 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 20 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 51 ns/q) · H2 fusion 2.34 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## sst5

Verdict: **a0_stands** — instrument pick A1 refused — paired LB95 -0.0129 (Issue 008 T2).

Posture: cap 64 · head 0.00 · nb 16.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.005/0.417 · specialist bag count. Questions: 600 over 600 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.2900 | 0.2408 | 0.000 | 116 |
| ✓ | A1 | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H1 | 0.3000 | 0.2501 | 0.510 | 116 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |

Served arm: **A0** (rank-0 12/48 candidates; the full table rides `registration.json`). The instrument's pick **A1** was REFUSED by the superiority gate (paired LB95 -0.0129 ≤ 0) — A0 serves and the suite is not sold (Issue 008 T2).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.3967 | 0.000 | 97 | 114 |
| A1 | 0.4217 | 1.000 | 1 | 2 |
| H1 | 0.4133 | 0.532 | 97 | 114 |

H1 decomposition: escalation 0.532 · decision p50 0 µs · total p50 97 µs (reflex 97 µs) · total p99 114 µs.

### Gates

- **G7 product gate (Issue 008 T2):** A1 − A0 mean +0.0250 · paired LB95 -0.0129 → REFUSED — A1 is not sold; A0 serves
- **G1 calibration:** raw ECE 0.1008 · Platt 0.0835 (fit engaged: true) · conformal floor 0.2121 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.0250 · UB95 +0.0129 vs δ 0.0392 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 11 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 33 ns/q) · H2 fusion 2.15 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## xnli_en

Verdict: **a0_stands** — the instrument registered A0 outright.

Posture: cap 64 · head 1.00 · nb 16.00 · ridge 0.00 (pair) · fused-gate thresholds \n0.003/0.511 · specialist bag count. Questions: 300 over 300 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5900 | 0.5318 | 0.000 | 109 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.5150 | 0.4570 | 1.000 | 4 |
| ✓ | H2(β=2,nmin=4,τ=2) | 0.5150 | 0.4570 | 1.000 | 4 |
| ✓ | H2(β=2,nmin=8,τ=2) | 0.5150 | 0.4570 | 1.000 | 4 |

Served arm: **A0** (rank-0 4/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.5233 | 0.000 | 91 | 107 |
| A1 | 0.4100 | 1.000 | 1 | 2 |
| H1 | 0.5000 | 0.533 | 91 | 107 |

H1 decomposition: escalation 0.533 · decision p50 0 µs · total p50 91 µs (reflex 91 µs) · total p99 107 µs.

### Gates

- **G1 calibration:** raw ECE 0.5027 · Platt 0.0067 (fit engaged: true) · conformal floor 0.1351 → PASS
- **G5 pre-registered (gap suite):** no hybrid arm registered (A0 stands) — the gate is refused, never a pass → FAIL
- **G2 fusion overhead:** H1 fusion-only 9 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 21 ns/q) · H2 fusion 3.01 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## massive_intent_en

Verdict: **hybrid_arm**.

Posture: cap 48 · head 0.00 · nb 4.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.058/0.526 · specialist bag count. Questions: 300 over 300 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5700 | 0.5117 | 0.000 | 146 |
| ✓ | H1 | 0.6900 | 0.6336 | 0.480 | 146 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.8300 | 0.7813 | 1.000 | 4 |

Served arm: **H2(β=1,nmin=2,τ=8)** (rank-0 3/48 candidates; the full table rides `registration.json`). The superiority gate certified it strictly above Reflex (paired LB95 +0.0124 > 0).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.7800 | 0.000 | 118 | 144 |
| A1 | 0.8167 | 1.000 | 1 | 3 |
| H1 | 0.7900 | 0.313 | 118 | 144 |
| H2(β=1,nmin=2,τ=8) ★ | 0.8267 | 1.000 | 1 | 3 |

H1 decomposition: escalation 0.313 · decision p50 0 µs · total p50 118 µs (reflex 118 µs) · total p99 144 µs.

### Gates

- **G7 product gate (Issue 008 T2):** H2(β=1,nmin=2,τ=8) − A0 mean +0.0467 · paired LB95 +0.0124 → PASS (strictly above Reflex)
- **G1 calibration:** raw ECE 0.0607 · Platt 0.0556 (fit engaged: true) · conformal floor 0.2943 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.0467 · UB95 -0.0124 vs δ 0.0673 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 115 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 156 ns/q) · H2 fusion 1.09 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## banking77

Verdict: **a0_stands** — instrument pick H2(β=2,nmin=8,τ=8) refused — paired LB95 -0.0013 (Issue 008 T2).

Posture: cap 40 · head 1.00 · nb 1.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.021/0.555 · specialist bag presence. Questions: 500 over 500 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.8100 | 0.7597 | 0.000 | 731 |
| ✓ | H1 | 0.8200 | 0.7704 | 0.475 | 731 |
| ✓ | H2(β=2,nmin=8,τ=8) | 0.8450 | 0.7976 | 1.000 | 17 |

Served arm: **A0** (rank-0 3/48 candidates; the full table rides `registration.json`). The instrument's pick **H2(β=2,nmin=8,τ=8)** was REFUSED by the superiority gate (paired LB95 -0.0013 ≤ 0) — A0 serves and the suite is not sold (Issue 008 T2).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.8260 | 0.000 | 411 | 751 |
| A1 | 0.8280 | 1.000 | 6 | 22 |
| H1 | 0.8320 | 0.468 | 411 | 752 |
| H2(β=2,nmin=8,τ=8) ✗ | 0.8540 | 1.000 | 6 | 22 |

H1 decomposition: escalation 0.468 · decision p50 0 µs · total p50 411 µs (reflex 411 µs) · total p99 752 µs.

### Gates

- **G7 product gate (Issue 008 T2):** H2(β=2,nmin=8,τ=8) − A0 mean +0.0280 · paired LB95 -0.0013 → REFUSED — H2(β=2,nmin=8,τ=8) is not sold; A0 serves
- **G1 calibration:** raw ECE 0.0841 · Platt 0.1109 (fit engaged: true) · conformal floor 0.3103 → FAIL
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.0280 · UB95 +0.0013 vs δ 0.0357 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 119 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 166 ns/q) · H2 fusion 0.97 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## typed_decisions

Verdict: **hybrid_arm**.

Posture: cap 48 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.017/0.929 · specialist bag count. Questions: 2000 over 400 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5220 | 0.4852 | 0.000 | 1668 |
| ✓ | A1 | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H1 | 0.5720 | 0.5353 | 0.462 | 1668 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.25,nmin=2,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.25,nmin=2,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.25,nmin=2,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.25,nmin=4,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.25,nmin=4,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.25,nmin=4,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.25,nmin=8,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.25,nmin=8,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.25,nmin=8,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.5,nmin=2,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.5,nmin=4,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.5,nmin=4,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.5,nmin=4,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.5,nmin=8,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.5,nmin=8,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=0.5,nmin=8,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=1,nmin=2,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=1,nmin=4,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=1,nmin=8,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=1,nmin=8,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=2,nmin=2,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=2,nmin=2,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=2,nmin=4,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=2,nmin=4,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=2,nmin=4,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=2,nmin=8,τ=2) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=2,nmin=8,τ=4) | 0.6140 | 0.5776 | 1.000 | 10 |
| ✓ | H2(β=2,nmin=8,τ=8) | 0.6140 | 0.5776 | 1.000 | 10 |

Served arm: **A1** (rank-0 48/48 candidates; the full table rides `registration.json`). The superiority gate certified it strictly above Reflex (paired LB95 +0.1391 > 0).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.4655 | 0.000 | 520 | 1732 |
| A1 | 0.6300 | 1.000 | 3 | 12 |
| H1 | 0.5940 | 0.602 | 520 | 1733 |

H1 decomposition: escalation 0.602 · decision p50 0 µs · total p50 520 µs (reflex 520 µs) · total p99 1733 µs.

### Gates

- **G7 product gate (Issue 008 T2):** A1 − A0 mean +0.1645 · paired LB95 +0.1391 → PASS (strictly above Reflex)
- **G1 calibration:** raw ECE 0.4345 · Platt 0.0160 (fit engaged: true) · conformal floor 0.2070 → PASS
- **G2 fusion overhead:** H1 fusion-only 11 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 26 ns/q) · H2 fusion 2.63 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## prompt_injections

Verdict: **hybrid_arm**.

Posture: cap 64 · head 0.00 · nb 1.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.067/0.462 · specialist bag count. Questions: 116 over 116 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.8000 | 0.7254 | 0.000 | 92 |
| ✓ | A1 | 0.9600 | 0.9117 | 1.000 | 3 |
| ✓ | H1 | 0.8800 | 0.8146 | 0.510 | 92 |
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

Served arm: **A1** (rank-0 48/48 candidates; the full table rides `registration.json`). The superiority gate certified it strictly above Reflex (paired LB95 +0.0082 > 0).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.7672 | 0.000 | 74 | 85 |
| A1 | 0.8534 | 1.000 | 0 | 3 |
| H1 | 0.8448 | 0.681 | 74 | 85 |

H1 decomposition: escalation 0.681 · decision p50 0 µs · total p50 74 µs (reflex 74 µs) · total p99 85 µs.

### Gates

- **G7 product gate (Issue 008 T2):** A1 − A0 mean +0.0862 · paired LB95 +0.0082 → PASS (strictly above Reflex)
- **G1 calibration:** raw ECE 0.0824 · Platt 0.1017 (fit engaged: true) · conformal floor 0.3769 → FAIL
- **G2 fusion overhead:** H1 fusion-only 7 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 13 ns/q) · H2 fusion 4.25 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## harness_visibility

Verdict: **a0_stands** — no specialist artifact (Issue 010 T2).

Posture: cap 18446744073709551615 · head 0.00 · nb 4.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.034/0.710 · specialist bag count. Questions: 16 over 16 cases. A0/G0-only posture — no specialist artifact (Issue 010 T2).

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.7500 | 0.5630 | 0.000 | 16 |

Served arm: **A0** (rank-0 1/1 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.5625 | 0.000 | 10 | 12 |

### Gates

- **G1 calibration:** raw ECE 0.4815 · Platt 0.4815 (fit engaged: false) · conformal floor 0.1190 → FAIL
- **G2 fusion overhead:** N/A — no hybrid lane exists (A0/G0-only posture); A0's absolute latency is in the table.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## harness_permissions

Verdict: **a0_stands** — no specialist artifact (Issue 010 T2).

Posture: cap 18446744073709551615 · head 0.00 · nb 1.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.034/0.384 · specialist bag count. Questions: 12 over 12 cases. A0/G0-only posture — no specialist artifact (Issue 010 T2).

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.7778 | 0.5809 | 0.000 | 12 |

Served arm: **A0** (rank-0 1/1 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.6667 | 0.000 | 7 | 8 |

### Gates

- **G1 calibration:** raw ECE 0.6058 · Platt 0.6058 (fit engaged: false) · conformal floor 0.2544 → FAIL
- **G2 fusion overhead:** N/A — no hybrid lane exists (A0/G0-only posture); A0's absolute latency is in the table.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## harness_tool_fit

Verdict: **a0_stands** — no specialist artifact (Issue 010 T2).

Posture: cap 18446744073709551615 · head 0.00 · nb 32.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.037/0.178 · specialist bag count. Questions: 12 over 12 cases. A0/G0-only posture — no specialist artifact (Issue 010 T2).

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.7778 | 0.5809 | 0.000 | 13 |

Served arm: **A0** (rank-0 1/1 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.9167 | 0.000 | 9 | 9 |

### Gates

- **G1 calibration:** raw ECE 0.5808 · Platt 0.5808 (fit engaged: false) · conformal floor 0.2018 → FAIL
- **G2 fusion overhead:** N/A — no hybrid lane exists (A0/G0-only posture); A0's absolute latency is in the table.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## harness_routing

Verdict: **a0_stands** — no specialist artifact (Issue 010 T2).

Posture: cap 18446744073709551615 · head 0.00 · nb 1.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.005/0.469 · specialist bag count. Questions: 16 over 16 cases. A0/G0-only posture — no specialist artifact (Issue 010 T2).

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.8125 | 0.6044 | 0.000 | 14 |

Served arm: **A0** (rank-0 1/1 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.7500 | 0.000 | 9 | 11 |

### Gates

- **G1 calibration:** raw ECE 0.7216 · Platt 0.7216 (fit engaged: false) · conformal floor 0.1728 → FAIL
- **G2 fusion overhead:** N/A — no hybrid lane exists (A0/G0-only posture); A0's absolute latency is in the table.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## harness_sensitivity

Verdict: **a0_stands** — no specialist artifact (Issue 010 T2).

Posture: cap 18446744073709551615 · head 0.00 · nb 1.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.022/0.417 · specialist bag count. Questions: 15 over 15 cases. A0/G0-only posture — no specialist artifact (Issue 010 T2).

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.8000 | 0.6156 | 0.000 | 13 |

Served arm: **A0** (rank-0 1/1 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.8000 | 0.000 | 9 | 10 |

### Gates

- **G1 calibration:** raw ECE 0.7627 · Platt 0.7627 (fit engaged: false) · conformal floor 0.3397 → FAIL
- **G2 fusion overhead:** N/A — no hybrid lane exists (A0/G0-only posture); A0's absolute latency is in the table.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## harness_cache_reuse

Verdict: **a0_stands** — no specialist artifact (Issue 010 T2).

Posture: cap 18446744073709551615 · head 0.00 · nb 1.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.040/0.965 · specialist bag count. Questions: 12 over 12 cases. A0/G0-only posture — no specialist artifact (Issue 010 T2).

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.7500 | 0.5630 | 0.000 | 12 |

Served arm: **A0** (rank-0 1/1 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.9167 | 0.000 | 9 | 11 |

### Gates

- **G1 calibration:** raw ECE 0.7555 · Platt 0.7555 (fit engaged: false) · conformal floor 0.0635 → FAIL
- **G2 fusion overhead:** N/A — no hybrid lane exists (A0/G0-only posture); A0's absolute latency is in the table.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## code_fixtures

Verdict: **a0_stands** — no specialist artifact (Issue 010 T2).

Posture: cap 18446744073709551615 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.000/0.337 · specialist bag count. Questions: 32 over 16 cases. A0/G0-only posture — no specialist artifact (Issue 010 T2).

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.4688 | 0.3977 | 0.000 | 379 |

Served arm: **A0** (rank-0 1/1 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.3750 | 0.000 | 190 | 389 |

### Gates

- **G1 calibration:** raw ECE 0.3742 · Platt 0.0938 (fit engaged: true) · conformal floor 0.3510 → PASS
- **G2 fusion overhead:** N/A — no hybrid lane exists (A0/G0-only posture); A0's absolute latency is in the table.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

