# Issue 014 — sst5 encoder-arm seating: lane confirmed at 0.5267, serve posture is the owner gate

**Status:** OPEN — OWNER GATE (the T5 pattern: build the encoder-backed arm
class, or keep the incumbent bag arm). The lane verdict is MEASURED and
closed; nothing here blocks on agent work. Filed 2026-09-30 (riir-train
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
