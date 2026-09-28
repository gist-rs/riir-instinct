# Issue 005 — the Moka+PUCT-style hybrid POC: instinct (model) × reflex (modelless), three compositions + one GOAT gate

**Status:** OPEN — T1–T5 MEASURED; the GOAT verdict stands at the
ALIGNED Bench-052 protocol (Bench 002, the publishable record — the
Bench 001 v2 numbers below are superseded; Issue 006's bridge fix
corrected the v2 instrument): **ag_news promotes H2(β=0.25,nmin=2,τ=2)
0.8975 (G1 FAIL on the calibration axis — the raw fused readout is
already calibrated at 0.0133; the raw readout stands); emotion A1
0.8550 + sst5 A1 0.4217 (G1+G3 PASS); massive H2(β=1,nmin=2,τ=8)
0.8267 vs A0 0.7800 (+4.7 pt — the v2 "CHANCE/0.42" reading was the old
first-N sample's unrepresentative A0); banking77 H1 0.8060 @ 46.8%
consult — G3 FAIL, no promotable hybrid arm, A0 stands; xnli A0
stands.** The laya paired face RAN (Bench 002: lane sub-2 µs vs laya
metal 118–187 ms p50 — PASS) and the reflex-site hybrid lane publish
LANDED (test_publish_bench 22/22; the arena card + #sizes presence
`0a18105`). The single frozen test read is spent; predictions frozen in
`.benchmarks/002_hybrid_052_protocol/predictions.json` (Bench 001 v2's
frozen read is the superseded instrument's). Open faces, both
data-gated: **H3 (PUCT over chains)** — its only chain-shaped suite
(typed_decisions) NOW HAS a trained specialist (riir-train Issue 581's
full-pool retrain + Bench 015's multi-question serve extension: A1
0.6300 serves, T2-certified), so the specialist precondition is MET —
what remains is E0's route-terms caveat (typed's options are state-field
values, so the nb_scope margin term m_i never activates; arming H2/H3
there needs the option-conditioned scorer first). The arm re-opens when
that scorer exists; the specialist half is done. **The armed-off/serving faces rode the
arsenal Proposal 001 — CLOSED: T8's `arsenal_budget_goat` passed 7/7
(Bench 003, 2026-09-27; the signed-centroid defect it caught fixed in
the same landing).**

Superseded Bench 001 v2 status, for the record: ag_news H2(β=1,nmin=2,τ=4)
0.8975 vs A0 0.8625 / A1 0.8875; emotion A1 0.8550, sst5 A1 0.4217 vs
A0 0.5750 / 0.2017; massive read CHANCE (Issue 006); xnli 0.41 vs A0
0.5167; predictions frozen in `.benchmarks/001_hybrid_goat/`.

## Lineage (what the workspace already measured)

| precedent | prior | value | result |
|---|---|---|---|
| Moka PUCT (katgpt-rs Bench 205) | model policy head | model value head | **98.0%** vs Moka greedy at budget 200, c_puct 2.5, top_k 8 (p≈10⁻⁸). Bench 205:86: "NOT a modelless gain". |
| `engram_puct` (katgpt-rs Proposal 013, Bench 848) | model prior sharpened by a frozen count table, evidence-gated `sigmoid((n − n_min)/τ_n)` | model value, Q initialised from the table mean | G1/G4/G6 PASS, **G5 FAIL 48.1%** (Wilson low 44.8%). Cause: **rumor fraction 100%**: the count table rarely had evidence (gate ≥ 0.5 on 4 / 16,930). |
| `chance_puct` (katgpt-rs Bench 892) | modelless σ board score | modelless σ board score | G1–G4 PASS; wins only in the hardest regime, at 5–150× latency |
| riir-clippy Plan 049 (trained drafter + PUCT) | trained prior | V(passes-verify) | CLOSED-STOP: 0.94 pp < 2.0 pp bar. Its successor went back to modelless work. |
| `successor_density_critic` (katgpt-rs Issue 860, Bench 818) | — | modelless count critic `log p(s⁺=g\|s,a)/p(g)`, argmax 4.3 ns | GOAT PASS, opt-in. The one shipped modelless (s,a) value source. |

Moka's lesson is that the MODEL carries the gain. Proposal 013's lesson is that a
count-table fusion is only as good as its evidence density. Measure that FIRST.

## The arms (one harness run; each suite's gated arm pre-registered on the held-out train/cal slice; the test split read once, its predictions frozen and reused by P3)

- **A0 reflex alone:** riir-reflex modelless (`nb_scope`, the Bench 051 posture).
- **A1 instinct alone:** the specialist, no modelless term (the attribution
  control Bench 205:86 demands: the hybrid must beat THIS, not just reflex).
- **H1 cascade** (Proposal 047's default): reflex answers when the fused abstain
  gate passes; else the specialist decides among reflex's top-k. (The Salience
  Tri-Gate delegate — Bench 565, G3 PASS, ΔF1 +0.3145 — is the in-repo
  escalate-to-the-stronger-lane-only-when-informed precedent for this gate
  family.)
- **H2 prior fusion** (the Proposal 013 shape, transplanted): specialist prior
  `p_i` sharpened by reflex evidence:
  `p'_i = normalize(p_i · exp(g · β · m_i))`, where `m_i` = the nb_scope
  per-token margin for option i and `g = σ((n − n_min)/τ_n)` with `n` = the
  number of the state's tokens the tables have seen. β, n_min and τ_n are
  selected on the held-out TRAIN slice, never on test.
- **H3 PUCT-over-options** (multi-question / sequential decision chains only,
  e.g. typed_decisions cases whose questions condition each other): prior =
  H2's `p'`; value = the specialist value head (Moka's shape), optionally
  initialised from `successor_density_critic` Q. Defaults copied, not tuned:
  c_puct 1.5, top_k 8. Single-shot classification needs no tree, so H3 is
  scored only where a chain exists. **Value-head category rule (the in-repo
  measured lessons):** the value must be a per-option critic. A STATE
  forecaster cannot substitute — Research 322's category-confusion verdict
  (conformal-naive / BoMSampler / Sleep-Time / Best-Belief all produce state
  forecasts and fit no `Q(s,a)` seam), and RPE-style value feeding existing
  selection is the measured TMNF-C negative (Research 380 §5, the
  mb_personality rule). H3 stands on the Moka shape's own measured win
  (Bench 205) or it dies — it never substitutes a cheaper value source.

## E0 — measure evidence density before building H2 — MEASURED 2026-09-26

**Result (riir-reflex Bench 053, `harness --e0`: per suite, on the
stratified selection slice, the distribution of `n` over the DEPLOYED
count tables — full record + artifacts in
`../riir-reflex/.benchmarks/053_e0_evidence_density/`):**

| suite | view | med n | min n | seen/total | rumor fraction (< 4) | verdict |
|---|---|---|---|---|---|---|
| typed_decisions | bag | 129 | 61 | 100.0% | 0.0% | ARMED-PENDING |
| ag_news | bag | 81 | 32 | 98.8% | 0.0% | ARMED-PENDING |
| emotion | bag | 37 | 6 | 99.7% | 0.0% | ARMED-PENDING |
| sst5 | bag | 33 | 4 | 96.8% | 0.0% | ARMED-PENDING |
| prompt_injections | bag | 20 | 6 | 94.4% | 0.0% | ARMED-PENDING |
| xnli_en | bag | 57 | 10 | 96.3% | 0.0% | ARMED-PENDING |
| xnli_en | pair | 26 | 7 | 100.0% | 0.0% | ARMED-PENDING |
| massive_intent_en | bag | 9 | 3 | 81.0% | 6.5% | ARMED-PENDING |
| banking77 | bag | 18 | 8 | 95.0% | 0.0% | ARMED-PENDING |

The Proposal-013 death class does not reproduce on this substrate — **no
suite is pre-declared NOT ARMED; T3 is unblocked by E0 on all 8 dataset
suites.** Two recorded notes: (a) the evidence gate `g` will sit at ~1
almost everywhere, so H2's discriminating work is the margin term β·m_i,
with the gate a safety floor for the thin tail (massive's n = 3);
(b) typed_decisions is dense BUT Bench 051 measured its tables never arm
(options are state-field values, so route terms never activate) — arming
H2/H3 there needs the option-conditioned scorer first; density ≠
armability. Original expectation, for the record: "Expected (unmeasured):
low. Bench 051's NB tables see 91–95% of test tokens, unlike Go
transpositions, but the number decides." Confirmed and made precise — the
tables see 81–100% of every state's events, and the number now decides: GO.

## GOAT gate (merged from Proposal 013 + Plan 001)

- **G0 identity:** the kill switch (specialist absent / hybrid disabled) →
  output byte-identical to A0 (Proposal 013 G1). Separately — and NOT the
  same flag — `g ≡ 0` → output byte-identical to A1: with the evidence gate
  at zero, H2's fused prior IS the specialist prior, so a `g ≡ 0` run must
  never read as reflex. Two identity checks, two different collapse points.
- **G1 calibration:** the calibrated readout's ECE/Brier beats BOTH its own
  uncalibrated readout AND the conformal-naive-calibrated readout, all three
  scored on the SAME metric over the same questions (the riir-reflex G1
  posture; the floor is a competing calibrator, never a cross-metric number).
- **G2 latency:** the fusion overhead < 100 ns per option (H2/H3) or per
  question (the H1 gate check) (Proposal 013 G2 bar), AND the absolute p99
  published per suite. On escalated questions the instinct lane must be
  faster than laya, with laya − lane having a 95% upper bound ≤ δ, paired
  (the accuracy condition on this latency gate — same form as G3/G5).
- **G3 no regression (paired non-inferiority; regression = A0 − hybrid):** on
  the suites reflex already wins (emotion, sst5, massive, banking77 at Bench
  051), the per-question paired difference A0 − hybrid has a 95% upper bound
  ≤ δ (McNemar on the discordant pairs — both arms answer the same
  questions). Sign matters: the bound caps the REGRESSION; a hybrid better
  than A0 passes trivially, a hybrid worse than A0 by more than δ fails. A
  plain Wilson bound on hybrid accuracy cannot pass a parity hybrid (matching
  A0 exactly at 0.95 / n≈2000 reads a Wilson LB ≈ 0.940, i.e. "significantly
  worse"), which would have disqualified H1 — identical to A0 on every
  non-escalated question — for being no better. **δ per suite =
  max(1.0 pp, 2.5·SE_suite), pre-declared from a
  power calculation:** SE_suite is the paired SE computed on the held-out
  train/cal discordant rate at the test n. α = 5% is the chance of passing
  a hybrid truly δ-worse; the PARITY fail rate is the other error — at a
  fixed δ = 1.0 pp and SE ≈ 0.5–0.7 pp it is 36–58%, which is why δ scales
  with the measured SE: 2.5·SE ≈ 80% power at parity (≈20% parity-fail,
  stated, not hidden). H1's disagreements are confined to escalated
  questions, so its measured SE — and therefore its δ — is far smaller than
  a fixed-SE argument can show.
- **G4 alloc-free:** the hybrid hot path makes 0 allocations after warmup.
- **G5 quality:** BEFORE the test read, each gap suite (xnli,
  typed_decisions, ag_news) has its candidate arm pre-registered on the
  held-out train/cal slice — which arm is gated on that suite is a
  train-side decision, never picked off test results (three arms × three
  suites with "at least one" allowed is up to 9 looks at one test read;
  pre-registration is what keeps the single read honest; unregistered arms
  are REPORTED, never gated). A pre-registered arm passes if its Wilson 95%
  lower bound of accuracy > max(A0, A1) point accuracy with no G3 breach,
  **or** A1 − arm has a 95% upper bound ≤ δ, paired, under the same power
  rule (the Proposal 013 budget face, where the hybrid's value is the
  latency it saves). The pre-registration instrument is named in its own
  section below — Pareto rank-0 + argmax Beta-LCB on train/cal, never vibes.
- **G6 purity:** the reflex half is frozen counts (no runtime gradient); the
  instinct half is a sealed HOSTED-ONLY vessel (Issue 001).

Promote the winning arm, demote the rest. Report every arm, losses included.

## The pre-registration instrument (train-side; the Proposal 042 seam shape)

G5's "pre-registered on train/cal" names a decision, not a method. The
method, so arm selection cannot drift into vibes:

- On the train/cal slice, run every arm; keep the PAIRED per-question
  outcomes (both arms answer the same questions, so win/loss/tie counts are
  direct).
- Score each arm on three axes: accuracy Beta-LCB, escalation rate, p99
  latency.
- **Pareto rank-0 filter** over (accuracy LCB ↑, escalation ↓, p99 ↓) using
  the `dominates` shape (~30 LOC — riir-clippy `ruliology_search.rs`,
  borrowed from `katgpt-ruliology::WinMatrix::pareto_front`, Bench 572 GOAT
  lineage; the same seam shape Proposal 042 §3 prescribes at
  `QuestLeoScorer`).
- **The gated arm per suite = argmax Beta-LCB among rank-0**
  (`katgpt_core::beta_lcb_order_into` — the riir-dao strategy-selection
  substrate; conservative under small n, which is what train/cal slices
  are). Elo (`katgpt_core::rating`) is the companion READOUT where pairwise
  win-rates are reported, not the selector.
- H2's β / n_min / τ_n are selected by the SAME instrument over the
  train/cal grid — no test-side tuning anywhere.
- In-repo precedents for the shapes involved: `attack_reasoning` (Plan 575,
  Bench 898) is the LIVE `katgpt_core::rating` Elo option-choice shape at
  runtime (expected-score + deterministic UCB1 bonus) — the shape a PROMOTED
  lane would use for per-question routing, distinct from this one-shot
  selector; riir-games' `ruliology_arena` (Plan 213) is the in-repo
  WinMatrix/ParadigmRanking head-to-head shape (pattern reference only —
  riir-instinct never deps riir-ai); `best_belief`'s ε-quantile Beta
  conservative selection is the third corroboration of the Beta-LCB idiom.

Why not 042's pick verbatim: there it is a RUNTIME per-candidate seam (among
generated quest candidates, per request) and its wiring is gated OFF — Phase
0 closed UNMEASURABLE 2026-09-22 (dead completion channel), Phase 3 dead
until re-open. Here the shape transplants to the EXPERIMENT pre-registration
seam, where the channel is live: train/cal accuracy exists. Five
lineage-motivated arms do not need a full L3 enumeration; they need a
dominated-candidate filter that cannot be argued with after the fact.

## Plan

- [x] **T1 — E0** evidence-density measurement (reflex-only) — MEASURED
      2026-09-26: ARMED-PENDING on all 8 dataset suites (rumor fraction
      ≤ 6.5%, median n 9–129); record = riir-reflex Bench 053
      (`../riir-reflex/.benchmarks/053_e0_evidence_density/BENCH.md`).
- [x] **T2** — arms A0/A1/H1 MEASURED 2026-09-27 (Bench 001).
- [x] **T3** — H2 on the suites E0 clears; β/n_min/τ selected on the
      cal front by the pre-registration instrument (45-point grid;
      ag_news's winning point β=1, nmin=2, τ=4; on emotion/sst5 the
      grid's β=0 points collapse to A1 — the margin term adds nothing
      where the tables are unarmed or the prior dominates). MEASURED.
- [x] **T4** — H3: EXCLUDED with reason (no chain-shaped suite has a
      specialist; the value-head category rule bars substitution). The
      arm re-opens when a typed_decisions specialist is trained.
- [x] **T5** — GOAT gate G0–G6 recorded as Bench 001
      (`.benchmarks/001_hybrid_goat/RESULTS.md`), ONE test read
      (predictions frozen), losses reported. Open faces: the laya
      paired row (`--features arena-laya`) and the reflex-site publish
      (Issue 003 T4).

## References

katgpt-rs Bench 205, Proposal 013 + Bench 848, Bench 892, Issue 860 / Bench
818, Proposal 014:45 ("PUCT-over-options for sequential decision chains — the
Bench-205 recipe"); riir-clippy Plan 049 + Proposal 009; riir-ai Proposal 047;
riir-reflex Bench 051.
