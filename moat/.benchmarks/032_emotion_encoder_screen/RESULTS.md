# Bench 032 — the emotion encoder screen (Issue 016 T7): NEGATIVE by the pre-registered screen — emotion stays A0

**Status:** RECORD — a negative screen, recorded per the 014 law (the read that was
earned is the SCREEN, not the train). 2026-09-30.

## The pre-registered protocol (written before any run — 016 T7 + the 600 screen-first law)

- The ONE unscreened board-relevant encoder suite: emotion, whose armed-seat A0 reads
  **0.8850** (the highest bar on the board — the T4-recorded earn gate: the head must
  read > 0.8850 to matter).
- The class: the NLEH encoder lane (`instinct_encoder_trainer` — the wave-3
  suite-generic), SAME architecture, FRESH fit — over the emotion **t20k** pool in the
  EMOTION SUITE'S OWN WIRE (a CHOICE question, the fixed 6-option order — NOT issue
  602's sst5-score-form emotion distill volume, whose labels were ignored).
- **Screen FIRST** (the 600 law): `dump_encoder_states` on the TEST split →
  `instinct_encoder_eval --ref-only` → the class's floor. A floor at/above the bar is
  a dead lane (no lift room); a floor FAR under the bar is dead unless the measured
  head-lift ceiling (+14.7 pt, wave-3) can close the gap. Only a mid-ceiling floor
  with a lift-capable gap earns the training run.

## The screen (executed 2026-09-30, M3 Metal)

| Step | What | Result |
|---|---|---|
| Cases | `make_emotion_cases.py` over `datasets_t20k/emotion` (the reflex `build_emotion` wire verbatim; dedup by text) | **400 unique test** (the bar's split ✓) + **15,969 unique train** |
| Encode | `dump_encoder_states`, ckpt english, `LAYA_DEVICE=metal` | 400 rows in **15.3 s** → `emotion_test_lenc.bin` (98.6 MB) |
| Screen | `instinct_encoder_eval --ref-only` | **reference floor 0.5950** (238/400) |

Per-class recall (the floor's shape): sadness 0.73 · joy 0.74 · love 0.38 · anger 0.47 ·
fear 0.37 · surprise 0.10 — the frozen reference is weak exactly on the tail classes
the modelless count-tables nail lexically.

## The verdict (arithmetic, not taste)

```
earn bar            0.8850  (the A0 — the T4-recorded gate)
reference floor     0.5950
gap                 0.2900
max lift (measured) +0.147  (the wave-3 head-lift ceiling)
best achievable     0.7420  <  0.8850  → DEAD LANE
```

Even the class's measured-maximum lift cannot close a 29-point gap. The training run
(the 5-head sweep + the frozen read, ~20–30 min GPU) is **not earned** — the screen
exists precisely to prevent that spend. **Emotion stays A0-served; the posture-gap
state stands** (a reflex republish at the armed posture can still collapse the +11.5
to a tie — the risk T7 was queued to price; the answer is that THIS class cannot
secure the row, and the row's security waits on a different model class or the D1
posture itself).

## Box state (the Issue-021 law)

M3 Max (Apple silicon, 16-core), macOS 26.6.2, AC plugged (battery 100%), the encode
the only GPU workload (Zed idle-edit). Metal lane (`LAYA_DEVICE=metal`). ~35 min wall
total (case build + encode + eval, incl. a 35 s release build of the dumper).

## Artifacts (gitignored data, regenerable)

`riir-train/.raw/t607/` — `make_emotion_cases.py` (the wire-faithful case-maker),
`emotion_{test,train}_cases.jsonl`, `emotion_test_lenc.bin`. The train pool was NOT
encoded (the screen killed the lane first — the 15,969-row train cache would have
been ~20–30 min Metal for a certain miss).
