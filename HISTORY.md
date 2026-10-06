# HISTORY.md — riir-instinct

## 2026-10-06 — issue-file hygiene: 020 / 022 removed (both fully landed; the residue is by-design contract surface, not actionable leads)

020 (artifact-class adoption) LANDED `f78a45f` (record `6d601b1`, boundary follow-up `c21984e`): `artifacts/manifest.toml` (two public weights rows, zero protected), demo winners published to gist-rs/riir-instinct-artifacts (dataset commits `1799a755` choice / `976548f5` noul), anonymous fetch round-trip via `scripts/fetch_artifacts.sh`, 8 defaults re-pointed `artifacts/cache`. 022 both leads landed — LEAD 1 `f8ee3a6` (`rendered_options` leaves the serve loop; steady/cold 6 → 0), LEAD 2 `1ed58b7` (decide max 36 → 28, deterministic ×3; record `c617805`); the remaining 28-alloc surface is BY DESIGN (the wire response, the receipt contract, caller bytes), pinned by `tests/serve_g4_alloc.rs` (28).

## 2026-10-06 — boundary: the fence's two post-split REDs fixed (found during the 020/022 pass; both pre-existing, not introduced by it)

Pre-existing moat references in `*.rs` (verified `git log -S`): `tests/serve_gates.rs:206` comment reworded token-free (the A6 production digest re-pinned `47fcd95c…`, comment-only delta, zero data rows) and `tests/serve_g4_alloc.rs:115` — the G4 winners-dir DEFAULT was the trained-specialists dir; re-pointed to public `artifacts/cache`, opt-in via `INSTINCT_WINNERS_DIR`, pin 28 reproduced through the new gate. Fence `--post-split` GREEN; serve_gates 24/24; lib 86/86; clippy `-D warnings` clean.

## 2026-10-05 — reflex issue 070 lead 2 adopted: bag serve path 83 → 42 allocs/decision (instinct `9ac84c8`)

reflex `e0c43c7` landed `eval_case_into` + `CaseEvalScratch` — one caller-owned frame refilled in place, taken out of the server around the receipt loop (take/replace); pin re-measured **83 → 42** (deterministic ×3; lineage 107 → 83 → 42). Byte-parity: serve_gates 24/24 full material; the remaining surface is dominated by the synth case build (`.issues/022_synth_case_build_alloc_lead.md`).

## 2026-10-05 — Issue 021 closed: the ESC cheap-path allocation surface — bag serve path 107 → 83 allocs/decision, composed ESC 190 → 161; every pick bit-identical

Ownership/scratch refactor (lead from riir-refine Issue 146 T0.6's hotpath profile): instrument born (`tests/serve_g4_alloc.rs`, baseline 107), `synth_served_case_into` + an allocation-free bridge pass; bag serve path **107 → 83**, composed ESC **190 → 166 → 161** (Rethink's pin re-sharpened with the re-measure disclosed). Remainder ~69 reflex-side filed as reflex Issue 070; wall-time value deliberately UNMEASURED (box-noise-dominated); every pick bit-identical.

## 2026-10-05 — riir-rethink Issue 023 T2 lockstep: the production manifest pin re-synced — xnli_en + emotion are benchmark-only

Rethink demoted the licence-barred pair (CC BY-NC / research-only data cannot back the paid lane); `PRODUCTION_MANIFEST_TOML` re-synced 9 → 7 rows byte-verbatim, digest re-pinned `blake3:e77a42ce008d36f0629176103e128c8652d2396353211734f3f684fabfba47d8`, face-6 asserts **ag_news is the SOLE ESC-armed row**; new gate `licence_barred_suites_carry_no_production_row` (re-source = `.issues/023`). Serve gates 24/24 with real data.

## 2026-10-05 — riir-rethink Issue 023 T3 lockstep: the ⚠ pair settled — the manifest is the five permissive rows; ESC zero-armed pinned

sst5 + ag_news also left the production manifest (SST's README carries no licence over scraped Rotten Tomatoes text; AG News is Gulli's no-grant 2004/05 corpus): `PRODUCTION_MANIFEST_TOML` 7 → 5 rows, digest `blake3:5e1ff4755a889e2474dcac78c31cd5530c04ffb900ef3fdc238e73951d30127e`. Replay-manifest parsing keeps the ag_news data-gated gates alive; face-6 became `escalate_grammar_pins_the_zero_armed_production_state` (NO escalate row without its own GOAT + the re-source); the licence gate pins all FOUR barred suites. Verified 24/24 with the REAL winners dir.

## 2026-10-05 — Issue 013 closed: pool re-baseline (a) EXECUTED — arena → canonical pool at the PINNED reflex baseline; standing triggers → Issue 019

Claude verdict GO(a): the arena's datasets default = canonical `../riir-reflex/.raw/datasets` at NAMED pin `REFLEX_BASELINE_SHA = 6365fab0261b2123ae0953a12b06ef0eec596c2b` (stamped in the startup banner + RESULTS.md); t20k stays the ARCHIVED pool (`--datasets-dir ../riir-reflex/.raw/datasets_t20k`), the SERVE lane untouched (t20k default stands). E2 divergence root-caused reflex-side (`f1371c1` + `7d725b2`, reflex `.issues/058` — pool × engine, structural); en-route compile repair `density_gate: false`; standing triggers → `.issues/019_standing_triggers.md`; `.issues/.highwater` 018 → 019.

## 2026-10-03 — B4 LANDED: the L1 training lane is in-tree (the carve's deferred riir-train half)

L1 trainers + teaching recipes moved from `../riir-train` into THIS repo (Plan 008 B4: `src/instinct_specialist*.rs`, `instinct_nbsvm`, `instinct_bank77_cases` + the six `examples/instinct_*` lanes); the teaching arc runs end-to-end from a clone; trainer output defaults `data/trained_specialists/` (the `data/instinct_specialists` literal is fence-banned). Boundary now "training scope": L1 CPU teaching in-tree; GPU/distill/self-evolve stay riir-train, encoder trainers in Rethink. lib 57→85, fence `--post-split` GREEN.

## 2026-10-03 — MOAT-LEAK AUDIT (post-reversal): fence GREEN on code; records arm ADDED (the GREEN said nothing about .benchmarks); r5_launch_ab.ps1 removed; B2 record corrected

Full public-tree sweep: fence GREEN on code, `sync_mirror.py --check` 13/13, zero moat code, zero artifact bytes. New fence direction R: `.benchmarks/` moat-class dirs RED unless on RECORDS_ALLOWED — the eleven deliberate stays BY NAME (0029/036–041/049/0051–0053; 0029 is a public test dependency — serve_gates READS its predictions.json); `scripts/r5_launch_ab.ps1` git rm'd (its recorded deferral was silently defeated by the force-push); B2 record corrected (0029 + 036–041 + 049 never moved); reflex BOUNDARY dep row fixed (`vessel_public_read` only).

## 2026-10-03 — the two-repo arrangement RETIRED (owner directive): single source of truth = gist-rs/riir-instinct (full history, public)

Owner: "git history leak is fine… i dont want 2 confusing repo i want single source of truth" — the full-history `develop` force-pushed to the public gist-rs/riir-instinct (`main` = same tip; the fresh-root export `382b6bd` superseded); this checkout's origin points at `gist-rs/riir-instinct.git` again. `gist-rs/riir-instinct-internal` DELETED-owner-side; UNCHANGED: the moat law binds the working TREE (`../riir-rethink`, fence `--post-split`); the 4090 box's remote must point at `gist-rs/riir-instinct.git`.

## 2026-10-03 — PHASE C LANDED: the public export is LIVE (gist-rs/riir-instinct, public, `382b6bd`); this repo renamed gist-rs/riir-instinct-internal

Owner green-lit in-band; plan 008 Phase C same day: the `.export/` build tree (41 [copy] rows verbatim minus `r5_launch_ab.ps1`), fresh README/AGENTS/BOUNDARY/HISTORY, MIT LICENSE, `scripts/setup_siblings.sh` (four public siblings — the fourth, `../riir-infer`, surfaced when the export build failed to resolve reflex's laya face), CI (fence + clippy/test). Gates at birth: clippy `-D` clean; lib 56/56 · serve_gates 18/18 · staleness 6/6 · calibrator 2/2 · tetris 1/1 · g4 1/1; live boot smoke over the public datasets; fence `--post-split` + `--history` GREEN on the PUSHED clone (single root `382b6bd`).

## 2026-10-03 — the Instinct/Rethink SPLIT CARVE landed: B1 seam + B2 move family + B3 in-repo registration; fence --post-split GREEN (riir-ai Proposal 052 / riir-instinct Plan 008)

B1 `22ae291` the SEAM: `server::LaneBackend` + `ExtBoots` + `install_ext_boots` + the `AnySuiteServer::Ext` seat; parity gates byte-identical (shared/per-lane fingerprints sst5 `2e89e9cb12f6e9ec`, xnli `b481fbdb7d039921`). B2 the MOVE family (issue closed `f262da8`): encoder/decstat src+test modules, `deploy.yaml`, the PRODUCTION `arsenal.toml`, the moat records (029–048, 0050², 0054–0056) and docs → `moat/`; features `serve-encoder*`/`decstat` deleted, `arena-laya` → `laya-face` renamed; deps `riir-kat`/`papaya` removed, `ed25519-dalek` dev-only; the teaching manifest `data/arsenal.toml` digest `1eb91da4…` pinned, the PRODUCTION verdict rides INLINE in `tests/serve_gates.rs` at `4c4356c6…` (the SAME digest the old embedded default carried); the bag vessel boot rewritten on reflexer's public decode (HOSTED-ONLY refuses at boot). B3: `BOUNDARY.md` rewritten, `moat/BOUNDARY.md` + `moat/README.md` filed, export manifest FINAL; workspace registration WAVE-GATED. Fence `--post-split` GREEN; `--history` RED here BY DESIGN (GREEN only on the export).

## 2026-10-03 — riir-rethink BORN (plan 008 B3's wave, the owner's in-band green light)

gist-rs/riir-rethink born at `e263686` (fresh root, 242 files), seeded verbatim from `moat/` at `4ffd08f`; the SEED-CARVE model (riir-refine shape) settled the plan's open pick — whole git history stays HERE, `moat/` a pointer; the seam needed ZERO open-side widenings. The visibility flip (Phase C) = the owner's personal act.

## 2026-10-02 — the six harness families RETIRED (owner call, reflex `31b11d2` + instinct this commit)

The owner removed the six home-made harness families from the reflex harness itself (at-chance at honest wide-eval populations; "so no one benches it anymore"); instinct-side the six artifact-less A0 rows left `arsenal.toml` (digest re-pinned `4c4356c6…`), the arena population 15 → 9, the family frozen-picks parity gate removed (the Bench 0051 record stays). A6 honored: the digest pin moved in the SAME commit as the TOML edit.

## 2026-10-02 — Issue 018 closed: the Rethink encoder lean+GOAT arc — the shared encode worker PROMOTED default, Q8 adopted end-to-end, Q4 measured out

Lane B shared encode worker `41234d4` (Bench 0050, RAM 4938→1690 MB = 2.92×; residue `eef77d1` — ENC rows skipped the bag-winner coupling in `load_lane`) PROMOTED default `e6e703b`: `serve-encoder` implies `serve-encoder-shared`, runtime demote `RIIR_INSTINCT_ENCODER_SHARED=0`; Lane C NEGATIVE `8a3b721`+`f3ae573` (typed head over the Q8 english encode doesn't carry). Lane D: **Q8_0 ADOPTED end-to-end** (Bench 0046: 14 paired reads, 5 PASS / 2 UNDECIDED / 0 FAIL; D2a Bench 0047, riir-infer `10e33de` — 842.6 → 447.7 MB = 53.1%, read-back bit-exact); **Q4 FAILED** (Bench 0055 `6f8ec72`: banking77 Δacc −10.0pt UB95 −0.1410, retention 0.568 — `LAYA_WEIGHTS_VARIANT=q4` may not serve anything; re-open bar = head-class change). Lane E never triggered; the katgpt-rs Research 562 two-case calibration branch stayed armed, unfired.

## 2026-10-02 — Bench 0051: the families wide-eval re-baseline — the serve-parity gate re-pinned on the template-disjoint populations

reflex `e78c0e6` (Plan 009 REVISED-2) widened the five families' evals to 96–100-case template-disjoint populations, invalidating Bench 049's n=12–16 serve-parity record BY CONSTRUCTION; this record re-measures A0 on the wide evals (all a0_stands) and re-pins `served_family_decisions_are_the_frozen_a0_picks`. Site pin leg stale by the eval swap — `--skip-pin-a0` with the reason (the Bench-016 precedent). Record `.benchmarks/0051_families_wide_eval/`.

## 2026-10-01 — Bench 041: the typed_decisions Rethink cell SEATED — the v2 arena replay reproduces 0.7550 EXACT (016 T9 complete)

instinct `73cd6d6` + reflex-site `245bf34` + CF `8abd1add`: `encoder_arm.rs` reads NLEH v2 (the law-copy t608 mirror; per-question flattening 5 q/case → 2000 rows, `--encoder-ckpt typed` parse-time validated); the arena replay reproduces the trainer's frozen read **0.7550 (1510/2000)** EXACT on BOTH runs (the third-posture witness); vs A1 mean +0.1250 · LB95 +0.1032. En-route: a PRE-EXISTING `laya_face` per-question index bug fixed (multi-question suites SKIP the face loudly, never mis-measure). Rethink lane 4/9.

## 2026-10-01 — Bench 039+040: the three unscreened encoder suites screened + the EARNED typed run — the NLEH v2 per-option head reads 0.7550

Screens (Bench 039): banking77 DEAD (floor 0.4980 + max lift 0.645 < 0.8540), prompt_injections DEAD (0.845 < 0.8534), typed ALIVE (reference floor 0.7445 = +9.7 pt over the 0.6475 bar); the three wire-fidelity witnesses byte-exact. The EARNED run (Bench 040; codec/trainer/eval in riir-train's `instinct_encoder_lane`/`instinct_typed_head_trainer`, `e118c29d` + `4a9e66cf`): the NLEH v2 per-option head holdout 0.7975 vs reference 0.7762 — EARN; the single frozen read **0.7550** (`../riir-train/.raw/t608/typed_encoder_v2.bin`, blake3 `c2e9f346…`).

## 2026-10-01 — Bench 038: the emotion confusion read — ±0.0 explained with mechanism, and every remaining lever priced (records-only)

From FROZEN artifacts (reproduces A0 0.8850 / A1 0.8550 / H1 0.8775 EXACTLY): emotion serves A0 — reflex's ridge@8 base, the board's highest bar; all 46 misses at conf < 0.30 (32/46 abstained). Synth lever REFUTED for emotion (0/15 love→joy recoverable — annotation softness, not coverage; massive's gap was coverage, emotion's is labels); fusion ceiling negative (the specialist is worse on A0's consult subset); verdict: 0.8850 is the pool-consistent ceiling, the only remaining mover a contextual class (the wave-1 re-open bar, oracle-max +3.7 pt); `.benchmarks/.highwater` repaired 0035→038.

## 2026-10-01 — Issue 017: the xnli_en encoder cell SEATED (record-only, serve ✗) — the owner's progress-display call reversing the 014-round-3 sub-decision

Owner call: the measured cell shows the product's real posture ("no progress at all and -4pt gimme some hope") — reverses 014-round-3's "no separate xnli cell"; NOT reversed: 014's class-wide serve refusal, the reference-identified disclosure (the head ties laya-english 300/300, riir-train 599 T5a). Bench 036: **ENC 0.8600 (258/300)** cell-identical third posture (head `xnli_en_encoder_v1.bin` blake3 `a1e2380b7c6451bb…`); vs A1 mean +0.4500 · LB95 +0.3832; p50 231,709 µs (the 021-cleared span). Site: lane-scoped `PUBLISH_BENCH_LANES=encoder`; Rethink 2/9.

## 2026-09-30 — Issue 015 closed: the canonical massive winner synced to the 4090 — the serve's A5/A9 pin validates and boots (the drift was box sync, not the pin)

Canonical `massive_intent_en_winner_v1.bin` (blake3 `7bc3ee385f81abc0f2a91e47f6dca08db37f9fb6cee6d48121b80e3f11378556`) scp'd into the 4090's `E:\git\riir-train\data\instinct_specialists\`; verified by the serve's own A5/A9 gate at clean HEAD `274d039` (no digest refusal) — the drift was box sync, not the pin. The sync-checklist idea NOT built (if the gap recurs, file against the serve boot, not the pin).

## 2026-09-30 — issue-file hygiene: 005 removed (every measurable face landed, refuted, or absorbed; the GOAT verdict is the serving posture)

005 (the Moka+PUCT hybrid POC) complete across every face: T1–T5 measured (Bench 001 v2 → the ALIGNED Bench 002); the GOAT verdict = the standing serving posture (massive H2 0.8267 T2-certified SERVING; typed H2 0.6475 via Benches 019+020; banking77 H1 G3 FAIL → A0; xnli A0); H3 CLOSED premise-refuted (typed questions independent-given-case — no tree for PUCT). Arsenal Proposal 001 CLOSED (Bench 003 `arsenal_budget_goat` 7/7); the serving law is the owner verdict `4cc4441`.

## 2026-09-30 — Issue 014 C1 EXECUTED: the sst5 encoder arm reads 0.5267 on the ARENA (T2-certified above A1, LB95 +0.0535) — published `serve: ✗`; en-route: the T8 name-first noul regression fixed

`src/encoder_arm.rs` + `--encoder-art` (features `arena-laya`/`arena-laya-metal`): the sealed NLEH v1 head `t6_s0.bin` (BLAKE3-verified) over a live M3-Metal laya-english encode; **ENC 0.5267 (316/600)** cell-identical THIRD posture; T2 vs A1 mean +0.1050 · LB95 +0.0535 CERTIFIED; latency p50 14,709 µs / p99 17,452 µs — `serve: ✗` stands. Site LIVE (reflex-site `b9b2ee1`). En-route: the T8 name-first noul regression fixed (`2d20397` dropped the presentation-width guard; restored on the pair_named branch, serve_gates 15/15).

## 2026-09-30 — Issue 008 T8 EXECUTED: the code_fixtures tie BROKEN (A1 0.5625 vs A0 0.3750, T2-certified) and the six harness families DROPPED; the serving path landed (S8 + the name-first noul law + the 9th manifest row)

Trainer floor 0.45 via riir-train's `--abs-floor`/`--tie-break` flags; minted `code_fixtures_nbsvm_v2.bin` (blake3 `264714b9e7518e33…`); the frozen read (`.benchmarks/028_code_fixtures_tie_break/`): **A1 0.5625 vs A0 0.3750 — T2 PASS** (LB95 +0.0021 at n=32; 2 µs vs 290 µs); still −6.3 under paw 0.6250 → unsold, untied. Serving landed same day: winner_bridge MultiQuestion + **S8** in AnySuiteServer + the name-first noul law + the 9th manifest row (instinct `0c35063`); the six families DROPPED (n=12–16 template-shared — a win would be unfalsifiable memorization).

## 2026-09-30 — Issue 008 T4's armed-posture caveat RESOLVED (records-only, no compute): emotion's board edge is posture-gap, not specialist value

From EXISTING records: the armed seat reads **A0 0.8850** == reflex Bench 057's armed ridge@8 cross-pool while A1 0.8550 loses — emotion's board edge is POSTURE-GAP, not specialist value; the serving posture was already honest (A0 serves). Repairs: the floor table's emotion row re-labeled `(seat)`, "8/15 ahead" corrected to 6 with an arm; a bag sweep is the closed class per the wave-1 law (only the encoder class could beat 0.8850).

## 2026-09-30 — Issue 009 round 5 closed on both forms: the teacher A/B (Bench 026) measured NO-GO beside the residual pilot's null (Bench 027); the lane stops, eval seeds still at 5 reads

Teacher z-blend A/B (Bench 026, component `5c7eb6ca4ae3bb3e` = Bench 025's attempt-2 rebuild): **NO-GO** per the pre-registered rule (16@75 lb95 −119.7; 18@75 leans the other way; 20@80 DEGENERATE — both arms 0 pieces on all 20 seeds); residual pilot null (Bench 027). Round-6 label-blend ANALYTICALLY CLOSED: self-distillation fixed point + a win could not ship (the r4 config ≈3.76 ms = 3.4–3.8× over the 1 ms serve bar). T7 out of priced levers (capacity 010/018/021, loss 017, teacher 026+027, label blend); the issue stays OPEN owner-visible; eval seeds at 5 reads.

## 2026-09-29 — the instinct flow figure reflowed to two bands (owner ask: too wide, rendered small)

The source mermaid in `.docs/03_decision_flow/instinct_flow.md` reflowed to the Reflex hero's flywheel shape (top band "the question in — modelless, always first, free"; bottom band "on abstain — the trained add-on, paid only here") and re-rendered to both mirrors by reflex-site's `scripts/render_tetris_flows.py` (viewBox 2304×574 → 1751×730); smokes PASS.

## 2026-09-29 — Issue 008 T7 wave 1 (owner GO): six vs-best specialist lanes measured — every verdict a NEGATIVE; sst5's holdout win refused at the frozen read (Bench 024); Issue 005 H3 closed by a measured premise refutation; Issue 009's critic-guided-search lever re-derived GO-as-teacher

All six lanes NEGATIVE: xnli LB95 −0.0045; massive +6.5 lift (LB95 +0.0316) still misses the +9.3 gap; ag_news at ceiling; sst5 holdout PASS (+3.10 pt, `sst5_nbsvm_v2.bin` blake3 `633a3a67ed9a6987…`) REFUSED at the frozen read (Bench 024: A1 0.4217 == v1, LB95 −0.0151, the gliner 0.4383 bar unmet); code_fixtures' flat-union collapse found+fixed, still 0.5536 < 0.72; typed rides 581. Issue 005 H3 CLOSED premise-refuted (independent-given-case — no PUCT tree); Issue 009's critic-guided search re-derived GO-as-teacher; re-opens ride a CLASS upgrade, never another bag sweep.

## 2026-10-02 — the m3 serve plane repaired: the typed t20k pool + the code_fixtures winner recovered byte-exact

typed t20k pool synced to the full 1200-row pull from `riir-train/.raw/datasets_typed_full` (the 960-row slice guard, reflex Issue 058, refused the stale 800-row envelope; winner blake3 `7f7a39e1…` verified); `code_fixtures_nbsvm_v2.bin` recovered byte-exact from the 4090 (a re-mint reproduced the numbers but drifted bytes — determinism is within-binary, never cross-build; blake3 == the arsenal pin `264714b9e7518e33…`). `deploy.yaml` gained the 9th winner row; no product code changed.

## 2026-09-29 — issue-file hygiene: 010 removed (every measurable task landed; the residual is an owner decision, moved to 013)

All six tasks landed 09-27/28: Bench 005 (`run_suite_a0_only`, the three-state lane doc), the 8/8 A0 pin (the oc-armed typed catch), Bench 011, the Bench-016 `harness_cache_reuse` seat, and the SITE half reflex-site `74b49e4` (15 hybrid rows live, smokes PASS). Residual = the cross-lane pool/engine divergence — an OWNER decision recorded in `.issues/013_owner_gate_pickup.md`; `--skip-pin-a0` stays the documented posture until the owner picks.

## 2026-09-28 — the 581 lane lands end to end: typed_decisions seats, certifies, and SERVES (Bench 015 + Issue 011)

Root cause (riir-train `d59d3984`, Bench 614): Bench 014's "template drift" was a FETCH-CAP ARTIFACT (800 of 1200 train rows); retrained at 610's config over 1200 rows / 49 classes (`typed_decisions_armA_v1.bin` blake3 `7f7a39e1935f8665…`); reflex's cap fix filed (riir-reflex Issue 052 — a +10.7 pt MODELESS A0 gain for their lane). Bench 015 `745d157`: A0′ 0.5725 · A1 0.6300 (T2-certified against BOTH A0 legs, G1 PASS); serving `e599ecc`: `SuiteServer::decide_multi` + the ServeContract axis + the 8th manifest row (serve_gates 14/14, the typed parity replay 12×5 questions); Issue 011 removed.

## 2026-09-28 — issue-file hygiene: 001 / 002 / 004 removed (work verifiably landed, records durable)

001 HOSTED-ONLY vessel reader (T1–T5, `src/vessel.rs` + `tests/vessel_gates.rs` 7/7, opt-in feature `vessel`), 002 hosted lane via riir-deployer (T1–T4 `deploy.yaml` cf-container, zigbuild x86_64, local e2e verified end to end; the real CF push owner-adjacent BY DESIGN), 004 decstat flywheel (T1–T3 wire `9da822b` → capture `3a1eb09` → store/route `c285435`, live to devnet epoch 2960; T4 tracked at riir-train Issue 577) — all landed; durable records in AGENTS.md.

## 2026-09-28 — Issue 579 T3: the arena bridge — banking77 serves the nbsvm v2 winner (Bench 012)

`specialist::winner_bridge` (the ONE home for the per-suite winner file + bag convention) + `presence_bag_into` + `BagConvention::bag_into` at every bag site + `check_winner_file` (a raw boot/swap naming another artifact for a bridged suite refuses loud); banking77 → `banking77_nbsvm_v2.bin` (Bench 612's mint, byte-verified) over L2-normalized PRESENCE bags. Frozen read n=500 (`.benchmarks/012_banking77_nbsvm_v2_bridge/`): A0 0.8260 · A1 0.8280 · H1 0.8320 · **H2(β=2,nmin=8,τ=8) 0.8540** — T2 REFUSED (LB95 −0.0013) but SERVES under the best-measured law; a banking77 HOSTED-ONLY vessel must be re-minted from v2 by riir-train first.

## 2026-09-27 — Proposal 001 T8: `arsenal_budget_goat` GOAT PASS — and the gate caught the unsigned-fold defect (Bench 003)

`benches/arsenal_budget_goat.rs` (opt-in `arsenal_goat`, refuses exit 1 without data) ran **7/7 GOAT PASS** (`.benchmarks/003_arsenal_budget_goat/`); the gate caught the UNSIGNED-fold defect — token-hash histograms are bucket-uniform, so every real centroid was colinear and admits read 0/6 (unit tests fed only synthetic orthogonal vectors). Fix same landing: `corpus_centroid` = SIGNED simhash fold (`bucket.wrapping_mul(0x9E37_79B1)` token-level signs) — 600/600 admits; the centroid feeds ONLY the hoarding gate, never a served decision. The bench is the standing re-run gate for any centroid/gate/manifest change.

## 2026-09-27 — Issue 007 closed: the missing builder self-test landed, and the committed Bench-002 lane doc predates the scope field

T4's builder self-test did not exist — landed as `scripts/build_hybrid_doc.py --self-test` (5 known-answer fixtures: scope inference both directions, the A0 skip, replicated metrics vs hand-computed values); the committed Bench-002 lane doc was generated by the PRE-T2 builder — regenerated in place with the scope fields, provenance preserved (`--git-sha 0959928`, the original arena timestamp).

## 2026-09-27 — Bench 002: the aligned Bench-052-protocol read + the reflex-site hybrid lane publish (Issue 003 T4 closed)

The arena re-ran at the **Bench-052 protocol on the same `datasets_t20k` bytes** (977/977 byte-verified); the comparability proof = **A0 == the published 052 modelless rows 6/6** + the in-run xnli drift pin. Registered arms: ag_news H2(0.25,2,2) 0.8975, emotion A1 0.8550, sst5 A1 0.4217, massive H2(1,2,8) 0.8267, banking77 H1 0.8060 (G3 FAIL), xnli A0. The t20k massive split lacks `cooking_query` → the cal-front gap gained the NaN-no-evidence extension (`h2_nan_evidence_mutes_the_margin_and_is_never_a_rival`); the reflex-site hybrid lane published (`scripts/build_hybrid_doc.py`, the publish_bench hybrid lane class, site tests 22/22).

## 2026-09-27 — Issue 006 RESOLVED: the arena's position-vs-index instrument defect (Bench 001 v2)

Fix `3a12a70`: the v1 massive anomaly was the ARENA — (1) space mismatch (scoring/picking in label space vs presented-option gold; only massive's 20-of-59 sampled options could expose it) and (2) double-indirect `cands[rank0_sorted[select_arm(..)]]`; `h1_decide` takes a per-case `class_of_pos` bridge, pinned `h1_scores_the_class_the_position_denotes`. Bench 001 v2 supersedes v1 (massive H2 0.8300 vs A0 0.4200; banking77 A1 0.7960; ag_news H2 0.9000; emotion/sst5 A1; xnli A0).

## 2026-09-27 — Bench 001: the hybrid GOAT run (P3 + P3a through the single test read)

First end-to-end hybrid measurement: the one-way seat seam lives in REFLEX (`harness::runner::seat` — reflex consuming instinct would be a cycle); the A0 drift pin (xnli 0.5167 byte-exact) proves the seat path every run. ⚠ SUPERSEDED same-day by Issue 006 — the massive chance reading and three A0 registrations were the instrument's position-vs-index defect; read Bench 001 v2 for corrected numbers. Instrument lessons (calibrator `apply()` identity until `refit()`, G5 must refuse on A0/A1 arms, black_box liveness, the seat⊆artifact join) → Lessons.

## 2026-09-27 — Bench 004: the re-baseline + the strict-superiority product gate (Issue 008 T1+T2)

T1: seat posture = the CURRENT PUBLISHED reflex posture (`head+nb+ridge` select; reflex Issue 050 `7a99431` — `nb_ridge`+`option_cond`, the latter because `nb_ridge` alone does not compile upstream); the A0 drift pin widened to all six suites + the site pin (6/6 + site ✓ at reflex `ad45067` and `7b5f0ba`). T2: registration REFUSES an arm not STRICTLY above the Reflex row (paired LB95 > 0) — registered == served. Verdicts: massive H2(1,2,8) 0.8267 LB95 +0.0124 the ONE certified arm; ag_news/sst5/banking77 REFUSED; emotion's ridge@8 base outruns A1 (0.8850 vs 0.8550) → A0. The abstention contract found (served-abstention == recorded-abstention, pinned); readiness ceilings 120 → 420 s.

## 2026-09-26 — birth

Created per riir-ai Proposal 047 (owner decision: the model-based/hybrid lane lives in a PRIVATE repo — hybrid wiring and weights are product moat; `riir-reflex` is public). Trainer: riir-train Issue 576. Motivation: riir-reflex Issue 038 / Bench 051 — the modelless lane plateaus below laya on xnli / typed_decisions / ag_news.

## Lessons

- A pick/gold pair is a SPACE contract — when a seat samples its option set, every scorer must speak the presented-option space (Bench 001 v1).
- Imitation top-1 agreement ≠ play strength; the disagreement concentrates in catastrophic placements (Tetris T7) — a re-open needs a tail-targeted loss or critic-guided search.
- Quantized-artifact determinism is within-binary, never cross-build — the katgpt-core path-dep tree moves bytes (the code_fixtures re-mint).
- Single-probe a new regime's playability before allocating an eval cell (the 20@80 DEGENERATE hard cell, Bench 026).
- Gate infrastructure with real-data benches, not fixture-only units — synthetic orthogonal vectors hid the unsigned-fold defect (Bench 003).
- The sigmoid-gate calibrator's `apply()` is the identity until `refit()`; `black_box` is the liveness guarantee; the label join's contract is seat⊆artifact (Bench 001).
- Quote `box_state.json` / the preflight PROVENANCE line beside any latency figure (the reflex Issue-021 law); read a re-baselined pool's rows as a NEW series, never comparable with archived-pool rows.
- Re-opens ride a model-CLASS upgrade, never another bag sweep (the wave-1 law).
