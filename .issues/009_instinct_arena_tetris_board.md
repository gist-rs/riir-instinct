# Issue 009 — Instinct plays Tetris on the arena (a real board, not a text-suite card)

**Status:** OPEN — filed 2026-09-27 (owner direction). The placeholder card is pulled from the arena until this lands.

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
