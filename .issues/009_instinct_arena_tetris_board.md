# Issue 009 — Instinct plays Tetris on the arena (a real board, not a text-suite card)

**Status:** OPEN — filed 2026-09-27 (owner direction). The placeholder card is pulled from the arena until this lands. HOW section added 2026-09-27 (training flow: search-distill via expert iteration, chance_puct teacher — verdict AGREE after 2 rounds); T5–T9 gate T7's start. KARC adjudicated same day: refused as drop predictor, added as a T8 modelless readout arm. **T5 MEASURED 2026-09-27 (Bench 006): WIRE-MUST-WIDEN — the five-ordinal spot wire carries at most 0.30 of the champion 1-ply evaluator's ranking (maximal decoder, full v2 state sentence included; 0.5470 demeaned) — the widened wire (raw afterstate, Instinct-only sidecar) is a precondition of the serve path, and its two cross-repo halves (reflex contract sidecar field, reflex-site sender) are the next lands. T6 MEASURED (Bench 007): PASS — the serving-matched teacher beats free Reflex by Δ+977.9 lb95 +969.7 pieces (b1600, paired seeds 1..=20); T7 unblocked. T8's second modelless arm MEASURED (Bench 009): NEGATIVE — the closed-form KARC basis-ridge critic (d=202) loses to b0 at every teacher budget (377.4 vs 907.8 pieces at b1600 targets, 2/6/12); b0 stands as the modelless floor and the trained critic's bar is b0 AND that arm. T7 substrate landed (feature contract + collector + fit + eval lane); training seeds 201..=300 pinned, targets at b1600 minimum. T7 EXECUTED 2026-09-28 (Bench 010): NEGATIVE at round-1 capacity — the trained critic beats the ridge arm in both regimes but loses to b0 (554.6 vs 907.8; 427.3 vs 532.5); no mint/serve; re-open levers are tail-targeted loss or critic-guided search. ROUNDS 2–4 (Benches 017/018/021) all NEGATIVE — capacity exhausted into the G2 breach. Round-5 pre-registered 2026-09-29 (teacher-side lever: r4-critic z-blend into the chance_puct teacher at w=0.5; verdict R1 REVISE → R2 AGREE); its first r4' rebuild was an INVALID BUILD (missing `--warm-epochs 80` — amended, discarded, no A/B spend); attempt 2 QUALIFIED 0.7679 ≥ floor 0.7647 (Bench 025, digest `5c7eb6ca4ae3bb3e`); the teacher A/B (Bench 026) is the next gate. **THE TEACHER A/B MEASURED 2026-09-30 (Bench 026, M3 session, digest-verified r4' component): NO-GO — both round-5 forms (z: Bench 026; residual: Bench 027) measured, neither promotable; no dataset, no r5 student, eval seeds still at 5 reads. The lane stops; the next priced lever is the loss/data-shape lane's offline label-blend test.** **ROUND-6 (the label-blend, the last priced lever) ANALYTICALLY CLOSED the same day (verdict R1 REVISE, adopted): self-distillation fixed point (r6 = r4's arch/config/rows trained on y′ = (1−λ)q + λ·f_r4 converges to f_r4 — no new ranking information, the same reason round 5 nulled) AND the decisive ground that a win could not ship (r6 is the r4 config at ≈3.76 ms/decision, 3.4–3.8× over the 1 ms serve bar). **T7 IS OUT OF PRICED LEVERS — every recorded candidate measured or analytically closed** (capacity: 010/018/021; loss family: 017; teacher blend both forms: 026+027; label blend: this closure). A future lever must bring new information the student can represent WITHIN the 1 ms bar (the widened-wire/afterstate feature side, or a different teacher signal). The issue stays OPEN owner-visible — T1/T2/T4/T9 gated on a mechanism that does not yet exist; eval seeds at 5 reads; nothing minted, nothing served.** **Concurrent M3 session (Bench 027, same seam, DIFFERENT form): the RESIDUAL value-seam blend measured NULL-TO-HARMFUL across its tested grid (−73.0 at b400/w=0.5 lb95 −120.4; −0.2 exact-null ±86 at b1600/w=0.5; −185.9 at w=1.0) — adjacent evidence for the A/B, NOT a falsification of the z-blend form (logit vs sigmoid, interpolation vs residual, top-8 vs all-options frame, and the A/B's cap-5000 16@75 cell unmeasured by it); the plain teacher's budget ladder is flat in PLAY STRENGTH within the cap windows (t6400 − t1600 +11.7, lb95 −15.1; early-death discordance flat; label accuracy vs budget not measured).**

## Why

reflex-site `f7a0574` put an "Instinct (hybrid) · trained specialists" card in
every arena game. It was a list of text-suite accuracies (ag_news, emotion, …)
standing where a game board belongs, and its own note admitted it: *"a text-suite
card, not a game board: game spots answer through its Reflex half"*. The owner
expected Instinct playing Tetris. That card was an unrelated patch, so it is
removed (reflex-site, same day) until Instinct has its own play.

There was no plan for this. Proposal 001 A4/A10 names the tetris/flappy/lanes
heads as PUBLIC-RELEASE vessels, which is the substrate, but no task ever made
Instinct DECIDE a Tetris spot.

## Tasks

- [ ] T1 — Instinct `/decide` answers a Tetris spot through its OWN path
      (the tetris head vessel through the Instinct serving composition, not a
      pass-through to Reflex). If the answer is byte-identical to Reflex's,
      the board shows nothing new. Record that as a negative, don't ship it
      as a lane.
- [ ] T2 — Record a seed-607 walk (the arena's `tetris_walk` protocol) and
      chain-verify it with `arena_demo_check.mjs` like the other lanes.
- [ ] T3 — GOAT per Issue 008: Instinct must strictly beat free Reflex's
      board (score / lines / pieces on the same seed), with per-spot p50
      disclosed. A tie sells nothing.
- [ ] T4 — Site half (reflex-site): a `tc-instinct` board card (canvas +
      readout like every other board), raw still last. Arena smoke re-pins
      the grid. `arena_demo_smoke.mjs` now reds on any `*-hybrid` card, so a
      text card can't come back.
- [-] Flappy / lanes: after Tetris.

## How — the training flow (owner question 2026-09-27; verdict ×2 rounds → AGREE)

T1–T4 gate the WHAT (own path, strict beat, walk verify). This section records
the HOW. Verdict provenance: 2026-09-27, two-round verdict ping-pong. Round 1
REVISE moved the recipe off Bench 205 (Moka PUCT is TWO-PLAYER 9×9 GO — negamax
sign flip, no chance nodes; its budget 200 / c_puct 2.5 / top_k 8 are Go
settings and do not transfer; "self-play" is also the wrong term single-player —
this is EXPERT ITERATION) onto the Tetris-native precedent, and round 2 added
the two amendments folded into T6 and T8 below.

**Recipe — search-distill via expert iteration** (not GRPO-from-scratch; GRPO
stays the recorded fallback if the critic's eval cost proves prohibitive). The
lineage that decides it: Bench 892 `chance_puct` passes G1–G4 as a MODELLESS
search yet wins only the hardest regime at 5–150× latency, and Bench 205's own
verdict line says "NOT a modelless gain — the MODEL carries the gain". So the
modelless search is the TEACHER, the trained critic is what serves, and a
modelless-only board does not carry the gain.

- **Teacher:** `katgpt_core::chance_puct` (7-bag chance nodes, sigmoid priors,
  mean backup) over the rulebook-champion evaluator (`68cae9d382014662` — the
  Bench-891/892 champion riir-reflexer's forward-freeze law already pins) on the
  engine parity-checked against the arena's seed-607 walk (60/60 identical
  picks). Budget: Bench 205's budget 200 / c_puct 2.5 are GO settings and do
  NOT transfer — pin from the Bench-892 Tetris regimes
  (`tetris_08_puct_goat`: budgets 100/400/1600) and re-verify the teacher
  margin at T6.
- **History/sequence model: NONE, by construction.** The 7-bag is a uniform
  random permutation — the ONLY structure in drop history is the current bag's
  remaining composition, and `chance_puct` already consumes exactly that
  (samples the remaining-bag distribution at its chance nodes). A learned
  higher-order predictor over recent drops (LEO-style sequence memory,
  `QuestLeoScorer` transplant, …) would be fitting the RNG stream — the
  memorize-the-fixture class; Proposal 042's Phase 0 also closed UNMEASURABLE.
  **KARC included** (Plan 308, `katgpt_core::karc`): its GOAT G1 defines its
  domain of validity — DETERMINISTIC temporal structure (double-scroll
  attractor, 8 Lyapunov times; in riir-engine, HLA belief trajectories, a
  semantic-domain signal with real inertia). The 7-bag is i.i.d.-within-bag:
  `P(next | history) = P(next | bag state)` exactly, so the delay ring carries
  zero predictive bits and KARC would learn the uniform marginal that exact
  enumeration already gives for free (the Research-322 category-confusion
  class). Cheap closing gate if ever contested: measure `H(next | last-k)` —
  it equals `H(uniform over remaining bag)` by construction. If T5's widened
  wire carries the bag state or a preview, consume it DIRECTLY — exact
  enumeration of a known distribution beats any predictor of it. The
  head-to-head ("duel") instrument IS in the design already: paired per-seed
  win/tie/loss + sign test for selection, never memory. **Where KARC DOES
  belong: the value-function readout — see T8's second modelless arm.**
- **Serve:** ONE 1-ply afterstate value critic, argmax over the offered options.
  A placement is deterministic, so V(afterstate) IS the per-option critic
  Q(s,a) — Issue 005's value-head category rule holds. Policy+V at 1-ply is
  redundant: serve one or pre-register a blend, and report the
  student-vs-teacher gap. If 1-ply falls short the next rung is the serve-time
  expectation over the 7 next pieces (~240 head evals, <1 ms) — NEVER
  serve-time PUCT.
- **GOAT posture:** promotion gate = T3 (strict paired-seed beat) with T8 as
  the attribution arm; G2/G4 are inherited from chance_puct's own gate
  (µs latency, alloc-free on a heap-free game) and must be RE-ASSERTED on the
  Instinct serving path at T9. G1 calibration is NOT binding for pure argmax
  play with no confidence output — but if this lane ever serves confidence or
  abstention, the Report-the-Floor rule binds (beat the conformal-naive
  floor, katgpt-rs Plan 340; the repo AGENTS G1 wording).
- **Labels:** record as search-distill. Issue 005's H3 (serve-time PUCT over
  chains) stays open and unmeasured — this is NOT an H3 landing.
- **Elo:** readout/companion only. For a single-player game the direct
  instrument is paired per-seed win/tie/loss + sign test on held-out seeds; the
  selector stays Pareto rank-0 + Beta-LCB (Issue 005's instrument section); the
  gate stays T3.

**The binding constraint (why the order below is rigid):** the arena wire is
text-only — a state sentence plus one spot sentence per option, and each spot
decodes to FIVE coarse class ordinals (holes/side/surface/height/clears;
`riir-reflex/src/game_heads.rs`). No next-piece preview, no raw grid. A trained
head fed only those five categorical inputs is a tiny lookup table and reflex's
ridge fit is already near that ceiling — distilling into it would likely TIE,
which T1 records as a negative. Hence T5 first, and T6's information rule.

Execution order: T5 → T6 → T7 → T8 → T9. T7 must not start before T5 and T6
land.

- [x] T5 — **Pin the serving input contract.** MEASURED 2026-09-27 —
      **WIRE-MUST-WIDEN** (`.benchmarks/006_wire_ceiling/BENCH.md`; the
      instrument = katgpt-rs `examples/tetris_10_wire_ceiling.rs`,
      `required-features = ["template_decode"]`). champ1ply (the champion
      genome with NextPreview OFF + depth 1 — `plies()` clamps to ≥2 while
      the preview rule is on, and depth-2 injects `TOPOUT` values; both
      traps fixed) vs the maximal wire decoder (saturated cell tables over
      the 5-tuple ⊗ piece ⊗ the full v2 state sentence, fallback to the
      reflex-head-shaped ridge): rank agreement **0.3002** (n = 378 301
      held decisions), board ratio **0.001** (90 pts / 19 pieces vs 107 638
      / 3153), all 120 wire games top out, sign p < 0.0001 — both
      pre-declared MUST-WIDEN gates fire. Occupancy ruled out (t1 dense,
      agreement still 0.147). The binding-constraint hypothesis is
      confirmed quantitatively: the wire ceiling is far below tie.
      **Design decision recorded (per this task's own instruction): the
      wire widens INSTINCT-ONLY** — the Instinct `/decide` request carries
      the raw afterstate (grid rows + piece + bag remainder) as a sidecar
      sidecar beside the unchanged 5-class text; the modelless lane, v0.2.x
      serving, and every fixture/pin stay byte-identical. Owed lands:
      the reflex contract's optional sidecar field + the reflex-site
      sender/board-card half (T4's session). T6 (teacher check,
      serving-matched) is now the gate on all GPU spend; note the probe
      already shows the full-feature 1-ply champion is very strong under
      serving information (3153 pieces / 1256 lines avg on held garbage
      seeds), so the T6 separating-regime bar is reachable.
      **REFLEX HALF LANDED 2026-09-28 (riir-reflex `0c9fcab`): the
      `/decide` wire carries the named optional `sidecar` member —
      parse-and-ignore on the modelless lane (byte-identical answers
      pinned with/without + permissive garbage shapes + malformed-body
      still 400s; README wire doc updated). Permissive by contract: this
      half does NOT pin the afterstate schema — the consuming side pins
      it when T9's serving lane lands. Remaining halves: the instinct
      sidecar acceptance/consumption (T1/T9) + the reflex-site sender
      (T4's session).**
      **CROSS-MEASURED same day (`.benchmarks/008_signature_spread`, a
      second instrument, run concurrently — the Issue-825 class, resolved
      by cross-reading both instruments + a Claude verdict round):** the
      5-tuple pools value-different options in ~89% of teacher-play
      decisions (mean max within-group spread ≈ 29 pts), yet the
      per-decision max-oracle still reaches the eval argmax 95% of the
      time — pooling costs ≤ ~4–6 pick points, so 006's decoder loss is
      dominated by CONTEXT loss (the wire cannot tell which signature
      group wins in a new decision). Consistent with 006; the spread
      record's first draft read the oracle as "no-widening" and was
      re-scoped (a ceiling computed WITH the full board is not a wire
      ceiling).
      **Demeaned-decoder addendum (the verdict round's owed run, landed
      katgpt-rs `5961e0991`):** 006's tables averaged ABSOLUTE values —
      a target that tracks position goodness as much as option rank. The
      demeaned-table class (cells bumped with value − decision mean;
      `wire-t1d`/`wire-t3d`) more than doubles the best agreement:
      **0.5470** (t1d; mean rank of the champ pick 0.978, top-3 89.2%) vs
      t3's 0.3002 — **~half of the first-read gap was the ESTIMAND, not
      missing wire information; the information is still the bound** —
      and 0.547 remains ≤ the 0.85 MUST-WIDEN gate, board ratio 0.013,
      sign p < 0.0001. Both decoder classes fail; WIRE-MUST-WIDEN is
      robust across the family.
- [x] T6 — **Teacher check, serving-matched, BEFORE any distillation.**
      chance_puct over the rulebook champion must STRICTLY beat free Reflex's
      board on held-out seeds in a separating regime (garbage starts of the
      Bench-892 16:75/18:75 class, or no-cap + score — a saturated
      both-survive-to-cap run ties and sells nothing; keep seed 607 beside it).
      Information rule (verdict amendment): run with NO PREVIEW and a FRESH
      BAG — exactly what serving sees — else the check can pass on information
      the student never gets. (Critic TARGETS may still be mined with the full
      teacher: a board's value averages over the preview anyway.) A teacher
      that cannot win under serving information makes T7 pointless — this
      check fails cheaply before GPU time.
      **LANDED 2026-09-27 (`.benchmarks/007_tetris_teacher_check`;
      harness `src/tetris_lane.rs` + `benches/tetris_teacher_check`,
      feature `tetris_goat`): PASS — T7 UNBLOCKED.** Teacher = chance_puct
      over the champion evaluator, no preview + fresh bag (uniform-7
      chance), budgets 0/100/400/1600 (c1.5 k8 — the 08-goat posture, NOT
      Bench 205's Go settings), paired seeds 1..=20, seed 607 beside
      (excluded from every gate column), teacher rng seed ^ 0x05EE_D892.
      Garbage 16@75 cap 1000: reflex-head 22.1 pieces/g (0/20 survive) vs
      teacher b1600 1 000.0 pieces/g 20/20 — paired Δ+977.9, lb95 +969.7
      (house instrument `stats::paired_upper_bound_f64`). Garbage 18@75:
      Δ+788.2, lb95 +612.2, W/T/L 19/1/0; the budget ladder separates
      (559 → 561 → 753 → 802). Empty-board context (cap 300): reflex
      67.4 pieces/g, 0/20 to cap (seed 607 tops out at 44); teacher b400
      20/20. **Even b0 — the bare 1-ply champion evaluator, T8's baseline
      shape — wins 18–20/0/1**, so the T8 bar is now concrete: the trained
      critic must beat that b0 row fed the WIDENED input. Per-spot p50
      disclosed (b1600 ~5 ms in-process; the T9 serving posture is 1-ply
      ~µs — G2/G4 re-asserted there).
      **Information rule (pinned):** this teacher ran on a FRESH BAG — less
      than the widened wire will carry (grid + piece + bag remainder); the
      T7 teacher consumes the bag remainder once the sidecar ships. A
      teacher that wins on less wins all the more on more.
      **Seed pins for T7:** training seeds must avoid ALL of 1..=40 (006's
      training), 101..=140 (006's held-out), 1..=20 (this bench's eval),
      and 607 — **suggested T7 training set: 201..=300**; the eval/T3 set
      stays 1..=20 + 607-beside.
- [-] T7 — **Expert iteration.** Round-0 SFT prior from
      `tetris_oracle_laya_en_v3` (BLAKE3-pinned; NEVER ships — by construction
      it reproduces Reflex, which is T1's recorded negative). Targets =
      search-root Q or a truncated n-step return, sigmoid-normalised (the
      chance_puct way — never a raw ~1000-piece episode outcome; sparse and
      noisy). 2–3 iterations. Training seeds DISJOINT from seed 607 and the T3
      seeds (a pinned seed is a memorizable piece sequence). lr=0 control arm
      separates a training gain from the fixture prior.
      **T7 MEASURED 2026-09-28 (`.benchmarks/010_tetris_trained_playoff`; trainer
      `riir-train crates/riir-train-engine/examples/tetris_critic_trainer`;
      play-off `benches/tetris_critic_goat --model`, the `LanePolicy::Mlp`
      arm): NEGATIVE at round-1 capacity — b0 stands.** The trained MLP critic
      (43→128→128→1, tanh hidden, f64, BCE on teacher q + 0.3×pairwise rank,
      b1600 dataset, val agree 0.7110 — CLEARS the ridge's 0.66 plateau, the
      linear-in-basis hypothesis confirmed; R² 0.2765; Arm A law held 0.711
      vs control 0.378) beats the ridge arm decisively in BOTH regimes
      (16@75: 554.6 vs 218.6, Δ+304.4 lb95 +96.5; 18@75: 427.3 vs 117.0,
      Δ+325.9 lb95 +104.4 — also above the ridge's own b1600 cell 377.4) but
      LOSES to b0 (907.8 / 532.5; trained-vs-b0 lb95 −609.2 / −359.7). The
      T3 gate (beat b0 AND the ridge, both regimes) FAILS → no mint, no
      serve wiring; round 2 stays gated. The informative finding:
      **imitation top-1 agreement is not play strength** — agreement rose
      0.66→0.71 while play went 218→555, still 350 short of b0; the residual
      disagreement concentrates in catastrophic placements. A re-open needs
      a tail-targeted loss or critic-guided search (not more top-1), or a
      much larger fit (perfect-imitation ceiling ≈ teacher ≈ 985 > 908, so
      the prize exists). Serve G2 measured anyway: trained p50 0.228
      ms/decision (1.3% of the 1 ms bar; ridge 0.006 ms). T5's parity
      contract LANDED + green: `tests/tetris_critic_parity` — the
      `TrainedMlp::score` serve-side forward replays the trainer bit-exactly
      (512/512 fixture rows, f64 op order, format v1 with the activation id
      in the header; model digest `a12f4a69c167b207`).
      **T7 ROUND-2 MEASURED 2026-09-28 (`.benchmarks/017_tetris_round2_playoff`; trainer `riir-train data/tetris_critic_r2/`, digest `66069784fb0fadd1`): NEGATIVE — the tail-targeted loss went BACKWARDS on play; round-1 weights stand.** The lever (pre-registered above): gap-weighted rank pairs (`--rank-gap 1.0`) + tail-weighted BCE (`--bce-tail 4.0 --bce-tail-pow 2.0`, the low-q tail 5×). Val agreement fell 0.7110 → 0.6270 (below the ridge's own 0.657 plateau) and play fell with it: 16@75 173.5 vs round-1 554.6 (vs b0 907.8, vs ridge 218.6 — lost to the RIDGE too, Δ−61.0 lb95 −233.6); 18@75 89.5 vs 427.3. The T3 gate (beat b0 AND the ridge, both regimes) FAILS by a wider margin than round 1. The honest reading: the tail-upweighting over-fit the tail (labels ≈0 everywhere) at the cost of the value surface's shape — round-1's "agreement ≠ strength" lesson cuts both ways: destroying agreement destroyed strength. **Round-1 artifacts (digest `a12f4a69c167b207`) untouched, still the strongest critic. Standing round-3 levers: critic-guided search, or a much larger fit (ceiling ≈ teacher ≈ 985 > b0 908); if the loss is revisited, run the gap and tail levers in ISOLATION first — this round confounded them.**
      **T7 ROUND-3 MEASURED 2026-09-28 (`.benchmarks/018_tetris_round3_playoff`; trainer `riir-train data/tetris_critic_r3/`, digest `bf9464925be910d8`): NEGATIVE on the T3 bar (b0 stands), POSITIVE on the capacity lever — the strongest trained critic yet.** Single-knob change from round 1: `--hidden 256,256` (77,313 params), everything else identical (same loss/schedule/seed/dataset). Val agreement **0.7574** (round-1 0.7110, round-2 0.6270; ridge plateau 0.657) — the trainer's own T2 gate reads INTERESTING for the first time. Play: 16@75 **743.9** vs b0 907.8 (Δ−172.1 lb95 −326.2, W/T/L **1/14/5**) and 18@75 **455.7** vs 532.5 (Δ−80.6 lb95 −306.5, 6/7/7); the ridge half now passes in BOTH regimes (Δ+503.2 lb95 +312.7 / Δ+355.7 lb95 +117.1). The width→play trend is monotone across rounds 1/2/3; the loss shape moved from "dies in most games" to "ties b0 in 14/20 and loses 5" — the catastrophic-pick deficit is down to ~25% of games. Per-spot p50 0.840 ms (3.7× round-1; within the 1 ms serve bar but tight — noted for T9). **No mint, no serve wiring. Eval seeds read four times now (009/010/017/018), every read a pre-registered lever, every negative recorded.** Round-4 levers priced in the record: 512×512 (~4–5 h) or critic-guided search (the ~240-eval next-piece expectation is TIGHT against G2 at 0.84 ms/critic — budget re-derivation before building).
      **T7 ROUND-4 MEASURED 2026-09-28 (`.benchmarks/021_tetris_round4_playoff`; trainer `riir-train data/tetris_critic_r4/`, ~287k params): NEGATIVE on the T7 bar — b0 stands; the capacity lever is EXHAUSTED.** Val agree **0.7697** (the trainer's T2 gate INTERESTING; rounds: 0.7110 → 0.7574 → 0.7697). Play: 16@75 **804.8** vs b0 907.8 (Δ−59.4 lb95 −214.2, W/T/L **2/16/2**) and 18@75 **462.7** vs 532.5 (Δ−73.2 lb95 −338.5, 7/5/8); the ridge half passes both regimes (+615.9/+363.1). The width→play trend is MONOTONE across rounds 1–4 (554.6 → 743.9 → 804.8) with DECAYING returns (+189.3 → +60.9) while G2 grows ~linearly: trained p50 **3.760/3.422 ms** — 3.4–3.8× OVER the 1 ms serve bar. The next width rung prices 8–15 ms for a return under +61: **no round 5 on this lever**. The recorded alternatives stand as NEW pre-registrations: critic-guided search (needs the budget re-derivation; at 3.76 ms/critic the next-piece expectation is ~900× the G2 bar) or a different loss/data shape. Eval seeds read FIVE times (009/010/017/018/021), every read a pre-registered lever, every negative recorded. No mint, no serve wiring; T9's gate (a critic that strictly beats b0) remains unmet.
      **The critic-guided-search BUDGET RE-DERIVATION (2026-09-29, the recorded GO/NO-GO):** the round-4 pricing conflated two lanes. As a SERVE policy, critic-guided search is dead by the same arithmetic — ~240 critic evals × 3.76 ms ≈ 0.9 s/spot, and the T9 serve posture's µs claim cannot carry it. As a TEACHER it is UNBOUNDED by serve G2: expert iteration's teacher runs offline (Bench 007's b1600 teacher ran ~5 ms/spot in-process), so a teacher that mixes the round-3/4 critic (256k params, val 0.7697 — the strongest fit) into the chance_puct evaluator costs seconds per spot and is FREE at serve, where the 1-ply student stays. **GO as a teacher-side lever** — pre-registered round-5: teacher = chance_puct over (champion evaluator + the r4 critic's afterstate values blended at the search's eval seam), budgets 100/400/1600, the T6 information rule (no preview, fresh bag), targets sigmoid-normalised as before, training seeds 201..=300 unchanged, the T3 bar unchanged (beat b0 AND the ridge in both regimes on seeds 1..=20 + 607-beside). The prize arithmetic: b0 907.8 < perfect-imitation ceiling ≈ teacher; a teacher STRONGER than b0 raises the ceiling the student distills toward — the mechanism round 1–4 lacked. NO-GO on building it THIS session (a full teacher build + dataset regen + retrain + playoff is a multi-hour lane); the pre-registration above is the next session's entry point. The OTHER recorded lever (a different loss/data shape) stays priced behind this one — the teacher ceiling bounds every student, so the teacher lever subsumes it.
      **ROUND-5 EXECUTION PRE-REGISTRATION (2026-09-29, the GO above; verdict R1 = REVISE, all revisions adopted before any spend).** The blend form: per-decision z-space blend at chance_puct's eval seam — `value()` on DECISION states carrying an incoming transition (verified against katgpt-core `chance_puct.rs`: afterstates are `Kind::Chance` nodes whose `outcomes()` is never empty, so they are resolved, never `value()`-ed; the seam is the resolved decision state at depth ≥ 1, whose board IS the post-placement board — the root stays plain champion so `value_ref`/`value_scale` keep champion units). `TeacherState::apply` stashes the option's `RawOpt` eagerly (computed in `actions()` where the `Placement` is in hand); `value()` returns `mean_e + ((1−w)·(e−mean_e)/std_e + w·(c−mean_c)/std_c)·std_e` where e = the champion eval (the same closure the plain teacher uses) and c = the r4 critic's LOGIT (`TrainedMlp::score_logit` — σ compresses exactly the catastrophic tail the critic exists to separate); w = 0.5 (the only w this round reads; a NO-GO closes w=0.5, not the lever). The z-fit set = the root's KEPT top-8 options by champion eval (the searcher's own value_scale calibration set); std 0 → 1.0 guard. Priors stay pure champion (the reviewer's recorded limit: the critic can reorder the champion's top-8 but never rescue a move below it — root-prior blending is the deferred second lever, not folded in here). w=0 short-circuits to the champion eval exactly (the plain path stays byte-identical). No-op guards, read BEFORE the A/B numbers: blend-path call share > 0 and blended-vs-plain root-pick diff share > 0. Blend component: "the r4 critic" rebuilt on THIS box (the M3 weights are absent; the r4 dataset is deterministically regenerable — export defaults b1600, seeds 201..=280/281..=300, regimes 16@75+18@75, cap 1000, threads 10) at the EXACT r4 config (hidden 512,512, epochs 130, **warm_epochs 80** — AMENDED 2026-09-29, see below, batch 2048, lr 1e-3, lr_min_frac 0.01, wd 1e-5, rank_weight 0.3, rank_beta 8.0, tanh, threads 10, seed 0x5eed0097); qualification floor = r4's own 0.7697 − 0.005 = **0.7647** (below that: provenance defect, lane stops); dataset + model digests recorded. M3 copy attempted first (removes the provenance question entirely). Teacher A/B (the GO/NO-GO, before any blended-dataset spend): plain b1600 vs blended b1600, seeds **401..=420** (disjoint from 1..=40, 101..=140, 201..=300, and 607 — the A/B picks a teacher, and choosing it on the student's eval seeds would leak the choice into the playoff), cells: garbage 16@75 **cap 5000** (b1600 saturates cap 1000 there), garbage 18@75 cap 1000 (b1600 survived only 16/20 at cap 1000 — already separating), garbage 20@80 cap 1000 (A/B-only hard cell; the student T3 eval stays 16@75/18@75). Per-cell censoring disclosed; a cell where BOTH arms hit cap on every seed is declared non-separating before reading. **GO iff blended−plain paired lb95 > 0 on ≥ 1 cell AND no cell has ub95 < 0** (the no-harm guard). NO-GO ⇒ the teacher-AB bench (allocated as **Bench 026**; the pre-reg's original `.benchmarks/023` spelling was stale — 023/024 were taken) records the negative and the lane stops. If GO: blended dataset via `export_tetris_critic --blend`, r5 = the r4 config with ONLY the dataset changed, playoff `tetris_critic_goat --model` (T3 bar: beat b0 AND the ridge, both regimes, seeds 1..=20 + 607-beside) → the round-5 playoff bench; the record discloses that the r5 student distills toward a teacher partly built from r4, so a GO at the A/B alone does not show the student passes b0. Eval seeds read exactly once more (the playoff).
            **ROUND-5 AMENDMENT (2026-09-29, verdict R1 = REVISE on the floor-failure reading; all revisions adopted): the first r4' rebuild is an INVALID BUILD, not a floor failure.** Attempt 1 (digest `7df0615d5c11a9de`, out `data/tetris_critic_r4p/`) omitted `--warm-epochs 80` — the flag defaults to **0** (`tetris_critic_trainer.rs:724`), so it ran a full 130-epoch cosine instead of rounds-1/3/4's 80-constant-then-cosine-50 (Bench 018 README:14: "130 epochs, 80 warm"; the attempt-1 log's own lr column decays from epoch 2, where a warm schedule holds 1.00e-3 through epoch 81). Measured best val agree 0.7333@127 — below the floor, but the cause is the schedule drift (a first-order training change), NOT the dataset: the `warm_epochs` knob was missing from this pre-reg's "EXACT r4 config" list (now added above) AND from the trainer's `metrics.json` config block (fixed in riir-train in the same commit). DISCARDED — no A/B spend, no r5, eval seeds untouched (still 5 reads). Also measured while adjudicating: the plain-export digest-clear — the dataset regenerated at the pre-blend commit 67b18e4 is byte-identical (all 4 file BLAKE3s) to the 60bd324-era regen on this box, so the blend commit did not change the plain teacher. Numbering corrections: `.benchmarks/.highwater` was stale (read 0023, disk had 024); the round-5 records allocate 025+ and the `.raw/` dataset digests are copied into the bench record (`.raw/` is gitignored — the manifest alone is not a record). Attempt 2 = the identical command + `--warm-epochs 80`, out `data/tetris_critic_r4p2/`, SAME unchanged floor 0.7647; if it also misses, the provenance-stop reading applies (and the cross-arch question becomes real — until then the ~0.3% sample-count delta vs the M3 counts stays an observation, its mechanism unattributed: the M3 per-file digests are not on this box, only Bench 010's counts). Attempt 2 landed as **Bench 025 (QUALIFIED, 0.7679 ≥ 0.7647, digest `5c7eb6ca4ae3bb3e`)**.
            **T7 ROUND-5 PILOT, CONCURRENT M3 SESSION (2026-09-29/30, `.benchmarks/027_tetris_round5_pilot`, revised after its own two-round reviewer pass; verdict AGREE): the RESIDUAL value-seam blend measured NULL-TO-HARMFUL across its tested grid — NO-GO for THAT form; adjacent evidence for the z-blend A/B (Bench 026), NOT a falsification of it.** The two sessions collided on round 5 (the Issue-825 class — recorded here per its own discipline): the M3 session read the same pre-registration's "afterstate values blended at the search's eval seam" as a RESIDUAL form (`champ + w·gain·(m − m̄)` on the critic's SIGMOID score, frame over ALL root options — `src/tetris_blend.rs`, `--critic`) and cheap-gated it before any spend, on non-eval/non-train seeds ≥ 401 with paired bounds: 18@75 — blend b400 w=0.5 **−73.0** vs plain (lb95 −120.4, n=100, cap 600; borderline under a six-test Bonferroni); blend b1600 w=0.5 **−0.2** (exact null, ±86 detection limit, n=60, cap 1000); blend b1600 w=1.0 **−185.9** (lb95 −283.5). No tested corner has a positive lower bound. The r4 critic is a bounded-weaker 1-ply ranker than the champion leaf (−98.1 vs b0, ub95 −36.6 — the M3 r4 weights, digest `7e319da89b7f5a7c`); the plain teacher's budget ladder is flat in PLAY STRENGTH within the cap windows (t6400 − t1600 +11.7, lb95 −15.1; early-death discordance 3-vs-5 flat; label accuracy vs budget NOT measured). **What the null does NOT cover (the z-blend A/B stays live):** the z-form's three deltas — LOGIT vs sigmoid (the z-form stretches exactly the catastrophic tail), interpolation vs residual, top-8 vs all-options frame — and the A/B's cap-5000 16@75 cell, which the pilot never measured (its 16@75 cells saturated at cap 600). The A/B (Bench 026, seeds 401..=420) proceeds per its pre-registration; the pilot's numbers are the prior beside it. w=0 is the plain teacher BIT-EXACT (unit-pinned). The pilot spent no eval-seed read. The residual-form instrument retires if the A/B settles either way (the loser retires — recorded in the export bin's doc). Remaining priced lever regardless of the A/B: the loss/data-shape lane (incl. the offline label-blend test on the existing dataset — cheap, low prior).**
                        **T7 ROUND-5 TEACHER A/B MEASURED 2026-09-30 (`.benchmarks/026_tetris_round5_teacher_ab`, executed on the M3 with the digest-verified r4' component `5c7eb6ca4ae3bb3e` after the 4090 session moved to the t599 encoder lane; the A/B is a CPU play-strength eval, seed-deterministic, box-independent in its verdict — **independently CONFIRMED on the 4090 the same night: two byte-identical runs, verdict inputs reproduced, addendum in the same record**): NO-GO — the lane stops, per the pre-registered GO rule.** Cells (seeds 401..=420, both arms b1600): 16@75 cap 5000 — plain 4535.1 pieces (17 cap) vs blended 4931.1 (19 cap), Δ+396.0 **lb95 −119.7** ub95 +911.7, W/T/L 3/17/0 (blended survives longer but NOT significant; 17/20 ties at cap); 18@75 cap 1000 — plain 654.95 vs blended 605.9, Δ−49.05 lb95 −145.8 ub95 +47.7, W/T/L 1/16/3 (leans the OPPOSITE way); 20@80 cap 1000 — **DEGENERATE: both arms died at 0 pieces on all 20 seeds** (the pre-registered "hard cell" was unplayable — no legal first placement; a pre-reg design miss recorded in the bench: future pre-registrations single-probe a new regime's playability before allocating a cell to it; it matched neither the censoring rule nor any verdict path and changed nothing). No-op guards PASS (177,104,397 blend-path value() calls; 15,370 pick diffs — a real blend was measured). GO rule: lb95>0 on 0 cells, ub95<0 on 0 ⇒ **NO-GO**. The points column agrees harder than pieces: the blended teacher scores FEWER POINTS in BOTH live cells (219,615 vs 274,957 at 16@75 despite +396 pieces; 23,429 vs 36,542 at 18@75) — a survival-biased style shift, a WORSE distillation target for a game scored on points. **Both round-5 forms are now measured and neither is promotable** (z-form: this bench; residual form: Bench 027) — the r4-class critic adds no ranking information the champion evaluator lacks at these budgets, in either blend form. Closed by scope: the z-blend at w=0.5 (the pre-reg's own wording). NOT closed: other w values (thin — the residual form measured w=1.0 harmful and w=0.5 null; the two forms bracket the range), and the prior-seam blend (never priced, ~27× critic calls). No blended dataset built, no r5 student trained, **eval seeds still at 5 reads** (the A/B spent only the disjoint 401..=420 block). Next priced lever (the issue's standing order): the loss/data-shape lane's offline label-blend test — q_teacher blended toward the critic's score on the EXISTING b1600 dataset (no teacher regen; cheap; low prior, both round-5 nulls depress it further).
                                    **ROUND-6 PRE-REGISTRATION VERDICTED AND ANALYTICALLY CLOSED (2026-09-30, verdict R1 = REVISE — paper-close on revised grounds, all revisions adopted; NO compute, NO eval-seed read).** The draft pre-reg asked RUN / PAPER-CLOSE / AMEND for the label-blend test (`--label-blend <model.bin>:<λ>`: TRAIN labels y′ = (1−λ)·q_teacher + λ·σ(critic_logit(x)), VAL untouched, r6 = EXACT r4 config + labels only, T3 playoff). The verdict closed it on three grounds, none depending on the draft's own (struck, unmeasured) memorization premise — the r4 metrics record no train-row agreement and everything they do record (287k params vs 1.07M rows; best epoch = the LAST epoch; the monotone width→play returns of rounds 1–4) reads as UNDERFITTING, not memorization: **(1) the fixed-point refutation, valid in both branches** — r6 has r4's architecture, config, and rows; if the critic memorized, the blend injects its fitting error as label noise; if it underfits, critic(x) is already the projection of q onto what this model class can learn, and for the squared-error part training on y′ = (1−λ)q + λ·f_r4 converges to (1−λ)·f_r4 + λ·f_r4 = f_r4 (the rank term, weight 0.3, pulls the same way) — r6 learns r4 again; self-distillation theory (Mobahi et al. 2020) adds that repeated self-distillation acts as extra regularization, pointing WEAKER for a capacity-bound model. Neither branch adds ranking information the student lacks — the same reason both round-5 forms nulled. **(2) the decisive ground the draft missed: A WIN COULD NOT SHIP** — r6 is the r4 config, per-decision p50 ≈ 3.76 ms = 3.4–3.8× OVER the 1 ms serve bar; even a b0-beating result would be unmintable and unservable (its only use would be motivating a smaller student, and the servable widths ≤256×256 already lost by Δ−172.1). Spending eval-seed read #6 on an experiment whose best outcome cannot ship fails the issue's own read discipline. **(3) the escapes are void** — a disjoint-seed critic is the same model class distilling the same teacher (the fixed point applies to it too, at the price of the same retrain); large-|ε|-row blending and λ sweeps are extra degrees of freedom on a mechanism with no expected effect. **T7 IS OUT OF PRICED LEVERS** — every recorded candidate is now measured or analytically closed: capacity (rounds 1–4, Benches 010/018/021), the tail/rank loss family (round 2, Bench 017 — NEGATIVE; the confound note stands for any revisit: isolate gap and tail first), the teacher-side blend in both forms (round 5, Benches 026+027), and the label blend (this closure). **What any future lever MUST bring: new information the student can represent WITHIN the 1 ms serve bar** — e.g. the widened wire / raw-afterstate feature side (the T5 WIRE-MUST-WIDEN finding is the standing precondition), or a different teacher signal — not a reshaping of the same rows with a model of the same class. The issue stays OPEN (T1/T2/T4/T9 gated on a mechanism that does not yet exist, owner-visible); eval seeds stay at 5 reads; nothing minted, nothing served.
- [ ] T8 — **Ablation — the win must come from the trained part.** Baseline =
      1-ply rulebook-champion evaluator fed the SAME (widened) input as the
      trained head (verdict amendment). The trained critic must strictly beat
      THIS baseline, not just Reflex: preview search + Dellacherie-style rules
      are modelless gains that belong in free Reflex, and shipping them there
      raises Instinct's bar (Issue 008 root cause 1). **Second modelless arm
      (KARC readout, added 2026-09-27):** a closed-form KARC basis-ridge fit of
      V(afterstate) over the same board features (Plan 308 — no delay ring,
      plain basis expansion + ridge; the reflex head's linear micro-fit with a
      nonlinear basis; modelless per Plan 332's own annotation). The trained
      critic must beat BOTH arms; if the KARC readout wins, the modelless path
      wins and that is the honest verdict under the modelless-first mandate
      (same vessel serving, closed-form weights, no expert iteration). Note
      the readout upgrade does NOT escape T5: the reflex head's ceiling is the
      5-coarse-class input, not the ridge shape.
      **Second arm MEASURED 2026-09-27 (`.benchmarks/009_tetris_critic_arm`;
      harness `src/tetris_critic.rs` + `benches/tetris_critic_goat`, feature
      `tetris_goat`): NEGATIVE for the modelless path — b0 stands.** The
      closed-form KARC chebyshev-4 basis-ridge critic (d=202; mode×champion-set
      interactions generalize the FSM; fit on the teacher's sigmoid-normalised
      search-root Q; λ selected on val 281..=300 by top-1 agreement) beats free
      Reflex easily but loses to b0 at both teacher budgets: 16@75 b0 907.8 vs
      critic 218.6 (b400 targets) / 377.4 (b1600 targets), 2/2/16 and 2/6/12;
      18@75 the same shape. b0's row reproduces T6 at 20/0/0 vs reflex
      (Δ+881.0 lb95 +752.1) — a cross-bench consistency pin. Determinism:
      digest + all eval numbers byte-identical across worker counts. The
      trained critic's bar is now concrete: strictly beat b0 AND this arm
      (digest `48b92d26b04f3ecb`). T7 GO — see the notes on T7 above.
- [ ] T9 — **Mint + serve.** riir-train trains + mints the PUBLIC-RELEASE
      vessel (Proposal 001 A4/A10 — no game-IP content) via `vessel-mint`;
      file the training run as its own riir-train issue. Instinct serves the
      1-ply critic through its own path (T1); per-spot p50 disclosed with T3's
      numbers. Adapter home: promote the katgpt-rs
      `examples/common/tetris_puct.rs` adapter out of examples/ via the
      boundary-guard skill — do NOT write a fourth Tetris engine (three
      already exist: reflex-site JS, katgpt-rs examples, riir-reflex).

---

**LEVER PRE-REGISTRATION (2026-10-02, from riir-ai/.research/393 + riir-train Plan 434 — no compute, no seed read):** the NCA-critic lever (iterative local-rule critic over the raw 10×20 grid — a different model class than the rounds-1–4 stateless MLP, per arXiv:2609.36126) is pre-registered CONDITIONAL on two gates that must BOTH pass before eval-seed read #6 is spent: (a) the widened-wire precondition (T5 raw-afterstate sidecar — Bench 006 WIRE-MUST-WIDEN) — without it the raw-grid input cannot reach serve at all; (b) the serve-budget derivation (Plan 434 T1.1, HARD STOP per the round-6 ruling — no offline rescope): a priori the dense 3×3 × K-step × ~40-afterstate arithmetic prices ABOVE round 4's 3.76 ms even at C=8, so the expected verdict is NO-GO recorded arithmetic. Trainer home: riir-train `--arch nca` arm; adapter via the tetris_puct promotion path (no fourth engine).

**LEVER VERDICT — NOT-BENCHABLE-WITHIN-THE-BAR (2026-10-03, riir-train Plan 434 T1.1 EXECUTED, `78ba638a`): GATE (b) FAILED on measured arithmetic — the lever is DEAD, gate (a) is moot, eval-seed read #6 UNTOUCHED (ledger stands at 5 reads).** Forward-only microbench (`riir-train-engine/examples/plan434_nca_budget.rs`), RANDOM weights (cost does not depend on trained values), single-threaded scalar loops in the incumbent MLP's own code style, B=40 afterstates/decision, n=500, 4090 workstation (loaded box — concurrent sibling builds; min-vs-p50 spread ~2–4%, cannot flip a 2.6× gap): **scoped cheapest arm C=8, K=4, f32 p50 2.604 ms vs the 1 ms bar** (quick pass n=100 read 2.647 — stable); full grid C=8/K=4 2.604 · C=8/K=8 5.170 · C=16/K=4 6.217 · C=16/K=8 13.422 ms (f64 twins 2.608 / 15.404 — ≈ f32: the forward is tanh-bound, not multiply-bound, so the dtype escape buys nothing). MACs/decision 4.29M–33.2M = **4.9×–37.6× the round-4 MLP (880,640) that already breached at 3.76 ms**; the implied per-MAC rate (~1.6 GMAC/s) is ~7× the incumbent posture's own efficiency, and still 2.6× over — the NO-GO is robust beyond naive MAC scaling. Per the plan's frozen hard stop: NO offline rescope, NO arena-offline fallback. Plan 434 CLOSED same day.
