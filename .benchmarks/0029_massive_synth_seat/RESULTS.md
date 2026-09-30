# Bench 0029 — the Reflex · instinct hybrid GOAT run

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome off — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power None · powermode None · load1m None · swap_mb None · quotable None · refusals ["power source unreadable — UNJUDGED"]

## massive_intent_en

Verdict: **a0_stands** — instrument pick H2(β=1,nmin=2,τ=4) refused — paired LB95 -0.0039 (Issue 008 T2).

Posture: cap 48 · head 0.00 · nb 4.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.201/0.526 · specialist bag count. Questions: 300 over 300 cases.

Synth corpus seated: rows=2048 extra_cap=128 digest=8eed806a1bde8467 (Plan 426 T5/T6 — blake3-verified at seat \
load; the posture is gold-fit, the corpus is the only difference).

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5900 | 0.5318 | 0.000 | 141 |
| ✓ | H1 | 0.7000 | 0.6439 | 0.915 | 142 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.8400 | 0.7921 | 1.000 | 4 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.8400 | 0.7921 | 1.000 | 4 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.8400 | 0.7921 | 1.000 | 4 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.8400 | 0.7921 | 1.000 | 4 |

Served arm: **A0** (rank-0 6/48 candidates; the full table rides `registration.json`). The instrument's pick **H2(β=1,nmin=2,τ=4)** was REFUSED by the superiority gate (paired LB95 -0.0039 ≤ 0) — A0 serves and the suite is not sold (Issue 008 T2).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.8133 | 0.000 | 91 | 166 |
| A1 | 0.8167 | 1.000 | 1 | 2 |
| H1 | 0.8333 | 0.877 | 91 | 166 |
| H2(β=1,nmin=2,τ=4) ✗ | 0.8400 | 1.000 | 1 | 2 |

H1 decomposition: escalation 0.877 · decision p50 0 µs · total p50 91 µs (reflex 91 µs) · total p99 166 µs.

### Gates

- **G7 product gate (Issue 008 T2):** H2(β=1,nmin=2,τ=4) − A0 mean +0.0267 · paired LB95 -0.0039 → REFUSED — H2(β=1,nmin=2,τ=4) is not sold; A0 serves
- **G1 calibration:** raw ECE 0.0687 · Platt 0.0595 (fit engaged: true) · conformal floor 0.2995 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.0267 · UB95 +0.0039 vs δ 0.0661 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 95 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 125 ns/q) · H2 fusion 1.26 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (263 escalated):** lane p50 0 µs vs laya p50 457784 µs · lane−laya UB95 -465462 µs ≤ 0 → PASS
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

