# Bench 0060 — the s1mb trio's A0-pin close (dataset wiring + cross-host identity + the float_roundtrip parse fix)

**Status:** MEASURED — closes bench 0059's standing disclosure ("the s1mb trio's dataset dirs are absent — their site rows still carry the d009604 vintage, unverified by this run"). **A0 pin GREEN 3/3, both halves** — the first verification of the s1mb site rows since the 650202c section landed.

## What ran and why

Bench 0059 verified the nine dataset suites' site rows but the s1mb trio's dirs sat outside the canonical pool (`.raw/datasets_s1mb`, Plan 010's converter output) — the arena's default datasets dir never saw them. This bench wires the trio in (symlinks `datasets/s1mb_* → ../datasets_s1mb/*` on the m3; plain copies on the 4090 — Windows) and runs the scoped arena seat + pin.

## The two findings

1. **s1mb_score's site row was genuinely stale** — `0.49669389342668224 → 0.4982497082847141` (+4 questions, n 2571): the d009604-vintage row predates the issue-079 `per_byte` drafter lever. choice (`0.22961422961422961`) and noul (`0.7054916572169123`) are bit-stable across the engine move. Republished through `publish_bench.py` (the sanctioned writer; `PUBLISH_BENCH_LANES=modelless:acc-only`, the d3aeeae load-wall convention) with BOTH hosts refreshed in one publish — the publisher's cross-host bit-identity gate first REFUSED the m3-only update ("m3=0.4982, 4090-win=0.4967 — stop and file"), which is the discipline working: the 4090 leg re-measured at the same engine and confirmed **cross-host bit-identity 3/3** (m3 `0413dcd` == 4090 `741bda3`, engine-identical src — the range touches no `src/`), then the publish passed with both hosts agreeing.
2. **The choice pin red was a serde_json 1-ULP parse artifact, not engine drift.** Runs 1–2 printed `reflex run() 0.229614 != PUBLISHED site row 0.229614` — visually equal, bit-unequal: serde_json (1.0.151, crates.io) parses the 17-digit literal `0.22961422961422961` to bits `…d63f` while the engine's `994.0/4329.0` computes `…d640` (std `str::parse` agrees with the division; serde_json issue #505's documented class — fixed by the `float_roundtrip` feature). Probe: `/tmp/parse_probe` (rustc + serde_json 1.0.151). **Run 1's initial attribution to a sibling's uncommitted-WIP engine was WRONG** — the WIP was innocent; the artifact fired on every run against the 17-digit value. The fix (`serde_json` gains `float_roundtrip` in this repo's Cargo.toml) is in this commit; with it the pin compares exact bits and choice goes green. The 16-digit site values (noul, score-fresh) parsed exactly all along; sst5's 17-digit `0.39666666666666667` happens to sit mid-interval and also parsed exactly — bench 0059's 9-suite green stands.

## Verdicts (this run)

| suite | A0 | registered arm | serves | pin |
|---|---|---|---|---|---
| s1mb_choice | 0.22961422961422961 | A0 (cal-front rank-0; A1's test-read 0.3024 is post-hoc, never a serving signal) | A0 | site ✓ |
| s1mb_noul | 0.7054916572169123 | A1 (T2 mean −0.0031, LB95 −0.0169 → REFUSED) | A0 | site ✓ |
| s1mb_score | 0.4982497082847141 | — (no winner artifact, Issue 010 T2) | A0 | site ✓ |

G1 (A0 disclosure): FAIL on all three (raw/platt vs the conformal floor) — the standing A0 confidence posture on the wide-label suites, unchanged from the section's launch.

## Disclosures

- **Engine**: compiled at reflex src `0c6b8a8`-era (docs-only commits ahead of the recorded pin `6365fab0261b`; the pplx-lane commits between touch no modelless `src/` — verified by range diff). The pin bump remains the owner's re-baseline decision (bench 0058's standing note).
- **Box state**: load 16–24.6 (sibling sessions) throughout — preflight REFUSED; latency cells unquotable, accuracy cells load-insensitive (the acc-only publish posture).
- **The pin's arena RunOptions gained `pplx: false`** (the pplx lane's `RunOptions` field add, reflex issue 082 — a comparison lane, never the pin's subject; the cross-repo field broke this arena's build, fixed in the same commit).
- **Site hybrid/encoder cells**: the suite rows' `hybrid`/`encoder` blocks (A0 mirrors with `serves: A0` / `tier-fallback`) got the score accuracy refresh in the follow-up site commit; their auxiliary seat-readout fields (ece, mean_confidence, acc_at_50) remain b6425bf-era — stale-but-unrendered (the section renders acc/n/serves only); refreshing them needs a seat-readout bench if the site ever renders them.
- The publisher self-test passed 92/92 post-publish; the reflex-site smokes run in the site commit.

---

# Bench 0060 — the Reflex · instinct hybrid GOAT run (the arena's own record follows)

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline, amended: `--nb-select --oc-select --ridge-select`, registry caps, genome off, head-select RETIRED at the 10-01 full-pool re-basis + the issue-079 `per_byte` drafter lever (Bench 131 cell 2 + the `d3aeeae` site republish — engages only on the drafter-only path: typed the sole mover) — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power Some("AC Power") · powermode Some("high") · load1m Some(24.63) · swap_mb Some(24079.38) · quotable Some(false) · refusals ["load 24.63 > 6 — a sibling job is on the box"]

Pool: ../riir-reflex/.raw/datasets · reflex engine baseline pinned at `6365fab0261b2123ae0953a12b06ef0eec596c2b` (Issue 013 re-baseline a: hybrid rows published before it are t20k-seated, archived references — `.raw/datasets_t20k` stays on disk to reproduce them).

## s1mb_choice

Verdict: **a0_stands** — the instrument registered A0 outright.

Posture: cap 16 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.010/0.576 · specialist bag count. Questions: 4329 over 4329 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.1550 | 0.1183 | 0.000 | 5232 |
| ✓ | A1 | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.25,nmin=2,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.25,nmin=2,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.25,nmin=2,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.25,nmin=4,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.25,nmin=4,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.25,nmin=4,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.25,nmin=8,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.25,nmin=8,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.25,nmin=8,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.5,nmin=2,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.5,nmin=4,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.5,nmin=4,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.5,nmin=4,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.5,nmin=8,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.5,nmin=8,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=0.5,nmin=8,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=1,nmin=2,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=1,nmin=4,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=1,nmin=8,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=1,nmin=8,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=2,nmin=2,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=2,nmin=2,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=2,nmin=4,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=2,nmin=4,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=2,nmin=4,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=2,nmin=8,τ=2) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=2,nmin=8,τ=4) | 0.1250 | 0.0922 | 1.000 | 54 |
| ✓ | H2(β=2,nmin=8,τ=8) | 0.1250 | 0.0922 | 1.000 | 54 |

Served arm: **A0** (rank-0 47/48 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.2296 | 0.000 | 616 | 2820 |
| A1 | 0.3024 | 1.000 | 5 | 222 |
| H1 | 0.2885 | 0.780 | 616 | 2820 |

H1 decomposition: escalation 0.780 · decision p50 0 µs · total p50 616 µs (reflex 616 µs) · total p99 2820 µs.

### Gates

- **G1 calibration:** raw ECE 0.2060 · Platt 0.2060 (fit engaged: true) · conformal floor 0.4061 → FAIL
- **G2 fusion overhead:** H1 fusion-only 622 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 667 ns/q) · H2 fusion 1.54 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## s1mb_noul

Verdict: **a0_stands** — instrument pick A1 refused — paired LB95 -0.0169 (Issue 008 T2).

Posture: cap 64 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.455/0.601 · specialist bag count. Questions: 6173 over 6173 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5000 | 0.4423 | 0.000 | 715 |
| ✓ | A1 | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0,nmin=2,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0,nmin=2,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0,nmin=2,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0,nmin=4,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0,nmin=4,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0,nmin=4,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0,nmin=8,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0,nmin=8,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0,nmin=8,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.25,nmin=2,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.25,nmin=2,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.25,nmin=2,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.25,nmin=4,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.25,nmin=4,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.25,nmin=4,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.25,nmin=8,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.25,nmin=8,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.25,nmin=8,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.5,nmin=2,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.5,nmin=2,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.5,nmin=2,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.5,nmin=4,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.5,nmin=4,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.5,nmin=4,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.5,nmin=8,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.5,nmin=8,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=0.5,nmin=8,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=1,nmin=2,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=1,nmin=2,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=1,nmin=2,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=1,nmin=4,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=1,nmin=4,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=1,nmin=4,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=1,nmin=8,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=1,nmin=8,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=1,nmin=8,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=2,nmin=2,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=2,nmin=2,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=2,nmin=2,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=2,nmin=4,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=2,nmin=4,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=2,nmin=4,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=2,nmin=8,τ=2) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=2,nmin=8,τ=4) | 0.5200 | 0.4620 | 1.000 | 34 |
| ✓ | H2(β=2,nmin=8,τ=8) | 0.5200 | 0.4620 | 1.000 | 34 |

Served arm: **A0** (rank-0 47/48 candidates; the full table rides `registration.json`). The instrument's pick **A1** was REFUSED by the superiority gate (paired LB95 -0.0169 ≤ 0) — A0 serves and the suite is not sold (Issue 008 T2).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.7055 | 0.000 | 498 | 1169 |
| A1 | 0.7024 | 1.000 | 13 | 107 |
| H1 | 0.7024 | 1.000 | 498 | 1170 |

H1 decomposition: escalation 1.000 · decision p50 1 µs · total p50 498 µs (reflex 498 µs) · total p99 1170 µs.

### Gates

- **G7 product gate (Issue 008 T2):** A1 − A0 mean -0.0031 · paired LB95 -0.0169 → REFUSED — A1 is not sold; A0 serves
- **G1 calibration:** raw ECE 0.0846 · Platt 0.1680 (fit engaged: true) · conformal floor 0.2077 → FAIL
- **G2 fusion overhead:** H1 fusion-only 14 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 27 ns/q) · H2 fusion 8.38 ns/option (bar < 100) · absolute p99 in the table
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

## s1mb_score

Verdict: **a0_stands** — no specialist artifact (Issue 010 T2).

Posture: cap 16 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.230/0.415 · specialist bag count. Questions: 2571 over 2571 cases. A0/G0-only posture — no specialist artifact (Issue 010 T2).

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.2300 | 0.1854 | 0.000 | 660 |

Served arm: **A0** (rank-0 1/1 candidates; the full table rides `registration.json`).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.4982 | 0.000 | 454 | 792 |

### Gates

- **G1 calibration:** raw ECE 0.0041 · Platt 0.2681 (fit engaged: true) · conformal floor 0.4870 → FAIL
- **G2 fusion overhead:** N/A — no hybrid lane exists (A0/G0-only posture); A0's absolute latency is in the table.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

