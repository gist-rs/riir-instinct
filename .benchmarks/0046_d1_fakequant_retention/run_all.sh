#!/bin/sh
# Bench 0046 — the D1 fake-quant probe's 14 reads (issue 018 Lane D1).
# Sequential (GPU exclusivity — one compute consumer); each read is one
# arena invocation into its own runs dir. The commands are the seated
# cells' commands verbatim, +- --fake-quant.
set -e
cd "$(dirname "$0")/../.."
BIN=/tmp/d1-instinct/release/arena
export LAYA_DEVICE=metal
T20K=../riir-reflex/.raw/datasets_t20k
TYPED=../riir-train/.raw/datasets_typed_full
OUT=.benchmarks/0046_d1_fakequant_retention/runs

run() { # suite art ckpt datasets extra... suffix
  suite=$1; art=$2; ckpt=$3; ds=$4; suffix=$5; shift 5
  echo "=== $suite $suffix"
  "$BIN" --suite "$suite" --encoder-art "$art" --encoder-ckpt "$ckpt" \
    --datasets-dir "$ds" --skip-pin-a0 --out "$OUT/${suite}_${suffix}" "$@"
}

# sst5 (v1, english)
run sst5 ../riir-train/.raw/t599/t6_s0.bin english "$T20K" f16
run sst5 ../riir-train/.raw/t599/t6_s0.bin english "$T20K" q8 --fake-quant
# xnli_en
run xnli_en ../riir-train/.raw/t599/xnli_en_encoder_v1.bin english "$T20K" f16
run xnli_en ../riir-train/.raw/t599/xnli_en_encoder_v1.bin english "$T20K" q8 --fake-quant
# ag_news
run ag_news ../riir-train/.raw/t599/ag_news_encoder_v1.bin english "$T20K" f16
run ag_news ../riir-train/.raw/t599/ag_news_encoder_v1.bin english "$T20K" q8 --fake-quant
# massive_intent_en
run massive_intent_en ../riir-train/.raw/t599/massive_encoder_v1_probe.bin english "$T20K" f16
run massive_intent_en ../riir-train/.raw/t599/massive_encoder_v1_probe.bin english "$T20K" q8 --fake-quant
# banking77
run banking77 ../riir-train/.raw/t608/banking77_encoder_v1.bin english "$T20K" f16
run banking77 ../riir-train/.raw/t608/banking77_encoder_v1.bin english "$T20K" q8 --fake-quant
# prompt_injections
run prompt_injections ../riir-train/.raw/t608/prompt_encoder_v1.bin english "$T20K" f16
run prompt_injections ../riir-train/.raw/t608/prompt_encoder_v1.bin english "$T20K" q8 --fake-quant
# typed_decisions (v2, TYPED checkpoint, full pool)
run typed_decisions ../riir-train/.raw/t608/typed_encoder_v2.bin typed "$TYPED" f16
run typed_decisions ../riir-train/.raw/t608/typed_encoder_v2.bin typed "$TYPED" q8 --fake-quant
echo "=== ALL 14 RUNS DONE"
