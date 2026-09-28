# Bench 013 — the Issue 578 noul bridge: prompt_injections seated + T2-certified

**Status:** MEASURED — 2026-09-28. Train-side lane: riir-train Issue 578 T2
/ Bench 611 (winner minted there, `prompt_injections_armA_v1.bin`, BLAKE3
`ee0b4eb4336299b2…`); this record is the consumer half — the arena's single
frozen test read through Plan 003's noul bridge, and the serving
registration. Records: `RESULTS.md` / `predictions.json` /
`registration.json` (arena-written).

## The bridge (what landed)

riir-instinct Plan 003 (`1e9f1b1` + `ed592ba` + `d644ea5`): the specialist
bridge's third presented-option arm. A `QKind::Noul` question's engine
rendering is the FIXED `[false, true]` (reflex `suites.rs`'s law; gold idx
0/1 speak it) — the two positions map onto the producer's unified
`no`/`yes` pair:

- **Arena** (`SuiteCtx::fill_positions`, per question now): by NAME through
  the key map. prompt_injections' seat labels are the modelless positional
  int spelling (`"0"`/`"1"`) — `seat_join`'s positional arm joins the pair
  by position (seat label `"p"` ≡ pair entry p, the train side's own
  dataset map), so the key map carries REAL seat indices for both noul
  positions and their NB evidence stays live (here: nb off by posture).
- **Serve** (`SuiteServer::decide`): positional ALWAYS — the presented
  names never reorder the rendering; the count must be exactly the seat
  universe; the pick index speaks `[false, true]` whatever names arrive.
  Pinned by `noul_suite_serves_positionally_through_the_bridge` (three
  presentation spellings → one pick; a 3-option presentation refuses).
- **Arity**: `AnySuiteServer::S2` added (the arena's `run_suite_n::<2>`
  already existed).
- **The specialist is the A1 arm** — the whole suite is noul, so every
  question presents the pair and the count-bag specialist scores both
  class rows (`no` = class 0, `yes` = class 1 — the sorted producer
  universe).

## The frozen read (n = 116 test questions)

| arm | acc | note |
|---|---|---|
| A0 | 0.7672 | == the published reflex row (A0 pin: arena == reflex `run()` == site ✓) |
| **A1** | **0.8534** | instrument pick; paired vs A0 mean **+0.0862** |
| H1 (top-k 8) | 0.8448 | escalation 0.681 — the fused gate abstains often here |

Gate faces on the pick: **T2 PASS** — paired LB95 **+0.0082 > 0**: the
578 issue's "likely honest refusal" risk INVERTED; the specialist's edge
cleared the small-suite bar with room, and prompt_injections is only the
second suite ever T2-certified (massive H2 the first) — at n=116, where
the plan priced the bar at ≈ mean + 2 pt. **G1 FAIL (disclosed)** — raw
ECE 0.0824 beats the conformal floor 0.3769, but Platt HURT (0.1017): the
2-class sigmoid scores saturate, the fit has almost no confidence
resolution to learn from. The ag_news disclosure class: the face is
recorded, the arm serves, the calibration work is the backlog. G2: H1
fusion-only 7 ns/q, H2 4.23 ns/option, A1 p99 3 µs. Latency scope law
unchanged (`contains_seat_solve`).

Non-noul byte-identity control (the Plan-003 refactor touched every
suite's eval loops): banking77 re-run through the final binary into a
scratch dir reproduced the Bench 012 frozen predictions byte-identically
(A0/A1/H1/H2(2,8,8) picks, abstains, correct flags, probs, confs — zero
diffs, timings aside).

## The serving decision

The manifest gains its SEVENTH row: `prompt_injections` serves **A1**
(digest `blake3:ee0b4eb4…37cddd3` = Bench 611's mint, byte-verified) —
best measured AND T2-certified. `PINNED_MANIFEST_DIGEST` re-pinned
(`blake3:a0a520e2…7dd0f9`), the serve-gates posture table grew with it
(7 rows; `typed_decisions` asserted ABSENT — see Bench 014), and
`prompt_injections_serves_the_frozen_a1_picks` pins the serve to the 013
record's picks (16-case parity, A1 never abstains).
`deploy.yaml` ships the winner + the `prompt_injections` dataset dir.
The lane doc for the reflex-site session:
`.benchmarks/014_typed_decisions_specialist/hybrid_lane_doc.json`
(merged: the 012 legacy rows + this record + 014).
