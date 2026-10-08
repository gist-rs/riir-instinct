# Bench 0059 — the seat re-baseline at the issue-079 published posture (head retired + per_byte armed)

**Status:** COMPLETE — the A0 identity pin **GREEN on all nine suites, both halves** (arena == reflex run() == the PUBLISHED site row) for the first time since the 10-01 site re-basis: banking77 **0.842000** (head-off), typed_decisions **0.570000** (per_byte), every other suite digit-exact. The served cells digit-hold — typed H2 0.6475 (T2 LB95 **+0.0598 PASS**), massive H2 0.8267 (PASS), banking77 H2 0.8540 (T2 refused, −0.0163 — the row's standing "T2 uncertified" annotation stays true), prompt A1 0.8534 (PASS), code_fixtures A1 0.5625 (PASS) — `arsenal.toml` NOT touched, the serve parity gates green without a re-freeze (registered == served holds).

## Why this bench exists (two posture drifts, both caught by the pin)

Bench 0058 was a typed-only focused read; this is the first FULL-pin run since the
10-01 site re-basis, and it caught two drifts the focused runs could not see:

1. **Head-select retirement.** The site's rows are generated head-off since the
   10-01 full-pool re-basis (reflex issue 058, site `d750175` — `head_selection`
   null on every published row since); the arena/server knobs still carried
   `head_select: true` from the Issue-008-T1 era (bench 004, 2026-09-27, when the
   site matched at 0.8260 head@1). Outcome-load-bearing ONLY on banking77
   (0.8260 head@1 vs the published 0.842); inert on every other suite (xnli's
   head@1 read is outcome-identical to head-off). First run's pin:
   `banking77: run() 0.826000 != PUBLISHED site row 0.842000 — re-baseline the
   knobs`. Repair: `head_select: false` in the arena seat knobs, the server
   boot, and the pin's own `RunOptions`.
2. **The pin's RunOptions lacked the drafter lever.** The seat knob threading
   (reflex `PostureKnobs.drafter_fix`) fixed `fit_posture`, but the pin's own
   `reflex run()` invocation still hardcoded `DrafterFix::Off` — first run's
   pin: `typed_decisions: reflex run() 0.563000 != arena A0 0.570000 — the seat
   path diverged`. Repair: the pin's RunOptions arms `PerByte` too.

The reflex-side surface: `PostureKnobs.drafter_fix` (DEFAULT `Off`, byte-identical
for every existing caller) — the seam the `d3aeeae` site publish implicitly
required (its "this publish turns the pin's site half green" claim was not true
until the seat + pin carried the lever). The site rows were generated
`--nb-select --oc-select --ridge-select --drafter-fix per_byte` (no head flag) —
now the arena seats byte-identical knobs, per the pin's own law ("the seat
posture = the CURRENT PUBLISHED reflex posture").

## Disclosures

1. **Population**: the nine suites with datasets + site rows on this box (the
   s1mb trio's dataset dirs are absent — their site rows still carry the
   `d009604` vintage, unverified by this run; wanli_en is `named_only`). The
   first full-population attempt died at `s1mb_choice: read_dir ... No such file
   or directory` — a pre-existing box state, not this change.
2. **Engine pin**: the banner prints the RECORDED pin `6365fab0261b`; the engine
   that ran is the sibling checkout `44e7f70` (docs-only on top of `f068ae6`,
   knowingly ahead per the pin's own law; the bump remains the owner's
   re-baseline decision, bench 0058's standing note).
3. **Box state**: load 16.5–22 throughout (sibling sessions) — preflight
   REFUSED; every µs cell is provisional, correctness cells load-insensitive.
4. **sst5's T2** reads REFUSED (mean +0.0250, LB95 −0.0129) at A1 0.4217 — the
   cells are byte-identical to the standing board (A0 0.3967 = site ✓); the
   refusal is the gate's read at the canonical population, disclosed here
   without a manifest edit (registered == served).

---

# Bench 0059 — the Reflex · instinct hybrid GOAT run (the arena's own record follows)

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline, amended: `--nb-select --oc-select --ridge-select`, registry caps, genome off, head-select RETIRED at the 10-01 full-pool re-basis + the issue-079 `per_byte` drafter lever (Bench 131 cell 2 + the `d3aeeae` site republish — engages only on the drafter-only path: typed the sole mover) — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power Some("AC Power") · powermode Some("high") · load1m Some(16.48) · swap_mb Some(6100.88) · quotable Some(false) · refusals ["load 16.48 > 6 — a sibling job is on the box"]

Pool: ../riir-reflex/.raw/datasets · reflex engine baseline pinned at `6365fab0261b2123ae0953a12b06ef0eec596c2b` (Issue 013 re-baseline a: hybrid rows published before it are t20k-seated, archived references — `.raw/datasets_t20k` stays on disk to reproduce them).

## ag_news

Verdict: **a0_stands** — winner present but UNSEATABLE — 600 of 600 case(s) present option key(s) with no artifact class row (template drift vs the train pool; 4 distinct key(s), e.g. ["business", "sci_tech", "sports", "world"]); the context seat answers only by name (Plan 003), so the specialist cannot seat — Issue 010 T2's A0-only posture with this reason.

Posture: cap 64 · head 0.00 · nb 4.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.836/0.411 · specialist bag count. Questions: 400 over 400 cases. A0/G0-only posture — no specialist artifact (Issue 010 T2).

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.8550 | 0.8085 | 0.000 | 323 |

Served arm: **A0** (rank-0 1/1 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.8825 | 0.000 | 162 | 297 |

### Gates

- **G1 calibration:** raw ECE 0.8305 · Platt 0.0459 (fit engaged: true) · conformal floor 0.2482 → PASS
- **G2 fusion overhead:** N/A — no hybrid lane exists (A0/G0-only posture); A0's absolute latency is in the table.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## emotion

Verdict: **a0_stands** — winner present but UNSEATABLE — 600 of 600 case(s) present option key(s) with no artifact class row (template drift vs the train pool; 6 distinct key(s), e.g. ["anger", "fear", "joy", "love"]); the context seat answers only by name (Plan 003), so the specialist cannot seat — Issue 010 T2's A0-only posture with this reason.

Posture: cap 64 · head 0.00 · nb 4.00 · ridge 8.00 (bag) · fused-gate thresholds \n0.763/0.427 · specialist bag count. Questions: 400 over 400 cases. A0/G0-only posture — no specialist artifact (Issue 010 T2).

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.7750 | 0.7224 | 0.000 | 182 |

Served arm: **A0** (rank-0 1/1 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.8850 | 0.000 | 130 | 172 |

### Gates

- **G1 calibration:** raw ECE 0.8621 · Platt 0.0488 (fit engaged: true) · conformal floor 0.2781 → PASS
- **G2 fusion overhead:** N/A — no hybrid lane exists (A0/G0-only posture); A0's absolute latency is in the table.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## sst5

Verdict: **a0_stands** — instrument pick A1 refused — paired LB95 -0.0129 (Issue 008 T2).

Posture: cap 64 · head 0.00 · nb 16.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.281/0.417 · specialist bag count. Questions: 600 over 600 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.2900 | 0.2408 | 0.000 | 131 |
| ✓ | A1 | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.3400 | 0.2877 | 1.000 | 2 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.3400 | 0.2877 | 1.000 | 2 |

Served arm: **A0** (rank-0 11/48 candidates; the full table rides `registration.json`). The instrument's pick **A1** was REFUSED by the superiority gate (paired LB95 -0.0129 ≤ 0) — A0 serves and the suite is not sold (Issue 008 T2).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.3967 | 0.000 | 104 | 129 |
| A1 | 0.4217 | 1.000 | 1 | 2 |
| H1 | 0.4217 | 1.000 | 104 | 129 |

H1 decomposition: escalation 1.000 · decision p50 0 µs · total p50 104 µs (reflex 104 µs) · total p99 129 µs.

### Gates

- **G7 product gate (Issue 008 T2):** A1 − A0 mean +0.0250 · paired LB95 -0.0129 → REFUSED — A1 is not sold; A0 serves
- **G1 calibration:** raw ECE 0.1008 · Platt 0.0835 (fit engaged: true) · conformal floor 0.2121 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.0250 · UB95 +0.0129 vs δ 0.0392 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 13 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 45 ns/q) · H2 fusion 2.39 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## xnli_en

Verdict: **a0_stands** — winner present but UNSEATABLE — 500 of 500 case(s) present option key(s) with no artifact class row (template drift vs the train pool; 3 distinct key(s), e.g. ["contradiction", "entailment", "neutral"]); the context seat answers only by name (Plan 003), so the specialist cannot seat — Issue 010 T2's A0-only posture with this reason.

Posture: cap 64 · head 0.00 · nb 4.00 · ridge 0.00 (pair) · fused-gate thresholds \n0.557/0.511 · specialist bag count. Questions: 300 over 300 cases. A0/G0-only posture — no specialist artifact (Issue 010 T2).

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5950 | 0.5368 | 0.000 | 131 |

Served arm: **A0** (rank-0 1/1 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.5233 | 0.000 | 95 | 133 |

### Gates

- **G1 calibration:** raw ECE 0.5133 · Platt 0.1119 (fit engaged: true) · conformal floor 0.1480 → PASS
- **G2 fusion overhead:** N/A — no hybrid lane exists (A0/G0-only posture); A0's absolute latency is in the table.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## massive_intent_en

Verdict: **hybrid_arm**.

Posture: cap 48 · head 0.00 · nb 4.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.111/0.526 · specialist bag count. Questions: 300 over 300 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5450 | 0.4868 | 0.000 | 145 |
| ✓ | H1 | 0.6450 | 0.5875 | 0.735 | 145 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.8300 | 0.7813 | 1.000 | 3 |

Served arm: **H2(β=1,nmin=2,τ=8)** (rank-0 3/48 candidates; the full table rides `registration.json`). The superiority gate certified it strictly above Reflex (paired LB95 +0.0124 > 0).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.7800 | 0.000 | 119 | 155 |
| A1 | 0.8167 | 1.000 | 1 | 3 |
| H1 | 0.8133 | 0.513 | 119 | 155 |
| H2(β=1,nmin=2,τ=8) ★ | 0.8267 | 1.000 | 1 | 3 |

H1 decomposition: escalation 0.513 · decision p50 0 µs · total p50 119 µs (reflex 119 µs) · total p99 155 µs.

### Gates

- **G7 product gate (Issue 008 T2):** H2(β=1,nmin=2,τ=8) − A0 mean +0.0467 · paired LB95 +0.0124 → PASS (strictly above Reflex)
- **G1 calibration:** raw ECE 0.0607 · Platt 0.0556 (fit engaged: true) · conformal floor 0.2943 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.0467 · UB95 -0.0124 vs δ 0.0691 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 104 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 139 ns/q) · H2 fusion 0.97 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## banking77

Verdict: **a0_stands** — instrument pick H2(β=2,nmin=8,τ=8) refused — paired LB95 -0.0163 (Issue 008 T2).

Posture: cap 40 · head 0.00 · nb 1.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.748/0.555 · specialist bag presence. Questions: 500 over 500 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.7950 | 0.7436 | 0.000 | 683 |
| ✓ | H2(β=2,nmin=8,τ=8) | 0.8450 | 0.7976 | 1.000 | 22 |

Served arm: **A0** (rank-0 2/48 candidates; the full table rides `registration.json`). The instrument's pick **H2(β=2,nmin=8,τ=8)** was REFUSED by the superiority gate (paired LB95 -0.0163 ≤ 0) — A0 serves and the suite is not sold (Issue 008 T2).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.8420 | 0.000 | 376 | 664 |
| A1 | 0.8280 | 1.000 | 9 | 32 |
| H1 | 0.8380 | 1.000 | 377 | 665 |
| H2(β=2,nmin=8,τ=8) ✗ | 0.8540 | 1.000 | 9 | 32 |

H1 decomposition: escalation 1.000 · decision p50 1 µs · total p50 377 µs (reflex 376 µs) · total p99 665 µs.

### Gates

- **G7 product gate (Issue 008 T2):** H2(β=2,nmin=8,τ=8) − A0 mean +0.0120 · paired LB95 -0.0163 → REFUSED — H2(β=2,nmin=8,τ=8) is not sold; A0 serves
- **G1 calibration:** raw ECE 0.0841 · Platt 0.0347 (fit engaged: true) · conformal floor 0.3103 → PASS
- **G3 non-inferiority (reflex-won suite):** A0−hybrid mean -0.0120 · UB95 +0.0163 vs δ 0.0363 (from the cal discordant rate) → PASS
- **G2 fusion overhead:** H1 fusion-only 124 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 161 ns/q) · H2 fusion 0.98 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## typed_decisions

Verdict: **hybrid_arm**.

Posture: cap 48 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.565/0.912 · specialist bag count. Questions: 2000 over 400 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5580 | 0.5212 | 0.000 | 1858 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=0.5,nmin=2,τ=8) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=0.5,nmin=4,τ=2) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=0.5,nmin=4,τ=4) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=0.5,nmin=4,τ=8) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=0.5,nmin=8,τ=2) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=0.5,nmin=8,τ=4) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=0.5,nmin=8,τ=8) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=1,nmin=2,τ=2) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=1,nmin=4,τ=2) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=1,nmin=8,τ=4) | 0.6380 | 0.6019 | 1.000 | 12 |
| ✓ | H2(β=1,nmin=8,τ=8) | 0.6380 | 0.6019 | 1.000 | 12 |

Served arm: **H2(β=0.5,nmin=2,τ=2)** (rank-0 19/48 candidates; the full table rides `registration.json`). The superiority gate certified it strictly above Reflex (paired LB95 +0.0598 > 0).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.5700 | 0.000 | 534 | 1864 |
| A1 | 0.6300 | 1.000 | 3 | 12 |
| H1 | 0.6300 | 1.000 | 534 | 1865 |
| H2(β=0.5,nmin=2,τ=2) ★ | 0.6475 | 1.000 | 3 | 12 |

H1 decomposition: escalation 1.000 · decision p50 0 µs · total p50 534 µs (reflex 534 µs) · total p99 1865 µs.

### Gates

- **G7 product gate (Issue 008 T2):** H2(β=0.5,nmin=2,τ=2) − A0 mean +0.0775 · paired LB95 +0.0598 → PASS (strictly above Reflex)
- **G1 calibration:** raw ECE 0.1899 · Platt 0.1387 (fit engaged: true) · conformal floor 0.1701 → PASS
- **G2 fusion overhead:** H1 fusion-only 9 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 27 ns/q) · H2 fusion 2.64 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## prompt_injections

Verdict: **hybrid_arm**.

Posture: cap 64 · head 0.00 · nb 1.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.799/0.462 · specialist bag count. Questions: 116 over 116 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.8000 | 0.7254 | 0.000 | 90 |
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
| A0 | 0.7672 | 0.000 | 73 | 92 |
| A1 | 0.8534 | 1.000 | 0 | 4 |
| H1 | 0.8534 | 1.000 | 73 | 92 |

H1 decomposition: escalation 1.000 · decision p50 0 µs · total p50 73 µs (reflex 73 µs) · total p99 92 µs.

### Gates

- **G7 product gate (Issue 008 T2):** A1 − A0 mean +0.0862 · paired LB95 +0.0082 → PASS (strictly above Reflex)
- **G1 calibration:** raw ECE 0.0824 · Platt 0.1017 (fit engaged: true) · conformal floor 0.3769 → FAIL
- **G2 fusion overhead:** H1 fusion-only 7 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 14 ns/q) · H2 fusion 4.41 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## code_fixtures

Verdict: **hybrid_arm**.

Posture: cap 18446744073709551615 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.461/0.337 · specialist bag presence. Questions: 32 over 16 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.4688 | 0.3977 | 0.000 | 416 |
| ✓ | A1 | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.25,nmin=2,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.25,nmin=2,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.25,nmin=2,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.25,nmin=4,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.25,nmin=4,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.25,nmin=4,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.25,nmin=8,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.25,nmin=8,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.25,nmin=8,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=2,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=4,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=4,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=4,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=8,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=8,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=8,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=2,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=4,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=8,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=8,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=2,nmin=2,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=2,nmin=2,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=2,nmin=4,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=2,nmin=4,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=2,nmin=4,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=2,nmin=8,τ=2) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=2,nmin=8,τ=4) | 0.8203 | 0.7570 | 1.000 | 13 |
| ✓ | H2(β=2,nmin=8,τ=8) | 0.8203 | 0.7570 | 1.000 | 13 |

Served arm: **A1** (rank-0 47/48 candidates; the full table rides `registration.json`). The superiority gate certified it strictly above Reflex (paired LB95 +0.0021 > 0).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.3750 | 0.000 | 198 | 401 |
| A1 | 0.5625 | 1.000 | 1 | 9 |
| H1 | 0.5625 | 1.000 | 198 | 403 |

H1 decomposition: escalation 1.000 · decision p50 0 µs · total p50 198 µs (reflex 198 µs) · total p99 403 µs.

### Gates

- **G7 product gate (Issue 008 T2):** A1 − A0 mean +0.1875 · paired LB95 +0.0021 → PASS (strictly above Reflex)
- **G1 calibration:** raw ECE 0.2524 · Platt 0.1519 (fit engaged: true) · conformal floor 0.3430 → PASS
- **G2 fusion overhead:** H1 fusion-only 27 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 67 ns/q) · H2 fusion 1.62 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

