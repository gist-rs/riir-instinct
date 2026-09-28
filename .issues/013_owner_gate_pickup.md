# Issue 013 — owner-gate pickup: E2 manual push; T4 deferred; pool re-baseline

**Status:** OPEN — pickup tasks from the workspace owner-gated summary (riir-ai 1016); execution gates per item; prep/recording tasks are agent-pickupable.

Master: `../riir-ai/.issues/1016_workspace_owner_gated_decisions_summary.md` (riir-ai commit e8cbd91ff).
Owner direction 2026-09-28: riir-mmorpg-examples / seal-online-remaster / seal-game-editor / sealm-toolkit are DEFERRED; ALL mainnet actions are ON HOLD.

- Hybrid-lane pool/engine RE-BASELINE (residual of Issue 010, closed +
  removed 2026-09-29, HISTORY.md) — OWNER decision, two options:
  (a) re-baseline the arena pool to the canonical `.raw/datasets` + the
  reflex HEAD engine (the arena's own record declares re-pointing the
  datasets dir a RE-BASELINE decision, never a cleanup), or (b) keep
  `datasets_t20k` frozen and RELABEL the A0-pin's site leg as a
  pool-mismatch advisory (relabel, never silence). Until decided,
  `--skip-pin-a0` stays the documented posture with this reason; the
  site's modelless rows (reflex HEAD, canonical pool) and the hybrid rows
  (t20k seat) compare across (pool × engine) by design.
  **2026-09-29 dated note:** the site's typed modelless row now REALLY
  carries that posture — republished at reflex Bench 078/079 (0.5725,
  both hosts; reflex-site `a2f1f9e`) — so the visible typed pair is
  modelless 0.5725 (canonical pool) vs hybrid t20k-seat rows; this row's
  (a)/(b) decision is unchanged and still open.

- E2 — OWNER-GATED / cred-holder: the devnet container push is an explicitly MANUAL push under the manual-deploy posture (local e2e green, staged artifact verified — `AGENTS.md:103`). No agent execution.
- [-] MAINNET ceremony T4 (`AGENTS.md:400`) — DEFERRED: no mainnet until the owner lifts the hold.
- decstat KAT reward pricing (`.plans/002` T3b) — RECORD-ONLY: TRIGGER is a priced verification design first; the never-lever doctrine bars a free mint.
