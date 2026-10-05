# Issue 019 — standing triggers: E2 manual push; mainnet T4; decstat pricing (record-only)

**Status:** OPEN — the deferred owner-gate rows moved out of Issue 013 at its
close (2026-10-05, delegated Claude verdict on the pool re-baseline = GO
option (a)); each row fires on its trigger, none is agent-executable now.

- [ ] **E2 — devnet container push (OWNER-GATED / cred-holder).** The devnet
  `wrangler containers build` + deploy is an explicitly MANUAL credentialed
  push under the manual-deploy posture — Cloudflare creds are an
  owner/cred-holder act; no agent execution. Pre-conditions already hold:
  local e2e green (docker run, HEALTHCHECK healthy, all lanes ready ≤ 14.7 s,
  live decisions 0.2–5 ms with receipts — the P5 record) and the staged
  artifact verified (`RIIR_DEPLOY_STAGE_ONLY=1` plan → zigbuild → 948-row
  BLAKE3 manifest → stage; "the real `wrangler containers build` + deploy is
  the owner-adjacent step", AGENTS.md §Build commands / Deploy).
- [ ] **Mainnet ceremony T4 (DEFERRED — owner hold).** The mainnet vessel /
  serving ceremony stays deferred until the owner lifts the mainnet hold
  (owner direction 2026-09-28: ALL mainnet actions ON HOLD; "mainnet is the
  owner ceremony (T4)", the P5 record). Nothing here is armed or staged
  against mainnet.
- *Note-only (no checkbox — record-only, `.plans/002` T3b):* **decstat KAT
  reward pricing** stays record-only. The trigger is a PRICED VERIFICATION
  DESIGN first (a self-reported outcome row has no server-side oracle — a
  per-row reward would be a free mint); the never-lever doctrine
  (riir-dao) bars any unbacked-issuance lever in the meantime.
