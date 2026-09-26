# Bench 002 — alignment record: the hybrid lane at the Bench-052 protocol

**Status:** MEASURED 2026-09-26 — the aligned re-run Issue 003 T4's site
publish demanded. The v2 numbers (Bench 001, `.raw/datasets` old-pull
bytes) stay superseded by this read; this record is the one comparable
with the published reflex-site lanes.

## Why this run exists

Issue 003 T4: "the site's published modelless/laya lanes are the Bench
052 protocol runs, while the v2 arena numbers come from THIS box's
current `.raw/datasets`, which the record itself flags as differing from
earlier pulls — publishing the hybrid lane beside them without a
same-protocol, same-bytes re-measurement would put non-comparable
numbers on the public tables."

The alignment has three legs, all satisfied here:

1. **Same bytes** — `../riir-reflex/.raw/datasets_t20k`, the exact dir
   the Bench-052 runs read (`--datasets-dir .raw/datasets_t20k`), and
   the same bytes the 4090 T5 re-run byte-verified 977/977 (sorted
   SHA256-manifest diff empty; reflex `.docs/02_protocols/dataset_manifest.md`
   cross-host byte-discipline note).
2. **Same protocol** — the stratified split + `--head-select
   --nb-select` + registry caps + dispatch readout, through the SAME
   code reflex's runner uses (`harness::runner::seat::prepare_seat` /
   `fit_posture` — the seat adds no selection of its own).
3. **Same questions, proven** — the arena's A0 (the seat eval of the
   reflex engine) equals the PUBLISHED Bench-052 modelless hard
   accuracy on all six suites:

| suite | published 052 modelless | arena A0 | match |
|---|---|---|---|
| ag_news | 0.8825 | 0.8825 | ✓ |
| emotion | 0.7375 | 0.7375 | ✓ |
| sst5 | 0.3967 | 0.3967 | ✓ |
| xnli_en | 0.5233 | 0.5233 | ✓ |
| massive_intent_en | 0.7800 | 0.7800 | ✓ |
| banking77 | 0.8260 | 0.8260 | ✓ |

(The in-run A0 drift pin additionally re-derived reflex's xnli row via
`run()` and matched byte-exact.)

## The registered arms (single frozen test read)

| suite | registered arm | acc | A0 | consult | p50 |
|---|---|---|---|---|---|
| ag_news | H2(β=0.25,nmin=2,τ=2) | 0.8975 | 0.8825 | 1.000 | ~2 µs |
| emotion | A1 | 0.8550 | 0.7375 | 1.000 | ~1 µs |
| sst5 | A1 | 0.4217 | 0.3967 | 1.000 | ~1 µs |
| xnli_en | **A0 stands** — no hybrid lane published | 0.5233 | — | — | — |
| massive_intent_en | H2(β=1,nmin=2,τ=8) | 0.8267 | 0.7800 | 1.000 | ~1 µs |
| banking77 | H1 | 0.8060 | 0.8260 | 0.468 | ~380 µs |

Gate notes the record carries and the site tables do not:

- **banking77 G3 FAIL** — the registered H1 is not non-inferior to A0 at
  δ (mean +0.0200, UB95 +0.0466 vs δ 0.0320): the hybrid pays up to
  ~4.7 pt on banking77 at 95% confidence. No promotable hybrid arm on
  this suite; A0 0.8260 stands as the product posture.
- **ag_news G1 FAIL** — the fused readout is already well-calibrated
  (raw ECE 0.0133); the Platt refit moved it to 0.0324. Calibration
  hurt; the gate refuses. The raw readout stands.
- **massive bridge note** — the t20k test split carries 59 of the
  artifact's 60 intents (`cooking_query` has no test rows), so the
  train-derived cal front legitimately presents an artifact-known,
  seat-unknown option. The Issue-006 bridge refused that shape; the
  NaN-no-evidence extension (margin muted to 0, never a rival) resolves
  it — pinned by `h2_nan_evidence_mutes_the_margin_and_is_never_a_rival`.
- The v2 narrative "massive flips from A0 0.42" was an artifact of the
  old first-N sample's 30-of-60 label prefix (unrepresentative A0); at
  the stratified protocol the honest massive story is A0 0.7800 vs
  hybrid 0.8267 = +4.7 pt.

## Box state (the Issue-021 disclosure)

`power AC Power · powermode high · load1m 3.22 · swap_mb 2390.75 ·
quotable true` — preflight PASSED before the run (`PROVENANCE:
power=AC Power load=3.29 swap=2398.75M canary=skipped powermode=2(high)`).
Accuracy rows are box-independent; latency rows are this box at this
load class (read the laya face as magnitude: lane sub-2 µs vs laya
metal 118–187 ms p50 per escalated question).

## Artifacts

- `RESULTS.md` / `predictions.json` — the arena's frozen read.
- `registration.json` — the pre-registration tables.
- `hybrid_lane_doc.json` — the publish doc (built by
  `scripts/build_hybrid_doc.py` from `predictions.json`; regenerated
  with the landing sha — the `lane_sources.git_sha` on the site names
  the code that produced the numbers).
