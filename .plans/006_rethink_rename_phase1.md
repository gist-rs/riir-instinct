# Plan 006 — `riir-rethink` Phase 1: the repo rename + registration re-pin

**Status:** OPEN — filed 2026-09-30 as riir-ai Proposal 051 T0.4. **EXECUTION IS OWNER-GATED** (051's closer: *"open the Phase-1 plan when the owner waves it through"*); this plan is the ready-to-fire checklist, not permission to start.
Owner: katopz (051 consolidation directive 2026-09-30)
Parent: [riir-ai Proposal 051](../riir-ai/.proposals/051_rethink_all_tier_adaptive_serving_family.md) Phase 1 (T1.1–T1.3 there)
Precedents: the riir-llm registration (katgpt-rs Issue 793 — 19-of-21 sweeps red when done partially); the reflex `LANE_DISPLAY` cap-case rename (aliases keep history landing); the reflex bin-rename v0.2.2 (dist-side follow-ups).

## Goal

Rename the repo `riir-instinct` → `riir-rethink` everywhere the name is LIVE, with the workspace registration re-pinned in ONE commit family, the site lanes rebranded behind aliases (old `bench.json` rows keep landing byte-identical), and ZERO history rewrites — in a single owner-waved session, green before push, both boxes re-pointed the same day.

## Laws (bind execution)

1. **Alias law (051 G3):** the rename changes no behavior. Old site lanes (`Instinct (hybrid)`, `Instinct (encoder)`) keep landing through `LANE_DISPLAY` aliases — re-publishing the existing `bench.json` must place every historical row under its new display name with the old spelling still accepted. The alias proof is a gate, not a hope: the publish self-test carries an old-spelling fixture.
2. **One commit family, gate-green before push.** The registration set (katgpt-rs `repo_set.txt` + pin/floor files) lands together with the repo-internal rename in one session; `docs_gate` + the drift-sweep family + the boundary contract run green BEFORE the push. Partial pushes are the riir-llm 19-red precedent.
3. **Historical records stay.** Dated HISTORY entries, landed bench records, closed issues, and git history are NOT rewritten. The rename touches LIVE surfaces only: registration, manifests/package, AGENTS/BOUNDARY live rows, `deploy.yaml`, site code, the decstat lane's live docs. Historical citations naming `riir-instinct` stay valid via the retired-spelling leniency entry (D3 below) — never by editing the records.
4. **Dual-box, same session-day.** M3 and 4090 both re-point (directory + remote + any local scripts) in the same session-day window; a stale checkout on either box diverges path layouts for every sibling grep. (`ssh` to 4090 before starting; verify no agent is mid-batch on the old path.)
5. **The greps must not cross the streams** (051 naming law): the hero-strategy `rethinks`/`think_every`/`since_rethink` vocabulary in `riir-games-mmorpg/src/hero_strategy_moe.rs` is game-side cadence vocabulary — never a reference to this repo, never renamed, never cited as this product. The one-home disambiguation table is `../riir-game-sdk/.docs/10_multiplayer_topology/tick_tier_model.md` §(a) (landed with 051 T0.3).

## Grounded census (2026-09-30, `git grep -l "riir-instinct"` per repo — REFRESH AT EXECUTION, T0)

| Repo | files carrying `riir-instinct` | `Instinct (` lane refs | Class |
|---|---|---|---|
| riir-instinct (self) | 52 | 5 | mixed: LIVE (Cargo.toml, BOUNDARY, AGENTS, deploy.yaml, docs) + history (issues/benches) |
| katgpt-rs | 31 | 0 | **the registration**: `repo_set.txt` + ~20 `*_floors.txt` pins + `sweep_population.py` + `len_derived_drift_sweep.py` + docs |
| riir-train | 30 | 1 | mostly history (issues/plans/benches) + LIVE winners-path refs in AGENTS/docs |
| riir-reflex | 15 | 0 | LIVE bench docs + the seat/seam docs naming the sibling |
| reflex-site | 9 | 10 | **LIVE**: `instinct.js`, `LANE_DISPLAY`, lane cards, publish code |
| riir-ai | 12 | 2 | LIVE BOUNDARY/proposals + history |
| riir-dapps | 6 | 1 | LIVE decstat lane docs + history |
| riir-deployer | 5 | 0 | LIVE runbook/deploy refs |
| riir-kat | 5 | 0 | LIVE decstat wire docs |
| riir-clippy | 3 | 0 | LIVE proposal cross-refs |
| riir-reflexer | 2 | 0 | docs |
| riir-infer | 2 | 0 | docs |
| riir-game-sdk | 1 | 1 | **DONE (051 T0.3)** — `tick_tier_model.md` §(a) carries the new table |
| others (neuron-db, chain, dao, auth, seal-*, shader, llm) | 0 | 0 | — |

**Path-dep verification (051 caveat 2 says leaf — RE-VERIFY at T0):** `git grep -l 'path = "../riir-instinct"'` across every sibling's `Cargo.toml` must return NOTHING before the directory moves. If it returns a hit, that consumer re-points in the same commit family.

## Decisions (recommendations recorded; confirm at the wave)

- **D1 — the package renames WITH the repo** (`name = "riir-rethink"`). Private leaf, zero external cargo consumers (verified + re-verified at T0); keeping a `riir-instinct` package inside a `riir-rethink` repo re-creates the four-registry disease 051 was filed to kill. Bins keep their names (`arena`, `serve` — the reflex v0.2.2 bin-rename taught us the dist-side cost of renaming binaries; nothing external invokes these yet).
- **D2 — env prefixes stay `INSTINCT_*` / `RIIR_INSTINCT_*` in Phase 1.** They are operator/deploy surface (`deploy.yaml`, kill-switches, soak scripts), not the product name; renaming them buys coherence at the price of a coordinated deploy + operator re-learn mid-rename. A later deliberate pass (Phase 1.5, optional) may rename them with dual-read back-compat. Record in deploy.yaml comments so the divergence is visible, not discovered.
- **D3 — `riir-instinct` becomes a RETIRED SPELLING in katgpt-rs `issue_citation_gate.spelling_aliases` (leniency-only)**, exactly the `seal-*`→`mmorpg-*` precedent (Issue 846): a retired spelling CLEARS a historical citation, never accuses one. This is what keeps ~90 dated records valid without touching them.
- **D4 — order:** registration files + repo-internal LIVE surfaces first (green, committed, NOT pushed), then the `git remote set-url` + directory rename LAST, then push everything, then the site family. The directory move is the moment sibling greps break — minimize its un-pushed window.

## Tasks

### T0 — pre-flight (owner wave received)
- [ ] T0.1 census refresh: re-run the per-repo `git grep -l "riir-instinct"` table above; diff against it; the diff IS the execution file list
- [ ] T0.2 path-dep re-verification: `git grep -l 'path = "../riir-instinct"'` across all sibling `Cargo.toml` — expect ZERO; any hit joins the commit family
- [ ] T0.3 both boxes synced + idle: `ssh` 4090, verify no agent mid-batch on the old path; sync git both boxes (the global rule)
- [ ] T0.4 `python3 ../katgpt-rs/scripts/dual_allocation_gate.py` green (numbering discipline)

### T1.1 — the repo rename + registration (051 T1.1)
- [ ] T1.1a repo-internal LIVE surfaces: `Cargo.toml` package name (D1); `BOUNDARY.md`; `AGENTS.md` title + naming-law section flip to "landed" wording; `deploy.yaml` (+ the D2 divergence comment); README/.docs live rows; internal module docs naming the repo
- [ ] T1.1b katgpt-rs registration: `scripts/repo_set.txt` row; the ~20 `scripts/*_floors.txt` pin rows; `scripts/sweep_population.py` + `scripts/len_derived_drift_sweep.py` (code, not pins); `issue_citation_gate.spelling_aliases` gains `riir-instinct` (D3); AGENTS.md §Repo prose if it names the repo
- [ ] T1.1c the move: `git remote set-url` + directory rename (D4 — last); both boxes the same session-day
- [ ] T1.1d registration gates green: `scripts/docs_gate.sh` (with the box's markers) + the drift-sweep family + `../riir-ai/scripts/ci_boundary_contract.sh` — ALL green before the push

### T1.2 — the site (051 T1.2)
- [ ] T1.2a `LANE_DISPLAY`: `Rethink (hybrid)` / `Rethink (encoder)` with the old spellings as aliases (the cap-case precedent); publish self-test gains an old-spelling fixture (the alias law's gate)
- [ ] T1.2b `instinct.js` → the rethink verdict section (board semantics UNCHANGED: serving arms only; a served encoder cell counts once 016 D1 fires)
- [ ] T1.2c re-publish + live check: old `bench.json` rows land under the new names; smokes green

### T1.3 — sibling docs sweep (LIVE rows only — Law 3)
- [ ] T1.3a riir-train: AGENTS + live winners-path docs
- [ ] T1.3b riir-dapps: decstat lane live docs
- [ ] T1.3c riir-reflex: bench/seat docs naming the sibling
- [ ] T1.3d riir-kat, riir-deployer, riir-clippy, riir-reflexer, riir-infer, riir-ai: live cross-references
- [ ] T1.3e HISTORY one-liners: each swept repo's HISTORY gets the rename record line (dated, citing 051 + this plan)

### T1.4 — close out
- [ ] T1.4a the commit family pushed (all repos, `develop`); both boxes `develop == origin/develop`
- [ ] T1.4b post-landing census: `git grep -l "riir-instinct"` returns ONLY historical records (dated HISTORY/bench/issue/closed-plan rows) + the D3 alias + the D2 deploy comments; a live-surface hit is a red
- [ ] T1.4c 051 Phase 1 checkboxes ticked (T1.1–T1.3 there) with commit hashes; this plan → DONE

## Gates

- **G3 alias proof:** re-publishing the pre-rename `bench.json` lands every row (the T1.2a fixture).
- **Registration:** docs_gate + sweep family + boundary contract green pre-push (T1.1d).
- **Census residue:** post-landing hits are historical-only (T1.4b) — the class check, not a count.

## Rollback

Everything except the directory move is git-revertible per repo; the alias entries make the site side rollback-safe (rows land under either spelling). The remote rename is one `git remote set-url` back. No wire bytes, no ledger rows, no chain surface — the rename is prose + registration + site display only.

## Out of scope (051's own gates)

- Phase 2 (016's D1 trigger), Phase 3 (decstat rewards), Phase 4 (L4/L5 rungs), Phase 5 (dist/site plane) — each its own plan.
- Env-prefix renames (D2 divergence, Phase 1.5 candidate).
- Any history rewrite (Law 3 — never).
