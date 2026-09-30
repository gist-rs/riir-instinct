# decstat receipt FLOOR (0033_decstat_receipt_floor)

The measured same-release reproduction rate of the decstat receipt
reward class (riir-dapps Plan 043 Phase C / D5): the number that gates
arming — a RATE, never a latency claim. The numbers live in
[`floor.json`](floor.json) (the runner wrote them; nothing here is
hand-typed).

## What was measured

The submitter and verifier roles, ONE process over the SAME booted seat
engine (`--example decstat_floor --features decstat`):

1. **submit** — N engine decisions sampled round-robin across the
   manifest's booted suites; each receipt's input/decision hashes are
   computed from the ACTUAL decide runs (honest by construction), signed
   with the submitter key, pushed to `https://devnet.ai.gist.rs`.
2. **lease + judge + verdict** — the verifier key leases items, judges
   each per the C1 loop (toolchain screen → manifest screen → corpus
   resolution → re-run → hash compare), pushes verdicts, until ≥ N
   distinct inputs are covered or the round cap hits.

The FLOOR quantities: `reproduction_rate = verified/(verified+mismatch)`
(the D5 arming quantity) and `end_to_end_yield = verified/leased`, plus
the skew breakdown (toolchain / manifest / corpus / mismatch / verified
/ duplicate acks) and the box state (the G2 provenance law).

## Reproduce

```sh
INSTINCT_ACCOUNT_KEY=<64-hex seed file> \
INSTINCT_VERIFY_KEY=<64-hex seed file — a DIFFERENT account> \
INSTINCT_KAT_SERVICE_URL=https://devnet.ai.gist.rs \
cargo run --release --features decstat --example decstat_floor -- --n 40 --rounds 8
```

Boot env (the serve binary's contract): `INSTINCT_DATASETS_DIR`,
`INSTINCT_WINNERS_DIR`, `INSTINCT_SYNTH_CORPUS_DIR`, `INSTINCT_ARSENAL`.
Flags: `--n <decisions>` · `--rounds <max>` · `--limit <lease size>` ·
`--suites a,b` · `--benchmarks-dir <dir>`.

Refusals (exit 2): a missing key, a 503 (the lane is inert until the
reward class arms — the recorded posture), or zero suites booted.

## Posture (read before quoting the 1.0000)

- **Same-release, same-process, same-seat** — submitter and verifier ran in
  ONE binary at ONE build fingerprint over ONE booted seat. This is the
  MECHANICAL ceiling of the reproduction rate, not the adversarial floor:
  the cross-host / cross-corpus-build / release-skew readings are future
  postures of the same runner (point `INSTINCT_KAT_SERVICE_URL` at a second
  host; build a second binary). The devnet lane exercised here is the real
  wire — signatures, lease, canary state machine, verdict acks — so the
  lane mechanics are live-proven even where the determinism claim is the
  easy posture.
- **Canaries: 0 sampled** — the 20‰ mixing rate over 40 items produced no
  canary lease item in this run; the canary polarity is contract-tested
  server-side (the dapps worker contract) and client-side
  (`decstat_gates`), not floor-measured here.
- **Throwaway devnet accounts** — both keys are fresh 64-hex seeds (the
  submitter is NOT the fleet account); at `decstat_cut_bp = 0` +
  `decstat_reward_micro = 0` nothing pays, so the two-key inertness held
  for the whole run (zero reward rows).
- Power: AC plugged, charged (the box-state line in `floor.json`; a RATE,
  not a latency claim — recorded per the provenance law anyway).
