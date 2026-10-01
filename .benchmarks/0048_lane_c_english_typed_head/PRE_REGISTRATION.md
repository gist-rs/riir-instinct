# Bench 0048 — issue 018 Lane C: the typed head retrained over the adopted (Q8, english) encoding — PRE-REGISTRATION

**Status:** PRE-REGISTERED — margins + rules declared BEFORE any measurement
run. This file is committed before the instrument's first read; the
measurement commits cite it. Any change after the first read is recorded as
an amendment with its own commit, never an edit of the declared numbers.

Plan of record: `riir-instinct/.issues/018_rethink_encoder_lean_goat.md`
§Lane C ("typed head retrained over english encodes") on the D1-pass
branch: C trains ONCE, on the adopted quant's encode cache (the input-cache
note), after D2a. The branch's amended order (2026-10-02): D2a done → C
next → D2b kernels → Q4/PQ2; B/A daylight.

## The question (declared)

Can ONE checkpoint (english) serve the typed cell — i.e. does the NLEH v2
per-option head, retrained over the ENGLISH encode of the typed pool at the
ADOPTED Q8 encoding, hold the typed cell's quality vs the seated incumbent
(the typed-encoded v2 head)? A win re-seats the cell onto the new head and
makes the serving lane's hardcoded english correct-by-completeness; a fail
records the number with no posture rollback (the issue's negative branch).

## The instrument (declared)

- **The adopted encoding:** checkpoint ENGLISH weights loaded from the D2a
  Q8 artifact (`LAYA_WEIGHTS_VARIANT=q8`,
  `~/.cache/riir-reflex/laya/english/derived/model.q8.safetensors`,
  sidecar-verified). The decode values are byte-identical to the D1
  fake-quant probe's weights (Bench 0047's converter proof) — the encode
  pass therefore measures the adopted encoding, not a new numerics
  surface. Posture M3 Metal (`LAYA_DEVICE=metal`), the same posture every
  seated encoder cell was read at. Double-quantization is structurally
  refused (the loader's `--fake-quant` × q8-artifact refusal — never
  armed).
- **R1 — the encode pass** (`dump_encoder_states`, riir-infer HEAD): the
  typed pool's cases jsonl (4000 train rows / 2000 test rows, one question
  per row, unchanged) → LENC v1 caches
  `typed_train_lenc_english_q8.bin` + `typed_test_lenc_english_q8.bin`
  (riir-train `.raw/t608/`, gitignored data). The existing F16 english
  test cache is NOT read by this bench (it predates the adoption; C never
  trains on an encoding the branch is about to change — the issue's
  D-interaction law).
- **R2 — training** (`instinct_typed_head_trainer`, riir-train HEAD, the
  Bench 040 discipline verbatim): holdout 800 stratified over
  (presented-width, gold), 15 epochs, hidden 128, Adam batch-32, lr 1e-3,
  SplitMix64 seed 0x599599 (bit-deterministic), **plain CE** (the
  refit-first posture; the tripwire loss option stays pre-declared — see
  the absolute-floor tripwire — and is used ONLY if that branch fires).
  `--out-name typed_encoder_v2_q8en` — the seated incumbent artifact
  `typed_encoder_v2.bin` is NEVER overwritten. Earn bar: holdout acc >
  the reference logits' holdout acc on the same cache (the class-relative
  bar; `--min-holdout 0`).
- **R3 — the frozen test read** (`instinct_encoder_eval` + the additive
  `--picks-out` flag, committed with this bench's instrument): over
  `typed_test_lenc_english_q8.bin`. Per-row picks/correct/gold written to
  `runs/new_head_picks.json`. One read, nothing tuned against it.
- **R4 — the incumbent half: NO new run.** Bench 0046's
  `runs/typed_decisions_q8/` IS the incumbent encoder arm
  (`typed_encoder_v2.bin`, typed ckpt) at the adopted encoding: acc 0.7570,
  per-row picks + correct + confs (fake-quant posture; byte-identical
  decode to the q8 artifact per Bench 0047's proof, cited — the pairing is
  therefore at one encoding, isolating the HEAD axis). Its F16 twin
  (0.7550, the seated record) is the record-consistency read.

## The pairing (declared)

Per-row index-aligned on the frozen test read (2000 rows): the trainer-side
cache order and the arena's seat-builder order are the same cases-jsonl
order (the D1 convention, where the same pairing held). Gate arithmetic is
`stats::PairedDiff`'s exact form mirrored: z = 1.959963984540054,
`se = sqrt(Σd² − (Σd)²/n) / (n−1) / sqrt(n)` over the per-row differences
`d_i = correct_new[i] − correct_inc_q8[i]`.

## The gates (declared BEFORE measuring)

### The re-seat gate (primary)

| Requirement | Declared value |
|---|---|
| Margin δ | **0.02** (flat — the D1 wide-n margin; typed n = 2000 resolves it) |
| **WIN** (re-seat) | paired LB95 of (new − incumbent@q8) **> 0** — the issue's "re-seats ONLY on a win", instantiated with the house paired LB95 instrument |
| EQUIVALENT (no re-seat) | LB95 ∈ (−δ, 0] — the cell's quality is preserved within margin; the record KEEPS the incumbent; the number is recorded as the lane's equivalence datum (the one-checkpoint story measured, not seated) |
| FAIL (negative branch) | LB95 < −δ — record the number, no posture rollback (the issue's negative branch) |
| LOW-POWER flag | `z·se > δ` (the CI can never decide; reported, the rule still applies) |

Rationale, recorded so the reading cannot be re-litigated after the fact:
the issue says "win", not "non-inferior" — a replacement head SEATS only by
being better with 95% confidence. A within-δ-equivalent head still
delivers the lane's engineering purpose (one checkpoint can serve all four
cells) and is RECORDED as such, but the seated record keeps the incumbent's
number until something strictly better replaces it. The strict-superiority
T2 advertising law is untouched by this bench (it governs arms vs the
serving posture, not head-for-head replacement).

### Retention table (reported, NOT gated)

Σ[pick_new == pick_inc]/n, the per-gold flip table, and the top (from→to)
pick pairs — the aggregate-flat-while-families-flip detector (the Orthrus
lesson; per-family is the only read that counts). The per-presented-width
breakdown (2/4/5) is reported for both arms.

### Calibration plane (REPORTED, not gated — the issue's calibration gate lives in Lane D)

Brier (picked-class form `(p_pick − hit)²`) + ECE (10 equal-width bins over
max-prob, left-open (0,1] — the reflex protocol convention) for the new
head, disclosed beside the incumbent's (0046 carries confs for both its
postures). Declared report-only BEFORE the read: the re-seat bar is the
accuracy pairing; a calibration disclosure rides the record.

### The absolute-floor tripwire (the issue's branch (b), instantiated)

If the new head's test acc reads BELOW the reference logits' test acc on
the SAME cache (the class floor — the 039 screen's form), the tripwire
fires: the number is recorded and C's pre-declared loss option (CE+λ·Brier,
one refit — the issue's landing pad, katgpt-rs Research 576's PASS-Redirect
line) is the fix path. It does NOT change the re-seat gate by itself.

### The witness (required on a WIN; reported on EQUIVALENT/FAIL)

The arena replay — instinct `arena --suite typed_decisions --encoder-art
typed_encoder_v2_q8en.bin --encoder-ckpt english --datasets-dir
datasets_typed_full --skip-pin-a0` with `LAYA_WEIGHTS_VARIANT=q8`,
M3 Metal — must reproduce R3's per-row picks EXACTLY (the third-posture
cell-identity law: live encode == cached encode == trainer-side read). A
break invalidates the re-seat (seat/box drift) and must be resolved before
any verdict is claimed — a witness over drifted rows is not a measurement.

## What this bench does NOT claim

- No serving change: the typed cell is record-only (`serve: ✗` — the
  class-wide per-request encoder refusal stands, issue 016, trigger-blocked
  on a real GPU serving deploy). A WIN re-seats the RECORD and the lane
  doc; nothing serves differently today.
- No site publish unless the re-seat WINS (and the site half then follows
  the reflex-site publish law: self-tests, smokes, parity, manual deploy).
- No posture change on EQUIVALENT/FAIL: typed keeps
  `typed_encoder_v2.bin` (typed ckpt) as its record exactly as seated.

## Box state

- `bench_preflight.sh` PROVENANCE quoted beside the FIRST Metal run of the
  session (the Issue-021 law — AC plugged, load ceiling, settle time).
- Runs are sequential (GPU exclusivity — one compute consumer).
- Accuracy-only reads; latency rows ride the runs' own box disclosure (any
  re-published cell's latency re-measures on a fit-box run per Issue-021).

## Declared BEFORE the first read

Commit of record: the commit adding this file (the measurement commits
reference it by hash). Nothing below this line is measured yet.
