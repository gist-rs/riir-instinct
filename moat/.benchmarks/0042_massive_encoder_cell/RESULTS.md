# Bench 042 — the massive_intent_en Rethink cell (record-only seating)

**Status:** MEASURED — the arena's single frozen read, RECORD-ONLY (`serve: ✗`); the better-only law's massive clause is superseded by tonight's owner instruction ("no more owner gated plz i need the result tmr" + the pasted "Rethink — not run" board), extending the Issue-017 seating precedent (owner call 2026-10-01: "no progress at all reads worse than a seated −4.0"). Claude verdict round 1 fold: the reversal is recorded EXPLICITLY here, quoting the owner messages, and the vs-best card's family-arm counting was confirmed (the renderer picks best-by-accuracy across lanes — a weaker encoder cell cannot lower any headline).

**The head's provenance (selection-bias check, the verdict's condition):**
`massive_encoder_v1_probe.bin` (11,192,928 B + BLAKE3 sidecar; scp'd from
the 4090 `E:\git\riir-train\.raw\t599\` this session, digest verified at
load by the arena's sealed-head reader) is riir-train Issue 600 T3's ONE
trained head for the suite — no sweep, no selection among heads: one fit
(holdout 0.7407 < ref holdout 0.7700 → gate REFUSED), one test read
(0.6752 on the 2974-row screen split, the `_probe` export convention).
The 0.6752 is therefore a single frozen read of an unselected head —
publishable under the same rule as any refused-lane probe, with its
refusal disclosed.

**The read:** arena `--suite massive_intent_en --encoder-art
../riir-train/.raw/t599/massive_encoder_v1_probe.bin --encoder-ckpt
english --skip-pin-a0` · `LAYA_DEVICE=metal` · `--features
arena-laya-metal`. **ENC 0.6567 (197/300)** vs the seat's A1 0.8167 (mean
−0.1600, LB95 −0.2194) · p50 31,332 µs · device metal. A0 0.7800 ==
the published modelless row exactly (same split witness ✓).

The arena split (the reflex seat's 300-case massive test) differs from
600 T3's 2974-row screen split, so the two numbers are not one number:
0.6752 (screen split) and 0.6567 (seat split) tell the same story — the
head sits below its own reference on test either way. The seat-path
number is the published cell (the serve-path law: the arena read IS the
cell read).

**Record-only:** the suite serves H2 (the synth seat, 0.8400 on the
serving manifest); this cell never sells. The `serves` prose in the lane
doc carries the refusal class verbatim.

Reproduction: the identical command re-reads the same accuracies
(bit-identical encode, fixed head bytes).
