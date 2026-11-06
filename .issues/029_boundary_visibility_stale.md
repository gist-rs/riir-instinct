# Issue 029: BOUNDARY.md says "private TODAY" — the repo is PUBLIC on GitHub

**Status:** Open — doc drift found at verdict round 1 of the dcog distill (2026-11-06): `gh repo view gist-rs/riir-instinct` reports `"visibility":"PUBLIC"`, while `BOUNDARY.md` line 3 still claims the repo is private. The carve (Proposal 052, 2026-10-03) opened this repo as the public teaching lane — the line predates or ignores the flip. A public-facing visibility claim that is wrong is exactly the leak-class error the boundary docs exist to prevent (a private note was nearly filed here on the strength of that line).

## Task

- [ ] **T1** — Owner/lane confirms intended visibility: PUBLIC per the carve law ("Instinct OPENS as the public teaching lane, SINGLE-SOURCE-OF-TRUTH") vs any remaining row that should move to `../riir-rethink` (the private moat). If any moat-class content still sits in this tree, list it for migration first.
- [ ] **T2** — Fix the `BOUNDARY.md` visibility line in one commit (boundary docs are deliberate-edit territory — no drive-by fix from an unrelated lane; this issue is the record).
- [ ] **T3** — Sweep the repo for other stale "private" claims (README, AGENTS.md, `.docs/`) and fix in the same commit; add a boundary-guard visibility check row so the drift cannot regrow.
