# Issue 014 — sst5 encoder-arm seating: lane confirmed at 0.5267, serve posture is the owner gate

**Status:** DECIDED 2026-09-30 — **conditional staged GO, class-wide** (owner
ratified after the Claude verdict ping-pong: REVISE → AGREE, 3 rounds).
**The serve-posture gate is CLOSED — agents: do not re-ask it.** What
remains is C1/C2 execution (see §Decision). Filed 2026-09-30 (riir-train
issue 600 T6 confirmed; instinct 008 T7 wave-3).

## The measured verdict (riir-train 600, `6f24cd92`)

- **The lane**: encoder-feature class (the laya-english encoder's cached
  states → standardized features → 128-wide sigmoid MLP head), trained
  gold-only on the 8544-row sst5 train pool.
- **The frozen test read: 0.5267 (316/600)** — pre-registered protocol
  (class-relative-only earn gate, 5-head pool-side sweep, best-holdout
  selection, ONE read): +8.8 pt over the 0.4383 gliner bar, +15.5 pt over
  its own reference read (0.3717, reproduced exactly — cache witness),
  above the T2 probe's ungated 0.5183.
- **The incumbent serve**: sst5's bag arm A1 serves **0.4217** (the 008
  T7 wave-1 verdict; A0 = free Reflex reads 0.3967 on the t20k seat).
- A 0.5267-class encoder arm would be sst5's first strictly-superior
  specialist: **+10.5 pt over the incumbent arm, +8.8 over the best
  published lane** (gliner 0.4383). This is the ONLY suite in the 008
  board with a confirmed above-bar lane.

## The owner question

Build the **encoder-backed arm class** in riir-instinct's serving path, or
keep sst5 on the bag arm?

**What the class costs at serve** (the same pricing as the xnli T5 gate):

- The arm needs the laya-english encoder at serve time: the heavyweight,
  GPU-backed posture (~2 GB VRAM class) — a `riir-infer-laya` dependency
  in the serving binary, encoder inference per decision request.
- The alternative recorded for xnli applies here too, with the same
  precondition: the head read 0.5267 while the UNTRAINED reference read
  0.3717 — unlike xnli (where reference logits == the trained head), the
  TRAINED HEAD is the value here; a reference-logits-only serve posture
  would serve 0.3717-class, NOT 0.5267. So sst5 has no cheap
  logits-carrying candidate: the encoder+head inference (or a distilled
  surrogate of it) is load-bearing.
- Bag-arm status quo: 0.4217 at zero added serve cost.

**If GO**: the arm reader shape follows 008's wave-2 note — an
encoder-backed `winner_bridge` arm (bag artifacts today; this class needs
the encoder + NLEH v1 head replay at serve), the winner artifact
`.raw/t599/t6_s0.bin` (BLAKE3-sealed, riir-train side), the sst5 test
split's 0.5267 as the registered posture number, and the 008 T2 product
gate (strictly above the incumbent on the frozen read — already measured
at +10.5 pt, pending the seat's own replay).

**If NO-GO**: sst5 stays on A1 (0.4217); the lane verdict stands as
recorded; this issue closes with the decision noted in 008's T7 row.

## Scope notes

- The arena/seat protocol (the t20k pool vs the reflex harness split) —
  the 0.5267 read is on the reflex-harness sst5 test split (600 rows, the
  same split every published lane bar was measured on). The 008 board's
  sst5 row (0.4217) is the t20k-seat read; the +10.5 pt comparison is
  cross-pool (bar-split vs seat). A seated arm's own arena read under the
  008 protocol is part of the GO path, not a premise of this gate.
- The trainer substrate is suite-generic (riir-train
  `instinct_encoder_trainer`, `--min-holdout`/`--seed`); re-training at a
  new pool is one command, not a lane rebuild.

## Decision — 2026-09-30: conditional staged GO, class-wide

Ratified by the owner after a 3-round Claude verdict ping-pong (round 1
REVISE — two corrections ACCEPTED: the "1 ms serve bar" is the TETRIS bar
(issue 009), not a text-lane bar, so it does not bind here; and a
bag-class distillation student repeats the Tetris round-6 self-distillation
trap — soft labels change the target, not what bag features can express.
Round 2 AGREE, final; round 3 closed with four pre-registration pins).

This ONE decision settles the encoder-class serve posture for **sst5
(this issue), the xnli T5 gate (same pricing), and the 008 T6 re-scope**.
009 is unaffected. Agents reading this issue: the gate is decided — do not
re-open the serve-posture question; the remaining work is C1/C2 below.

1. **REFUSED — per-request encoder inference at serve (whole encoder
   class).** Grounds are MEASURED text-lane numbers, not the Tetris bar:
   the serve deploys as a CPU-only standard-2 cf-container (x86-64
   zigbuild — a ~2 GB GPU encoder cannot live there at all), and CPU-lane
   encoder at ~157 ms/row is ~540× A0's 290 µs measured class (~78,000×
   A1's 2 µs); GPU-posture 8–15 ms is still 27–50× A0 and no GPU tier
   exists in the deploy shape. Working latency gate: **≤ A0-class
   (~300 µs), PROVISIONAL** until the owner sets an explicit text-lane
   bar — do not let it harden into a law by default.
2. **C1 — record, don't serve.** Arena-side encoder-arm reader; sync the
   sealed head (`.raw/t599/t6_s0.bin`) from the 4090 (one scp, no GPU time
   needed there); ONE frozen arena read on M3 Metal (~minutes; weights +
   dataset already cached on that box). The seat read settles ONLY the
   cross-pool claim (+10.5 vs A1 → the T2 paired LB95 gate). The +8.8 over
   gliner 0.4383 is same-split and stands as recorded — C1 does not
   re-open it. Record per-row latency + box state beside the accuracy.
   Publish the cell as `serve: ✗ (encoder class refused at serve)`. No
   serve change.
3. **C2 — static-vector surrogate is the primary rung.** Model2Vec-style
   static per-token vectors from the laya-english encoder's own output
   states, mean-pooled at serve, 128-wide sigmoid head on top — lookup +
   mean + small MLP, µs-class, container-safe. Train on soft labels over
   UNLABELLED in-domain volume beyond the 8544 gold rows (the 600-row
   test split quarantined; the teacher's selection protocol — 5-head
   sweep, best-holdout — stays frozen/spent). The bag-class surrogate is
   pre-registered as the expected-negative control, NOT the main bet.
   Gates: `> 0.4383` (gliner bar) on its own frozen read AND T2 paired
   LB95 > 0 vs A1 AND latency ≤ ~300 µs (provisional).
4. **Failure path:** sst5 stays on A1 (0.4217), both numbers on record,
   this issue closes.

**Pre-registration pins (binding before any C2 run):**

- ONE static-vector extraction method, chosen in advance — corpus-
  averaged contextual states is the default candidate (stronger for
  sentiment); never both-then-pick against the frozen split (a hidden
  extra selection read).
- "Same 128-wide head" = same ARCHITECTURE, fresh fit. Pooled static
  vectors have a different input distribution — teacher head weights are
  NOT reusable. Standardized pooled inputs, own pool-side holdout for
  selection.
- Frozen-split read budget: TWO total (static-vector surrogate + bag
  control). Nothing else touches the 600-row split.
- Shrink the vocab×d table Model2Vec-style (PCA ~256 dims + quantization)
  BEFORE the latency gate; the serve path gains the laya TOKENIZER-only
  dependency (not the encoder) — run boundary-guard against BOUNDARY.md
  before that dep lands.
- **M3-side replay (2026-09-30, post-sync)** — the frozen read now has a
  second-box witness: `t6_s0.bin` + the test cache synced from the 4090,
  then re-read on the M3 (riir-train `instinct_encoder_eval`):
  - **Cache-witness posture** (4090 bytes, M3 CPU eval): winner **0.5267**
    (316/600), reference **0.3717** (223/600) — both confusion matrices
    cell-identical to the 4090 read. The sealed artifact + eval path are
    box-independent.
  - **M3-Metal posture** (fresh `dump_encoder_states --checkpoint english`
    encode of the same 600 cases on M3 Metal, d 1024): winner **0.5267**
    (316/600), reference **0.3717** (223/600) — again cell-identical. The
    Metal-vs-CUDA feature drift flips ZERO cell-level outcomes in either
    arm, so the serve posture's per-request encoder inference is not
    CUDA-bound. Encode cost: 600 rows in 21.7 s (~36 ms/row) on a loaded
    box (a sibling rustc held one core at 100% throughout; AC power,
    battery 100%).
  - This is the TRAIN-side replay, not the seat's own arena read — that
    read stays part of the GO path (owner-gated) as scoped above.
