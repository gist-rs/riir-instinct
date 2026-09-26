# Issue 004 — the decstat flywheel (riir-clippy's mining loop, carried for decisions)

**Status:** OPEN — filed 2026-09-26 (Plan 001 P6). Blocked on Issues 002/003 (a served lane to collect from).

## Why

katgpt-rs Proposal 014:45: consent-gated decision outcomes → `decstat` rows
`{domain, primitive, question-shape, outcome}` → riir-kat wire → riir-dapps
epoch settle + KAT rewards → corpus grows → at the measured threshold the
specialist retrains → sealed vessel → selectable profile. riir-clippy runs
exactly this loop for code fixes (mine → contribute → settle → corpus →
healer improves). Nothing carries it for decisions yet.

## Plan

- [ ] **T1** — outcome capture in the served lane: consent-gated, **Unset
      never pushes** (the `--stats` semantics), no payload text beyond the
      row schema.
- [ ] **T2** — decstat row schema + riir-kat wire client (riir-kat owns the
      client plane; no transport code here).
- [ ] **T3** — riir-dapps settlement, devnet first (Proposal 014 ladder).
- [ ] **T4** — riir-train intake: settled rows join the specialist corpus;
      retrain fires only at the measured threshold and is GOAT-gated (the
      Bench-062 ID + withheld-pair OOD discipline), then mints a new vessel
      (Issue 001) and redeploys (Issue 002).

## References

katgpt-rs Proposal 014 §Phase 3; riir-clippy Issue 125 (reopen threshold);
riir-kat / riir-dapps BOUNDARY.md.
