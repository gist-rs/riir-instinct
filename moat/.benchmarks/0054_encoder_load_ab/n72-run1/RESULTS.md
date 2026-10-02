# Encoder concurrent-load A/B — the LANDED Lane B implementation, public surface

Instrument: `scripts/encoder_load_ab.py` · 2026-10-02 16:54:45 +0700  
PROVENANCE: power=AC Power load=3.24 swap=2507.25M canary=skipped powermode=2(high) load_at_launch=3.24  
Postures: per-lane, shared · suites: sst5, xnli_en, ag_news · 6 clients × 12 rounds = n 72 · device metal · profile release · states: first 8 test rows per suite from the frozen pool (`/Users/katopz/git/riir-reflex/.raw/datasets_t20k`) · heads: `/Users/katopz/git/riir-train/.raw/t599`

Warmup decisions (one per suite) are EXCLUDED from the stats and recorded separately; their `receipt.decision` digests are the cross-posture pick-parity witness.

| posture | n | p50 µs | p99 µs (tail) | mean µs | max µs | RSS boot MiB | RSS load MiB |
|---|---|---|---|---|---|---|---|
| per-lane | 72 | 64,516 | 122,188 (1) | 62,292.0 | 122,188 | 10,145.4 | 10,177.1 |
| shared | 72 | 88,882 | 143,286 (1) | 94,995.4 | 143,286 | 4,092.2 | 4,098.5 |

**shared/per-lane:** p50 **1.378×** · p99 **1.173×** · RSS **2.48×**

Context (NOT a gate): Update 8's provisional loaded-box p99 envelope 1.217× — the soak owns the decision cell; this instrument makes that cell re-measurable without re-deriving.

Decision parity across postures: **OK** — warmup `receipt.decision` digests equal on every suite (byte-level witness across builds)

## Posture fingerprints

| posture | binary sha256 | build | features |
|---|---|---|---|
| per-lane | `9e040c3767cb4d12…` | `01313b5244c3999e` | `["arena-laya", "default", "serve-encoder"]` |
| shared | `a66cce1858559c65…` | `75c399d28ac6b90a` | `["arena-laya", "default", "serve-encoder", "serve-encoder-shared"]` |

## Warmup probes (excluded from stats)

| posture | suite | pick | server µs | wall µs | decision b3 |
|---|---|---|---|---|---|
| per-lane | sst5 | 0 | 36,853 | 37,348 | `79c6c6108a2daa41…` |
| per-lane | xnli_en | 2 | 29,594 | 30,039 | `2056528c1ee23156…` |
| per-lane | ag_news | 2 | 29,128 | 29,558 | `f47dc95152fe7e17…` |
| shared | sst5 | 0 | 51,966 | 52,608 | `79c6c6108a2daa41…` |
| shared | xnli_en | 2 | 15,561 | 16,087 | `2056528c1ee23156…` |
| shared | ag_news | 2 | 15,103 | 15,583 | `f47dc95152fe7e17…` |

## Honest scope

- Client-observed wall includes connection setup (the std-only server closes per request) — identical shape on both postures; the server-side `us` fields are in `results.json`.
- The per-lane numbers include 3 full checkpoint residencies; the shared numbers one — the RSS ratio is Lane B's whole point, the latency ratio is its cost.
- A one-session reading on a shared box; the soak re-measures at the serving posture. Quote the PROVENANCE line, never this file's numbers alone.
