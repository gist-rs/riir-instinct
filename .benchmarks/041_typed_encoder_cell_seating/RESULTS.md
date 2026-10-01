# Bench 041 — the typed_decisions encoder cell seated: the v2 arena replay — witness 0.7550 EXACT (the third-posture law)

**Status:** RECORD + SEATED — 016 T9's seat half COMPLETE: the arena
`--encoder-art` replay of the NLEH v2 per-option head over the live
m3-Metal **typed**-checkpoint encode reproduces the trainer-side frozen
read **cell-for-cell (0.7550, 1510/2000)** — the third posture (4090
cache-witness → trainer read (Bench 040) → live arena encode) all
identical, the 017-pattern witness law. The auto-emitted lane doc
(`hybrid_lane_doc.json`) is the publish input; the site publish is the
follow-up commit.

**The code half (instinct `73cd6d6`)**: `encoder_arm.rs` reads NLEH v2
(the law-copy mirror of riir-train's t608 codec — `read_nleh_v2` + the
per-option forward), the sealed loader dispatches on the header's version
word, v2 flattens PER QUESTION (typed's 5 q/case → 2000 rows — the A0
arms' own convention), and the CHECKPOINT is the caller's
(`--encoder-ckpt typed`; the artifact does not carry its training
checkpoint). En-route: a PRE-EXISTING latent `laya_face` index bug fixed
(the per-question escalation indices were mapped straight into `cases[]`
— valid only at 1 q/case; typed panicked at row 400; multi-question
suites now SKIP the face loudly — the reflex helper measures whole-case
forwards, not comparable, never mis-measured).

**Box state (preflight-cleared before the read)**: PROVENANCE:
power=AC Power load=4.86 swap=3648.44M canary=136.8us/best5
powermode=2(high). The run itself executed under sibling load (the
auto RESULTS.md box line) — irrelevant to the accuracy witness
(deterministic, reproduced 0.7550 on BOTH runs) and to every latency
conclusion (the arm reads ~365 ms/row against a ≤ ~300 µs bar — 3 orders
of magnitude, load-noise cannot flip it).

**Datasets**: the FULL-POOL typed dir (`../riir-train/.raw/datasets_typed_full`,
train 1200 incl. security_incidents; TEST byte-identical to the t20k
pull — all four test-*.json SHA-256-verified) — the slice-integrity gate
refuses the t20k dir's shrunken 700-row pool (the silent-move class); the
full-pool lane is Bench 015/019/020's own posture, and every standard
arm reproduced the published full-pool rows byte-exactly (A0 0.5725 ·
A1 0.6300 · H2 0.6475 — the displayed arm).

**Board effect (after the site publish)**: the Rethink lane grows
3/9 → **4/9** (sst5 0.5267 · xnli 0.8600 · ag_news 0.9475 · **typed
0.7550** — the STRONGEST cell vs the displayed arm, +10.75 pt over the
serving H2 0.6475); vs the board's best (AgentJev 0.7715) it reads −1.65
pt (recorded honestly — their stack stays ahead on typed).

---

(the auto-generated GOAT record follows)

# Bench 041 — the Reflex · instinct hybrid GOAT run

**Status:** MEASURED — the single frozen test read (Issue 005 T5 / Issue 003 T4); arms pre-registered on the cal front by the Pareto rank-0 + argmax Beta-LCB instrument; predictions frozen in `predictions.json`.

Protocol: the seat posture = the CURRENT PUBLISHED reflex posture (Issue 008 T1's re-baseline: `--head-select --nb-select --oc-select --ridge-select`, registry caps, genome off — oc and ridge arm only where their cal-slice selection clears the bar: oc on typed_decisions, ridge on emotion, byte-identical to off elsewhere) fit through the SAME code reflex's runner uses (`harness::runner::seat`). The A0 drift pin asserts the arena's A0 accuracy equals reflex's own `run()` row on EVERY arena suite, and — when the reflex-site checkout stands beside the workspace — that reflex's rows equal the PUBLISHED bench.json numbers. Population (Issue 010 T1/T2): every reflex dataset suite — a suite with no winner artifact runs the A0/G0-only posture (verdict a0_stands, never a crash; Issue 010 T2). H1 top-k = 8 (default). H2 grid: β ∈ [0.0, 0.25, 0.5, 1.0, 2.0] × n_min ∈ [2.0, 4.0, 8.0] × τ ∈ [2.0, 4.0, 8.0] — 45 candidates, train-side only. Product gate (Issue 008 T2): the registered arm must be STRICTLY above the current Reflex row — paired (pick − A0) LB95 > 0 on this frozen test read — else the registration refuses and A0 serves. PICK SPACE (Issue 006, the v2 instrument): A0's probs and every gold idx speak the question's PRESENTED-option space; A1/H1/H2 resolve each presented option to its specialist class row (by name for the suites whose keys are the label strings — massive/banking77 — by index under k == N for the fixed-criteria suites), and every hybrid pick is a position, directly comparable with gold. A0 rows are PER QUESTION (reflex's hard-metrics convention; latency stays per-case for seat-composing arms, `n_cases` disclosed).Box state: power Some("AC Power") · powermode Some("high") · load1m Some(5.39) · swap_mb Some(2691.25) · quotable Some(true) · refusals []

## typed_decisions

Verdict: **hybrid_arm**.

Posture: cap 48 · head 0.00 · nb 0.00 · ridge 0.00 (bag) · fused-gate thresholds \n0.579/0.912 · specialist bag count. Questions: 2000 over 400 cases.

### Pre-registration (cal front, train-side only)

| rank-0 | arm | cal acc | Beta LCB₅ | consult | p99 µs |
|---|---|---|---|---|---|
| ✓ | A0 | 0.5720 | 0.5353 | 0.000 | 2212 |
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

Served arm: **H2(β=0.5,nmin=2,τ=2)** (rank-0 19/48 candidates; the full table rides `registration.json`). The superiority gate certified it strictly above Reflex (paired LB95 +0.0580 > 0).

### The single test read

| arm | accuracy | consult | p50 µs | p99 µs |
|---|---|---|---|---|
| A0 | 0.5725 | 0.000 | 656 | 2106 |
| A1 | 0.6300 | 1.000 | 4 | 16 |
| H1 | 0.6300 | 1.000 | 656 | 2107 |
| H2(β=0.5,nmin=2,τ=2) ★ | 0.6475 | 1.000 | 4 | 16 |

H1 decomposition: escalation 1.000 · decision p50 0 µs · total p50 656 µs (reflex 656 µs) · total p99 2107 µs.

### Gates

- **G7 product gate (Issue 008 T2):** H2(β=0.5,nmin=2,τ=2) − A0 mean +0.0750 · paired LB95 +0.0580 → PASS (strictly above Reflex)
- **G1 calibration:** raw ECE 0.1899 · Platt 0.1387 (fit engaged: true) · conformal floor 0.1701 → PASS
- **G2 fusion overhead:** H1 fusion-only 16 ns/question (bar < 100; the full escalated decision incl. specialist scoring is 36 ns/q) · H2 fusion 3.67 ns/option (bar < 100) · absolute p99 in the table
- **C1 encoder arm (RECORD-ONLY — serve: ✗, the encoder class is refused at serve; issue 014 decision 1):** NLEH v2 per-option head `../riir-train/.raw/t608/typed_encoder_v2.bin` (d 1024 · hidden 128 · widths 2/4/5) over the live laya-typed encode (device metal) · accuracy **0.7550** (2000 rows) · vs the incumbent A1: paired mean +0.1250 · LB95 +0.1032 (strictly above — the T2 form, A1 the comparator) · per-row p50 364486 µs / p99 782684 µs (the full arm cost per QUESTION row: encode + head; the provisional text-lane bar is ≤ ~300 µs — the arm reads ms-class, the recorded ground of the refusal). No serve change: A1 keeps serving.
- **G4 alloc-free:** `tests/g4_alloc.rs` — a counting global allocator over 10k H1 + H2 + prune decisions: 0 allocations (its own binary; no concurrent test traffic).
- **G0 identity:** the kill switch (`HybridLane::ReflexOnly`) is byte-identical to A0 incl. the abstain passthrough, and the H2 `g ≡ 0` collapse lands on the A1 ordering — both pinned by unit tests; the A0 drift pin above holds the seat path to reflex's own run().
- **G6 purity:** the reflex half is the frozen count tables (built once, read-only) and the instinct half a BLAKE3-sealed RISP artifact verified at load. The HOSTED-ONLY vessel wrapper is P4 (Issue 001) — disclosed, not claimed here.

