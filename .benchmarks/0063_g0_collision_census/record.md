# G0 collision census — the un-collided oracle ceiling (issue 028 T1)

**Pre-registration (pinned before the first run):** member support = train-row presence; high-support = support ≥ S_MIN; dominant = per-class argmax ties-to-lowest; qualifying pair = two high-support same-bucket members with differing dominants; the oracle retrain splits EVERY member of every flagged component into its own fresh bucket from the suite's UNUSED pool; arms = train_arm_a vs train_arm_a_remap over ArmAConfig::default() (the Bench-608 recipe) on the SAME stratified_holdout split; material = paired delta ≥ 1.0 pt AND one-sided LCB95 > 0; every-suite-immaterial ⇒ class 2 folds (the issue's fold law). Level readout — no dynamics claim; per-pair retrains are a follow-on gated on a material total.

| datasets | S_MIN | holdout budget | material bar | trainer |
|---|---|---|---|---|
| ../riir-reflex/.raw/datasets | 5 | 200 | 1 pt | instinct_specialist::train_arm_a(_remap) @ ArmAConfig::default() (the Bench-608 recipe) |

## Per-suite totals

| suite | rows | labels | train/holdout | distinct src | buckets used (pool) | flagged buckets | pairs | remapped | incumbent acc | oracle acc | Δ pt | verdict |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ag_news | 20000 | 4 | 19800/200 | 352814 | 122455 (8617) | 2598 | 3040 | 5499 | 0.8700 | 0.8600 | -1.00 | Immaterial |
| emotion | 15999 | 6 | 15799/200 | 121329 | 79571 (51501) | 291 | 298 | 588 | 0.7300 | 0.7450 | +1.50 | Immaterial |
| sst5 | 8533 | 5 | 8333/200 | 94242 | 68019 (63053) | 119 | 120 | 239 | 0.3350 | 0.3400 | +0.50 | Immaterial |
| massive_intent_en | 11514 | 60 | 11314/200 | 29315 | 26395 (104677) | 38 | 38 | 76 | 0.6600 | 0.6600 | +0.00 | Immaterial |
| banking77 | 9993 | 77 | 9793/200 | 22668 | 21010 (110062) | 73 | 75 | 147 | 0.8100 | 0.8100 | +0.00 | Immaterial |
| xnli_en | 20000 | 3 | 19800/200 | 272069 | 114905 (16167) | 1056 | 1164 | 2193 | 0.3650 | 0.3650 | +0.00 | Immaterial |
| prompt_injections | 546 | 2 | 346/200 | 5556 | 7706 (123366) | 0 | 0 | 0 | 0.7700 | 0.7700 | +0.00 | Immaterial |

## Paired reads (shared holdout, row-aligned)

| suite | Δ mean | 95% CI | one-sided LCB95 | oracle-only hits | incumbent-only hits | n |
|---|---|---|---|---|---|---|
| ag_news | -0.0100 | [-0.0296, +0.0096] | -0.0264 | 1 | 3 | 200 |
| emotion | +0.0150 | [-0.0069, +0.0369] | -0.0034 | 4 | 1 | 200 |
| sst5 | +0.0050 | [-0.0170, +0.0270] | -0.0134 | 3 | 2 | 200 |
| massive_intent_en | +0.0000 | [+0.0000, +0.0000] | +0.0000 | 0 | 0 | 200 |
| banking77 | +0.0000 | [+0.0000, +0.0000] | +0.0000 | 0 | 0 | 200 |
| xnli_en | +0.0000 | [-0.0241, +0.0241] | -0.0202 | 3 | 3 | 200 |
| prompt_injections | +0.0000 | [+0.0000, +0.0000] | +0.0000 | 0 | 0 | 200 |

## Flagged-bucket detail (top members per suite, by support)

- **ag_news**: 2598 flagged bucket(s), 5499 remapped member(s). Largest:
  - bucket 68340 (1 pairs): uni#a4eea112a4090af4 sup=5 dom=3 | uni#c00934e2e5830af4 sup=15995 dom=2
  - bucket 26708 (1 pairs): bi#46041d1d13f66854 sup=16 dom=3 | uni#d2a26d8bfda06854 sup=3932 dom=1
  - bucket 68479 (1 pairs): uni#5b100dca54590b7f sup=3659 dom=2 | bi#d55acbae3f830b7f sup=8 dom=0
  - bucket 33329 (1 pairs): bi#28bf4335c4648231 sup=5 dom=2 | uni#430aa6eeddca8231 sup=1710 dom=0
  - bucket 89886 (1 pairs): bi#1ca3f54ec1d35f1e sup=10 dom=1 | uni#43379ceeddf15f1e sup=1107 dom=2
- **emotion**: 291 flagged bucket(s), 588 remapped member(s). Largest:
  - bucket 88396 (1 pairs): uni#432c9deedde7594c sup=1060 dom=5 | uni#4907e29ecc0f594c sup=10 dom=1
  - bucket 70827 (1 pairs): uni#7ba55b1b082914ab sup=788 dom=0 | bi#b7266a69467714ab sup=266 dom=5
  - bucket 63845 (1 pairs): uni#14d4fb1da380f965 sup=746 dom=0 | bi#860cb02bafaaf965 sup=6 dom=5
  - bucket 129965 (1 pairs): bi#98f358e1d739fbad sup=5 dom=0 | uni#c9ec712153a9fbad sup=658 dom=5
  - bucket 129236 (1 pairs): uni#448b98c785cbf8d4 sup=478 dom=5 | bi#4a4aaead8471f8d4 sup=11 dom=1
- **sst5**: 119 flagged bucket(s), 239 remapped member(s). Largest:
  - bucket 17428 (1 pairs): uni#3b4e007234984414 sup=6 dom=0 | uni#7fb3a9e352624414 sup=1129 dom=3
  - bucket 19831 (1 pairs): bi#05c0b95dea7a4d77 sup=6 dom=4 | uni#0c6d85e310564d77 sup=514 dom=1
  - bucket 44118 (1 pairs): bi#0565351bf926ac56 sup=315 dom=3 | uni#eb531093af62ac56 sup=51 dom=0
  - bucket 55363 (1 pairs): bi#4203bc4c878ed843 sup=7 dom=1 | uni#ff93fea82394d843 sup=205 dom=3
  - bucket 64775 (1 pairs): uni#4ee74278d15efd07 sup=6 dom=2 | uni#ac9889c7c0c2fd07 sup=191 dom=3
- **massive_intent_en**: 38 flagged bucket(s), 76 remapped member(s). Largest:
  - bucket 60395 (1 pairs): uni#43269ceedde2ebeb sup=1568 dom=35 | bi#f8c55f6fc1c6ebeb sup=5 dom=37
  - bucket 97096 (1 pairs): uni#432cb1eedde77b48 sup=1145 dom=35 | bi#ad4aca94fb9d7b48 sup=8 dom=39
  - bucket 119186 (1 pairs): bi#28142627df73d192 sup=5 dom=11 | uni#5d7c4dda1313d192 sup=159 dom=50
  - bucket 86294 (1 pairs): uni#5c24a812b73f5116 sup=8 dom=15 | uni#7ebe0a21afcd5116 sup=107 dom=0
  - bucket 113043 (1 pairs): uni#08ee8fcdac97b993 sup=10 dom=43 | uni#eb5be6e2fe0db993 sup=71 dom=0
- **banking77**: 73 flagged bucket(s), 147 remapped member(s). Largest:
  - bucket 72725 (1 pairs): uni#501eba931e131c15 sup=425 dom=51 | bi#e1132ecdd0bb1c15 sup=40 dom=37
  - bucket 72970 (1 pairs): uni#24d00dcb99b51d0a sup=333 dom=42 | bi#e05455f486771d0a sup=6 dom=30
  - bucket 1742 (1 pairs): bi#45b407704e7806ce sup=6 dom=43 | bi#4a6d607161c006ce sup=217 dom=20
  - bucket 42178 (1 pairs): bi#6f91f2dc3d46a4c2 sup=10 dom=37 | uni#a9ea8c624988a4c2 sup=192 dom=2
  - bucket 99972 (1 pairs): uni#7ba59e1b08298684 sup=81 dom=4 | bi#eec2df666ab18684 sup=186 dom=20
- **xnli_en**: 1056 flagged bucket(s), 2193 remapped member(s). Largest:
  - bucket 79092 (1 pairs): uni#7ba56e1b082934f4 sup=8044 dom=2 | bi#85b03fb3842134f4 sup=5 dom=0
  - bucket 84616 (1 pairs): uni#5b6203e33d574a88 sup=3872 dom=2 | bi#675acd69c7914a88 sup=6 dom=1
  - bucket 107259 (1 pairs): uni#28ec0ee3212fa2fb sup=1528 dom=2 | bi#99e4761cb2cfa2fb sup=13 dom=0
  - bucket 128812 (1 pairs): uni#631316e3416bf72c sup=1489 dom=2 | bi#6c75565ad4c5f72c sup=5 dom=0
  - bucket 19831 (1 pairs): bi#05c0b95dea7a4d77 sup=33 dom=0 | uni#0c6d85e310564d77 sup=1438 dom=2
- **prompt_injections**: no flagged bucket at S_MIN — no qualifying pair exists.

## Fold verdict

**Every suite immaterial at 2¹⁷ ⇒ class 2 FOLDS as a recorded negative** (the collision cost of the frozen feature law is immaterial on these corpora). Class 1 is unaffected.
