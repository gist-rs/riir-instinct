# Plan 005 — Issue 012: frozen-artifact staleness probe + pre-swap impact report (POC)

**Status:** LANDED 2026-09-29 — T1–T5 complete (Bench 022: drift row FIRED,
13/64 flips, NOT dead-by-domination); T6 (swap-path hook) + part 3
(overlay-refresh policy arm) deferred pending a real refresh decision, per
the issue's own "measure before building".

## Goal

The serving half of riir-train Research 460 (arXiv:2609.30652): a soft
early-warning readout that fires BEFORE any pick flips, where today's only
instrument is the hard pick-parity gate (`tests/serve_gates.rs`). Report-only —
the T2/LB95 gates keep owning the promote/demote verdict.

## Measured material (on disk, verified 2026-09-29)

| Pair | Bytes | Meaning |
|---|---|---|
| banking77 v1 (`banking77_winner_v1.bin`, count) → v2 (`banking77_nbsvm_v2.bin`, presence) | same size, different sha | the REAL Bench-012 bridge drift event |
| ag_news armA → winner v1 | different sha | same-generation different-run pair |
| massive armA → winner v1 | different sha | same-generation different-run pair |
| xnli armA → winner v1 | different sha | same-generation different-run pair |
| emotion / sst5 / prompt_injections / typed armA == winner | IDENTICAL sha | byte-identical pairs → canary/control rows |

## Tasks

- [x] T0 substrate-first: no staleness/divergence-probe substrate in instinct or
  reflex (greps: staleness/stale_reference/divergence_probe/probe_set/pre_swap ×2
  repos → 0 hits; `tests/calibrator_probe.rs` is the katgpt-core calibrator
  contract pin, unrelated; the consumed substrate is `Specialist`/`BagConvention`
  /`winner_bridge` (specialist.rs) + `SpecialistLane` (hybrid.rs) + the
  serve_gates hard instrument the probe extends).
- [x] T1 `src/staleness.rs` — pure, ungated: `ProbeSet` fixture schema + BLAKE3
  digest; artifact-space `Readout` (gold-class sigmoid score, top-1 + margin,
  the engine argmax law: strictly-greater, ties → lowest class); per-item
  divergence (gold Δ, pick flip, margin Δ); `compare_pair` (per-artifact gold
  resolution by name with the two normalization fallbacks, then identity-by-
  count — the Issue-006 bridge rules, refuse loud when unresolvable; label-
  universe delta reported); fire rule PRE-REGISTERED here: any pick flip = FIRED,
  or mean |Δgold| ≥ 0.02 = FIRED-soft (both recorded; the boolean is either).
- [x] T2 probe fixture `tests/fixtures/staleness_probe_set.json` — committed,
  self-contained (probe time needs NO dataset dir), label-stratified round-robin
  (the banking77 mirror's first rows are label-clustered — first-N is wrong),
  64 items/suite, suites: banking77 + ag_news + emotion. Built once via the
  example's `--build-probe` (reads `INSTINCT_DATASETS_DIR`, default
  `../riir-reflex/.raw/datasets_t20k`).
- [x] T3 `examples/staleness_probe.rs` — default mode runs the report rows
  (canary: banking77 v2 vs itself → MUST read exactly zero; drift:
  banking77 v1→v2; candidate: ag_news armA→winner; control: emotion armA→winner
  byte-identical → zero) and writes `.benchmarks/022_staleness_probe/`
  (REPORT.md + results.json, box-state provenance recorded — divergence is a
  correctness/determinism readout, NOT a latency claim, so the preflight gate
  does not bind; still recorded per the box-state rule).
- [x] T4 `tests/staleness_gates.rs` — skip-loud without artifacts/datasets
  (the serve_gates convention): canary-zero (structural), byte-identical report
  across two runs (determinism), fixture digest stability, and the drift row's
  verdict PINNED AS MEASURED (pinned after T3's first run — never the hoped
  verdict; if the drift row does NOT fire, the pin records NOT-FIRED and the
  issue takes its DEAD-BY-DOMINATION negative per acceptance).
- [x] T5 verdict + records: `.benchmarks/022` REPORT.md (verdict table +
  box state), the measured verdict decided on the numbers (probe valuable as
  canary + pre-swap report vs DEAD-BY-DOMINATION), issue 012 status updated,
  AGENTS.md current-state row, `.benchmarks/.highwater` repaired 0015→0022 (it
  was STALE — dirs 016..021 landed without a bump; dir reality + git history
  wins) and `.plans/.highwater` 004→005.
- [-] T6 (deferred) pre-swap report wired into `arsenal_ops`' swap path — only
  if the T5 verdict keeps the probe; part 3 (overlay-refresh policy arm) —
  explicitly gated on T5's measurement.

## Guardrails

- Report-only: nothing here touches the serve path, the manifest, or the swap
  FSM. `check_winner_file`'s raw-boot refusal is NOT bypassed for serving — the
  report deliberately instantiates both artifacts side-by-side (that IS the
  pre-swap use case) and serves nothing.
- Each artifact scored under its OWN training convention (v1 banking77 = count,
  v2 = presence) — scoring v1 under presence would read a model that was never
  trained; the conventions are recorded per row.
- Determinism: same artifact pair + fixture → byte-identical report (asserted
  by T4). No RNG anywhere.
