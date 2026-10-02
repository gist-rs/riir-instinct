# Bench record (banking77_f16) — the Reflex · instinct hybrid GOAT run

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome off — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power Some("AC Power") · powermode Some("high") · load1m Some(7.53) · swap_mb Some(2499.25) · quotable Some(false) · refusals ["load 7.53 > 6 — a sibling job is on the box"]

## banking77

Verdict: **a0_stands** — instrument pick H2(β=2,nmin=8,τ=8) refused — paired LB95 -0.0013 (Issue 008 T2).

Posture: cap 40 · head 1.00 · nb 1.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.749/0.555 · specialist bag presence. Questions: 500 over 500 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.8100 | 0.7597 | 0.000 | 741 |
| ✓ | H2(β=2,nmin=8,τ=8) | 0.8450 | 0.7976 | 1.000 | 28 |

Served arm: **A0** (rank-0 2/48 candidates; the full table rides `registration.json`). The instrument's pick **H2(β=2,nmin=8,τ=8)** was REFUSED by the superiority gate (paired LB95 -0.0013 ≤ 0) — A0 serves and the suite is not sold (Issue 008 T2).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.8260 | 0.000 | 409 | 750 |
| A1 | 0.8280 | 1.000 | 8 | 28 |
| H1 | 0.8380 | 1.000 | 410 | 751 |
| H2(β=2,nmin=8,τ=8) ✗ | 0.8540 | 1.000 | 8 | 28 |

H1 decomposition: escalation 1.000 · decision p50 1 µs · total p50 410 µs (reflex 409 µs) · total p99 751 µs.

### Gates

- **G7 product gate (Issue 008 T2):** H2(β=2,nmin=8,τ=8) − A0 mean +0.0280 · paired LB95 -0.0013 → REFUSED — H2(β=2,nmin=8,τ=8) is not sold; A0 serves
- **G1 calibration:** raw ECE 0.0841 · Platt 0.0347 (fit engaged: true) · conformal floor 0.3103 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.0280 · UB95 +0.0013 vs δ 0.0357 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 129 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 177 ns/q) · H2 fusion 0.99 ns/option (bar < 100) · absolute p99 in the table
- **G2 laya paired (500 escalated):** lane p50 1 µs vs laya p50 45276 µs · lane−laya UB95 -46309 µs ≤ 0 → PASS
- **C1 encoder arm (RECORD-ONLY — serve: ✗, the encoder class is refused at serve; issue 014 decision 1):** NLEH v1 head `../riir-train/.raw/t608/banking77_encoder_v1.bin` (77 classes · feat 79872) over the live laya-english encode (device metal) · accuracy **0.4420** (500 rows) · vs the incumbent A1: paired mean -0.3860 · LB95 -0.4363 (not certified above A1) · per-row p50 55110 µs / p99 74266 µs (the full arm cost per QUESTION row: encode + head; the provisional text-lane bar is ≤ ~300 µs — the arm reads ms-class, the recorded ground of the refusal). No serve change: A1 keeps serving.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

