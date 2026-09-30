# decstat receipt FLOOR (0034_decstat_receipt_floor)

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

## Posture 2 — cross-process (the honest verifier shape)

The second cell, same binary, FRESH process + FRESH seat boot + fresh
throwaway keys: **reproduction rate 1.0000 (63 re-runs / 0 mismatch)** —
boot-to-boot determinism holds (the ridge-ladder cal derivations and
every boot-time table reproduce exactly), which is the load-bearing
assumption for third-party verification.

Read the yield line's arithmetic before quoting it: run 1's items were
still at 1-of-2 verdicts (one verifier never reaches K=2), so this
run's verifier ALSO judged 23 run-1 leftovers whose input hashes
overlapped its own 40-sample → the server answered `duplicate` on the
re-verdicts (23 duplicate acks) and the runner's yield numerator
counted all 63 verified verdicts against its 40 distinct tracked
inputs (yield > 1 is that bookkeeping artifact, not a lane defect —
the reproduction rate, the arming quantity, is per-re-run and clean).
One leased item hit the toolchain screen (skipped, no verdict) — the
screen demonstrating itself on a foreign-fingerprint item.

Postures measured so far: same-process (0033) · cross-process
(0034). The cross-HOST cell needs a second box running the same runner
against the same devnet lane — recorded as the remaining posture.
