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
  **2026-09-29 second dated note (the 4090 box):** the divergence is
  now CONFIRMED from the other side — the arena's A0 pin SITE leg reds
  on this box exactly as the (a)/(b) analysis predicts (fresh
  reflex-site pull `ccb98d2`: site sst5 0.2017 canonical-pool vs arena
  t20k 0.3967; the same shape on every dataset suite). The documented
  `--skip-pin-a0` posture is what makes arena runs possible here; the
  reflex-RUN leg (arena == reflex run()) held on every suite measured
  this session (sst5 0.3967 == published t20k row, cross-host anchor
  half-verified). This note is evidence FOR the divergence being
  structural (pool × engine), not a drift — it does not decide (a)/(b).

  **2026-09-30 third dated note (from the reflex slice-integrity session):** the E2 divergence is
  now ROOT-CAUSED on the reflex side — the board's "canonical-pool" runs (Bench 076+ onward) read
  the DEFAULT `.raw/datasets` (the Sep 22 TRAIN_CAP=4000 pull) while every posture-selecting run
  before them read this frozen t20k pool; the A/B at HEAD (same binary, same flags, only
  `--datasets-dir` changed) reproduces BOTH columns exactly (emotion 0.8850 t20k / 0.7700
  default; massive 0.7800/0.4067; banking77 0.8420/0.4020). No engine move. Full verdict + the
  landed slice-integrity gate (which pins raw slice digests into every future results.json, so
  this class of silent pool move can no longer publish through): reflex `.issues/058`, commits
  `f1371c1` + `7d725b2`. ALSO: the frozen t20k sst5 pool carries 3 exact-duplicate rows (1
  cross-split + 2 train-internal); reflex's gate now accepts exactly that pinned state under a
  loud acknowledgement (KNOWN_DIRTY triple pin) — measured effect on the published posture:
  sub-measurable (t20k sst5 acc 0.2017 == the deduped canonical pool's reading). Whether to
  dedupe t20k (a re-baseline: the manifest pins its aggregate digest `ebfb0317…`, and published
  rows were measured on those bytes) stays THIS pickup's owner call, unchanged by the
  acknowledgement.

- E2 — OWNER-GATED / cred-holder: the devnet container push is an explicitly MANUAL push under the manual-deploy posture (local e2e green, staged artifact verified — `AGENTS.md:103`). No agent execution.
- [-] MAINNET ceremony T4 (`AGENTS.md:400`) — DEFERRED: no mainnet until the owner lifts the hold.
- decstat KAT reward pricing (`.plans/002` T3b) — RECORD-ONLY: TRIGGER is a priced verification design first; the never-lever doctrine bars a free mint.
