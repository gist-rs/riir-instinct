# Issue 028: twinarm probe-repair pass PoC (Research 653 rev 2 — 649 × twinarm consumer fusion)

**Status:** CLOSED — FOLDED 2026-11-05 (both legs): class 2 folded at T1 (census-only
negative, record 0063: every suite immaterial at 2¹⁷); class 1 folded at T2 (G1 value
claim failed — the repair beat its unrepaired base Δ +0.24 but LOST to the fair b1
baseline, LCB95 −0.0132; record 0064). The fold law executed in the fold commit: the
lane module + bins + feature REMOVED (no zombie module); the trainer-side primitives
stayed as tested capabilities. **Lease closed:** zed-4090-4fa6, 1793897593, expired
2026-11-07T16:53:00+00:00 (closed 2026-11-05 on the fold)
**Research:** [katgpt-rs/.research/653_Twinarm_Probe_Repair_Fusion.md](../katgpt-rs/.research/653_Twinarm_Probe_Repair_Fusion.md) · anchor: 649 (probe-authored circuit grammar, Bench 978) · prior sweep: 650
**Target:** `src/instinct_probe_repair.rs` (new module, opt-in feature `probe_repair = ["katgpt-core/probe_circuit_grammar"]`) — trainer-side census + repair pass; grammar record as a BLAKE3 sidecar beside the RISP artifact (the RIDT sidecar pattern; no format break)

## Goal

A post-training, probe-authored repair pass for twinarm specialist heads (Research 649's
pipeline at linear-head scale), targeting **the documented weak-teacher collapse class**
(riir-train Bench 609 — private record, class fact only): the distilled arm's weights
encode teacher soft-target errors as systematic bucket→class arcs that gold-labelled
authoring pools can see; the pass parses those arcs into a fn-grammar record, emits the
repair from the discrete `EditForm` vocabulary over a probe-derived strength window, and
verifies on the authoring holdout **against a fair weight-side baseline battery** — the
existing frozen-read winner law gates any seating. A collision census runs alongside as
**measurement only** (an un-collided oracle prices the frozen feature law's aliasing
cost; no weight-edit claim — shared weight columns make that void). Tier verdict
finalizes with these gates; a fold is a recorded negative, never a zombie module.

## Pre-registered gates (verbatim from Research 653 §5 — no re-tuning mid-run)

- **G0 oracle census (base-rate, FIRST):** bucket member tables over the trainer's
  tokenizer law (`bag_into`, `VOCAB = 1<<17`); flag colliding pairs with two high-support
  class-disagreeing members; collision cost measured by the **un-collided oracle** — ONE
  diagnostic retrain per suite splitting ALL flagged pairs into separate buckets at once
  (a different feature map, NEVER shipped), giving the suite's total collision ceiling;
  per-pair retrains only if that total is material. **Oracle ≈ 0 on every suite ⇒ class
  2 folds as a recorded negative** (class 1 unaffected). Not a zero-the-bucket delta —
  that measures importance, not interference.
- **The tune/eval split (pre-registered, binds G1 AND G2):** the authoring holdout is
  split ONCE, before anything trains, by a PoC-local deterministic rule — **proportional
  alternation**: within each label's authoring rows in first-appearance order, odd
  positions (1st, 3rd, …) → tune slice, even positions (2nd, 4th, …) → eval slice. No
  RNG and no seed constant — none exists to pin: the reflex `stratified_split` and the
  trainer's `stratified_holdout` (`instinct_specialist.rs:425`) are budget-only laws
  whose docs say "no seed to record"; the rule itself is the pin. Per-label proportions
  hold in BOTH slices (each label splits ⌊n_label/2⌋ vs ⌈n_label/2⌉), asserted by a
  unit test before any arm runs. Deliberately NOT the shipped `stratified_holdout`
  round-robin: that law fills its budget one-row-per-label-per-round, yielding a
  label-BALANCED eval slice and the SKEWED remainder as tune — a selection/scoring
  distribution shift that a prior-sensitive arm (b2, bias recalibration) is most
  exposed to; the shipped law stays untouched for seat-time holdouts. Every arm — the
  repair's dose sweep and b1–b3 alike — selects on the tune slice ONLY; G2's paired
  read happens ONCE on the eval slice. Without this the value gate reads on the
  selection set — the one-read problem the winner law exists to prevent, at authoring
  level.
- **G1 fixture (teacher-noise collapse):** synthetic corpus with a pinned
  corrupted-teacher construction (soft targets flipped toward specific bucket-correlated
  wrong classes; constants pinned before the run); train Arm A (gold reference) + Arm B
  (corrupted teacher); the gold-pool parse recovers the poisoned arcs (bucket + wrong
  class row) exactly; dose-sweep repair. Gates: zero-strength bit-identity; isolation
  gold-pool parse recovers the poisoned arcs (bucket + wrong class row) exactly;
  dose-sweep repair selected on the TUNE slice. Gates: zero-strength bit-identity;
  isolation (untouched weights byte-identical, unaffected-row predictions unchanged);
  **fair baseline battery** — (b1) continue-train-on-gold k steps, (b2) per-class bias
  recalibration, (b3) per-bucket coordinate search, each tuned on the same TUNE slice
  only — the repair must beat the best of these (paired LCB95 > 0 read ONCE on the
  EVAL slice) for the "probe-authored" value claim. No "retrain-alone reproduces the
  deficit" control: the trainer is bit-deterministic, so that control cannot fail — it
  is evidence of nothing.
- **G2 value:** repaired arm strictly beats its unrepaired base AND the best baseline —
  paired LB95 > 0, read ONCE on the eval slice (never the tune slice any arm selected
  on); seating claims additionally require the ONE frozen-read winner law (no second
  read, no threshold iteration against test). The honest seat target is **sst5**;
  emotion (A0 0.8850, every challenger measured out) and xnli_en (bag-view ceiling)
  are attribution-record targets, not seat targets.
- **G3 no-regression:** non-targeted artifacts byte-identical.
- **G4 attribution:** grammar sidecar round-trips, digest-verified, stale-refusing, names
  the G1 arcs correctly.
- **Fold law:** G0 oracle ≈ 0 ⇒ class 2 folds (census-only negative); G1 failure ⇒ the
  whole lane folds into the trainer (no zombie module), note demoted in the same commit.

## Tasks

- [x] **T1.** G0 oracle census over the Bench-608 suites + the three no-specialist
      suites (emotion / xnli_en / sst5): member tables, flagged colliding pairs,
      split-bucket oracle ceilings; the census verdict recorded in a `.benchmarks/`
      record. FOLD class 2 here if every ceiling ≈ 0.
      **DONE (86a8de2, 2026-11-05): [.benchmarks/0063_g0_collision_census](../.benchmarks/0063_g0_collision_census/record.md)
      — 7 suites (the six Bench-608 suites + prompt_injections via the same
      `suite_row` law; `typed_decisions` DISCLOSED EXCLUSION — its row space rides the
      typed loader, a separate feature law; can join a follow-up cell if the lane
      survives). The incumbent arm EXACTLY reproduces the Bench-608 table (0.8700 /
      0.7300 / 0.3350 / 0.6600 / 0.8100 / 0.3650) — the harness drives the trainer law
      correctly. Oracle deltas: ag_news **−1.00 pt** (removing collisions HURT — the
      merged columns were acting as useful co-occurrence features), emotion +1.50 pt
      (fails the one-sided LCB95 −0.0034 at n=200), sst5 +0.50, the rest 0.00.
      Flag counts: ag_news 2598 buckets / 5499 members, xnli_en 1056/2193, emotion
      291/588, sst5 119/239, banking77 73/147, massive 38/76, prompt_injections 0/0
      (too few rows for S_MIN=5 co-support). **Verdict: every suite immaterial at 2¹⁷
      ⇒ CLASS 2 FOLDS as a recorded negative** (the fold law, executed); class 1
      unaffected. Pre-registration pinned in `src/instinct_probe_repair.rs` module
      docs BEFORE the run (support = train-row presence; S_MIN = 5; dominant =
      ties-to-lowest; oracle = train_arm_a_remap over ArmAConfig::default() on the
      Bench-608 stratified_holdout(200) split; material = Δ ≥ 1.0 pt AND LCB95 > 0).
- [-] **T2.** G1 fixture: pinned corrupted-teacher synthetic corpus; build the
      tune/eval split by the proportional-alternation rule (+ its unit test), THEN
      train; Arm A + Arm B train; gold-pool parse recovery; dose-sweep repair (tune
      slice); the full gate arms incl. the baseline battery; the single eval-slice
      read.
      **DONE FOLDED (2026-11-05, record [.benchmarks/0064_g1_probe_repair_fixture](../.benchmarks/0064_g1_probe_repair_fixture/G1.md)):
      the machinery worked — proportional-alternation split (per-label floor/ceil
      asserted by unit test + runtime assert BEFORE any arm), fixture built (4 poison
      buckets, teacher flip re-pinned 0.25→0.05 BEFORE the first gate run: the first
      pin's 0.475/0.525 target was absorbed by the topical signal — Arm B 1.0000 with
      ZERO failing rows, no pathology to repair; two token-assignment pins also
      corrected pre-run: `r % 4` token keying left tokens 2–3 rowless, and an
      `r % 2` poison rule put every poison row in one alternation slice — both
      measured, both fixed pre-run), Arm B collapsed 0.7550 on both slices (exactly
      the poison rows), the within-gold pool-contrast parse recovered the 4 planted
      arcs EXACTLY at ranks 1–4 (0.07–0.09 contrast vs 0.0068 noise; the
      CROSS-gold contrast is confounded by the topical signal — measured on the
      first cut, the within-gold refinement is the recorded law), zero-strength
      bit-identity ✓, isolation ✓ (exactly the words' edit set), repair
      tune 0.7550 → 1.0000, eval → 0.9950, vs base Δ +0.2400 (LCB95 +0.1902).
      **THE VALUE CLAIM FAILED: b1 continue-train-on-gold (k=1 epoch, warm-started
      via the new `train_arm_a_warm`) reached eval 1.0000 — vs best Δ −0.0050,
      LCB95 −0.0132 < 0.** The fold law executes: G1 failure ⇒ the whole lane folds
      into the trainer, no zombie module. b2 bias-recal 0.9950 eval (the runner-up);
      b3 blind coordinate search 0.7550 (its greedy first-improvement over 858
      buckets wasted the 8-edit budget — generic search is NOT automatically
      strong; the battery's strength concentrated in b1). Honest reading: on a
      linear fixture with adequate clean gold, the generic move toward the gold
      optimum wins — exactly 653 §3's documented boundary ("the repair can only
      gain where training stopped short of the gold optimum... generic weight-side
      alternatives exist"). The fixture was pinned pre-run; re-designing it until
      the repair wins would be the threshold-iteration the pre-registration
      forbids. FOLD EXECUTED in the same commit: `instinct_probe_repair.rs` + the
      two gate bins + the `probe_repair` feature REMOVED (records 0063/0064 + git
      history at 86a8de2 carry the full lane); the trainer-side primitives STAY as
      tested capabilities (the "folds into the trainer" reading, the 649
      substrate-absorption precedent): `source_events_into` (the source-granularity
      projection of the ONE tokenizer walk), `bag_into_remap` + `BucketRemap` +
      `train_arm_a_remap` (the un-collided oracle instrument, record 0063),
      `train_arm_a_warm` (the k-step gold continuation) — each with in-tree unit
      tests incl. the warm-start-from-zero ≡ train_arm_a bit-identity pin.
- [-] **T3.** Mode T repair path — FOLDED with the lane (G1 failure); the Mode-T
      edit mechanics are in git history at 86a8de2 if a future pre-registration
      wants them.
- [-] **T4.** Mode A repair path — FOLDED with the lane.
- [-] **T5.** G4 grammar sidecar — FOLDED with the lane (T7's condition never fired).
- [-] **T6.** G2/G3 real-suite repair — FOLDED with the lane (no repair to seat).
- [-] **T7.** katgpt-rs pool-contrast filing — the CONDITION (G1 passes) never
      fired. Separate note: the katgpt-core substrate's own demotion law ("the PoC
      battery failing its identification/repair arms records the negative and the
      lane folds into weight_edit_overlay") has now FIRED — filed as a katgpt-rs
      follow-up, not executed from this repo.
- [-] **T8.** Negative-control ledger — the record 0064 carries the measured dose
      arms + the declined final round; the lane's fold supersedes the ledger.

## Records

- Research 653 rev 4 (the fusion note; §3 carries the shared-column retraction, §5 the
  unified gate names + the proportional-alternation split law) · Research 649 + Bench 978 + Bench 979 (the substrate)
- Trainer facts verified this session: `train_arm_a` L679 / `train_core` L767 /
  `train_arm_b` L1115, `quantize_row` i8+scale L1170, `bag_into` L112, `VOCAB = 1<<17`
  L50, `winner_bridge` L211, lexicon-fork width refusal (specialist.rs L23-24)
- Collision prior art: Booking.com 2018 hashing-trick study; hash embeddings
  (arXiv:1709.03933) — train-time remedies only; the census's un-collided oracle ceiling
  is the unmeasured delta (Research 653 §4)
- Discipline adopted: winner law untouched (one frozen read); A8 (offline re-mint +
  monotonic hot-swap, never runtime); plan-016 §6 prereg law for any threshold; fair
  baselines before any "probe-authored beats training" claim.
