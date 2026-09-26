# Issue 002 — hosted instinct lane via riir-deployer (`deploy.yaml`, cf-container)

**Status:** OPEN — filed 2026-09-26 (Plan 001 P5). Blocked on Issue 003 (a servable binary) and Issue 001 (vessel loading).

## Why

katgpt-rs Proposal 014 §"The hosted serving plane" (b): the native binary in
a distroless image behind the Worker edge, "the container exists for process
isolation + specialist vessels". "Hosted = modelless + specialist lanes
only" (014:98): laya is never hosted, and the specialist IS the hosted
product. The deploy mechanics are riir-deployer's (the manifest-driven
`build → BLAKE3-commit → stage → adapter → verify` pipeline), exactly as
riir-clippy's `deploy.yaml` drives its Workers.

## Plan

- [ ] **T1** — `deploy.yaml` (name `riir-instinct`, domain `decisions`): a
      `cf-container` target (generated wrangler.toml + Dockerfile), a
      linux-x86 artifact staged from the M3 (cross-build), HEALTHCHECK +
      `--cpus`/`--memory` budgets.
- [ ] **T2** — secrets: vessel decryption key + API tokens via `wrangler
      secret put` ONLY (the adapter refuses secret-shaped manifest keys);
      the canonical `CLOUDFLARE_API_TOKEN`, never the `CF_API_TOKEN` alias
      (the riir-clippy Issue 044 T16.1 trap).
- [ ] **T3** — stage-and-verify-first (`--stage-only`, then `verify`), a
      rolling drain/flip for vessel updates, explicit rollback.
- [ ] **T4** — devnet first. The mainnet flip is the owner ceremony
      (Proposal 014:79). Unfunded planes answer fail-closed `unconfigured`.

## References

riir-deployer README (providers table, cf-container row) + BOUNDARY;
riir-clippy `deploy.yaml`; katgpt-rs Proposal 014 §hosted plane, §Phase 4.
