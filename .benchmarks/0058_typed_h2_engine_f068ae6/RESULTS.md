# Bench 0058 — the typed H2 seat re-read at the reflex index-anchored engine (f068ae6)

**Status:** COMPLETE — the T2 re-gate at the moved inputs **HOLDS**: H2(β=0.5,nmin=2,τ=2) 0.6475 (digit-identical to the served cell, Benches 019+020) over A0 0.5630 (the new engine's default typed row, digit-exact vs reflex Bench 131), paired LB95 **+0.0657 > 0 → PASS** (was +0.0580 at bench 020). Registered == served unchanged — `arsenal.toml` NOT touched (the serving law: registered == served; the verdict needs no manifest edit).

## Why this re-read exists (the input delta)

The reflex sibling (`../riir-reflex`, this repo's path dep) replaced typed's legacy `k == N` position binding with the **index-anchored law** (`4ad25df` + `f068ae6`; reflex Bench 131): typed choice/score questions read content-bound (drafter + option_cond) at the shipped default. The seat consumes the sibling tree at build time, so the inputs moved under the served typed row without any instinct-side change. This bench is the actual re-read the reflex verdict requires.

| typed cell (2000 q / 400 cases, canonical pool) | served lineage (Bench 020, 2026-09-28) | this re-read (engine `f068ae6`) |
|---|---|---|
| A0 (pure reflex modelless) | 0.5725 | **0.5630** (−0.95 pt, digit-exact reflex bench 131 default) |
| A1 (specialist alone) | 0.6300 | **0.6300** (engine-independent — the oc tables read the train pool, not reflex's picks) |
| H1 (cascade) | 0.6335 (esc 0.717) | **0.6300** (esc 1.000 — see the gate note below) |
| H2(β=0.5,nmin=2,τ=2) ★ served | 0.6475 | **0.6475** |
| T2 paired (pick − A0) | mean +0.0750 · LB95 +0.0580 → PASS | mean **+0.0845** · LB95 **+0.0657** → PASS (275 wins / 1619 ties / 106 losses) |

The hybrid seat is NOT materially changed: the specialist/fusion side is untouched and the T2 margin actually widened (the denominator dropped, the pick held). The served row keeps its certificate.

Reproducibility: an independent second run of the final binary reproduced every arm byte-identically (same `test_digest`, same per-question picks, same T2 face) — the frozen read is deterministic at this engine + pool.

## Disclosures (all four load-bearing)

1. **Engine tree vs the recorded pin.** The arena banner prints `REFLEX_BASELINE_SHA = 6365fab0261b` — the RECORDED Issue-013 pin. The engine that actually ran is the sibling checkout **`f068ae6`** (knowingly ahead, per the pin's own law). Bumping the pin constant is a re-baseline decision left to the owner; this record is the evidence for it.
2. **A0 pin posture.** The run used `--skip-pin-a0` — LOUD skip. The pin's arena==reflex `run()` byte-identity half **PASSED** in the pin-on probe run immediately before (same binary, same inputs): arena A0 == reflex run() == 0.563000. The pin's SITE half failed, correctly: `../reflex-site/data/bench.json` still publishes the pre-change typed row 0.5725 — the site republish is the reflex sibling's lane. Until that republish, the site half of the pin stays red for typed on every box at the new engine.
3. **Instrument repair in the same landing (consumer-side).** On develop since `bbdb7f6` (2026-10-03, the Plan-010 s1mb digit branch), `presented_keys` spelled ALL array-criteria questions as index digits — typed's score questions presented `"0".."4"`, no artifact label names a digit, and the context-join template gate refused 500/500 cases: **the typed winner was UNSEATABLE and the suite silently degraded to `a0_stands`** (loud in stderr, but the served cell could not be re-measured at all). The typed artifact's label universe carries the score level VALUE strings ("Benign: read-only or clearly safe actions.", …) — verified in the artifact bytes — and that value spelling is what Benches 019+020 seated and served. Repair: string levels carry their value (typed), numeric/other levels keep the Plan-010 digit spelling (s1mb_score, whose winner artifact does not exist on this box — no behavior to preserve was harmed). Without this repair there is NO H2 typed cell at the new engine, and every future typed re-gate would read a lie (`a0_stands` for a seated specialist).
4. **Box state — latency not quotable.** `../riir-reflex/scripts/bench_preflight.sh`: `PROVENANCE: power=AC Power load=7.34 swap=6108.88M canary=skipped powermode=2(high)` → **REFUSED** (load > 6, a sibling build on the box; the arena's own capture agreed — load 17.16 mid-run). Every µs cell below (and in the arm tables) is provisional under sustained multi-tenant load; the CORRECTNESS cells are load-insensitive and are the only claims. The G2 fusion-overhead bars (<100 ns) pass with 40× headroom and are ordering-stable, but quote them as loaded-box numbers only.

Gate note: at the arena's fitted fused gate (0.550/0.912), typed's modelless lane abstained on **100% of test questions** (was 71.7% at bench 020) — the content-bound engine reads are less confident, so H1 degenerates to A1 (escalation 1.000, H1 == A1 == 0.6300). The SERVED arm is unaffected: H2 always consults the specialist and never reads the gate.

## Verdict for the owner

- **T2 re-gate: PASS.** No re-gate issue filed — the verdict did not flip and the seat did not move materially (task law: record-only).
- **`arsenal.toml`: NOT edited.** The typed row keeps H2(β=0.5,nmin=2,τ=2); registered == served holds at the new inputs.
- Recommended owner follow-ups (record-only, no issue): (a) bump `REFLEX_BASELINE_SHA` to `f068ae6…` as the re-baseline decision; (b) republish reflex-site `data/bench.json` from the new engine so the pin's site half goes green again.

---

# Bench 0058 — the Reflex · instinct hybrid GOAT run (the arena's own record follows)

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome off — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power Some("AC Power") · powermode Some("high") · load1m Some(17.16) · swap_mb Some(6100.88) · quotable Some(false) · refusals ["load 17.16 > 6 — a sibling job is on the box"]

Pool: ../riir-reflex/.raw/datasets · reflex engine baseline pinned at `6365fab0261b2123ae0953a12b06ef0eec596c2b` (Issue 013 re-baseline a: hybrid rows published before it are t20k-seated, archived references — `.raw/datasets_t20k` stays on disk to reproduce them).

## typed_decisions

Verdict: **hybrid_arm**.

Posture: cap 48 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.550/0.912 · specialist bag count. Questions: 2000 over 400 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5440 | 0.5072 | 0.000 | 1990 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=2,τ=8) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=4,τ=2) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=4,τ=4) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=4,τ=8) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=8,τ=2) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=8,τ=4) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=0.5,nmin=8,τ=8) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=2,τ=2) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=4,τ=2) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=8,τ=4) | 0.6380 | 0.6019 | 1.000 | 13 |
| ✓ | H2(β=1,nmin=8,τ=8) | 0.6380 | 0.6019 | 1.000 | 13 |

Served arm: **H2(β=0.5,nmin=2,τ=2)** (rank-0 19/48 candidates; the full table rides `registration.json`). The superiority gate certified it strictly above Reflex (paired LB95 +0.0657 > 0).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.5630 | 0.000 | 562 | 2004 |
| A1 | 0.6300 | 1.000 | 4 | 16 |
| H1 | 0.6300 | 1.000 | 562 | 2004 |
| H2(β=0.5,nmin=2,τ=2) ★ | 0.6475 | 1.000 | 4 | 16 |

H1 decomposition: escalation 1.000 · decision p50 0 µs · total p50 562 µs (reflex 562 µs) · total p99 2004 µs.

### Gates

- **G7 product gate (Issue 008 T2):** H2(β=0.5,nmin=2,τ=2) − A0 mean +0.0845 · paired LB95 +0.0657 → PASS (strictly above Reflex)
- **G1 calibration:** raw ECE 0.1899 · Platt 0.1387 (fit engaged: true) · conformal floor 0.1701 → PASS
- **G2 fusion overhead:** H1 fusion-only 13 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 29 ns/q) · H2 fusion 2.89 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

