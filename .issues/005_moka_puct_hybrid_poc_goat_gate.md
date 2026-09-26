# Issue 005 — the Moka+PUCT-style hybrid POC: instinct (model) × reflex (modelless), three compositions + one GOAT gate

**Status:** OPEN — filed 2026-09-26 (Plan 001 P3 design). Blocked on riir-train
Issue 576 (a specialist to plug in). The E0 measurement below can run NOW
(reflex half only). Gate AMENDED 2026-09-26 per verdict round 1 (proposal-001
reviewer): G0 kill switch split from the evidence gate, G1 floor-metric pin,
G3 → paired non-inferiority (δ = 1.0 pp), G5 per-suite arm pre-registration
on train/cal + δ on the budget face, G2 per-question wording for H1, T5/P3
single frozen test read (P3's laya row joins this run). Round 2: G3 sign
corrected (regression = A0 − hybrid) and δ re-based on a power calculation
(δ = max(1.0 pp, 2.5·SE_suite) from the train/cal discordant rate; a fixed
1.0 pp fails a parity hybrid 36–58% of the time — that was α/power
confusion); G5's budget clause given the same paired, correctly-signed form.
Round 3: AGREE — G2's laya clause also given the paired signed form.

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
  gate passes; else the specialist decides among reflex's top-k.
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
  scored only where a chain exists.

## E0 — measure evidence density before building H2 (runs now)

Per suite, on the stratified selection slice: the distribution of `n` (seen
tokens per state) and the "rumor fraction" (states with n < 4). Proposal 013
died at 100%. If a suite's rumor fraction is > 50%, H2 is pre-declared NOT
armed there, recorded as a measurement rather than tried and refuted.
Expected (unmeasured): low. Bench 051's NB tables see 91–95% of test tokens,
unlike Go transpositions, but the number decides.

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
  better. **δ per suite = max(1.0 pp, 2.5·SE_suite), pre-declared from a
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

Why not 042's pick verbatim: there it is a RUNTIME per-candidate seam (among
generated quest candidates, per request) and its wiring is gated OFF — Phase
0 closed UNMEASURABLE 2026-09-22 (dead completion channel), Phase 3 dead
until re-open. Here the shape transplants to the EXPERIMENT pre-registration
seam, where the channel is live: train/cal accuracy exists. Five
lineage-motivated arms do not need a full L3 enumeration; they need a
dominated-candidate filter that cannot be argued with after the fact.

## Plan

- [ ] **T1 — E0** evidence-density measurement (reflex-only, now).
- [ ] **T2** — arms A0/A1/H1 once riir-train Issue 576 ships a specialist.
- [ ] **T3** — H2 on the suites E0 clears; β / n_min and τ_n selected on held-out train by the pre-registration instrument (Pareto rank-0 + argmax Beta-LCB, below).
- [ ] **T4** — H3 on chain-shaped cases only; `successor_density_critic` Q-init as a sub-arm.
- [ ] **T5** — GOAT gate G0–G6, record as a Bench; promote the winner as the
      "Reflex · instinct" lane (Issue 003). ONE test read total: arms are
      compared and per-suite candidates registered on the held-out
      train/cal slice, the test split is read once, and P3's laya row is
      JOINED onto this run's frozen predictions — never a second pass.

## References

katgpt-rs Bench 205, Proposal 013 + Bench 848, Bench 892, Issue 860 / Bench
818, Proposal 014:45 ("PUCT-over-options for sequential decision chains — the
Bench-205 recipe"); riir-clippy Plan 049 + Proposal 009; riir-ai Proposal 047;
riir-reflex Bench 051.
