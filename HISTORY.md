# HISTORY.md — riir-instinct

Compacted 2026-10-30: every `##` heading kept verbatim, one compact entry per record (≤2 short lines); the full pre-compaction text is in git history (pre-compaction HEAD 844f9170d72d). Operational rules live in `AGENTS.md`.

## 2026-10-30 — Issue 024 CLOSED: serve_edge request read under a TOTAL deadline (`RequestDeadlineGuard`)

**Status: Resolved. Fix `c2f909b` (HEAD at close; the general law = riir-refine batch 316 `per-read-timeout-vs-request-deadline`,
corpus `4d1fdb2e`).** (`02523aec` · `d947fa5`)

## 2026-10-27 — owner verdicts on the 008/009/019 remainders: Tetris DEFERRED entirely; holds moved to riir-rethink Plan 007

Owner call: **Tetris is deferred — no value in training it for research's sake.** (`78f9cae` · `a2a6df1`)

## 2026-10-27 — issue-file hygiene: 008 / 009 / 019 closed + removed (every remaining box superseded, out of levers, or owned elsewhere)

Owner direction "make decision and fix all this". No compute; dispositions from the measured record. (`0c9fcab` · `78ba638a`)

## 2026-10-21 — the Phase-C remote think leg disclosures (`3d7fc31`): additive healthz + gate_abstained on the wire (consumer: rethink `c3a73cd`)

riir-rethink Issue 024 Phase C (plan 010) needed three facts a light client cannot derive without the local encoder material, plus
one decision field its `ServedDecision` reconstruction cannot honestly fill: `/healthz` gains `centroid` (the DIM) …

## 2026-10-16 — Issue 023 CLOSED: the RunOptions field-add break class closed at the root (`lanes_off()`)

The pplx → d3 → jeva E0063 lineage (3rd occurrence live 1.5 h before a clippy sweep caught it) is structurally gone: reflex
`a7ba797` landed `RunOptions::lanes_off(dir, suites)` — the single struct-literal home for the no-flags CLI posture …

## 2026-10-11 — code_fixtures RETIRED from the board (owner call; board 9 → 8)

The last self-authored synthetic suite left the serving board: `code_fixtures` was generated from reflex's own fn spans (n=32
questions / 16 cases, gold programmatic, seat-marked `synthetic`) — the same class the six Issue-004 families were retired …

## 2026-10-09 — Plan 010 LANDED: the Runetrace scene-loop composition (seal-remake Proposal 005 Phase 4, first lane — katgpt-rs Research 613 executed)

`ffdc9a8` — the modelless scene-agent loop over shipped GOAT primitives, all consumed, none re-implemented (the substrate audit
table is the plan's §T0): scene in as a deterministic `RunetraceDoc`, judgment out on `decision_wire` with first-class …

## 2026-10-06 — issue-file hygiene: 020 / 022 removed (both fully landed; the residue is by-design contract surface, not actionable leads)

020 (artifact-class adoption) LANDED `f78a45f` (record `6d601b1`, boundary follow-up `c21984e`): `artifacts/manifest.toml` (two
public weights rows, zero protected), demo winners published to gist-rs/riir-instinct-artifacts (dataset commits) …

## 2026-10-06 — boundary: the fence's two post-split REDs fixed (found during the 020/022 pass; both pre-existing, not introduced by it)

Pre-existing moat references in `*.rs` (verified `git log -S`): `tests/serve_gates.rs:206` comment reworded token-free (the A6
production digest re-pinned `47fcd95c…`, comment-only delta, zero data rows) and `tests/serve_g4_alloc.rs:115` — the G4 …

## 2026-10-05 — reflex issue 070 lead 2 adopted: bag serve path 83 → 42 allocs/decision (instinct `9ac84c8`)

reflex `e0c43c7` landed `eval_case_into` + `CaseEvalScratch` — one caller-owned frame refilled in place, taken out of the server
around the receipt loop (take/replace); pin re-measured **83 → 42** (deterministic ×3; lineage 107 → 83 → 42).

## 2026-10-05 — Issue 021 closed: the ESC cheap-path allocation surface — bag serve path 107 → 83 allocs/decision, composed ESC 190 → 161; every pick bit-identical

Ownership/scratch refactor (lead from riir-refine Issue 146 T0.6's hotpath profile): instrument born (`tests/serve_g4_alloc.rs`,
baseline 107), `synth_served_case_into` + an allocation-free bridge pass; bag serve path **107 → 83**, composed ESC …

## 2026-10-05 — riir-rethink Issue 023 T2 lockstep: the production manifest pin re-synced — xnli_en + emotion are benchmark-only

Rethink demoted the licence-barred pair (CC BY-NC / research-only data cannot back the paid lane);

## 2026-10-05 — riir-rethink Issue 023 T3 lockstep: the ⚠ pair settled — the manifest is the five permissive rows; ESC zero-armed pinned

sst5 + ag_news also left the production manifest (SST's README carries no licence over scraped Rotten Tomatoes text; AG News is
Gulli's no-grant 2004/05 corpus): `PRODUCTION_MANIFEST_TOML` 7 → 5 rows, digest …

## 2026-10-05 — Issue 013 closed: pool re-baseline (a) EXECUTED — arena → canonical pool at the PINNED reflex baseline; standing triggers → Issue 019

Claude verdict GO(a): the arena's datasets default = canonical `../riir-reflex/.raw/datasets` at NAMED pin
`REFLEX_BASELINE_SHA = 6365fab0261b2123ae0953a12b06ef0eec596c2b` (stamped in the startup banner + RESULTS.md); t20k stays the ARCHIVED pool …

## 2026-10-03 — B4 LANDED: the L1 training lane is in-tree (the carve's deferred riir-train half)

L1 trainers + teaching recipes moved from `../riir-train` into THIS repo (Plan 008 B4: `src/instinct_specialist*.rs`,
`instinct_nbsvm`, `instinct_bank77_cases` + the six `examples/instinct_*` lanes); the teaching arc runs end-to-end from a clone …

## 2026-10-03 — MOAT-LEAK AUDIT (post-reversal): fence GREEN on code; records arm ADDED (the GREEN said nothing about .benchmarks); r5_launch_ab.ps1 removed; B2 record corrected

Full public-tree sweep: fence GREEN on code, `sync_mirror.py --check` 13/13, zero moat code, zero artifact bytes.

## 2026-10-03 — the two-repo arrangement RETIRED (owner directive): single source of truth = gist-rs/riir-instinct (full history, public)

Owner: "git history leak is fine… i dont want 2 confusing repo i want single source of truth" — the full-history `develop`
force-pushed to the public gist-rs/riir-instinct (`main` = same tip; the fresh-root export `382b6bd` superseded); this …

## 2026-10-03 — PHASE C LANDED: the public export is LIVE (gist-rs/riir-instinct, public, `382b6bd`); this repo renamed gist-rs/riir-instinct-internal

Owner green-lit in-band; plan 008 Phase C same day: the `.export/` build tree (41 [copy] rows verbatim minus `r5_launch_ab.ps1`),
fresh README/AGENTS/BOUNDARY/HISTORY, MIT LICENSE, `scripts/setup_siblings.sh` (four public siblings — the fourth) …

## 2026-10-03 — the Instinct/Rethink SPLIT CARVE landed: B1 seam + B2 move family + B3 in-repo registration; fence --post-split GREEN (riir-ai Proposal 052 / riir-instinct Plan 008)

B1 `22ae291` the SEAM: `server::LaneBackend` + `ExtBoots` + `install_ext_boots` + the `AnySuiteServer::Ext` seat; parity gates
byte-identical (shared/per-lane fingerprints sst5 `2e89e9cb12f6e9ec`, xnli `b481fbdb7d039921`). (`f262da8`)

## 2026-10-03 — riir-rethink BORN (plan 008 B3's wave, the owner's in-band green light)

gist-rs/riir-rethink born at `e263686` (fresh root, 242 files), seeded verbatim from `moat/` at `4ffd08f`; the SEED-CARVE model
(riir-refine shape) settled the plan's open pick — whole git history stays HERE, `moat/` a pointer; the seam needed ZERO …

## 2026-10-02 — the six harness families RETIRED (owner call, reflex `31b11d2` + instinct this commit)

The owner removed the six home-made harness families from the reflex harness itself (at-chance at honest wide-eval populations;
"so no one benches it anymore"); instinct-side the six artifact-less A0 rows left `arsenal.toml` (digest re-pinned) …

## 2026-10-02 — Issue 018 closed: the Rethink encoder lean+GOAT arc — the shared encode worker PROMOTED default, Q8 adopted end-to-end, Q4 measured out

Lane B shared encode worker `41234d4` (Bench 0050, RAM 4938→1690 MB = 2.92×; residue `eef77d1` — ENC rows skipped the bag-winner
coupling in `load_lane`) PROMOTED default `e6e703b`: `serve-encoder` implies `serve-encoder-shared`, runtime demote …

## 2026-10-02 — Bench 0051: the families wide-eval re-baseline — the serve-parity gate re-pinned on the template-disjoint populations

reflex `e78c0e6` (Plan 009 REVISED-2) widened the five families' evals to 96–100-case template-disjoint populations, invalidating
Bench 049's n=12–16 serve-parity record BY CONSTRUCTION; this record re-measures A0 on the wide evals (all a0_stands) …

## 2026-10-01 — Bench 041: the typed_decisions Rethink cell SEATED — the v2 arena replay reproduces 0.7550 EXACT (016 T9 complete)

instinct `73cd6d6` + reflex-site `245bf34` + CF `8abd1add`: `encoder_arm.rs` reads NLEH v2 (the law-copy t608 mirror; per-question
flattening 5 q/case → 2000 rows, `--encoder-ckpt typed` parse-time validated); the arena replay reproduces the …

## 2026-10-01 — Bench 039+040: the three unscreened encoder suites screened + the EARNED typed run — the NLEH v2 per-option head reads 0.7550

Screens (Bench 039): banking77 DEAD (floor 0.4980 + max lift 0.645 < 0.8540), prompt_injections DEAD (0.845 < 0.8534), typed ALIVE
(reference floor 0.7445 = +9.7 pt over the 0.6475 bar); the three wire-fidelity witnesses byte-exact. (`e118c29d`)

## 2026-10-01 — Bench 038: the emotion confusion read — ±0.0 explained with mechanism, and every remaining lever priced (records-only)

From FROZEN artifacts (reproduces A0 0.8850 / A1 0.8550 / H1 0.8775 EXACTLY): emotion serves A0 — reflex's ridge@8 base, the
board's highest bar; all 46 misses at conf < 0.30 (32/46 abstained).

## 2026-10-01 — Issue 017: the xnli_en encoder cell SEATED (record-only, serve ✗) — the owner's progress-display call reversing the 014-round-3 sub-decision

Owner call: the measured cell shows the product's real posture ("no progress at all and -4pt gimme some hope") — reverses
014-round-3's "no separate xnli cell";

## 2026-09-30 — Issue 015 closed: the canonical massive winner synced to the 4090 — the serve's A5/A9 pin validates and boots (the drift was box sync, not the pin)

Canonical `massive_intent_en_winner_v1.bin` (blake3 `7bc3ee385f81abc0f2a91e47f6dca08db37f9fb6cee6d48121b80e3f11378556`) scp'd into
the 4090's `E:\git\riir-train\data\instinct_specialists\`; verified by the serve's own A5/A9 gate at clean HEAD …

## 2026-09-30 — issue-file hygiene: 005 removed (every measurable face landed, refuted, or absorbed; the GOAT verdict is the serving posture)

005 (the Moka+PUCT hybrid POC) complete across every face: T1–T5 measured (Bench 001 v2 → the ALIGNED Bench 002); the GOAT verdict
= the standing serving posture (massive H2 0.8267 T2-certified SERVING; typed H2 0.6475 via Benches 019+020; banking77) …

## 2026-09-30 — Issue 014 C1 EXECUTED: the sst5 encoder arm reads 0.5267 on the ARENA (T2-certified above A1, LB95 +0.0535) — published `serve: ✗`; en-route: the T8 name-first noul regression fixed

`src/encoder_arm.rs` + `--encoder-art` (features `arena-laya`/`arena-laya-metal`): the sealed NLEH v1 head `t6_s0.bin`
(BLAKE3-verified) over a live M3-Metal laya-english encode; **ENC 0.5267 (316/600)** cell-identical THIRD posture; (`b9b2ee1`)

## 2026-09-30 — Issue 008 T8 EXECUTED: the code_fixtures tie BROKEN (A1 0.5625 vs A0 0.3750, T2-certified) and the six harness families DROPPED; the serving path landed (S8 + the name-first noul law + the 9th manifest row)

Trainer floor 0.45 via riir-train's `--abs-floor`/`--tie-break` flags; minted `code_fixtures_nbsvm_v2.bin` (blake3 `264714b9e7518e33…`); the
frozen read (`.benchmarks/028_code_fixtures_tie_break/`): **A1 0.5625 vs A0 0.3750 — T2 PASS** (LB95 +0.0021) …

## 2026-09-30 — Issue 008 T4's armed-posture caveat RESOLVED (records-only, no compute): emotion's board edge is posture-gap, not specialist value

From EXISTING records: the armed seat reads **A0 0.8850** == reflex Bench 057's armed ridge@8 cross-pool while A1 0.8550 loses —
emotion's board edge is POSTURE-GAP, not specialist value; the serving posture was already honest (A0 serves).

## 2026-09-30 — Issue 009 round 5 closed on both forms: the teacher A/B (Bench 026) measured NO-GO beside the residual pilot's null (Bench 027); the lane stops, eval seeds still at 5 reads

Teacher z-blend A/B (Bench 026, component `5c7eb6ca4ae3bb3e` = Bench 025's attempt-2 rebuild): **NO-GO** per the pre-registered
rule (16@75 lb95 −119.7; 18@75 leans the other way; 20@80 DEGENERATE — both arms 0 pieces on all 20 seeds); residual …

## 2026-09-29 — the instinct flow figure reflowed to two bands (owner ask: too wide, rendered small)

The source mermaid in `.docs/03_decision_flow/instinct_flow.md` reflowed to the Reflex hero's flywheel shape (top band "the
question in — modelless, always first, free"; bottom band "on abstain — the trained add-on, paid only here") and re-rendered …

## 2026-09-29 — Issue 008 T7 wave 1 (owner GO): six vs-best specialist lanes measured — every verdict a NEGATIVE; sst5's holdout win refused at the frozen read (Bench 024); Issue 005 H3 closed by a measured premise refutation; Issue 009's critic-guided-search lever re-derived GO-as-teacher

All six lanes NEGATIVE: xnli LB95 −0.0045; massive +6.5 lift (LB95 +0.0316) still misses the +9.3 gap; ag_news at ceiling; sst5
holdout PASS (+3.10 pt, `sst5_nbsvm_v2.bin` blake3 `633a3a67ed9a6987…`) REFUSED at the frozen read (Bench 024: A1 0.4217) …

## 2026-10-02 — the m3 serve plane repaired: the typed t20k pool + the code_fixtures winner recovered byte-exact

typed t20k pool synced to the full 1200-row pull from `riir-train/.raw/datasets_typed_full` (the 960-row slice guard, reflex Issue
058, refused the stale 800-row envelope; winner blake3 `7f7a39e1…` verified);

## 2026-09-29 — issue-file hygiene: 010 removed (every measurable task landed; the residual is an owner decision, moved to 013)

All six tasks landed 09-27/28: Bench 005 (`run_suite_a0_only`, the three-state lane doc), the 8/8 A0 pin (the oc-armed typed
catch), Bench 011, the Bench-016 `harness_cache_reuse` seat, and the SITE half reflex-site `74b49e4` (15 hybrid rows live) …

## 2026-09-28 — the 581 lane lands end to end: typed_decisions seats, certifies, and SERVES (Bench 015 + Issue 011)

Root cause (riir-train `d59d3984`, Bench 614): Bench 014's "template drift" was a FETCH-CAP ARTIFACT (800 of 1200 train rows);
retrained at 610's config over 1200 rows / 49 classes (`typed_decisions_armA_v1.bin` blake3 `7f7a39e1935f8665…`); reflex's …

## 2026-09-28 — issue-file hygiene: 001 / 002 / 004 removed (work verifiably landed, records durable)

001 HOSTED-ONLY vessel reader (`src/vessel.rs`, opt-in `vessel`), 002 hosted lane (`deploy.yaml` cf-container, local e2e
verified), 004 decstat flywheel (`9da822b` → `3a1eb09` → `c285435`; T4 tracked at riir-train Issue 577) — all landed.

## 2026-09-28 — Issue 579 T3: the arena bridge — banking77 serves the nbsvm v2 winner (Bench 012)

`specialist::winner_bridge` (the ONE home for the per-suite winner file + bag convention) + `presence_bag_into` + `BagConvention::bag_into`
at every bag site + `check_winner_file` (a raw boot/swap naming another artifact for a bridged suite refuses) …

## 2026-09-27 — Proposal 001 T8: `arsenal_budget_goat` GOAT PASS — and the gate caught the unsigned-fold defect (Bench 003)

`benches/arsenal_budget_goat.rs` (opt-in `arsenal_goat`, refuses exit 1 without data) ran **7/7 GOAT PASS** (`.benchmarks/003_arsenal_budget_goat/`);
the gate caught the UNSIGNED-fold defect — token-hash histograms are bucket-uniform, so every real …

## 2026-09-27 — Issue 007 closed: the missing builder self-test landed, and the committed Bench-002 lane doc predates the scope field

T4's builder self-test did not exist — landed as `scripts/build_hybrid_doc.py --self-test` (5 known-answer fixtures: scope
inference both directions, the A0 skip, replicated metrics vs hand-computed values); the committed Bench-002 lane doc was …

## 2026-09-27 — Bench 002: the aligned Bench-052-protocol read + the reflex-site hybrid lane publish (Issue 003 T4 closed)

The arena re-ran at the **Bench-052 protocol on the same `datasets_t20k` bytes** (977/977 byte-verified); the comparability proof
= **A0 == the published 052 modelless rows 6/6** + the in-run xnli drift pin.

## 2026-09-27 — Issue 006 RESOLVED: the arena's position-vs-index instrument defect (Bench 001 v2)

Fix `3a12a70`: the v1 massive anomaly was the ARENA — (1) space mismatch (scoring/picking in label space vs presented-option gold;
only massive's 20-of-59 sampled options could expose it) and (2) double-indirect `cands[rank0_sorted[select_arm(..)]]`;

## 2026-09-27 — Bench 001: the hybrid GOAT run (P3 + P3a through the single test read)

First end-to-end hybrid measurement: the one-way seat seam lives in REFLEX (`harness::runner::seat` — reflex consuming instinct
would be a cycle); the A0 drift pin (xnli 0.5167 byte-exact) proves the seat path every run.

## 2026-09-27 — Bench 004: the re-baseline + the strict-superiority product gate (Issue 008 T1+T2)

T1: seat posture = the CURRENT PUBLISHED reflex posture (`head+nb+ridge` select; reflex Issue 050 `7a99431` — `nb_ridge`+`option_cond`, the
latter because `nb_ridge` alone does not compile upstream); the A0 drift pin widened to all six suites + the …

## 2026-09-26 — birth

Created per riir-ai Proposal 047 (owner decision: the model-based/hybrid lane lives in a PRIVATE repo — hybrid wiring and weights
are product moat; `riir-reflex` is public). Trainer: riir-train Issue 576.

## Lessons

- A pick/gold pair is a SPACE contract — when a seat samples its option set, every scorer must speak the presented-option space (Bench 001 v1).
- Imitation top-1 agreement ≠ play strength; the disagreement concentrates in catastrophic placements (Tetris T7) — a re-open needs a tail-targeted loss or critic-guided search.
- Quantized-artifact determinism is within-binary, never cross-build — the katgpt-core path-dep tree moves bytes (the code_fixtures re-mint).
- Single-probe a new regime's playability before allocating an eval cell (the 20@80 DEGENERATE hard cell, Bench 026).
- Gate infrastructure with real-data benches, not fixture-only units — synthetic orthogonal vectors hid the unsigned-fold defect (Bench 003).
- The sigmoid-gate calibrator's `apply()` is the identity until `refit()`; `black_box` is the liveness guarantee; the label join's contract is seat⊆artifact (Bench 001).
- Quote `box_state.json` / the preflight PROVENANCE line beside any latency figure (the reflex Issue-021 law); read a re-baselined
  pool's rows as a NEW series, never comparable with archived-pool rows.
- Re-opens ride a model-CLASS upgrade, never another bag sweep (the wave-1 law).

## 2026-09-30 — Naming law decided, SUPERSEDED 2026-10-03 by the split (moved from AGENTS.md)

Owner call 2026-09-30 to retire the Instinct brand was superseded by riir-ai Proposal 052's split: the repo keeps its name; site lanes hybrid → Instinct, encoder → Rethink.

## 2026-10-08/09 — Bench 0059 seat re-baseline + Bench 0060 s1mb A0-pin close (moved from AGENTS.md)

0059: seat = published posture (head-select retired, per_byte armed), A0 pin GREEN 9/9 both halves. 0060 (`60f373c`): s1mb trio wired, A0 pin GREEN 3/3; s1mb_score row republished both hosts; serde_json `float_roundtrip` fix.

## 2026-10-31 — Issue 027 CLOSED + removed: a stale int-label artifact on a content-bound seat was misread as a CONTEXT join

reflex 077 (`b3520a5`, 2026-10-08) content-bound the xnli/ag_news seat labels. The pre-077 `xnli_en_winner_v1.bin` still carries `"0","1","2"`, so `seat_join` returned Context and every request refused with a misleading bridge error. The parity-gated research G5 replay was red, unseen, from 10-08 to 10-31. **T1** (`383494d`): the mirror producer-contract break (a named seat over an int-spelled `"0".."n-1"` artifact of the same arity) now falls through to `Named(seat)`, so the join refuses at BOOT naming the drift. A unit row was added; no production row has the shape. **T2** (decided, not owner-gated): the rethink G5 replay is RETIRED-WITH-REASON at riir-rethink `cdea009`. The boot refusal is its pinned outcome, any other error reds, and a re-minted named-label artifact re-arms it unchanged. The declared-ClassLabel-alias grammar (option B) and a riir-train re-mint (option C) were declined: xnli is licence-demoted research posture, so the value does not cover a grammar change or a frozen-read re-cut. **T3**: no ag_news specialist artifact exists in the fleet (the 4090 carries only the `ag_news_encoder_v1.bin` think head), so the shape cannot arise there. With the corpus synced to amo, ag_news's ESC gate-fit pin reproduces post-077, and rethink esc_gates reads 18/18.
