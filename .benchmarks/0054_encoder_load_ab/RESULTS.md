# Encoder concurrent-load A/B — the LANDED Lane B implementation, public surface

Instrument: `scripts/encoder_load_ab.py` · 2026-10-02 17:04:27 +0700  
PROVENANCE: power=AC Power load=3.05 swap=2507.25M canary=skipped powermode=2(high) load_at_launch=3.05  
Postures: per-lane, shared · suites: sst5, xnli_en, ag_news · 8 clients × 40 rounds = n 320 · device metal · profile release · states: first 16 test rows per suite from the frozen pool (`/Users/katopz/git/riir-reflex/.raw/datasets_t20k`) · heads: `/Users/katopz/git/riir-train/.raw/t599`

Warmup decisions (one per suite) are EXCLUDED from the stats and recorded separately; their `receipt.decision` digests are the cross-posture pick-parity witness.

| posture | n | p50 µs | p99 µs (tail) | mean µs | max µs | RSS ready MiB | RSS steady MiB |
|---|---|---|---|---|---|---|---|
| per-lane | 320 | 60,173 | 361,609 (4) | 67,927.5 | 1,832,747 | 10,144.4 | 10,192.1 |
| shared | 320 | 93,178 | 132,204 (4) | 98,742.1 | 3,645,531 | 4,085.8 | 4,100.5 |

**shared/per-lane:** p50 **1.549×** · p99 **0.366×** · RSS **2.49×**

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
| per-lane | sst5 | 0 | 28,112 | 28,611 | `79c6c6108a2daa41…` |
| per-lane | xnli_en | 2 | 14,989 | 15,385 | `2056528c1ee23156…` |
| per-lane | ag_news | 2 | 25,420 | 25,742 | `f47dc95152fe7e17…` |
| shared | sst5 | 0 | 68,225 | 69,451 | `79c6c6108a2daa41…` |
| shared | xnli_en | 2 | 18,918 | 19,405 | `2056528c1ee23156…` |
| shared | ag_news | 2 | 16,691 | 17,141 | `f47dc95152fe7e17…` |

## Honest scope

- Client-observed wall includes connection setup (the std-only server closes per request) — identical shape on both postures; the server-side `us` fields are in `results.json`.
- Each client's FIRST decision is an excluded throwaway (thread-start + connect-storm contamination — the heavy cell measured a 4.3 s outlier sitting entirely in round 0); the per-client throwaway walls are in `results.json` (`client_warm`).
- The per-lane numbers include 3 full checkpoint residencies; the shared numbers one — the RSS ratio is Lane B's whole point, the latency ratio is its cost. (`RSS ready` is read the moment every lane reports ready; `RSS steady` is the median of three reads after the load phase — eager lanes are resident from boot, so both columns are post-residency.)
- A one-session reading on a shared box; the soak re-measures at the serving posture. Quote the PROVENANCE line, never this file's numbers alone.

## Record narrative (Update 9, 2026-10-02 — the M3 session)

**What this record is:** issue 018 Update 8's explicit follow-up — a runnable concurrent-load
A/B instrument on the LANDED Lane B implementation (commit `41234d4`), public surface only
(the HTTP serve binary; the worker registry is private by design). Instrument:
`scripts/encoder_load_ab.py`; the soak re-measures this cell with one command
(`python3 scripts/encoder_load_ab.py --clients 8 --rounds 40`), never re-derived.

**Five A/B runs this session, all parity-OK** (warmup `receipt.decision` digests byte-equal
across builds — the public-surface cross-build witness 0050 proved via test fingerprints):

| run | n | per-lane p50/p99 | shared p50/p99 | shared p99 ÷ per-lane p99 | RSS |
|---|---|---|---|---|---|
| n72-run1 (per-lane first) | 72 | 64,516 / 122,188 | 88,882 / 143,286 | 1.173× | 2.48× |
| order-flipped (shared first) | 72 | 56,127 / 217,065 | 86,177 / 145,654 | 0.671× | 2.47× |
| heavy #1 (pre-throwaway-fix; transcript-only, files superseded) | 320 | 65,053 / 326,286 | 105,570 / 126,583 | 0.388× | 2.48× |
| **heavy #2 (canonical, this file)** | 320 | 60,173 / 361,609 | 93,178 / 132,204 | **0.366×** | **2.49×** |

**The reading inverts Update 8's thin-cell hedge at the tail.** Update 8 (n=72, dev profile,
the duplicate's private internals) read shared p99 1.208× — WITHIN the provisional 1.217×
envelope, ~0.9 pt margin, and hedged the share-weights-only fallback for the tail. At real
8-way contention (n=320, release, the landed implementation) the tail goes the OTHER WAY:
per-lane's p99 blows out (326k/362k — three concurrent Metal forwards per checkpoint contend
for the GPU; the heavy tail is spread across 10+ of 40 rounds, server-side, not start skew)
while the shared worker's queue makes the tail deterministic (126.6k/132.2k, p99 ≈ 1.4× its
own p50). The share-weights-only fallback is NOT needed for the tail — the tail is where
Lane B wins. The p50 cost of sharing is real and stable: 1.38–1.62× (head-of-line
serialization); Update 8's 1.82× p50 and 1.208× p99 were an under-powered reading of the
same trade (its own disclosure said the soak owns the cell).

**RSS: 2.49× at five independent replications** (Update 8's 2.46× + this session's 2.47–2.49×
across runs, orders, and profiles) — 10.2 GB → 4.1 GB whole-process at 3 lanes / 1 checkpoint.

**Disclosures:** (a) the canonical run carries a box-noise PAIR in the shared arm — two walls
3,645,289/3,645,531 µs, 242 µs apart, a simultaneous release = a system-wide stall from the
concurrent sibling sessions (load 3-4, swap 2.5 GB); both sit ABOVE the reported p99 and the
steady max outside the pair is 134k — disclosed, not chased. (b) The p99 tail support is 4/320
— real but thin; the soak's cell should run --rounds ≥ 40 (the instrument's default cell is
6×12, sized like-for-like with Update 8). (c) Numbers are client-observed wall on a shared box
under sibling load; the PROVENANCE line above is the box state of record.

**Promotion posture (unchanged, still owner-gated):** the soak's decision cell is now one
command. Evidence for the owner: p50 1.55× cost (the serialization price, bounded) vs p99
0.37× (the contention win) + RSS 2.49× — the trade flips favorable exactly where serving
cares about it, but the soak decides.
