# Issue 008 — Instinct must strictly beat EVERY published lane on every suite it sells

**Status:** OPEN — filed 2026-09-27 (owner direction). T1+T2 EXECUTED 2026-09-27
(Bench 004); serving/display landed `4cc4441` + reflex-site `aeea24b`. **AMENDED
2026-09-29 (owner direction): the bar is no longer "beat free Reflex" — free
Reflex is the FLOOR.** The product claim is **"the best published lane on every
suite it sells"**: every comparison lane (laya's best checkpoint, clm, gliner,
agentjev, openthai, paw) and Reflex itself. Today it does not (measured below).
Site consequence landed same day (reflex-site): the "Instinct vs Reflex" row
MOVED from the arena TL;DR to `/bench/#instinct` (the arena stays Reflex's),
rendered beside the new "vs best lane" row and a **PoC** status chip — the chip
flips only when the bar is met, and the row returns to the arena TL;DR only on
an explicit owner call (T9).
**T7 WAVE 1 EXECUTED 2026-09-29 (owner GO; riir-train issues 592-597, all six
bars verified against the reflex bench tables): every wave-1 verdict is a
MEASURED negative — no suite's vs-best gap closed.** sst5 (the closest gap,
−1.66 pt): the 579 v2 lever PASSED its holdout gate (v2-vs-v1 mean +3.10 pt,
paired LB95 +0.0076, minted) and the single frozen arena read (Bench 024)
REFUSED it — A1 read 0.4217 == the v1 row (the holdout edge did not transfer),
T2 LB95 −0.0151, bar 0.4383 unmet → A0 serves, no registration. code_fixtures
(bar paw 0.6250): the flat-union training collapse was FOUND and FIXED (per-head
training + merge — the probe measured the flat union at constant while
module-only reaches 0.5714) and the merged v2 still read 0.5536 holdout vs the
0.72 pre-registered absolute floor → negative, no mint. massive (+6.5 pt measured
lever lift, the wave's largest — vs a +9.3 pt gap), ag_news (v2 never beat v1),
xnli (+4.0 pt vs a +37.7 pt gap): holdout-gate negatives (riir-train `d59a7e55`).
typed (agentjev 0.7715): the 581 full-pool retrain already measured the bag
class's ceiling on this pool (0.6475); this wave's largest measured lever lift
(+6.5 pt) does not close a −12.4 pt gap — the reachability verdict stands in
issue 594, re-opens only with a model-class upgrade. **What would reopen a
suite: an encoder-feature or distill lane that beats the named lane's CLASS,
not more bag-model sweeps** — the wave's data is in riir-train HISTORY.md.

## Why

Reflex is free. Instinct is the paid lane (trained specialists composed over
Reflex), so it must EARN its place — and a customer does not compare it to
Reflex alone; they compare it to **whatever is best on the board**. A tie with
the best lane sells nothing, and so does losing to openthai on a suite where
Instinct beats Reflex by +42 pt. The serving selector is unchanged (best
measured arm per suite, A0 a candidate); this issue is the ADVERTISING bar and
the training backlog.

## Measured — vs the floor (free Reflex; published data/bench.json @ reflex-site 1881624, 2026-09-29)

Ahead on **8/15** suites — **6 with an arm**; emotion + xnli ahead via the armed reflex seat (the seat arms reflex's own cal-selected heads: ridge@8 on emotion reads 0.8850 = the armed reflex, so the specialist has nothing to add there — see T4's resolved caveat). tied on 7.

| suite | Instinct arm | Instinct | Reflex (published) | edge |
|---|---|---|---|---|
| massive_intent_en | H2(β=1,nmin=2,τ=8) | 0.8267 | 0.4067 | **+42.0 pt** |
| banking77 | H2 (nbsvm v2) | 0.8540 | 0.4020 | **+45.2 pt** |
| sst5 | A1 | 0.4217 | 0.2017 | **+22.0 pt** |
| emotion | (seat) | 0.8850 | 0.7700 | **+11.5 pt** |
| prompt_injections | A1 | 0.8534 | 0.7672 | **+8.6 pt** |
| typed_decisions | H2(β=0.5,oc) | 0.6300 | 0.5725 | +5.8 pt |
| ag_news | H2(β=0.25) | 0.8975 | 0.8625 | +3.5 pt |
| xnli_en | (seat) | 0.5233 | 0.5033 | +2.0 pt |
| code_fixtures + 6 harness_* | — | = Reflex | — | +0.0 pt (ties) |

## Measured — vs the BAR (best published lane per suite, best non-multilingual checkpoint, any host)

Strictly best on **3/15** armed suites, tied on 6, **trailing on 6**:

| suite | Instinct | best published lane | edge | the bar to beat |
|---|---|---|---|---|
| xnli_en | 0.5233 | 0.9000 | **−37.7 pt** | openthai-systemone 0.9000 |
| code_fixtures | 0.3750 | 0.6250 | **−25.0 pt** | paw (hosted) 0.6250 |
| typed_decisions | 0.6300 | 0.7715 | **−14.1 pt** | agentjev-0.6B 0.7715 |
| massive_intent_en | 0.8267 | 0.9200 | **−9.3 pt** | openthai-systemone 0.9200 |
| ag_news | 0.8975 | 0.9500 | **−5.2 pt** | laya (english) 0.9500 |
| sst5 | 0.4217 | 0.4383 | **−1.7 pt** | gliner 0.4383 |

Strictly best: banking77 (+14.8 vs gliner 0.7060), emotion (+11.5 vs Reflex 0.7700 — **posture-gap, not specialist value**: the seat serves A0 = reflex's own armed ridge@8 0.8850; the specialist A1 0.8550 LOSES to it by −3.0; a reflex republish at the armed posture collapses this edge to a tie), prompt_injections (+8.6 vs Reflex 0.7672). Tied at +0.0 edge:
code_fixtures + the six harness_* families (the specialist adds nothing
measurable on the synthetic families — a tie sells nothing). No arm:
thai_wisesight, thai_sib200 (coverage, T5).

**What we don't beat yet — the work list, widest first:** xnli_en (openthai),
code_fixtures (paw), typed_decisions (agentjev), massive_intent_en (openthai),
ag_news (laya english), sst5 (gliner); then the seven 0.0-edge ties.

## Root causes (read before training anything)

1. **Instinct is scored against an older Reflex than the one it competes with.**
   Bench 002 registered every arm at the Bench-052 protocol, whose A0 is
   Reflex's 052 posture (emotion A0 **0.7375**). Reflex has since armed its
   cal-selected heads (riir-reflex Bench 057: emotion ridge@8 → **0.8850**).
   The same drift can reach every suite whenever Reflex improves, so this is a
   standing hazard, not a one-off. (Today's published emotion modelless row
   reads 0.7700 — the registry-defaults posture; re-verify against the ARMED
   posture before calling emotion sold. **RESOLVED 2026-09-30, see T4: the
   seat's armed A0 == 0.8850, the specialist loses to it — emotion's board
   edge is posture-gap.**)
2. **banking77 publishes an arm the product does not serve.** RESOLVED
   2026-09-28 (Bench 012): the nbsvm v2 winner serves H2 0.8540 (+45.2 vs the
   published Reflex row) — and it is strictly best on the board (+14.8 vs
   gliner).
3. **The bar moved under the same drift hazard as cause 1, one lane wider.**
   Comparison lanes keep landing (agentjev typed 0.7715, openthai massive
   0.9200 / xnli 0.9000, gliner sst5 0.4383, paw code_fixtures 0.6250), each
   raising the per-suite bar independently of Reflex's own posture. T1's
   re-baseline discipline applies to the WHOLE lane set, never just A0: every
   re-read must recompute the vs-best edge, and a "sold" claim must name the
   lane it beat.

## Tasks

- [x] T1 — **re-baseline A0 on the CURRENT published Reflex posture** and
  re-run the arena on the same `datasets_t20k` bytes; the drift pin (arena ==
  reflex run() == bench.json) extended to all suites AND the published site
  rows. ✅ DONE (Bench 004; extended 2026-09-28 — 9/9 dataset suites pinned).
- [x] T2 — **a product gate: registration REFUSES an arm not strictly above
  the current Reflex row** on the frozen test read (paired LB95 > 0). Makes
  the site's ✓ a riir-instinct GOAT gate instead of a page rule.
  ✅ DONE (Bench 004: `stats::PairedDiff::lb95` in the arena registration).
- [x] T3 — **publish the SERVED posture, not the registered one** — MOOT AS
  STATED (the T2 gate made registered == served). Site sync DONE reflex-site
  `4ca2ef6`; the four refused Bench-002 cells removed, only the served set
  renders. Since 2026-09-29 the row lives at `/bench/#instinct` (see the
  amendment) beside the vs-best row, both rendered from data by reflex-site
  `assets/instinct.js`.
- [x] T4 — **close the two Reflex gaps with specialists that clear G3**:
  banking77 ✅ (Bench 012 — nbsvm v2 serves, strictly best on the board);
  emotion ✅ on the published board (0.8850 vs 0.7700) — **the ARMED-posture
  caveat RESOLVED 2026-09-30 from existing measured data, verdict NEGATIVE for
  the specialist**: the Bench-004/016 seat already arms reflex's own cal-selected
  posture on emotion (posture line `ridge 8.00 (bag)`) and reads **A0 0.8850 —
  exactly reflex Bench 057's armed ridge@8 number, cross-pool** — while the
  specialist A1 reads 0.8550 (−3.0 pt, loses). The 0.8850 in this issue's floor
  table was the A0/armed number transcribed under an A1 label (fixed same day;
  AGENTS.md's Bench-004 bullet and arsenal.toml carried the correct 0.8550 all
  along). Consequence: emotion is sold vs the PUBLISHED board by **posture gap**
  (armed seat 0.8850 vs defaults-posture published 0.7700), not by specialist
  value — root cause 1 is live here by construction, and a reflex republish at
  the armed posture collapses the edge to a tie. The serving posture is already
  honest (A0 serves; arsenal.toml's own comment says "A1 0.8550 / H1 0.8775
  lose — specialist backlog"). An emotion arm that beats 0.8850 needs the
  encoder class (issue 014's owner-gated arm class, shared with xnli + sst5) —
  a bag sweep is the closed class per the wave-1 law.
- [-] T5 — **coverage: arms for the unsold suites.** Remaining no-arm:
  thai_wisesight, thai_sib200 (a Thai specialist — OpenThai-SystemOne holds
  the bar there at 0.475 / 0.8382). Previously-open xnli_en, code_fixtures
  and the six harness families HAVE arms now (xnli +2.0 vs Reflex but −37.7
  vs openthai — seated ≠ sold under the amended bar); the harness families
  are synthetic-authored — a specialist there needs a data-design decision
  first, never a silent run. DEFER on Thai until the six vs-best gaps close.
- [ ] T6 — **a general-domain arm**: one specialist whose fused read beats
  the best published lane on a majority of suites without per-suite tuning,
  so "Instinct is the best lane" holds as a general claim, not only per
  domain.
  **SCOPED 2026-09-30 (post-wave-3, from the measured class evidence — not
  started):** the only lane that has ever read at-or-above a named bar's
  class is the ENCODER class (xnli 0.86, sst5 0.5267-confirmed), and both
  of those seats are the OPEN owner gate (issue 014 / 599 T5 — the
  heavyweight encoder serve posture). A bag-class general arm cannot clear
  a majority of bars (the wave-1 law: per-suite tuning is what makes arms
  win; the bag class's ceilings are measured), and the encoder reference
  alone ties-or-trails its bars outside sst5 (ag_news ceiling-tie, xnli
  −4.0, sst5 −6.6 — the head is load-bearing). So T6 is PRACTICALLY
  BLOCKED on the owner gate's outcome: no encoder seating → no above-bar
  class → no majority claim to build. Re-scopes after the gate resolves;
  until then it is the issue's standing research lane, not a queued run.
- [ ] T7 — **close the six vs-best gaps** (the amended bar, widest first):
  xnli_en (openthai 0.9000), code_fixtures (paw 0.6250 — SAME-SHAPE bar; the
  037 gliner 0.6667 cell was the pre-freeze 24-question shape, not
  comparable — verified 2026-09-29), typed_decisions
  (agentjev 0.7715), massive_intent_en (openthai 0.9200), ag_news (laya en
  0.9500), sst5 (gliner 0.4383). Each needs a specialist that beats the
  NAMED lane, not just Reflex; runs in riir-train (`vessel-mint`), on the
  GPU; not deferred. Filed per-suite as riir-train issues using the 579
  protocol (the v2 bar + teacher-quality lever warnings).
  **WAVE 1 MEASURED 2026-09-29 — all six negative (see the status block;
  riir-train issues 592-597, removed on close-out per noise-reduction,
  records in riir-train HISTORY.md). The re-open bar is a CLASS upgrade,
  not another bag sweep.**
  **WAVE 2 — xnli_en RE-OPENED and measured POSITIVE (riir-train issue 599,
  `69962050`, 2026-09-30): the encoder-feature lane (the sanctioned class
  upgrade) reads 0.9630 on the 2000-row train-side holdout vs the 0.9000
  bar — the first lane ABOVE the named lane's class (bag 0.395-0.43,
  lexical pair-features ~0.52, laya reference logits 0.9654, openthai
  teacher itself 0.876 on the pool). The winner artifact is sealed
  (NLEH v1, `a1e2380b…`). REMAINING before xnli flips to sold: the single
  frozen test read in the arena + an ENCODER-BACKED arm reader in
  riir-instinct (today's winner_bridge consumes bag artifacts; this class
  needs the laya-english encoder at serve — a heavyweight, GPU-backed
  posture). OWNER GATE: build the encoder arm class, or keep xnli
  unsold.**
  **WAVE 2 ADDENDUM — the earned frozen test read (riir-train 599 T5a,
  train-side eval, 2026-09-30): 0.8600 (258/300, the exact Bench-084
  split) — UNDER the 0.9000 openthai class bar (−4.0 pt) but +33.7 pt over
  Reflex A0. The T3 holdout did NOT transfer: the laya class's pool→test
  drop is −10.5 pt (reference logits 0.9654 pool → 0.8600 test; openthai
  moved +2.4 the other way) — pool-side holdouts are not test forecasts
  for this class. And the UNTRAINED reference logits read the same 0.8600
  (identical hit count): the trained head adds no test picks over the
  frozen representation — if the owner green-lights, the serving class
  could carry the reference logits alone (pending the pick-agreement
  check). The wave-2 verdict stands amended: the class upgrade is REAL
  (+33.7 over A0) but does not clear the named lane's bar on the frozen
  read; the owner gate now prices a 0.86-serving arm against the 0.90
  bar.**
  **WAVE 3 — the five remaining gaps SCREENED in one session (riir-train
  issue 600, `d5e3a30b`, 2026-09-30): ag_news NEGATIVE (the reference
  0.9500 IS the laya bar; the head adds nothing at ceiling), massive
  NEGATIVE (the head LOSES 4.9 pt to its own reference on test),
  code_fixtures NEGATIVE-by-law (reference 26.7 pt under the bar, over the
  measured max head-lift of +14.7), typed_decisions not screenable
  typed_decisions not screenable
  position-free (per-case option sets), and **sst5 the ONE live lane —
  head 0.5183 on the test screen vs the 0.4383 gliner bar (+8.0 pt) and
  its own reference (0.3717, +14.7 pt) — PROVISIONAL-POSITIVE pending the
  T6 frozen-read re-run under the class-relative-only bar (the 0.75
  absolute earn floor misprices a 5-way sentiment suite).** The
  class-relative law (the head never beats the bar unless its reference
  does — except far-from-ceiling references, where lift is real) is the
  wave-3 design instrument; the seating question for an sst5 encoder arm
  rides T6's verdict.
  **WAVE 3 / T6 VERDICT — CONFIRMED, the sst5 lane is LIVE (riir-train
  600, `6f24cd92`, 2026-09-30): the properly-gated fresh head read
  0.5267 on the frozen test read — +8.8 pt over the 0.4383 gliner bar,
  +15.5 over its own reference, ABOVE the T2 probe's ungated 0.5183; the
  reference arm reproduced 0.3717 exactly (cache witness). Protocol:
  class-relative-only earn gate (`--min-holdout 0`), 5-head pool-side
  sweep, best-holdout selection (0.5050), ONE pre-registered frozen read.
  The suite's incumbent serve (bag arm A1) reads 0.4217 — a 0.5267-class
  encoder arm would be sst5's first strictly-superior specialist (+10.5
  pt). Seating = **owner gate** (the serve-posture decision: the
  encoder-backed arm class is the heavyweight GPU-backed serve posture,
  ~2 GB VRAM class): filed as **Issue 014** — same gate shape as the xnli
  T5 owner call, now priced with a bigger margin over BOTH the bar and
  the incumbent.
- [ ] T8 — **break the seven 0.0-edge ties or drop them from the sold set**:
  code_fixtures + the six harness families are ties with Reflex at +0.0 —
  a tie sells nothing (the original law). The families are synthetic:
  data-design decision first (T5's rule), then a specialist that wins, then
  the suite counts as sold; otherwise the site section keeps saying "tied".
  **T8 ADJUDICATION 2026-09-30 (the data-design decision, made from the
  measured board — the current published data/bench.json @ reflex-site):
  the six harness families DROP from the covered set; code_fixtures takes
  the tie-break lane.**
  **The families DROP — the reason is measured, not convenience.** Each
  family is n = 12–16 questions (visibility 16, permissions 12, tool_fit 12,
  routing 16, sensitivity 15, cache_reuse 12), and the eval instances are
  template siblings of ANY trainable corpus (the family generators
  randomize entities inside one template). A specialist trained on
  same-rule different-draw rows shares the template with the eval, so a
  win is indistinguishable from template memorization — the suite cannot
  CERTIFY specialist competence at this n, and a reflex-side engine lever
  (Bench 072's cache_reuse 0.9167) can move the same row without us
  publishing anything. Two of the six (tool_fit, cache_reuse at 0.9167 =
  11/12) already sit at reflex's own near-ceiling, where "strictly ahead"
  means 12/12 on twelve questions — noise. The honest T8 branch is the
  drop: the instinct card's covered set shrinks to the real suites; the
  lane re-opens only with a larger template-disjoint eval design (a
  reflex-side harness change, not an instinct training task). The
  site-side re-render (the six family cells leave the instinct card) rides
  the next reflex-site republish session — recorded here as the owner of
  that change.
  **code_fixtures — the ONE tie a real specialist can break (pre-registered
  protocol, written BEFORE any run):** the arena reads
  `code_fixtures_winner_v1.bin` (absent today → a0_stands → the tie); the
  wave-1 per-head merge substrate (`merge_per_head`/`per_head_nbsvm`,
  riir-train `instinct_v2_gate`) holds candidates whose train-side holdout
  reads 0.55–0.57 vs reflex A0 0.375 — on REAL disjoint code spans (the
  exporter's rows are real reflex-source fns; the eval's 32 questions are
  different fns; the module question's signal — the body names its module
  path — is a genuine codebase regularity, the banking77 presence-bag
  class, not template memorization). **Protocol:** (1) trainer side: the
  wave-1 shape (per-head arm A merged = v1, per-head NBSVM merged = v2),
  holdout 56 (the wave-1 protocol), train-side gate = candidate holdout
  (argmax of v1/v2) ≥ **0.45** (reflex A0 + 7.5 pt, so a weak candidate
  never burns the frozen read); (2) mint the argmax under its natural
  name — v1 → `code_fixtures_winner_v1.bin` (Count convention, unbridged
  default), v2 → `code_fixtures_nbsvm_v2.bin` + the winner_bridge entry in
  the same change (the 579 law); (3) ONE frozen arena read
  (`--suite code_fixtures`, `--skip-pin-a0`, the documented pool-divergence
  posture), adjudicated by the standing issue-008 T2 product gate (strictly
  above Reflex, paired LB95 > 0) — no new gate machinery; (4) PASS →
  manifest row + lane doc + the suite reads ahead-of-Reflex (still trailing
  the paw 0.6250 bar — the amended law keeps it unsold; the tie display
  dies); MISS → the negative is recorded, the suite joins the drop class
  (at n=32 the T2 gate needs ≈ +6 net questions — the read is honestly
  marginal, the Bench-024 sst5 precedent: a holdout PASS noise-refusing at
  the frozen read is a recorded outcome, not a protocol failure).
- [ ] T9 — **the site flip**: when the measured board shows Instinct strictly
  ahead of every published lane on every suite it covers, the PoC chip on
  `/bench/#instinct` reads GOAT on the next republish (rendered from data,
  no edit), and an owner call moves the row back into the arena TL;DR. Until
  then the home page and arena TL;DR stay Reflex-only — the owner's 09-29
  "let it shine later" call.
