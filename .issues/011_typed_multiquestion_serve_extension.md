# Issue 011 — the multi-question serve extension (typed_decisions' serving row)

**Status:** CLOSED — landed 2026-09-28, same day as filing. Unblock path
for Bench 015's serving half: the retrained typed_decisions specialist is
seated and T2-certified against BOTH A0 legs (A1 0.6300 vs A0′ 0.5725
LB95 +0.0373; vs the published A0 0.4655 LB95 +0.1432; G1 PASS), but the
serve path cannot boot the suite, so the manifest carries no typed row
yet.

## The blocker (measured)

`server.rs::from_parts` refuses any suite whose cases carry
`questions.len() != 1` — "the synthesized request carries the case[0]
template, so a multi-question set has no single serving kind". Every
typed_decisions case carries a 5-question set
(credential_compromise noul · disposition choice · severity score ·
true_positive noul · urgency score). And the blocker is not skippable at
the deployment layer: the serve binary's `--suites` defaults to ALL
manifest rows (serve.rs 241–249), so a manifest row today breaks the
DEFAULT container boot — an inert-but-boot-breaking row is the 014
lesson's mirror (a loud refusal is right; deferring it into a red boot
is not).

## The shape (the upstream law decides it)

`katgpt_core::decision_wire::DecisionRequest`: **"one state, ALL
questions answered in one call (laya's 'all questions in the call' law —
no per-question round trips)."** The typed serve contract is therefore a
MULTI-question decide — one state in, one pick per question out — not a
per-question squeeze and not a case[0]-only partial serve.

## Tasks

- [x] T1 — `SuiteServer::decide_multi(&mut self, state, questions:
      &[ServedQuestion]) -> Result<Vec<ServedDecision>, String>`: ONE
      multi-question `SuiteCase` through the same `eval_seat` path (A0's
      per-question bytes are the arena's), state bagged ONCE, each
      question's presented keys through the key-map bridge. Noul resolve
      is JOIN-FORM-aware: Named joins keep the single-question contract's
      positional law byte-for-byte; a Context join resolves NOUL_PAIR
      ("no"/"yes") BY NAME (the arena's fill_positions law — positional
      over a context perm would hit sentinels), and an EMPTY noul
      presentation takes the fixed [false, true] rendering (the
      decision_wire law). Single-question `decide` is the N=1 wrapper
      (template strings cloned out of self); MultiQuestion suites refuse
      it loud. AnySuiteServer::decide_multi forwards.
- [x] T2 — the HTTP edge: `POST /decide` accepts `{suite, state,
      questions: [{id, kind, instructions, options?}...]}`; the legacy
      body stays byte-compatible; `options` + `questions` together is a
      400 bad_field; an unknown kind is a 400 bad_field; the multi
      response is `{suite, lane, n_decisions, decisions: [...]}` with
      per-decision receipts (question_id echoed); decstat records per
      decision.
- [x] T3 — `from_parts`' shape guard split by the bridge's NEW
      `ServeContract` axis (`winner_bridge` — one home beside the file +
      bag coupling): SingleQuestion keeps the old guard; MultiQuestion
      asserts the suite actually carries multi-question cases (the
      declaration must not outlive the shape). typed_decisions is the
      one MultiQuestion suite; the bridge test pins all three forms.
- [x] T4 — the serving row: arsenal.toml's EIGHTH row (A1, digest
      `blake3:7f7a39e1935f8665beaf61106a84a65a0066a73fe3eee4ad7638fda0f776f2f0`
      = Bench 614's mint, byte-verified); `PINNED_MANIFEST_DIGEST`
      re-pinned (`blake3:1dd16be7…`); the serve_gates posture table 7→8
      with the typed assertion flipped from ABSENT to the A1 posture;
      `deploy.yaml` ships the winner + the FULL-pool dataset dir (the
      measured posture's seat corpus; the cal front is byte-identical to
      the canonical one).
- [x] T5 — parity gate: `typed_decisions_serves_the_frozen_a1_picks`
      replays 12 test cases' FULL question sets (60 questions) through
      `decide_multi` and asserts identity with the 015 frozen A1 picks —
      green. Plus the bridge-contract test, the 8-row manifest test, and
      a LIVE end-to-end smoke (serve binary on the full-pool dir: 5
      decisions in one call, noul positional, the single-question form
      refuses 422 bridge_undefined).

## Validation (the landing run)

serve_gates 14/14 (incl. the new parity gate, 982s — the typed seat
boots twice) · lib 49/49 default · 60/60 lib all-features · clippy
`-D`-clean default + all-features · full `cargo test --all-features`
battery green (94 passed, zero FAILED; the two seat-booting gates
dominate the wall).

## Honest scope notes

- The 015 record's serving law verdict stands: the served arm is A1 —
  H1's test edge (0.6335 vs 0.6300) is paired noise (LB95 −0.0048) and
  the instrument registered A1 pre-blinded.
- The per-key collapse on rare keys (close_benign/monitor/Low/Negligible
  ≈ never predicted; Bench 015's breakdown) is the class-rebalancing
  backlog — an improvement lane, not a blocker for serving.
- The latency rows in 015 were taken at load 8.06 (quotable false);
  T5's parity gate re-measures on a quiet box for any latency claim.
