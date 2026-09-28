# Bench record (arena) — harness_cache_reuse joins the arena population (Issue 010's OWED item)

**Status:** MEASURED — the single frozen test read; arms pre-registered on the
cal front (Pareto rank-0 + argmax Beta-LCB); predictions frozen in
`predictions.json`.

## What this run is

Issue 010's OWED item (filed 2026-09-28, deferred past the Issue-579 session's
specialist-bridge landing): `harness_cache_reuse` joins the arena SUITES and
carries its measured row. reflex Issue 045 REVERSED the old LLM-only carve-out
(a1980b2): the family is text-decidable ("does the described prefix still cover
the described next turn") and ships its authored corpus + cal; the cal-selected
noul polarity is the lever. reflex Bench 072 measured the modelless lane at
0.9167 — this run reproduces it at the seat.

- **`harness_cache_reuse`: A0 0.9167 (11/12), verdict `a0_stands`** — no
  specialist artifact (Issue 010 T2's no-winner posture). The A0 identity pin
  leg holds: arena == reflex `run()` == 0.9167 == reflex Bench 072's own row.
  The noul polarity cal-selected `Some(1)` (reuse-yes), scale 1
  observed-laplace — the seat's full knob grid armed it through the synthetic
  cal-front selection fallback, exactly as the OWED note predicted.
- Population is now 15 suites (was 14): the six new suites' rows (Bench 011)
  and the eight dataset suites' A0 rows reproduce their published records
  byte-identically (ag_news 0.8825 / emotion 0.8850 / sst5 0.3967 / xnli
  0.5233 / massive 0.7800 / banking77 0.8260 / typed 0.4655 / prompt 0.7672),
  and the serving arms reproduce (ag_news H2 0.8975 refused-pick registered,
  massive H2 0.8267 T2-certified, banking77 H2(nbsvm v2) 0.8540, typed A1
  0.6300, prompt A1 0.8534) — the Issue-045/078 sibling work moved nothing the
  arena reads except adding this suite.

## Disclosures

- **Run under `--skip-pin-a0`, deliberately (the Bench 012/015 precedent):**
  the pin's SITE leg is red on six dataset suites because the reflex-site
  checkout beside the workspace publishes reflex's modelless rows from a
  Bench-002-era record (`data/bench.json` `meta.git_sha: 8028a10`):
  ag_news 0.8625 / emotion 0.77 / sst5 0.2017 / xnli 0.5033 / massive 0.4067 /
  banking77 0.402 vs today's reflex `run()` 0.8825 / 0.885 / 0.3967 / 0.5233 /
  0.78 / 0.826. typed's site row (0.4655) matches the stale-pool state, not
  Bench 078's full-pool re-baseline (0.5725). That republish is reflex-site
  lane work (outside this workspace's roots — another session holds the
  checkout); the arena == reflex `run()` legs are 15/15 green, which is the
  half this lane owns. A first no-skip run of this exact tree wrote the six
  red site legs into the transcript before this record was written — the pin
  is working, the site is behind it.
- **The datasets dir is the frozen Bench-005 re-baseline pool**
  (`../riir-reflex/.raw/datasets_t20k`). reflex's canonical `.raw/datasets` is
  a DIFFERENT pool (ag_news 0.8625 / emotion 0.77 / sst5 0.2017 / xnli 0.5167 /
  massive 0.42 / banking77 0.396 / typed 0.5725 there) — one run of this bench
  was executed against it before the error was caught and re-run; its rows
  match no published record and it is NOT this record. Re-pointing the arena's
  default datasets dir is a re-baseline decision, never a cleanup (the hazard
  is now a comment at the arena's default).
- **typed_decisions reads the t20k pool's stale 800-row pull** (A0 0.4655,
  matching the site row) — the full-pool lane is Bench 015's scratch-dir read
  (A0' 0.5725), not this arena's pool. The pool refresh + republish is the
  reflex/078 site half's job; the arena picks it up when the canonical pool is
  re-baselined, not before.
- Box state (the arena's own capture): AC Power · powermode high · load1m
  4.07 · quotable true · refusals []. Latency rows quotable; accuracy rows are
  deterministic and byte-reproduced the published record regardless.

## The lane doc

`hybrid_lane_doc.json` (this dir) is rebuilt by
`scripts/build_hybrid_doc.py` from this run's `predictions.json` —
`harness_cache_reuse` carries the three-state `a0_stands` verdict with its
measured cell. The reflex-site republish (the card render, Issue 010 T4/T5's
site half) belongs to the reflex-site session; this doc is its input.

Box: M3 Max · recorded 2026-09-28.
