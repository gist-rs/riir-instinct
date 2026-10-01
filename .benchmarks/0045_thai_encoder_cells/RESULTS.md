# Bench 045 — the thai Rethink cells (record-only reference reads; the multilingual substrate's first board cells)

**Status:** MEASURED — train-side protocol (no arena seat exists: the
arena's SUITES registry carries 15 suites, thai not among them, and the
suites are unsold in every instinct lane — no incumbent A1 row to pair
against). Cells = the laya-MULTILINGUAL checkpoint's REFERENCE reads;
both trained heads REFUSED the class-relative gate. Publishable per
riir-train issue 603's own rule ("whatever the read, the reference number
itself is publishable as the laya lane's multilingual row").

**The chain (executed 2026-10-02, M3 metal):**
1. Case-makers written mirroring the reflex builders verbatim:
   `riir-train/.raw/t599/make_thai_wisesight_cases.py` (4 fixed criteria
   in ClassLabel order WITH descriptions, gold = int category) and
   `make_thai_sib200_cases.py` (sorted-unique category universe, gold =
   key position).
2. Encodes: multilingual checkpoint, test 400 + train 4000 (wisesight),
   test 204 + train 701 (sib200); encode walls 6.3 s / 91.9 s / 3.6 s /
   11.9 s.
3. Head fits (gold-only, NLEH, holdout 800 / 200, `--min-holdout 0.0`):
   wisesight holdout 0.4213 < ref 0.4550 → **REFUSED** (probe artifact
   only); sib200 holdout 0.6800 < ref 0.7200 → **REFUSED**.
4. Reference reads (the cells): **thai_wisesight 0.4075 (163/400)** ·
   **thai_sib200 0.7843 (160/204)**.

**Honest reading:** below the openthai comparison lane on both suites
(0.475 / 0.838) — the multilingual encoder is not yet competitive on
thai, and no head lifts it (the 600 law: the head does not rescue a
reference from below). The cells exist so the board says a MEASURED
number instead of "not run"; they never sell (record-only, `serve: ✗`).

**Box state:** M3 Max, AC power, high power mode, loaded box (a sibling
profiler + this session's cargo builds throughout) — the encode walls are
context, not quotable latency figures; no p50/p99 is published for these
cells (the train-side protocol measures no per-row percentile and none is
invented from a mean — `latency_quotable` carries the refusal).

**XNLI multilingual rider (riir-train issue 603, executed this session on
the M3 metal posture — the issue's 4090-only blocker did not apply; the
600 T6 cross-box cell-identity precedent covers Metal):** the
multilingual reference on the frozen n=300 xnli_en split reads **0.8433
(253/300)** ≤ the english reference 0.8600 → **the 603 lane is DEAD BY
LAW** (the pre-registered rule's first arm); xnli stays closed
record-only; no head run earned. The verdict + the M3-posture deviation
are recorded in 603's file. Gold dist [100,100,100]; recalls
0.7300/0.8900/0.9100; cache
`.raw/t599/xnli_test_lenc_multilingual_m3metal.bin` (6.6 s / 300 rows).
