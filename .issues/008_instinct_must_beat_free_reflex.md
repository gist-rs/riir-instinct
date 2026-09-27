# Issue 008 — Instinct must strictly beat free Reflex on every suite it sells

**Status:** OPEN — filed 2026-09-27 (owner direction). T1+T2 EXECUTED 2026-09-27
(Bench 004, `.benchmarks/004_rebaseline_current_reflex/`): the sold set is now
exactly ONE certified arm — massive H2 +4.7 pt, paired LB95 +0.0124 — with
ag_news/sst5/banking77 refused by the gate and emotion's specialist outrun by
reflex's ridge@8 base. T3 (site publishes the SERVED posture) is the next
owner-visible step; T4/T5 grow the sold set.

## Why

Reflex is free. Instinct is the paid lane (trained specialists composed over
Reflex), so it must EARN its place: ahead of Reflex, in general and/or in a
named domain. A tie sells nothing. reflex-site now renders the arena row
from Instinct's side, **"Instinct vs Reflex, accuracy"**, with ✓ only when
Instinct is STRICTLY ahead on every suite it has an arm for (reflex-site `bc7b1a0`, the row law in
`assets/arena_tldr.js`). Today it reads ✗.

## Measured (published data/bench.json, 2026-09-27)

| suite | Instinct arm | Instinct | Reflex (published) | edge |
|---|---|---|---|---|
| massive_intent_en | H2(β=1,nmin=2,τ=8) | 0.8267 | 0.7800 | **+4.7 pt** |
| sst5 | A1 | 0.4217 | 0.3967 | **+2.5 pt** |
| ag_news | H2(β=0.25,nmin=2,τ=2) | 0.8975 | 0.8825 | **+1.5 pt** |
| banking77 | H1 (NOT served — G3 FAIL, A0 serves) | 0.8060 | 0.8260 | −2.0 pt |
| emotion | A1 | 0.8550 | 0.8850 | −3.0 pt |
| 9 suites | — no arm | — | — | not sold |

No arm: typed_decisions, prompt_injections, xnli_en (A0 stands), code_fixtures,
and the six harness_* suites.

## Root causes (read before training anything)

1. **Instinct is scored against an older Reflex than the one it competes with.**
   Bench 002 registered every arm at the Bench-052 protocol, whose A0 is
   Reflex's 052 posture (emotion A0 **0.7375**). Reflex has since armed its
   cal-selected heads (riir-reflex Bench 057: emotion ridge@8 → **0.8850**).
   So emotion's A1 beat the Reflex it was registered on by +11.8 pt and trails
   today's free Reflex by 3.0 pt. The same drift can reach every suite
   whenever Reflex improves, so this is a standing hazard, not a one-off.
2. **banking77 publishes an arm the product does not serve.** The serving
   posture (`server::serving_posture`) is A0 there, i.e. exactly Reflex,
   because H1 FAILED G3. The site row shows the unserved H1 (−2.0 pt).
   AGENTS.md records that as deliberate ("the H1 row stays a published site
   measurement"); under the paid-lane framing it undersells what ships AND
   still cannot sell (a tie).

## Tasks

- [x] T1 — **re-baseline A0 on the CURRENT published Reflex posture** (the
  armed heads: emotion ridge@8, typed oc@2, sst5/xnli nb@16 … as published)
  and re-run the Bench-002 arena on the same `datasets_t20k` bytes. Every
  hybrid arm fuses over that Reflex, so A0 == the published modelless row
  becomes the comparability proof again (the 6/6 pin, re-pinned). Expected:
  emotion's fused arm re-scores against 0.8850; ties are the floor, not a loss.
  ✅ DONE (Bench 004): seat knobs = head+nb+ridge select (the ladder arms
  only where reflex's bar clears — emotion @8, all other arena suites 0,
  byte-identical to off); the drift pin extended to all six suites AND to
  the published site rows (arena == reflex run() == bench.json, 6/6 site ✓).
  Emotion measured: A1 0.8550 vs the new A0 0.8850 — the instrument picked
  A0 outright; the +11.8 pt story closed at −3.0 pt, exactly as this issue
  predicted.
- [x] T2 — **a product gate: registration REFUSES an arm not strictly above
  the current Reflex row** on the frozen test read (paired, with the existing
  `stats.rs` non-inferiority machinery turned into superiority: lower bound
  of the paired delta > 0). This makes the site's ✓ a riir-instinct GOAT
  gate instead of a page rule, and T1's drift can never silently reopen.
  ✅ DONE (Bench 004): `stats::PairedDiff::lb95` + the gate in the arena's
  registration (`src/bin/arena.rs`) — a refused pick prints its paired mean
  and LB95, A0 serves, and the refusal rides the frozen record
  (`predictions.json`'s `instrument_pick`/`superiority` fields) + the
  RESULTS.md gate table. Refusals measured: ag_news H2 (+0.0150/−0.0100),
  sst5 A1 (+0.0250/−0.0129), banking77 H1 (−0.0200/−0.0466).
- [ ] T3 — **publish the SERVED posture, not the registered one** — now
  MOOT AS STATED (the T2 gate made registered == served: a refused arm is
  not registered, so there is no unserved arm left to hide). The REMAINING
  half is the site sync: `scripts/build_hybrid_doc.py` + reflex-site must
  republish from Bench 004's record so the site row carries the served set
  (massive only, +4.7 pt certified) instead of the Bench-002 arms. Not done
  here — reflex-site is a separate checkout another session may hold.
- [ ] T4 — **close the two gaps with specialists that clear G3**: emotion
  (vs 0.8850) and banking77 (vs 0.8260; H1 lost up to ~4.7 pt at 95%).
  Training runs in riir-train (`vessel-mint`), on the GPU; not deferred.
- [ ] T5 — **coverage**: arms for the 9 unsold suites, starting with the
  ones where Reflex is weakest and a specialist has the most room
  (code_fixtures 0.25, harness_visibility 0.375, sst5-class tasks), and the
  typed_decisions / prompt_injections suites that products actually route.
- [ ] T6 — **a general-domain arm**: one specialist whose fused read beats
  Reflex on a majority of suites without per-suite tuning, so "Instinct beats
  Reflex" holds as a general claim and not only per domain.
