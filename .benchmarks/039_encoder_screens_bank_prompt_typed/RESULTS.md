# Bench 039 — the three unscreened encoder suites screened (016 T8): bank/prompt DEAD, typed ALIVE (head run earned)

**Status:** RECORD — screens only (issue 600's screen-first law); no train run
spent. Box: m3-max Metal, AC power, loadavg ~4.5-5 (sibling sessions active —
accuracy-only reads, latency NOT quoted). Dumps: `riir-infer-laya`
`dump_encoder_states` @ LAYA_DEVICE=metal (d 1024), caches + case-makers under
`../riir-train/.raw/t608/` (gitignored data; the makers are regenerable —
`make_banking77_cases.py` / `make_prompt_cases.py` / `make_typed_cases.py`).

## The screens (one frozen ref-only read each)

| suite | substrate | floor (ref logits) | seating bar (better-only law) | verdict |
|---|---|---|---|---|
| banking77 | laya-english | **0.4980** (249/500) | 0.8540 (hybrid H2 nbsvm-v2) | **DEAD** — floor + class-max lift (+14.7 pt) = 0.645 < 0.8540 |
| prompt_injections | laya-english | **0.6983** (81/116; recall benign 1.000 / injection 0.4167) | 0.8534 (hybrid A1) | **DEAD** — 0.6983 + 0.147 = 0.845 < 0.8534 |
| typed_decisions | laya-**typed** | **0.7445** (1489/2000; 2-wide 0.7850 · 4-wide 0.7164 · 5-wide 0.7667) | 0.6475 (hybrid H2) | **ALIVE — the head run is EARNED** (floor is +9.7 pt OVER the bar before any lift) |
| typed_decisions | laya-english | 0.3575 (715/2000) | — | the substrate the lane would NOT ride (recorded for the class-relative file) |

## Wire-fidelity witnesses (three exact matches to published board numbers)

The screen machinery (case-makers + dump + per-row-k eval) reproduces
independently published reads BYTE-EXACTLY — the strongest available evidence
the wires are faithful:

1. banking77 english floor **0.4980** == the published laya-english base read
   (the GLiNER lane's comparison cell, reflex `.issues/029`).
2. typed **typed**-checkpoint floor **0.7445** == the published laya-typed
   specialist read (the AgentJev comparison, reflex bench 039).
3. typed english floor **0.3575** == the published laya-english typed base
   (reflex `.issues/029`).

The typed case-maker's suite shape also matches the board exactly: 400 cases →
**2000 questions**, per-kind **choice 600 / noul 600 / score 800**. (En-route
maker fix: every score gold in this pool arrives as a STRING — "1" — so the
maker mirrors `value_as_i64`'s trimmed-string parse; the strict-int reading
dropped all 800 score questions.)

## What this changes

- **typed_decisions is the fourth winnable encoder lane** (after sst5 / xnli /
  ag_news) and the strongest: the floor ALONE beats the displayed hybrid arm by
  +9.7 pt. Per the 600 class-relative law a head over the typed-checkpoint
  feature cache is priced at ≈ the floor (sst5 added +14.7 over its floor;
  xnli added 0) → expected seat ~0.74–0.79, second-best on the board and
  possibly past AgentJev's 0.7715.
- **The blocker is CODE, not evidence**: the NLEH v1 head is fixed-class
  (`feat_dim = n_markers·d + d`, fixed `n_classes`) and cannot read a
  per-case-option cache (presented widths 2/4/5). The earned run needs the
  **NLEH v2 per-option scoring shape**: `score_i = MLP([marker_i ; pooled])`
  → sigmoid, pick = argmax over presented options (the Specialist law —
  per-option sigmoid, never softmax). Filed as 016 T9.
- banking77 / prompt_injections join emotion/massive/code_fixtures as
  **measured-dead** — the site's `Rethink — not run` cells on those suites are
  now recorded negatives, never backlog.

## Commands (reproducible)

```sh
# case-makers (riir-train cwd)
python3 .raw/t608/make_banking77_cases.py ../riir-reflex/.raw/datasets_t20k/banking77 .raw/t608/banking77_test_cases.jsonl test 500
python3 .raw/t608/make_prompt_cases.py  ../riir-reflex/.raw/datasets_t20k/prompt_injections .raw/t608/prompt_test_cases.jsonl test
python3 .raw/t608/make_typed_cases.py  ../riir-reflex/.raw/datasets_t20k/typed_decisions .raw/t608/typed_test_cases.jsonl test

# dumps (riir-infer cwd, m3 Metal)
CARGO_TARGET_DIR=/tmp/t608_dump_build cargo build --release -p riir-infer-laya --features laya-riir-metal --example dump_encoder_states
LAYA_DEVICE=metal /tmp/t608_dump_build/release/examples/dump_encoder_states --checkpoint english --cases ../riir-train/.raw/t608/banking77_test_cases.jsonl --out ../riir-train/.raw/t608/banking77_test_lenc_english.bin
LAYA_DEVICE=metal /tmp/t608_dump_build/release/examples/dump_encoder_states --checkpoint english --cases ../riir-train/.raw/t608/prompt_test_cases.jsonl   --out ../riir-train/.raw/t608/prompt_test_lenc_english.bin
LAYA_DEVICE=metal /tmp/t608_dump_build/release/examples/dump_encoder_states --checkpoint typed   --cases ../riir-train/.raw/t608/typed_test_cases.jsonl  --out ../riir-train/.raw/t608/typed_test_lenc_typed.bin
LAYA_DEVICE=metal /tmp/t608_dump_build/release/examples/dump_encoder_states --checkpoint english --cases ../riir-train/.raw/t608/typed_test_cases.jsonl  --out ../riir-train/.raw/t608/typed_test_lenc_english.bin

# screens (riir-train cwd) — the eval gained the per-row-k ref screen this session
CARGO_TARGET_DIR=/tmp/t608_train_build cargo build --release -p riir-train-engine --example instinct_encoder_eval
/tmp/t608_train_build/release/examples/instinct_encoder_eval --ref-only --cache .raw/t608/banking77_test_lenc_english.bin --cases .raw/t608/banking77_test_cases.jsonl
/tmp/t608_train_build/release/examples/instinct_encoder_eval --ref-only --cache .raw/t608/prompt_test_lenc_english.bin   --cases .raw/t608/prompt_test_cases.jsonl
/tmp/t608_train_build/release/examples/instinct_encoder_eval --ref-only --cache .raw/t608/typed_test_lenc_typed.bin       --cases .raw/t608/typed_test_cases.jsonl
```

Dump wall times (loaded box, record-only): banking77 500 rows 40.8 s · prompt
116 rows 3.4 s · typed 2000 rows 187.4 s (typed ckpt) / 325.1 s (english).
