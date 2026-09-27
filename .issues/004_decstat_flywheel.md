# Issue 004 — the decstat flywheel (riir-clippy's mining loop, carried for decisions)

**Status:** T1–T3 DONE 2026-09-27 (wire `9da822b` → capture `3a1eb09` →
store/route `c285435`; live-pushed to devnet epoch 2960 the same day).
T4 filed as riir-train Issue 577. Filed 2026-09-26 (Plan 001 P6);
was blocked on Issues 002/003 (a served lane to collect from).

## Why

katgpt-rs Proposal 014:45: consent-gated decision outcomes → `decstat` rows
`{domain, primitive, question-shape, outcome}` → riir-kat wire → riir-dapps
epoch settle + KAT rewards → corpus grows → at the measured threshold the
specialist retrains → sealed vessel → selectable profile. riir-clippy runs
exactly this loop for code fixes (mine → contribute → settle → corpus →
healer improves). Nothing carries it for decisions yet.

## Plan

- [x] **T1** — outcome capture in the served lane: consent-gated, **Unset
      never pushes** (the `--stats` semantics), no payload text beyond the
      row schema. (`src/decstat.rs` — the `decstat` cargo feature;
      `RIIR_INSTINCT_STATS=on` exact literal; `INSTINCT_ACCOUNT_KEY`
      64-hex seed; devnet-first default URL; flush tail 256 decisions /
      60 s / 5 s budget; failed push DROPS the window — no offline
      stacking; the `/decide` path is byte-identical with the feature
      off. Gates: `tests/decstat_gates.rs` 7/7.)
- [x] **T2** — decstat row schema + riir-kat wire client (riir-kat owns the
      client plane; no transport code here). (`kat_protocol_decstat` —
      `riir-kat-decstat-v1` domain, golden bytes, `DECSTAT_LANE="dec"` in
      the protocol tier — the server posture compiles protocol-only;
      `kat_decstat_client` — body/ack under `client`, POST under
      `kat_transport`.)
- [x] **T3** — riir-dapps settlement, devnet first (Proposal 014 ladder).
      Landed as the work-stats family's second lane: `ingest_stat_run`
      (the lane-generic core), `POST /mining/decstats`, `GET
      /mining/decstats` + `/mining/decstatroot`, the alarm leg fires the
      dec rebuild fail-soft. Contract:
      `decstat_contract_ingest_view_root_and_lane_separation`. **KAT
      rewards deliberately absent** — a self-reported outcome row has no
      server-side oracle, so a per-row reward would be a free mint;
      re-opens with a priced verification design (the arena's gold checks
      are the recorded candidate) and the owner's never-lever pricing
      posture.
- [-] **T4** — riir-train intake: settled rows join the specialist corpus;
      retrain fires only at the measured threshold and is GOAT-gated (the
      Bench-062 ID + withheld-pair OOD discipline), then mints a new
      vessel (Issue 001) and redeploys (Issue 002). DEFERRED to
      riir-train Issue 577 — rows accumulate first. PROGRESS 2026-09-27:
      the intake reader + the retrain demand floor (10⁴/suite, the
      Issue-125 band low end, pinned pre-read) LANDED in riir-train at
      `b4898e01`+`443d2900` — the dec lane's first root (`191db259…`)
      verified and its epoch recorded in the train-side ledger the same
      day; the retrain itself (577 T3/T4) stays deferred on rows.

## References

katgpt-rs Proposal 014 §Phase 3; riir-clippy Issue 125 (reopen threshold);
riir-kat / riir-dapps BOUNDARY.md. Design + defer notes:
`.plans/002_decstat_flywheel.md`.
