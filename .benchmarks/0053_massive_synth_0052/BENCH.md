# Bench 0053 — massive synth-seat refresh (merged into the 0052 lane doc)

**Status:** COMPLETE — merged per-suite into the 0052 hybrid lane doc;
published (reflex-site `bd8dc6d`): the massive Instinct cell reads
0.8400, the manifest's seated posture.

## Why

The arsenal manifest's massive row is the SYNTH-SEAT posture
(H2(β=1,nmin=2,τ=4) over the 2048-row vetoed synth corpus — Bench 0029,
Plan 426 T6). The 0052 full-arena run seats GOLD by default, so the
manifest's arm had no measured read there (the builder's loud refusal —
the law working). This record seats the synth corpus exactly per 0029's
command and merges per-suite into the lane doc.

## Command

```
target/release/arena --skip-pin-a0 --suite massive_intent_en \
  --synth-corpus ../riir-reflex/.raw/corpus_synth/massive_intent_en_synth.jsonl \
  --out .benchmarks/0053_massive_synth_0052
```

`--skip-pin-a0` is the documented overlay posture (the pin's reflex-run
leg reads the gold corpus and cannot see the overlay — 0029's law).

## Result

- massive H2(β=1,nmin=2,τ=4): acc **0.8400** — byte-identical to the
  Bench-0029 frozen read (the third posture agreeing: 0029, the instinct
  AGENTS board line, this run).
- The doc merge (`build_doc_merged`, argv order: 0052 then 0053) keeps
  every other suite from 0052; box_state rides from 0052's span.
