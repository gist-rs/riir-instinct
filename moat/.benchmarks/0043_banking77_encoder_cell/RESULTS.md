# Bench 043 — the banking77 Rethink cell (record-only, screened-dead class measured anyway)

**Status:** MEASURED — one trained head, one frozen read, RECORD-ONLY
(`serve: ✗`). Claude verdict round 1 call 3: the never-earned heads are
trained anyway for board completeness (owner instruction: "can we finish
Rethink all bench... i need the result tmr"), with the screen's DEAD
verdict carried on the cell; the screen floor (0.4980, Bench 039) was the
zero-compute fallback and is superseded by this measured cell.

**The head:** `banking77_encoder_v1.bin` (41,573,700 B, blake3
`f20b687f…`) — gold-only A arm, NLEH v1, riir-train t608 train pool
(9993 rows, the case-maker wire verbatim), holdout 2000, `--min-holdout
0.0` (the class-relative gate; the absolute 0.75 bar is meaningless on a
77-way suite whose best published lane reads 0.8540). Earn gate MET:
holdout **0.4500 > ref holdout 0.4325** → the single frozen test read
earned.

**The frozen read (train-side):** winner **0.3860 (193/500)** vs the
reference logits 0.4980 (249/500) — the head LOSES to its own reference
on test (the 600-T3 massive pattern: the pool-side holdout gain does not
transfer; the 77-way presented-option distribution the head saw on the
pool does not transfer). Below even the 039 screen floor's 0.4980.

**The arena read (the published cell):** `--suite banking77
--encoder-art ../riir-train/.raw/t608/banking77_encoder_v1.bin
--encoder-ckpt english --skip-pin-a0` · LAYA_DEVICE=metal. **ENC 0.4420
(221/500)** vs A1 −0.3860 (LB95 −0.4363) · p50 42,983 µs.

⚠ **Disclosed divergence (28 questions):** the arena seat-path read
(0.4420) and the train-side maker-path read (0.3860) differ on the same
head — the known maker-vs-builder wire divergence class (the sst5/typed
cells' cell-identity witnesses held because those makers matched the
builders byte-for-byte; banking77's maker diverges somewhere in the
rendered question wire). Both readings are far below the 0.8540
incumbent; the DEAD verdict is reading-independent. The seat-path number
is the published cell (the serve-path law). A maker/builder byte-diff is
the recorded follow-up if this suite's lane is ever re-opened.

**Record-only:** the suite serves H2 (nbsvm v2, 0.8540); this cell never
sells and never advertises — the verdict fold stands: nothing about this
cell reads as a capability.
