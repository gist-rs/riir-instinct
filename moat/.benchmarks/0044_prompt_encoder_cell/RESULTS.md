# Bench 044 — the prompt_injections Rethink cell (record-only; the one head that beat its own reference)

**Status:** MEASURED — one trained head, one frozen read, RECORD-ONLY
(`serve: ✗`). Claude verdict round 1 call 3 conditions applied: holdout
pick, one frozen read, the screen's DEAD verdict on the cell, and **the
cell reads as a benchmark measurement, never a security capability** (the
`serves`/`gate` prose carries that verbatim; nothing serves).

**The head:** `prompt_encoder_v1.bin` (1,599,000 B, blake3 `098ec5bd…`) —
gold-only A arm, NLEH v1, riir-train t608 train pool (546 rows, the
case-maker wire verbatim), holdout 150, `--min-holdout 0.0`. Earn gate
MET: holdout **0.8733 > ref holdout 0.7800** → the single frozen test
read earned.

**The frozen read (train-side):** winner **0.8017 (93/116)** vs the
reference logits 0.6983 (81/116) — the head BEATS its own reference by
+10.3 pt on test (the sst5 pattern: lift where the reference is far from
its ceiling). Still below the incumbent A1's 0.8534 (the Bench-039 bar).

**The arena read (the published cell):** `--suite prompt_injections
--encoder-art ../riir-train/.raw/t608/prompt_encoder_v1.bin
--encoder-ckpt english --skip-pin-a0` · LAYA_DEVICE=metal. **ENC 0.8017
(93/116)** — EXACTLY the train-side read (the cell-identity witness
holds on this suite) · vs A1 mean −0.0517, LB95 −0.1343 · p50 13,873 µs.

**Record-only:** the suite serves A1 (0.8534); this cell never sells.
Honest reading: the encoder class is the wrong shape for this suite's
bar — a 2-way noul gate at 0.8017 is below a specialist bag arm that
serves 0.8534, and nothing deploys. The cell exists so the board says a
MEASURED number instead of "not run".
