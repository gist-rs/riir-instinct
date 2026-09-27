# Bench 012 — the Issue 579 T3 arena bridge: banking77 nbsvm v2 seated + registered

**Status:** MEASURED — 2026-09-28. Train-side lane: riir-train Issue 579 /
Bench 612 (winner minted there); this record is the consumer half — the
arena's single frozen test read through the bridge, and the serving
registration. Records: `RESULTS.md` / `predictions.json` / `registration.json`
(arena-written, deterministic — the six dataset suites reproduced run 1
byte-identically).

## The bridge (what landed)

The v2 winner changes TWO things about seating banking77, and both now
resolve through ONE home — `specialist::winner_bridge(suite)`:

- **File**: `banking77_nbsvm_v2.bin` (Bench 612 minted it deliberately NOT
  under the `winner_v1` convention — winner names couple to the consumer;
  the 578 lesson). The manifest row carries the matching `file` override +
  digest `blake3:693b0d6c…43e37` (= Bench 612's mint digest, byte-verified
  here). The refused v1 file stays on disk for reproduction but no longer
  ships (`deploy.yaml` rows moved in the same change).
- **Bag convention**: L2-normalized PRESENCE bags (`presence_bag_into`,
  the train-side `instinct_nbsvm::presence_bag_into(norm=true)` mirror) —
  the scaling is folded into the exported weights, so `Specialist` scores
  verbatim. Every bag-building site (arena `SuiteCtx`, serve
  `SuiteServer::decide`, the hoard-gate centroid) dispatches through
  `BagConvention`; the other five suites stay count-bag (v1).

Coupling enforcement (raw mode): `check_winner_file` refuses any boot /
swap that names a file other than the bridge's for a bridged suite — the
convention cannot silently serve over weights not trained under it. Vessel
files are exempt by convention (`<suite>_v1.vessel`); a banking77 vessel
must be re-minted from the v2 artifact by its producer (riir-train
`vessel-mint`) before the vessel lane serves this posture.

## The frozen read (n = 500 test questions)

| arm | acc | note |
|---|---|---|
| A0 | 0.8260 | == the published reflex row (A0 pin: arena == reflex `run()` == site ✓) |
| A1 | 0.8280 | the v2 specialist alone — every arm now reads ≥ A0 (v1: A1 0.7960 lost) |
| H1 (top-k 8) | 0.8320 | |
| **H2(β=2, nmin=8, τ=8)** | **0.8540** | instrument pick; paired vs A0 mean +0.0280 |

Gate faces on the pick: **T2 REFUSED** (paired LB95 **−0.0013** — a hair
under the bar; the strict-superiority law stays the ADVERTISING gate) ·
G1 **FAIL** (raw 0.0841 / platt 0.1109 / floor 0.3103 — the fused
readout's calibration face, the same face ag_news's served H2 carries) ·
G3 PASS (UB95 +0.0013 ≤ δ 0.0357).

## The serving decision

Under the owner's serving law (arsenal.toml header; `serving law, owner
verdict 2026-09-27`) the served arm is the BEST MEASURED arm, A0 a
candidate: banking77 flips to **H2(2,8,8) 0.8540** — the same uncertified-
under-best-measured class as ag_news H2 (+1.5 pt, LB95 −0.0100) and sst5
A1 (+2.5 pt, LB95 −0.0129). The manifest row, `PINNED_MANIFEST_DIGEST`
(`blake3:5d334ec2…94440`), and the serve-gates posture table all moved in
this change; all 11 serve-gate tests green (incl. the manifest validation
against the real artifact bytes and the ag_news frozen-picks parity).

The 579 train-side holdout edge (+2.9 pt at LB95 +0.66 pt) transferred:
the test read lands at +2.8 pt — the presence-bag convention is
byte-faithful end to end.

## Pin status (honest disclosure)

- The pin-ON run (run 1) printed **A0 identity + site ✓ for all NINE
  dataset suites** (ag_news 0.8825 / emotion 0.8850 / sst5 0.3967 /
  xnli 0.5233 / massive 0.7800 / banking77 0.8260 / typed 0.4655 /
  prompt 0.7672 / code_fixtures 0.3750) — then DIED refusing to write
  records: the SIX HARNESS FAMILIES' site check reads reflex's in-flight
  tree past the PUBLISHED site rows (e.g. harness_visibility 0.5625 vs
  published 0.375). That is the reflex sibling's Issue-045 landing (the
  synthetic selection-slice + cache_reuse modelless lane, uncommitted in
  `../riir-reflex` during this window) postdating the published
  bench.json — not this lane's drift; the families reconcile when 045
  lands with its site republish.
- Records were therefore written under `--skip-pin-a0` (run 2). The
  dataset-suite pin evidence stands from run 1's log (byte-identical
  accuracies across both runs, all six dataset suites).

Box state (run 1, RESULTS.md): AC, high power mode, load 16.1 (a sibling
build active) — accuracy rows are deterministic and identical across both
runs; latency rows are load-affected disclosures, not gates here.

## Reproduce

```sh
CARGO_TARGET_DIR=/tmp/p579_bridge cargo run --release -p riir-instinct --bin arena -- \
  --out .benchmarks/012_banking77_nbsvm_v2_bridge   # + --skip-pin-a0 until reflex 045 lands
CARGO_TARGET_DIR=/tmp/p579_bridge cargo test -p riir-instinct --test serve_gates
```
