# Plan 002 — the decstat flywheel (Issue 004 T1–T3; the fixstat-lane shape)

**Status:** IN PROGRESS — filed 2026-09-27. Wire-first (the fixstat
precedent: proposal → wire → clients → server), one repo per phase.

## Shape

The riir-clippy `--stats` fixstat lane, carried for decisions: consent-gated
decision-outcome rows → riir-kat wire (lane `dec`) → riir-dapps store +
public view + per-epoch commitment root → (later) retrain threshold.
`Unset never pushes`; no payload text beyond the row schema; the served
`/decide` path stays byte-identical when the feature is off.

## Tasks

- [ ] **T2 — riir-kat wire.** `kat_protocol_decstat.rs` (feature-free:
      `DECSTAT_SIGNING_DOMAIN = riir-kat-decstat-v1`, ceilings, canonical
      `decstat_signing_message(run_id, machine, toolchain, decisions,
      abstains, counters, suites)`, golden bytes) +
      `kat_decstat_client.rs` (`DECSTAT_LANE = "dec"`, `DecStatRow`,
      body/ack under `client`, `DecstatTransport` + `push_decstat` under
      `kat_transport`; reuses `fresh_run_id` + `clamp_machine_label`).
- [ ] **T1 — instinct capture.** Opt-in `decstat` cargo feature
      (`dep:riir-kat` client+transport): `src/decstat.rs` aggregate sink
      (composed counters `suite:primitive:class`), consent
      `RIIR_INSTINCT_STATS=on` (exact literal; unset = no capture, zero
      overhead), key via `INSTINCT_ACCOUNT_KEY` (64-hex seed file),
      service URL `INSTINCT_KAT_SERVICE_URL` (default the devnet host —
      the Proposal-014 devnet-first ladder), flush tail (≥256 decisions or
      60 s, 5 s budget, never an exit-code/latency change). Serve lane
      records (suite, lane tag, pick/abstain) after the response.
- [ ] **T3 — riir-dapps store.** `DecStatDoc` parse/verify (the dec
      composer) + a lane-generic `ingest_stat_run` core (`ingest_fixstat`
      refactored onto it; `ingest_decstat` beside it) + the
      `/mining/decstats` worker route (POST ingest / GET public view) +
      the alarm rebuild leg extended to the dec lane + contract tests
      (ingest → duplicate → view → root → leaves verify; cross-account
      replay refusal; fix/dec lane separation).
- [-] **T3b — KAT rewards for decstat rows.** DEFERRED with the
      owner-gate named: fixstats pays nothing; mining pays behind the
      clippy-oracle proof pass. Self-reported decision outcomes have no
      server-side oracle, so a per-row reward is a free mint until an
      honest verification shape exists (arena gold checks are the
      recorded candidate). Pricing is owner territory besides
      (Proposal 014: the never-lever doctrine). The commitment root +
      views land now; rewards re-open with a priced verification design.
- [ ] **T4 — riir-train intake** (filed, deferred): settled rows join the
      specialist corpus; retrain at the measured threshold, GOAT-gated,
      then vessel mint + P5 redeploy. Blocked on accumulated rows.
- [ ] Docs: instinct BOUNDARY.md dep row + AGENTS.md sibling table,
      dapps lib counts, issue 004 checkboxes.

## Wire row (T2, the contract)

`{account_pubkey_hex, signature_hex, run_id_hex, lane:"dec", machine,
toolchain, decisions, abstains, counters:[[name,count]…], suites:[[name,count]…]}`
— counters are composed `suite:primitive:class` keys (`class ∈
{pick, abstain}` in v1), suites are per-domain totals. Signing covers the
canonical message only (the fixstat convention; body field order is free).
