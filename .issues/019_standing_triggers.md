# Issue 019 — standing triggers: E2 manual push; mainnet T4; decstat pricing (record-only)

**Status:** OPEN — E2 **COMPLETE** 2026-10-06 (second session): the decide-smoke blocker
was NOT Cloudflare entitlement — it was TWO agent-side defects, both fixed this session:
(1) the deployed image carried the OPEN teaching serve (deploy.yaml's moat-seed
`package: riir-instinct` never repointed — the boot refusal against the shipped winners),
and (2) the smoke probed in gaps, so the container idle-reset mid-boot every time (the
five eager lanes need ~7 CONTINUOUS minutes on the 1-vCPU standard-2 to reach ready).
With `package: riir-rethink` (rethink `abd5cd5`+ the deploy.yaml fix commit) and one
continuous poll: ALL FIVE LANES READY at ~t+425s, and the decide-smoke passed on every
suite (banking77 0.981 conf · massive · prompt_injections 0.977 · typed multi-question ·
code_fixtures 2-q multi) + the gate re-proven (anon 401 / wrong-token 401 / healthz 200).
Full evidence: riir-rethink issue 027 (closed with the smoke record). Rows 2–3 re-affirmed
2026-10-06. (Row provenance: moved out of Issue 013 at its close, 2026-10-05.)

- [x] **E2 — devnet container push (OWNER-GATED / cred-holder).** COMPLETE 2026-10-06:
  deploy green (image `sha256:b76767…`, worker version `e8aa909f`), bearer gate
  LIVE-proven (401/401), all five lanes ready (healthz, ~7 min continuous boot), decide
  200 on every suite with the CURRENT token (the repo `.env` — the `/tmp` copy is STALE,
  it holds the superseded first put), receipts riding every response. Owner-adjacent
  remainders (not this row): the stays-up/cost posture (cold-start reality: ~7 min eager
  boot on idle wake — keep-alive, lazy rows, or accept), old registry image cleanup.
- [-] **Mainnet ceremony T4 (DEFERRED — owner hold).** RE-AFFIRMED 2026-10-06 under the
  delegated verdict: the 2026-09-28 mainnet hold stands; nothing armed or staged; the
  serving lane's move into the private moat (riir-ai Proposal 052) reinforces it.
- *Note-only (no checkbox — record-only, `.plans/002` T3b):* **decstat KAT reward pricing**
  — RE-AFFIRMED 2026-10-06 under the delegated verdict: stays record-only. The trigger is a
  PRICED VERIFICATION DESIGN first (a self-reported outcome row has no server-side oracle —
  a per-row reward would be a free mint); the never-lever doctrine (riir-dao) bars any
  unbacked-issuance lever in the meantime. No priced design exists yet; commissioning one is
  itself an unpulled owner decision.
