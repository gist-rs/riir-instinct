# Encoder concurrent-load A/B — the LANDED Lane B implementation, public surface

Instrument: `scripts/encoder_load_ab.py` · 2026-10-02 16:56:09 +0700  
PROVENANCE: power=AC Power load=4.39 swap=2507.25M canary=skipped powermode=2(high) load_at_launch=4.39  
Postures: shared, per-lane · suites: sst5, xnli_en, ag_news · 6 clients × 12 rounds = n 72 · device metal · profile release · states: first 8 test rows per suite from the frozen pool (`/Users/katopz/git/riir-reflex/.raw/datasets_t20k`) · heads: `/Users/katopz/git/riir-train/.raw/t599`

Warmup decisions (one per suite) are EXCLUDED from the stats and recorded separately; their `receipt.decision` digests are the cross-posture pick-parity witness.

| posture | n | p50 µs | p99 µs (tail) | mean µs | max µs | RSS boot MiB | RSS load MiB |
|---|---|---|---|---|---|---|---|
| shared | 72 | 86,177 | 145,654 (1) | 94,844.8 | 145,654 | 4,096.7 | 4,106.2 |
| per-lane | 72 | 56,127 | 217,065 (1) | 61,227.5 | 217,065 | 10,145.3 | 10,175.2 |

**shared/per-lane:** p50 **0.651×** · p99 **1.490×** · RSS **0.40×**

Context (NOT a gate): Update 8's provisional loaded-box p99 envelope 1.217× — the soak owns the decision cell; this instrument makes that cell re-measurable without re-deriving.

Decision parity across postures: **OK** — warmup `receipt.decision` digests equal on every suite (byte-level witness across builds)

## Posture fingerprints

| posture | binary sha256 | build | features |
|---|---|---|---|
| shared | `a66cce1858559c65…` | `75c399d28ac6b90a` | `["arena-laya", "default", "serve-encoder", "serve-encoder-shared"]` |
| per-lane | `9e040c3767cb4d12…` | `01313b5244c3999e` | `["arena-laya", "default", "serve-encoder"]` |

## Warmup probes (excluded from stats)

| posture | suite | pick | server µs | wall µs | decision b3 |
|---|---|---|---|---|---|
| shared | sst5 | 0 | 40,398 | 41,031 | `79c6c6108a2daa41…` |
| shared | xnli_en | 2 | 15,467 | 15,995 | `2056528c1ee23156…` |
| shared | ag_news | 2 | 14,958 | 15,431 | `f47dc95152fe7e17…` |
| per-lane | sst5 | 0 | 45,369 | 45,949 | `79c6c6108a2daa41…` |
| per-lane | xnli_en | 2 | 30,919 | 31,422 | `2056528c1ee23156…` |
| per-lane | ag_news | 2 | 30,744 | 31,222 | `f47dc95152fe7e17…` |

## Honest scope

- Client-observed wall includes connection setup (the std-only server closes per request) — identical shape on both postures; the server-side `us` fields are in `results.json`.
- The per-lane numbers include 3 full checkpoint residencies; the shared numbers one — the RSS ratio is Lane B's whole point, the latency ratio is its cost.
- A one-session reading on a shared box; the soak re-measures at the serving posture. Quote the PROVENANCE line, never this file's numbers alone.
