# Issue 011 — the multi-question serve extension (typed_decisions' serving row)

**Status:** OPEN — filed 2026-09-28. Unblock path for Bench 015's
serving half: the retrained typed_decisions specialist is seated and
T2-certified against BOTH A0 legs (A1 0.6300 vs A0′ 0.5725 LB95 +0.0373;
vs the published A0 0.4655 LB95 +0.1432; G1 PASS), but the serve path
cannot boot the suite, so the manifest carries no typed row yet.

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

- [ ] T1 — `SuiteServer::decide_multi(&mut self, state, questions:
      &[(qid, QKind, instructions, options)]) -> Result<Vec<ServedDecision>,
      String>`: synthesize ONE multi-question `SuiteCase` (the arena's
      eval shape — `SuiteCase` already carries question sets), bag the
      state ONCE, resolve each question's presented keys through the same
      key-map bridge (context suites: by NAME only — identity-by-count is
      undefined when the label spaces are disjoint; refuse loud), arm
      dispatch per question (A0 from the shared eval, A1 argmax over that
      question's positions, H1/H2 per question). The single-question
      `decide` becomes the N=1 form over the same path (byte-identical
      existing behavior — the parity gates hold it).
- [ ] T2 — the HTTP edge: `POST /decide` accepts
      `{"suite", "state", "questions": [{id, kind, instructions,
      options}...]}` beside today's single-question body (old shape stays
      byte-compatible; a body with neither form refuses loud with the
      codes table's `bad_field`).
- [ ] T3 — relax `from_parts`' shape guard for suites whose serve
      contract is the multi-question form (typed_decisions): the guard
      stays for the one-question suites; a suite-level declaration (the
      winner_bridge is the natural home — a `serves: MultiQuestion` field)
      picks the contract, so no heuristic reads the case shapes.
- [ ] T4 — the serving row: `arsenal.toml` gains the typed row (A1,
      digest `blake3:7f7a39e1935f8665beaf61106a84a65a0066a73fe3eee4ad7638fda0f776f2f0`
      = Bench 614's mint — byte-verify at edit time; the artifact is
      data-side, re-minting moves the pin); `PINNED_MANIFEST_DIGEST`
      re-pin; the serve_gates posture table 7→8 with the typed assertion
      flipped from ABSENT to the A1 posture + the Bench-015 reason;
      `deploy.yaml` ships the winner + the FULL-pool dataset dir
      (`../riir-train/.raw/datasets_typed_full/typed_decisions` — the
      measured posture's seat corpus; the cal front is byte-identical to
      the canonical one).
- [ ] T5 — parity gate: the serve path replays committed test cases'
      question sets through `decide_multi` and asserts identity with the
      015 frozen per-question A1 picks (the
      `prompt_injections_serves_the_frozen_a1_picks` shape, per-question
      form); plus a wire test for the new request shape and the old-shape
      byte-compat arm.

## Honest scope notes

- The 015 record's serving law verdict stands: the served arm is A1 —
  H1's test edge (0.6335 vs 0.6300) is paired noise (LB95 −0.0048) and
  the instrument registered A1 pre-blinded.
- The per-key collapse on rare keys (close_benign/monitor/Low/Negligible
  ≈ never predicted; Bench 015's breakdown) is the class-rebalancing
  backlog — an improvement lane, not a blocker for serving.
- The latency rows in 015 were taken at load 8.06 (quotable false);
  T5's parity gate re-measures on a quiet box for any latency claim.
