# Issue 028: twinarm probe-repair pass PoC (Research 653 rev 2 — 649 × twinarm consumer fusion)

**Status:** Active — PoC filed 2026-11-05 (rev 4 same day, verdict rounds 1–3
addressed); G0 oracle census runs FIRST (fold law pre-registered); class 1 (weak-teacher
collapse) is the repair target, class 2 (collisions) is census-only; the tune/eval split
is pre-registered below (proportional alternation — no seed constant exists to pin) and
binds G1 AND G2. **Update 2026-11-05: T1 census DONE (86a8de2, record 0063) — every
suite immaterial at 2¹⁷ ⇒ CLASS 2 FOLDS as a recorded negative; class 1 (the repair
target) is unaffected and the lane proceeds to T2.**
**Claimed:** zed-4090-4fa6, 1793897593, expires 2026-11-07T16:53:00+00:00
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
- [ ] **T2.** G1 fixture: pinned corrupted-teacher synthetic corpus; build the
      tune/eval split by the proportional-alternation rule (+ its unit test), THEN
      train; Arm A + Arm B train; gold-pool parse recovery; dose-sweep repair (tune
      slice); the full gate arms incl. the baseline battery; the single eval-slice
      read.
- [ ] **T3.** Mode T repair path (f32 trainer-state pre-seal) + re-seal; isolation asserts.
- [ ] **T4.** Mode A repair path (±integer i8-artifact-direct, monotonic version bump);
      operates on the sealed artifact + a labelled authoring slice (skips the full
      training corpus, never the labels); salvage posture.
- [ ] **T5.** G4 grammar sidecar: record shape (drive bucket × gate pool × gap class ×
      window × word), BLAKE3 digest, stale-epoch refusal.
- [ ] **T6.** G2/G3: repair where class 1 applies on real suites (conditional on a
      surviving Bench-609-era teacher dump or a cheap re-capture); the single
      eval-slice paired read vs base + baselines; winner law for any seat (sst5 the
      candidate); board republish via the existing publish path.
- [ ] **T7.** Conditional katgpt-rs follow-up (ONLY if G1 passes — the parse needs the
      pool-contrast stats; the census G0 does not): file the pool-contrast
      generalization issue for `katgpt_core::probe_circuit_grammar` (the consumer's pull
      — katgpt-rs stays consumer-free by default).
- [ ] **T8.** Negative-control ledger: every refused repair word + the reason, appended
      to the bench record (the sweep's dose arms are the dose-response evidence, 649 §4
      style).

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
