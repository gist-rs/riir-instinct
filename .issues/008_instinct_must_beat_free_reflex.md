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

## Why

Reflex is free. Instinct is the paid lane (trained specialists composed over
Reflex), so it must EARN its place — and a customer does not compare it to
Reflex alone; they compare it to **whatever is best on the board**. A tie with
the best lane sells nothing, and so does losing to openthai on a suite where
Instinct beats Reflex by +42 pt. The serving selector is unchanged (best
measured arm per suite, A0 a candidate); this issue is the ADVERTISING bar and
the training backlog.

## Measured — vs the floor (free Reflex; published data/bench.json @ reflex-site 1881624, 2026-09-29)

Ahead on **8/15** suites with an arm; tied on 7.

| suite | Instinct arm | Instinct | Reflex (published) | edge |
|---|---|---|---|---|
| massive_intent_en | H2(β=1,nmin=2,τ=8) | 0.8267 | 0.4067 | **+42.0 pt** |
| banking77 | H2 (nbsvm v2) | 0.8540 | 0.4020 | **+45.2 pt** |
| sst5 | A1 | 0.4217 | 0.2017 | **+22.0 pt** |
| emotion | A1 | 0.8850 | 0.7700 | **+11.5 pt** |
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

Strictly best: banking77 (+14.8 vs gliner 0.7060), emotion (+11.5 vs Reflex
0.7700), prompt_injections (+8.6 vs Reflex 0.7672). Tied at +0.0 edge:
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
   posture before calling emotion sold.)
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
  emotion ✅ on the published board (0.8850 vs 0.7700) with the standing
  caveat: re-verify vs the ARMED Reflex posture (Bench 057's ridge@8 0.8850)
  before claiming the suite sold — cause 1 is live there.
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
- [ ] T7 — **close the six vs-best gaps** (the amended bar, widest first):
  xnli_en (openthai 0.9000), code_fixtures (paw 0.6250), typed_decisions
  (agentjev 0.7715), massive_intent_en (openthai 0.9200), ag_news (laya en
  0.9500), sst5 (gliner 0.4383). Each needs a specialist that beats the
  NAMED lane, not just Reflex; runs in riir-train (`vessel-mint`), on the
  GPU; not deferred. Filed per-suite as riir-train issues using the 579
  protocol (the v2 bar + teacher-quality lever warnings).
- [ ] T8 — **break the seven 0.0-edge ties or drop them from the sold set**:
  code_fixtures + the six harness families are ties with Reflex at +0.0 —
  a tie sells nothing (the original law). The families are synthetic:
  data-design decision first (T5's rule), then a specialist that wins, then
  the suite counts as sold; otherwise the site section keeps saying "tied".
- [ ] T9 — **the site flip**: when the measured board shows Instinct strictly
  ahead of every published lane on every suite it covers, the PoC chip on
  `/bench/#instinct` reads GOAT on the next republish (rendered from data,
  no edit), and an owner call moves the row back into the arena TL;DR. Until
  then the home page and arena TL;DR stay Reflex-only — the owner's 09-29
  "let it shine later" call.
