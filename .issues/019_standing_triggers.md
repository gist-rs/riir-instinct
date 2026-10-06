# Issue 019 — standing triggers: E2 manual push; mainnet T4; decstat pricing (record-only)

**Status:** OPEN — E2 executed 2026-10-06 under the delegated Claude verdict (owner
instruction "ask claude for owner gated and do it"; verdict AGREE at round 3 with
guardrails): stage + source-scan + auth all green, the real push BLOCKED by two wrangler
4.147 incompatibilities in the deployer (riir-deployer issue 013, commit `284fe6a`) —
nothing deployed, no CF resource created; full attempt record: riir-rethink issue 027
(commit `d45a815`). Rows 2–3 re-affirmed 2026-10-06. E2 fires again when the deployer fix
and the rethink auth-gate owner decision land. (Row provenance: moved out of Issue 013 at
its close, 2026-10-05.)

- [ ] **E2 — devnet container push (OWNER-GATED / cred-holder).** Pre-conditions re-verified
  green 2026-10-06 on this box: fresh stage (zigbuild x86_64 ELF gate, 310/310 hash rows),
  staged-context source scan CLEAN, auth = the owner's account confirmed by ID. The real
  push is blocked ONLY by the deployer's wrangler-4.147 defects (riir-deployer issue 013:
  missing `containers build` PATH positional; generated `wrangler.toml` schema invalid), and
  completion additionally needs the public-endpoint auth-gate owner decision (riir-rethink
  issue 027). Nothing deployed to date. Each future attempt re-runs the verdict guardrails:
  whoami matched by ID, source scan of exactly what is pushed, `max_instances` 1, one-shot
  smoke, evidence-backed teardown, full record in the private repo only.
- [ ] **Mainnet ceremony T4 (DEFERRED — owner hold).** RE-AFFIRMED 2026-10-06 under the
  delegated verdict: the 2026-09-28 mainnet hold stands; nothing armed or staged; the
  serving lane's move into the private moat (riir-ai Proposal 052) reinforces it.
- *Note-only (no checkbox — record-only, `.plans/002` T3b):* **decstat KAT reward pricing**
  — RE-AFFIRMED 2026-10-06 under the delegated verdict: stays record-only. The trigger is a
  PRICED VERIFICATION DESIGN first (a self-reported outcome row has no server-side oracle —
  a per-row reward would be a free mint); the never-lever doctrine (riir-dao) bars any
  unbacked-issuance lever in the meantime. No priced design exists yet; commissioning one is
  itself an unpulled owner decision.
