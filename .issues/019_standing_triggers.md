# Issue 019 — standing triggers: E2 manual push; mainnet T4; decstat pricing (record-only)

**Status:** OPEN — E2 advanced 2026-10-06 under the delegated Claude verdict (owner
instruction "ask claude for owner gated and do it"; verdict AGREE round 3 with guardrails)
PLUS the owner's follow-up call ("Bearer token gate…"): the deployer's wrangler-4.147
blockers FIXED (deployer `7328891`), the push LANDED (container app ready, image live, the
bearer gate LIVE-proven — 401 without the token), and the final decide-smoke is blocked
ONLY Cloudflare-side (no container instance ever schedules on this account; owner act:
confirm the account's Containers entitlement, then one re-smoke decides E2). Full record:
riir-rethink issue 027 (commits `d45a815` → `abd5cd5`). Rows 2–3 re-affirmed 2026-10-06.
(Row provenance: moved out of Issue 013 at its close, 2026-10-05.)

- [ ] **E2 — devnet container push (OWNER-GATED / cred-holder).** Everything
  agent-executable is DONE and recorded (2026-10-06): stage + source-scan green ×2, auth
  verified by account ID, deployer defects fixed upstream, `auth_bearer` armed, secret
  uploaded, `wrangler deploy` green (container app `ready`, image in the registry), the
  bearer gate proven (no-token → 401, wrong-token → 401, before any container work). The
  last step — one `POST /decide` with the token — waits on the CLOUDFLARE-side blocker:
  the container instance never schedules (`There is no container instance that can be
  provided to this Durable Object`, LIVE INSTANCES 0 across >1 h of retries). The owner
  confirms the account's Containers entitlement (Workers Paid plan), then the re-smoke
  decides this row. Each future attempt re-runs the verdict guardrails (whoami by ID,
  source scan of exactly what is pushed, `max_instances` 1, one-shot smoke,
  evidence-backed teardown if ever rolled back).
- [ ] **Mainnet ceremony T4 (DEFERRED — owner hold).** RE-AFFIRMED 2026-10-06 under the
  delegated verdict: the 2026-09-28 mainnet hold stands; nothing armed or staged; the
  serving lane's move into the private moat (riir-ai Proposal 052) reinforces it.
- *Note-only (no checkbox — record-only, `.plans/002` T3b):* **decstat KAT reward pricing**
  — RE-AFFIRMED 2026-10-06 under the delegated verdict: stays record-only. The trigger is a
  PRICED VERIFICATION DESIGN first (a self-reported outcome row has no server-side oracle —
  a per-row reward would be a free mint); the never-lever doctrine (riir-dao) bars any
  unbacked-issuance lever in the meantime. No priced design exists yet; commissioning one is
  itself an unpulled owner decision.
