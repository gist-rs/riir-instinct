# Issue 002 — hosted instinct lane via riir-deployer (`deploy.yaml`, cf-container)

**Status:** T1–T3 LANDED 2026-09-27 — the serving binary (`src/bin/serve.rs`), the multi-file cf-container stage, and the local e2e are done: plan → cross-build (zigbuild, ELF gate) → 948-row BLAKE3 manifest → stage (12 file rows + Dockerfile + wrangler.toml) → docker run (x86-64 image under Rosetta) → all six lanes ready → live decisions + receipts verified (ag_news/emotion/xnli/massive). **T4 stands as designed: devnet-first — the real `wrangler containers build` + deploy is the owner-adjacent step (CF creds); the mainnet flip is the owner ceremony (Proposal 014:79).** P4 (vessel loading) and Issue 003 (the serving composition) closed first; the deployer's multi-file `files:` rows landed sibling-side at riir-deployer `75b8240`.

## Why

katgpt-rs Proposal 014 §"The hosted serving plane" (b): the native binary in
a distroless image behind the Worker edge, "the container exists for process
isolation + specialist vessels". "Hosted = modelless + specialist lanes
only" (014:98): laya is never hosted, and the specialist IS the hosted
product. The deploy mechanics are riir-deployer's (the manifest-driven
`build → BLAKE3-commit → stage → adapter → verify` pipeline), exactly as
riir-clippy's `deploy.yaml` drives its Workers.

## Plan

- [x] **T1** — `deploy.yaml` (name `riir-instinct`, domain `ops`): a
      `cf-container` target (generated wrangler.toml + Dockerfile), a
      linux-x86 artifact staged from the M3 (cross-build), HEALTHCHECK +
      `--cpus`/`--memory` budgets. DONE 2026-09-27: the manifest carries
      the serve bin (zigbuild x86_64) + 12 `files:` rows (6 sealed winners
      + 6 t20k dataset suites, per-file BLAKE3 rows — the multi-file
      container shape; deployer support at riir-deployer `75b8240`), env
      `/data/{datasets,winners}` (the generated Dockerfile has no
      WORKDIR — relative `to` paths land at the image root),
      `standard-2`. The serving binary itself is `src/bin/serve.rs` —
      std-only HTTP edge (`/decide`, `/healthz`, `/`), the decision
      receipt (Proposal 014 §4: build fingerprint + feature set +
      BLAKE3(input) + decision hash), lanes load on 64 MiB-stack threads
      with the listener bound FIRST (healthz live during boot; a loading
      or failed lane answers 503, never a silent fallback), and the
      serving posture table pins the Bench-002 GOAT verdicts — banking77
      serves A0 (the registered H1 failed G3; the H1 row stays a
      published measurement, never the product posture).
- [x] **T2** — secrets: vessel decryption key + API tokens via `wrangler
      secret put` ONLY (the adapter refuses secret-shaped manifest keys);
      the canonical `CLOUDFLARE_API_TOKEN`, never the `CF_API_TOKEN` alias
      (the riir-clippy Issue 044 T16.1 trap). DONE 2026-09-27: cf-container
      env rows are now plan-time refused when secret-shaped (the cf-worker
      T1d rule extended; deployer `75b8240`). No secret exists in the
      manifest — env carries the two data-dir paths only.
- [x] **T3** — stage-and-verify-first (`--stage-only`, then `verify`), a
      rolling drain/flip for vessel updates, explicit rollback. DONE
      2026-09-27 for the stage-and-verify half: `plan` → cross-build via
      `.deploy/cross-build.sh` (cargo-zigbuild; ELF gate in-pipeline) →
      948-row BLAKE3 manifest + artifact gate → cf-container stage (12
      file rows + Dockerfile COPY lines + wrangler.toml env forwarding) →
      **local docker e2e PASSED** (x86-64 image under Rosetta; HEALTHCHECK
      healthy; all six lanes ready ≤ 14.7 s; live decisions 0.5–5 ms incl.
      the container's first-tick A0 solves; the artifact-known
      seat-unknown sentinel presentation answered over HTTP). Roll/flip:
      cf-container is instances=1 + deploy-pushes-a-revision (E5, the
      provider refuses in-place roll by design); vessel updates are the
      same redeploy shape when the first vessel ships.
- [x] **T4** — devnet first. The mainnet flip is the owner ceremony
      (Proposal 014:79). Unfunded planes answer fail-closed `unconfigured`.
      POSTURE DOCUMENTED 2026-09-27: everything above the CF push is
      verified locally; `wrangler containers build` + deploy needs the
      owner's CF creds (the wet-run precedent: verified steps pass, the
      cred-gated step is named and refused, never improvised).

## References

riir-deployer README (providers table, cf-container row) + BOUNDARY;
riir-clippy `deploy.yaml`; katgpt-rs Proposal 014 §hosted plane, §Phase 4.
