# Encoder concurrent-load A/B — the LANDED Lane B implementation, public surface

Instrument: `scripts/encoder_load_ab.py` · 2026-10-02 18:44:05 +0700  
PROVENANCE: power=AC Power load=4.94 swap=2499.25M canary=skipped powermode=2(high) load_at_launch=4.94  
Postures: per-lane, shared · suites: sst5, xnli_en, ag_news · 8 clients × 40 rounds = n 320 · device metal · profile release · states: first 8 test rows per suite from the frozen pool (`/Users/katopz/git/riir-reflex/.raw/datasets_t20k`) · heads: `/Users/katopz/git/riir-train/.raw/t599`

Warmup decisions (one per suite) are EXCLUDED from the stats and recorded separately; their `receipt.decision` digests are the cross-posture pick-parity witness.

| posture | n | p50 µs | p99 µs (tail) | mean µs | max µs | RSS ready MiB | RSS steady MiB |
|---|---|---|---|---|---|---|---|
| per-lane | 320 | 66,705 | 365,360 (4) | 77,679.8 | 994,558 | 10,137.2 | 10,168.6 |
| shared | 320 | 95,937 | 134,389 (4) | 107,438.8 | 3,980,436 | 3,240.4 | 3,254.2 |

**shared/per-lane:** p50 **1.438×** · p99 **0.368×** · RSS **3.12×**

Context (NOT a gate): Update 8's provisional loaded-box p99 envelope 1.217× — the soak owns the decision cell; this instrument makes that cell re-measurable without re-deriving.

Decision parity across postures: **OK** — warmup `receipt.decision` digests equal on every suite (byte-level witness across builds)

## Posture fingerprints

| posture | binary sha256 | build | features |
|---|---|---|---|
| per-lane | `0fbc6272f4fb9135…` | `01313b5244c3999e` | `["arena-laya", "default", "serve-encoder"]` |
| shared | `fe7987fa626d5c16…` | `75c399d28ac6b90a` | `["arena-laya", "default", "serve-encoder", "serve-encoder-shared"]` |

## Warmup probes (excluded from stats)

| posture | suite | pick | server µs | wall µs | decision b3 |
|---|---|---|---|---|---|
| per-lane | sst5 | 0 | 44,772 | 45,377 | `79c6c6108a2daa41…` |
| per-lane | xnli_en | 2 | 30,877 | 31,446 | `2056528c1ee23156…` |
| per-lane | ag_news | 2 | 29,195 | 29,766 | `f47dc95152fe7e17…` |
| shared | sst5 | 0 | 37,144 | 37,748 | `79c6c6108a2daa41…` |
| shared | xnli_en | 2 | 15,282 | 15,805 | `2056528c1ee23156…` |
| shared | ag_news | 2 | 14,710 | 15,178 | `f47dc95152fe7e17…` |

## Honest scope

- Client-observed wall includes connection setup (the std-only server closes per request) — identical shape on both postures; the server-side `us` fields are in `results.json`.
- Each client's FIRST decision is an excluded throwaway (thread-start + connect-storm contamination — the heavy cell measured a 4.3 s outlier sitting entirely in round 0); the per-client throwaway walls are in `results.json` (`client_warm`).
- `RSS ready` is read the moment every lane reports ready; `RSS steady` is the median of three reads after the load phase — eager lanes are resident from boot, so both columns are post-residency; the per-lane numbers carry 3 full checkpoint residencies, the shared numbers one (the RSS ratio is Lane B's whole point, the latency ratio is its cost).
- A one-session reading on a shared box; the soak re-measures at the serving posture. Quote the PROVENANCE line, never this file's numbers alone.

## ADDENDUM — the serving-soak confirmation run (2026-10-02 18:44, the promotion gate)

This file's body above is the **confirmation run** — the instrument REGENERATES
RESULTS.md/results.json on every invocation (its documented behavior; the
canonical run's version of this file is `b6d40de`, and its numbers are quoted
in issue 018 Update 9). This addendum records both runs side by side; this
run is the serving-soak green receipt the promotion cites (issue 018 Update
10).

| run | p50 ratio | p99 ratio | RSS ratio | parity |
|---|---|---|---|---|
| canonical (`b6d40de`, 8×40, load 3.05) | 1.549× (60,173/93,178) | **0.366×** (361,609/132,204) | 2.49× (10,192/4,100) | OK |
| confirmation (this body, 8×40, load 4.94) | 1.438× (66,705/95,937) | **0.368×** (365,360/134,389) | 3.12× (10,169/3,254) | OK |

Both runs read the same direction at every axis: the p50 cost is bounded
(1.44–1.55×), the p99 tail is where the shared worker WINS (0.37× — the
per-lane p99 blows out under concurrent Metal forwards, the shared queue
keeps it deterministic), RSS 2.5–3.1× better, decision parity byte-equal
across builds. The confirmation's shared-arm max (3,980,436 µs) is one
box-noise wall above the p99 (sibling sessions active at load 4.94, the same
disclosed class as the canonical run's 3,645,289/3,645,531 pair) — disclosed,
not hidden; the p99 tail support is 4/320 in both runs.

**SOAK VERDICT: GREEN** — every Lane B promotion gate measured (parity 0050
fingerprints + through-HTTP here · RAM 2.5–3.1× · mixed-load p50 bounded +
p99 favorable · fairness cap unit-gated · the soak cell reproduced
four times favorable across two sessions). The promotion landed the same day
(issue 018 Update 10).
