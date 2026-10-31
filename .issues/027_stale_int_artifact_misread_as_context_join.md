# Issue 027: an int-labelled artifact on a content-bound seat is misread as a CONTEXT join (xnli_en/ag_news since reflex 077)

**Status:** OPEN — T1 LANDED (boot refusal); T2/T3 open. Filed 2026-10-31 (amo, session `claude-amo-i047`). Root cause measured; the fix is not chosen. It only affects research posture: no production arsenal row has this shape today.

## Symptom

`riir-rethink/tests/esc_g5_xnli_replay.rs` (parity-gated G5 replay of Bench 0057's frozen xnli_en record) fails on case 0:

```
case xnli_en:0: the armed replay refused: presented options neither all name seat labels nor match the label count (3) — the specialist bridge is undefined; unmatched ["entailment", "neutral", "contradiction"]
```

## Mechanism (instrumented 2026-10-31; a temporary eprintln at `src/server.rs` bridge refusal, since reverted)

`perm=[MAX, MAX, MAX] keys=["0","1","2"] rendered=3`. The joined lane is CONTEXT.

1. riir-reflex Issue 077 (`b3520a5`, 2026-10-08) content-bound the classification suites' training-doc labels: `xnli 0→entailment`, `ag_news 1→sports`, `emotion 1→joy`. sst5, banking77-int-fallback and prompt_injections keep the integer spelling. The xnli seat labels are therefore now `entailment/neutral/contradiction`.
2. The sealed artifact `xnli_en_winner_v1.bin` (Bench 0057 era, before 077) still carries the labels `"0","1","2"`.
3. `hybrid::seat_join` sees two fully disjoint label sets with a non-int seat and returns `SeatJoin::Context`, a domain-space seat (the typed_decisions shape). Seat labels then contribute nothing to `key_map`, `perm` is all-sentinel, the positional fallback is disabled, and every presented option refuses per request.

This is the MIRROR of the drift `seat_join` already documents as "a producer-contract break, never read as context": an int-spelled SEAT whose artifact lacks the noul pair. A **renamed-ClassLabel** seat over an int-spelled ARTIFACT is the same break in the other direction, and today it IS read as context. The refusal arrives per request and with a misleading message, instead of at boot with drift context.

The G5 gate has been red since 2026-10-08, unseen: it is parity-gated (`INSTINCT_ENCODER_PARITY=1`) and research posture (xnli left production at rethink issue 023 T2, 2026-10-05). ag_news has the same shape; it was unreadable on amo because the corpus is absent.

## Options (pick at implementation)

- **A: fail closed at boot (smallest, recommended first).** In `seat_join`, an artifact whose labels are exactly `"0".."n-1"` with `n == seat.len()`, opposite a disjoint non-int seat, falls through to `Named(seat)` so that `SpecialistLane::join` refuses at BOOT naming the drift. This does not repair G5. It turns a silent per-request refusal into a loud boot refusal. Risk: a future legitimate context seat whose artifact happens to be int-spelled 0..n-1 would refuse. That is acceptable, because such an artifact would be ambiguous anyway.
- **B: a declared ClassLabel alias.** The artifact or manifest row declares that its int classes are ClassLabel positions; the join maps `i ↔ seat label i`. This repairs G5 without a re-mint. It needs a grammar or row field, and the arsenal grammar is byte-pinned by rethink, so it is owner-adjacent.
- **C: re-mint the research artifacts with named labels** (riir-train). The frozen record's digest pins move, so the G5 frozen read must be re-cut. This is the most honest for the record, but it is research-only value.

## Tasks

- [x] T1 Option A (LANDED `383494d`; xnli now refuses at boot: `join: seat label "entailment" not in the artifact`) plus a `seat_join` unit row for the mirror drift (named seat over an int artifact means NOT context)
- [ ] T2 Decide B or C for the research lane, or record G5 as RETIRED-with-reason in the rethink test doc
- [ ] T3 The ag_news check, on a box that has the corpus
