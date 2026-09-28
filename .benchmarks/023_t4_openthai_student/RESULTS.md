# Bench 023 — the T4 openthai-student arena read (V4): **FAIL vs the surpass bar**

**Status:** MEASURED 2026-09-29 — plan 426 T4 / V4 (riir-train); the pre-registered surpass bar REFUSED the student. The seat is UNCHANGED (the laya-era artifact stays seated; its file restored and blake3-verified `7bc3ee38…` == the served manifest digest).

**What this run measured:** the winner artifact the 609 recipe exported from the openthai-teacher dump (riir-train `scripts/t4_after_dump.sh`, teacher acc 0.9548 over the full 11514-row train split, digest `49b16a89…`). The distill arms LOST to plain gold on the train-side holdout (A 0.6600 · B@0 0.6450 · B@0.5 0.6550) → **WINNER A (gold)** — the strong teacher's soft targets underperform one-hot gold in this linear head (the 613 class, now measured with a STRONG teacher too). The exported winner (blake3 `a625624ad5263cb2…`, preserved at `riir-train/data/instinct_specialists/massive_intent_en_openthai_goldA_v1.bin`) is what the grid below scored.

**The V4 read:** A1 0.8033 · best arm H2(β=2,nmin=2,τ=2) **0.8067** (T2-certified vs A0, LB95 +0.0007) — **BELOW the served laya-era H2 0.8267** → V4 FAIL; no seating, no manifest change, no republish. The plan's fallback answer to "does the strongest teacher beat the served arm through the student?" is **NO on the frozen read**. (Note: the old artifact's best arm was H2(β=1,nmin=2,τ=8) — the winning posture moved with the artifact; per-artifact grids are not posture-comparable, only best-vs-best is.) The run rode `--skip-pin-a0` — the site-row half of the A0 pin reds on the documented owner-gated pool divergence (the site's modelless massive row was republished from reflex HEAD on the canonical pool, 0.4067 vs the t20k seat's 0.7800); the A0 == reflex `run()` half held exactly (0.7800).

---

# Bench 021 — the Reflex · instinct hybrid GOAT run (the arena's auto-written record, renumbered 021→023: the number was already held by `021_tetris_round4_playoff`, which landed without a highwater bump — the stale-0022 root cause is recorded in the repair note)

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome off — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power Some("AC Power") · powermode Some("high") · load1m Some(12.4) · swap_mb Some(404.06) · quotable Some(false) · refusals ["load 12.40 > 6 — a sibling job is on the box"]

## massive_intent_en

Verdict: **hybrid_arm**.

Posture: cap 48 · head 0.00 · nb 4.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.058/0.526 · specialist bag count. Questions: 300 over 300 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5700 | 0.5117 | 0.000 | 278 |
| ✓ | H1 | 0.6700 | 0.6130 | 0.480 | 279 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.7950 | 0.7436 | 1.000 | 4 |
| ✓ | H2(β=2,nmin=2,τ=4) | 0.7950 | 0.7436 | 1.000 | 4 |

Served arm: **H2(β=2,nmin=2,τ=2)** (rank-0 4/48 candidates; the full table rides `registration.json`). The superiority gate certified it strictly above Reflex (paired LB95 +0.0007 > 0).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.7800 | 0.000 | 169 | 416 |
| A1 | 0.8033 | 1.000 | 2 | 6 |
| H1 | 0.7833 | 0.313 | 169 | 416 |
| H2(β=2,nmin=2,τ=2) ★ | 0.8067 | 1.000 | 2 | 6 |

H1 decomposition: escalation 0.313 · decision p50 0 µs · total p50 169 µs (reflex 169 µs) · total p99 416 µs.

### Gates

- **G7 product gate (Issue 008 T2):** H2(β=2,nmin=2,τ=2) − A0 mean +0.0267 · paired LB95 +0.0007 → PASS (strictly above Reflex)
- **G1 calibration:** raw ECE 0.1067 · Platt 0.1267 (fit engaged: true) · conformal floor 0.2541 → FAIL
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.0267 · UB95 -0.0007 vs δ 0.0637 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 167 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 232 ns/q) · H2 fusion 1.57 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

