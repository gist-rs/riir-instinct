# Issue 017 — seat the xnli_en encoder cell (record-only, serve ✗): the owner's progress-display call

**Status:** OPEN 2026-10-01 — the owner call reversing the 014-round-3
"no separate cell" decision for xnli_en; the arena read + seating execute
under this issue. The 4090 is NOT touched (a sibling's L4 train holds it;
the read runs on m3 Metal).

## The owner call (2026-10-01, verbatim intent)

> "-4pt is better than −37.7pt rn? why can't we use as is and get better
> result later? currently it look like no progress at all and -4pt gimme
> some hope even next try failed we still get -4pt ?"

Decision: **seat the measured xnli encoder cell on the board as-is** —
record-only, `serve: ✗`, exactly the sst5 C1 posture. This reverses the
014-round-3 sub-decision ("no separate xnli encoder cell is published at
all") on product grounds: the Rethink column showing "not run" reads as
zero progress, while the seated cell shows the product's real posture
(+33.7 pt over A0, −4.0 pt under openthai). The attribution honesty is
KEPT as a disclosure on the cell, not as a reason to hide it.

What is NOT reversed:
- The 014 class-wide encoder-serve refusal stands (no serve change; A0
  keeps serving xnli).
- The attribution record stands: the head ties the laya-english reference
  300/300 picks (riir-train 599 T5a); the cell carries that disclosure
  ("reference-identified") so nobody reads 0.8600 as the trainer's lift.
- riir-train 599 stays the measured record; riir-train 603 (the
  multilingual screen) is unaffected — a negative there leaves this cell
  standing.

## Tasks

- [x] T1 — arena read: `LAYA_DEVICE=metal cargo run --release --bin arena
      --features arena-laya-metal -- --suite xnli_en --encoder-art
      ../riir-train/.raw/t599/xnli_en_encoder_v1.bin --skip-pin-a0`
      (preflight per the Issue-021 law first). Expect **0.8600 (258/300)**
      per 599 T5a — the cell-identity witness (the third-posture law:
      4090 cache → M3-Metal dump → live arena encode must all agree).
      ✅ EXECUTED 2026-10-01: preflight REFUSED first (load 18.64, siblings
      active — accuracy-only posture would have been the fallback), then
      the box cleared and the PREFLIGHT-CLEARED read ran:
      `PROVENANCE: power=AC Power load=2.38 swap=3519.00M canary=skipped
      powermode=2(high)` → **ENC 0.8600 (258/300), cell-identical to 599
      T5a** — the third-posture witness HOLDS (4090 cache-witness →
      M3-Metal dump → live arena encode, all three identical).
      vs A1: paired mean +0.4500 · LB95 +0.3832. Latency p50 231,709 µs /
      p99 428,943 µs per row (encode + head; xnli's long premise+hypothesis
      rows — ms-class, the recorded ground of the 014 refusal).
      Artifact BLAKE3-verified at load: `a1e2380b7c6451bb…` (the 599 pin).
      A0 0.5233 == published (the pin's reflex-run() leg held).
- [x] T2 — bench record 036 + the lane-doc entry (the `encoder` cell:
      lane Rethink, model ENC-xnliv1, n=300, acc 0.8600, latency rows
      QUOTABLE (preflight-cleared, box_state span shape), `serves: ✗`,
      `gate` carrying the reference-identified disclosure + the owner call).
      ✅ `.benchmarks/036_xnli_encoder_cell_seating/` (RESULTS.md +
      predictions.json + registration.json + hybrid_lane_doc.json).
- [x] T3 — reflex-site publish (the encoder lane renders "Rethink"; the
      site half: publish_bench + smokes) + instinct docs rows (AGENTS/
      HISTORY) recording the reversal.
      ✅ Lane-scoped publish `PUBLISH_BENCH_LANES=encoder` over
      bench.json-primary + the 036 lane doc: publish ✓ · pairing gate ✓
      (17 same-sample pairs) · mirror parity ✓ · bench-page smoke ✓ ·
      chart smoke ✓. The Rethink lane is now **2/9 partial** (coverage
      {suites: 2, of: 9}; areas: sentiment 0.408 + reasoning 0.79, index
      0.599). En-route smoke repairs (all stale-hardcode class, exposed by
      the lane growing 1→2 suites): bench_page_smoke "1/9"→"2/9" ·
      chart_render_smoke "1/9"→"2/9" + the suites-card polyline counter
      was DEAD CODE (`numOk(per_suite[n])` on an {acc,cc} object is always
      false — never counted the all-benchmarks card; repaired to mirror
      the renderer's read) · bench-charts.js + bench/index.html partial-
      lane prose ("one suite"→"two suites", "single-suite partial"→
      "two-suite partial").
- [ ] T4 — commit + push both repos; ref the commit hashes back into this
      file; close.
