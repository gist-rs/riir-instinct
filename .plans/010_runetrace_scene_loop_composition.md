# The Runetrace scene-loop composition (Proposal 005 Phase 4, first lane — Research 613 executed)

**Status:** LANDED 2026-10-09 — T1–T6 all complete; gates green (the record
below). The Ext-seat serve wiring / rethink arm / hosted leg stay deferred
as designed.

## Why this plan exists

Research 613 (katgpt-rs `.research/613_runetrace_loop_modelless_scene_agent.md`,
verdict AGREE 2026-10-09) pinned the Phase-4 composition: a **modelless
scene-agent loop** over shipped katgpt-core GOAT primitives — scene in as a
deterministic RunetraceDoc, judgment out on decision_wire with first-class
abstention, and the loop's OWN reasoning rendered back as a Runetrace
EntityBlock (self-inspectable + digest-freezable). Every stage ships upstream;
the LOOP is the contribution. This plan lands the loop + its G1/G2 harness in
THIS repo (Research 613 §8: instinct is where the composition + the riir-reflex
dep are both legal).

NOT this plan (later lanes): the Ext-seat serve wiring (`/decide` registration
+ `arsenal.toml` row), the rethink trained escalation arm, the hosted-LLM leg
(**owner-gated** per Research 613 §6.6 — untouched here).

## The substrate audit (T0 — done before the plan, the substrate-first law)

Research 613 §7 swept the cousins (2026-10-09): no `.research/` note or module
composes these stages (`runetrace|scene-as-text|agent loop` greps + the
per-stage cousins karc_bridge / attack_reasoning / hybrid.rs — none closes the
loop or self-traces). Verified API homes this session:

| Stage | Primitive (all consumed, none re-implemented) | Gate posture upstream |
|---|---|---|
| wire-in | `katgpt_core::runetrace::{RunetraceDoc, EntityBlock, DagRow, DagStage, ConditionRow}` | opt-in, G1 byte-pin |
| embed | `riir_reflex::embed::{Embedder, EMBED_DIM}` (256-bucket hashed bag, zero-alloc) | reflex modelless |
| forecast | `katgpt_core::karc::{KarcForecaster, FourierBasis, DelayRing}` — the DEPLOYED shape `KarcForecaster<FourierBasis<8>, 8, 8, 4>` (riir-engine `karc_bridge`'s `HlaKarcForecaster` precedent; Fourier R=1 — the Bench-849 record is the honest quality ref, Issue 866 QUALIFY disclosed) | default-on upstream |
| rate | `katgpt_core::rating::{expected_f32, update_scored}` | default-on |
| goal | `katgpt_core::{LeoHead, sigmoid_bounded_q, DualLeoMixer}` (modelless head impl) | default-on |
| refine-halt | `katgpt_core::gain_cost_halt::GainCostLoopHalter` | opt-in (Bench 304 GOAT) |
| refine-stop | `katgpt_core::risk_control_exit::DualExitPolicy` | opt-in (Bench 681 lane) |
| refine-worth | `katgpt_core::state_probe::probe` | opt-in (Plan 621 Ph-1) |
| refine-patience | `katgpt_core::ignition::IgnitionSchedule` | opt-in (Bench 666 GOAT) |
| refine-trap | `katgpt_core::saddle_escape::FlipDetector` | opt-in (PoC-gated) |
| refine-step | `katgpt_core::contrast_combine::affine_combine` (gated `loop_guidance`) | opt-in |
| refine-tilt | `katgpt_core::entropic_tilt::tilt_advantages_into` | opt-in (Plan 341 pending) |
| decide | `katgpt_core::decision_wire::{Question, Answer, DecisionRequest}` | opt-in |

## Design (the composition laws — Research 613 §4)

1. **Embed the DOC's structured fields, never the rendered text.** Stage 1
   embeds a canonical FIELD PROJECTION (id/kind tokens, vital names with
   quartile-bucketed fractions, condition/action tokens) — the human render is
   never the embedder's input.
2. **Raw stays raw.** Vitals/positions ride the doc verbatim; Elo outcomes are
   raw vital deltas squashed by one sigmoid; only ratings/forecasts/Q are
   latent. Nothing latent is parsed back into game truth.
3. **Sigmoid, never softmax.** Arm probabilities are linear-normalized
   sigmoid-bounded Q values; every gate/confidence is a sigmoid.
4. **Think-brain locality.** The per-entity memory (delay ring, forecaster,
   Elo book) is loop-local; only the Answer (+receipt) crosses.
5. **Cadence follows observation.** `state_probe` gates whether the refine
   loop runs AT ALL (stable picks ⇒ skip); KARC fit is belt-cadence, forecast
   is per-decision.
6. **No second brain.** The Answer is advisory data for a consumer's
   brain+FSM — this loop never becomes one (Proposal 005 caveat 7).
7. **Determinism by construction.** No HashMap iteration on any answer path
   (keyed Elo access only); arms iterate in Question order; entities in doc
   order; every tie breaks to the first index. Two runs of one history are
   byte-identical (G1).

## Tasks

- [x] T1 — `Cargo.toml` feature `runetrace_loop` (forwards the katgpt-core set
  above: runetrace, decision_wire, rating, leo_all_goals, dual_leo,
  karc_forecaster, gain_cost_halt, risk_control_exit, ignition_schedule,
  saddle_escape, state_probe, entropic_tilt, loop_guidance) + the module
  skeleton `src/scene_loop.rs` (config, per-entity memory, the deployed-shape
  KARC alias) + lib.rs wiring.
- [x] T2 — the canonical projections: `drive_channels_into` (8-slot hashed bag
  of vital fractions + fired-condition drives, clamped) and
  `scene_fields_into` (the structured-field token projection; the hypothetical
  arm-fired variant for refine) — the stage-0/1 laws above.
- [x] T3 — the stages: forecast (belt-cadence fit via
  `observe_and_maybe_pair`/`fit_ridge`, per-decision `forecast_now`), rate
  (Elo keyed access, raw-delta outcomes), goal (the modelless `LeoHead` —
  hashed goal directions + Elo prior + forecast bias, `sigmoid_bounded_q`;
  the `DualLeoMixer` α-blend of rating-teacher vs forecast-student), refine
  (state_probe worthiness gate; `affine_combine` steps at `lam = 1 + ω`
  (the LoopCD form); `entropic_tilt` over arm margins; `GainCostLoopHalter`
  + `DualExitPolicy` + the `ignition_time` patience law; `FlipDetector`
  saddle kicks), decide (`Answer::choice` with linear-normalized sigmoid
  probabilities + margin confidence, or `Answer::abstain`; the confidence
  is a RAW readout, no calibrated-UQ claim — Report-the-Floor not yet
  bound, binds at any future calibration claim).
- [x] T4 — the self-trace: the loop's own fired controls as `ConditionRow`s +
  the stage path as `DagRow`s in an agent `EntityBlock`; digest via
  `content_hash`.
- [x] T5 — the harness: deterministic fixtures (inline LCG, no new deps), G1
  (same history + question → byte-identical Answer JSON + self-trace render +
  digest, run twice + cross-instance), G2 (per-decision latency at the
  reference population — 1 subject, 32 entities, 4 arms, ≤8 refine steps;
  worst-case compute stated; measured then pinned honestly).
- [x] T6 — gates run + docs (the lib.rs module doc, this gate record).
  FENCE GREEN (`fence_gate.sh --post-split` — the delta stays open-shaped).

## Gates (this plan's own — the composition, not the primitives)

- **G1 determinism: PASS** — byte-identical answers + self-trace + digest
  across runs/instances (`g1_determinism_same_instance_same_answer` +
  `g1_determinism_cross_instance`; the per-episode control kernels make
  `decide` a pure function of (memory, doc, question) — repeatability by
  construction, not by reset discipline).
- **G2 latency: PASS** — **136 µs/decision** (release, M3, 32 entities, 4
  arms, worst-case ≤8 refine steps — the eprintln'd measurement in
  `g2_latency_ceiling_debug_profile`; the debug-profile ceiling row keeps
  the 10 ms regression bar). The 1 ms serve bar holds with 7.4× headroom;
  the belt-cadence KARC fit is amortized OUTSIDE the decision (law 5).
- **G5-style honesty: PASS (as scoped)** — abstention first-class
  (`Answer::abstain` under the margin floor, doubled when cold), the
  forecast-cold + elo-known counts SAID in the trace rows; NO
  calibration/quality claim made (Research 613 §6.1 — architectural note,
  not a measured quality claim; the tails' own pending records stand).

## Post-landing notes

- The KARC buffer posture follows the deployed bridge exactly (capacity
  2048, NO auto-evict) — an intermediate clear-on-256 policy was caught by
  the harness (it collided with `min_samples = 256` and left the forecast
  permanently cold); the deployed posture is the correct one.
- `karc_min_samples` defaults to `256 = d_h` (the bridge's numerical-stability
  law: `N ≥ d_h` for the f64 direct path).
- Worst-case compute per decision (stated per Research 613 §6.4): 1 embed
  + probe (ensemble ≤ 8 × arm scores) + ≤ `patience_steps` refine steps ×
  (2 embeds + arm scores + control kernels) + the final score pass. The
  patience law (`t* = ln(1/0.05)/0.5 ≈ 6`) binds before `refine_max_steps`
  (8).

## Deferred (with reasons)

- Ext-seat serve wiring — needs the loop's question surface soaked first
  (this plan's harness); its own task/plan.
- rethink escalation arm — the trained half; the split law keeps it private,
  rides after the modelless lane stands.
- Hosted-LLM leg — OWNER-GATED (Research 613 §6.6); do not touch.
