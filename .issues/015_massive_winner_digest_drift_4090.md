# Issue 015 — cross-box massive-winner digest drift: the only 4090 copy fails the manifest pin (the serve's A5/A9 gate refuses, correctly)

**Status:** OPEN — filed 2026-09-30 (found by the Plan-426-T6 serve smoke).
Not a defect of the gate: the drift is REAL and the serve refusing loud is
the designed behavior. The gap is BOX SYNC, not the pin.

## The facts (measured 2026-09-30 ~08:45, 4090 box)

- The embedded arsenal manifest pins massive's winner at
  `blake3:7bc3ee385f81abc0f2a91e47f6dca08db37f9fb6cee6d48121b80e3f11378556`
  (the T4-restore verified digest — the M3's canonical file).
- The only `massive_intent_en_winner_v1.bin` on the 4090 lives in the
  untracked sync dir `../riir-train/data/instinct_specialists_openthai_4090/`
  (05:37 today, the sibling session's crosscheck copy) and hashes
  `blake3:1f95e309e5991250659a4809b337979f9afdd43fe5fd8b308358e5001390ac98`.
- `cargo run --bin serve -- --suites massive_intent_en` with that winners
  dir refuses at manifest validation (law A5/A9, loud, correct):
  `artifact ... is blake3:1f95e309… but the manifest pins blake3:7bc3ee38…`.
- The canonical `../riir-train/data/instinct_specialists/` dir on this box
  carries only code_fixtures + sst5 artifacts — the 592–597-era winners
  never synced here. A workspace-wide search found NO file matching
  `7bc3ee38…` on the 4090.

## Why the ARENA is unaffected (the functional witness)

The sibling's `t4_4090_crosscheck` (untracked, 05:37 today) ran the arena
on this box WITH this winner copy: massive A0 **0.7800** (anchor exact),
A1 0.8033, H2(β=2,nmin=2,τ=2) 0.8067 — byte-equal to the M3's Bench-023
read at every arm. Two differently-hashed files producing identical picks
⇒ the hash delta is (very likely) non-semantic (export metadata), but
"likely" is not a digest match: the MANIFEST pins one digest and the file
carries another, and only the M3 can adjudicate which is canonical.

## Remedies (owner/session order)

1. **Sync the canonical artifact** (one scp from the M3's winners dir) and
   keep `data/instinct_specialists_openthai_4090/` as scratch — then this
   box's serve boots massive and the pin is honest.
2. **Or re-pin** the manifest to `1f95e309…` — ONLY with an owner call
   naming which artifact is canonical (a digest pin moved without knowing
   which file it names is the 579-class coupling trap).
3. The general gap: the winners dir on this box is PARTIAL (3 of 9 rows'
   artifacts). A `--suites <all>` serve boot on the 4090 fails on the
   first missing artifact. A sync checklist (the 9 manifest artifacts vs
   on-disk) would make the gap visible without a boot attempt.

## Scope notes

- Found en-route to the Plan-426-T6 serve smoke; the smoke's synth-seat
  wiring itself validated (the emotion fixture seated 6/6, A0 0.8850 ==
  the published armed number — the unchanged path untouched at trivial
  corpus mass).
- The Plan-426-T6 cert (the arena) proceeds with the crosscheck's winners
  dir + the drift disclosed in its record; it does NOT need the serve.
- The serve-side synth posture (`INSTINCT_SYNTH_CORPUS_DIR`) ships in
  `c7b7165` and is validated; it inherits this issue's sync precondition
  for a massive deploy on this box.
