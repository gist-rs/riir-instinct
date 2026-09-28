# Bench 022 — the frozen-artifact staleness probe (Issue 012 / Plan 005)

**Date:** 2026-09-29 · **Box:** m3-max (macOS, loaded — sibling twt_goat sweep
pegging ~10 cores; divergence is a correctness/determinism readout, NOT a
latency claim, so the load is provenance only) · **Commit:** this landing.

**Status:** MEASURED — the probe is NOT dead-by-domination. Verdict: the
instrument earns its place as canary + pre-swap report; the T6 swap-path hook
(issue 012) is live work, part 3 (overlay-refresh policy arm) stays deferred
pending a real refresh decision.

## The probe set

`tests/fixtures/staleness_probe_set.json` — committed, BLAKE3
`98a683969c285f157a3d81de7ffbdabebfac77873ce57eaa4b33ffe298d20f93`:
192 items (banking77 64 · ag_news 64 · emotion 64), label-stratified
round-robin over the test splits (first-N would read one label — the
banking77 mirror's early rows are label-clustered). Labels spent at record
time; probe time reads NO dataset dir.

## Rows (live = what serving carries; reference = stale/candidate)

| suite | live | reference | flips/64 | mean \|Δgold\| | max \|Δgold\| | mean \|Δmargin\| | verdict |
|---|---|---|---|---|---|---|---|
| banking77 | v2 (bridged winner, presence) | v2 self | 0 | 0.00000 | 0.00000 | 0.00000 | **quiet — the free canary, exact zero** |
| banking77 | v2 (bridged winner, presence) | v1 (pre-bridge, count) | **13** | **0.24003** | **0.85166** | 0.24138 | **FIRED — pick flip** |
| ag_news | winner v1 | armA v1 | 1 | 0.04076 | 0.23787 | 0.06537 | FIRED — pick flip |
| emotion | winner v1 | armA v1 (byte-identical) | 0 | 0.00000 | 0.00000 | 0.00000 | quiet — digest identity detected |

## Reading

- **The drift row is the finding.** The Bench-012 bridge swap (v1 count-bag
  winner → v2 nbsvm presence-bag) reads 13/64 pick flips and a 0.24 mean
  gold-score shift — a real, historical drift event, detected with wide
  margin by a probe that costs milliseconds. The paper's premise (a frozen
  reference diverges from the live runtime as it evolves, and acting on the
  stale side costs) transfers to the artifact plane.
- **Candidates read divergent too** (ag_news 1/64): even same-generation
  artifact pairs carry visible readout movement. A pre-swap impact report is
  real information, not ceremony — a swap's blast radius is measurable
  BEFORE the monotonic apply commits.
- **Identity is exact.** Both the self-canary and the byte-identical pair
  read exactly 0.0 on every metric — the probe has no false-positive floor
  at identity, which is what makes its FIRED verdicts trustworthy.
- **Conventions are per-artifact.** v1 banking77 scored under count, v2 under
  presence — each side under its own training convention; scoring v1 under
  presence would read a model that never existed.

## Fire rule (pre-registered in Plan 005 T1 before any run)

`fired = (pick flips > 0) OR (mean |Δgold| ≥ 0.02)`. Both disjuncts recorded
per row; the threshold is a 2-point sigmoid shift on the gold class.

## Gates

`tests/staleness_gates.rs` — 6 tests, count-pinned AS MEASURED (fixture
digest, 13 flips, mean 0.24003, 1 candidate flip), canary-exact-zero,
bit-identical double run, skip-loud without the fixture or winners dir.

## Reproduce

```sh
cargo run --release --example staleness_probe            # the report
cargo test --test staleness_gates                        # the pins
```
