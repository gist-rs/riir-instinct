# Issue 016 — the encoder class on GPU hosts: L3 think-depth seating via decision_wire (the promote-on-a-new-surface path)

**Status:** OPEN — filed 2026-09-30 (owner premise: "the prod is M5 ultra metal so we do
have gpu plan"). Carries the layer-fit ruling + the staged promotion path. The 014
serve-posture gate is NOT re-asked for the CF text lane; this issue is the different
decision surface the 014 close left open by construction (the extraction substrate
"stands reusable"). **Verdict round 1 (claude, 2026-09-30): REVISE — all seven reasons
incorporated** (arithmetic corrected to 80–1800 ms GPU/s + the admission cap made
required; resident-from-boot per L9 "never loads"; the sync-boundary bridge law; wire
RTT counted; M5-unmeasured premise; event-driven named consumers; D1 reframed as a
trigger; 014-still-governs-CPU line). **Verdict round 2: AGREE — ruling RATIFIED; the
four non-blocking edits folded (`a136ff6`: deadline fallback + G1b; T1 demand model =
event-arrival × admission + the G2 unit derived; certification scoped to the
utterance→sentiment/intent consumer; measured-vs-cited table wording). T1–T7 open** (T7 = the
queued emotion screen + the per-suite run-state record, added 2026-09-30 on the owner ask).
**Owner round 3 (2026-09-30): three clarifications folded — the no-new-product-lane note
(nothing named "Working"/"riir-working"; L0–L5 are 048's tier vocabulary), D1's host
generalized to ANY real GPU serving deploy (game prod OR text-lane serve host — the
board consumer's premise), and the ladder-monotonicity note (each rung strictly-additive
via deadline fallback; L2 skipped by law; L4 untasked because 048 deferred L4/L5, Plasma
reserved as the L4 salience-depth rung).**
**T8 added 2026-09-30 (owner round 4): the retrieval-gated corpus ladder — V2
same-space retrieval-distill with OOF teacher labels (V1 refused), consolidation moved
to L5 + L4-swap with the two invariants, and the full POC pre-registration ratified
(the n=600 identical read, paired G1 vs C2, multiplicity-adjusted G2, full-row G3,
alloc-0 G4, one sealed bundle; SIMD boundary honest — wasm-simd128 port-class,
unmeasured). Runs on the owner go. T1–T8 open.**

## The premise change

014's refusal grounds were MEASURED against ONE deploy shape: the CPU-only
standard-2 cf-container (x86-64 zigbuild) of the reflex.gist.rs text lane —
"no GPU tier exists in the deploy shape". The game-stack prod target is an
**M5 Ultra / Apple-Silicon Metal host** — a deploy shape where the encoder's
measured cost class is affordable by a wide margin, and where the serving
binary (plain std HTTP, no CF binding) runs natively.

## The layer-fit ruling (grounded in Proposal 048's authority map)

| Layer | Cadence | Per-slot budget | Encoder cost class |
|---|---|---|---|
| L0/L1 Reflex + basic instinct | 20 Hz | 50 ms | ✗ fixed by tier |
| L2 Surface | 5–10 Hz | 100–200 ms | ✗ for the LIVE encoder (lookup-only by law) — **but T8's retrieval surrogate is the L2-SHAPED encoder-class rung** (KNN over a frozen STATIC-embed index carrying frozen OOF head distributions = a lookup, not a think) |
| **L3 Working** | **1–2 Hz** | **500–1000 ms** | **✓ 1–7 % of slot (36 ms/row MEASURED on a loaded M3 + 4090 parity; the 8–15 ms GPU-posture figure is CITED, not measured on our boxes — see the premise note below); the "think depth per decision" knob IS this op** |
| L4 Critical | 0.2–0.5 Hz | 2–5 s | ✓ batch encode of salient sets |

The fit is architectural, not just numeric: limelight's function is deciding
WHO gets deep cognition; the encoder class is the deep-cognition op spent on
the salient few. **The arithmetic, honestly:** 10–50 salience-admitted
encodes/s × 8–36 ms/row = **80–1800 ms GPU/s** — the high end (50/s at the
loaded-M3 36 ms figure) exceeds a whole GPU, so the salience admission cap
is a REQUIRED control, not an option: the cap is derived from the measured
per-row cost on the serving host (T1), with window-batching amortizing the
per-row figure. At the 8–15 ms GPU-posture class, a 10–20/s cap lands at
80–300 ms GPU/s per 1000 NPCs — inside a cognition budget with headroom.
**Every figure is M3-Max-Metal or 4090 measured; the M5 Ultra is
UNMEASURED** — the M3 figure is the stand-in premise until the host exists
(T1 re-measures on arrival).

**The ladder is monotone-by-construction, not guaranteed-incremental**
(the "better result each step" question, answered): each rung up is
strictly-additive at the system level — the deadline fallback answers from
the equipped class below when the deeper op misses, so ACCURACY never
regresses; the cost axis is latency/GPU-budget, never correctness. The
measured step-ups: reflex-floor → bag arm (banking77 +45.2, massive +42.0,
sst5 +22.0 over A0) and bag → encoder think (sst5 +10.5 over A1). It is
NOT a guaranteed increment per suite — the class-relative law caps it
(at-ceiling suites gain nothing: ag_news). **L2 is skipped by law, not by
choice**: L2 is lookup-only (bark cost); the encoder is a think op; the
cheap class that IS the L1/L2 equipped set is the bag class. **L4 carries
no task here because 048 deferred L4/L5 shipping** (L9's interim: boot or
operator swaps only) — and 048 already reserves **Plasma as the L4
salience-depth rung**, so the natural L4 form of this class is batch
encode for Plasma-depth entities on the 2–5 s critical slots, tasked when
048's L4 ships. L4/L5 are cadence/authority layers (swap policy,
curation), not deeper thinkers — "better each step" holds through L3 and
then changes axis.

## The seam (048's own laws — no new architecture)

- **L8**: layers are game-side; serving lanes serve decisions over the wire.
  The game side reaches the encoder class through `decision_wire` + a
  feature-gated thin client — **consumer #3, the pattern after
  `riir-agents/decision_gates.rs`** (consumer #2). No cargo dep either
  direction; riir-ai's L3 call sites never link instinct.
- **L9**: L3 ROUTES among equipped vessels (ns–µs — the routing, never the
  think; the think-depth op is the per-decision budget spend the cognition
  budget arbitrates). **"Never loads" binds:** under L9's interim rule the
  encoder weights load and stay RESIDENT FROM BOOT on the GPU serve lane —
  a lazy first-route load would be L3 doing a load, which L9 forbids; swap
  policy stays L4/L5 game-side; the monotonic swap mechanism stays
  arsenal-side (`arsenal_ops.rs`, landed).
- **The sync boundary (the latent/raw bridge law)**: Metal and CUDA encoder
  outputs agree on the argmax, NOT bit-for-bit — so only the CLASS/scalar
  result may cross the wire and the sync boundary. The authority computes
  ONCE, records the decision, and deterministic replay reads that record;
  the embedding never crosses.
- **A1/A9 (arsenal laws)**: the trained NLEH head is a HOSTED-ONLY vessel
  (our trained IP; minted by riir-train, never runtime-minted); the laya
  encoder weights are the RUNTIME (the bag-count runtime's analogue — the
  serve host's capability, not a vessel payload).
- **L10**: no new katgpt-rs surface.

## What this issue is NOT

- NOT a new product lane: **nothing is named "Working" / "riir-working"** —
  L0–L5 are 048's internal tier vocabulary (the cognition-budget authority
  map), and any product surface born of this seats under the existing
  instinct/reflex family. "L3 Working" is a CADENCE LABEL (the 1–2 Hz think
  tier), not a brand.

- NOT a re-ask of 014's text-lane gate: the reflex.gist.rs lane keeps its
  µs-class CF serve and its board rows unchanged; the `Instinct (encoder)`
  site lane keeps `serve: ✗` unless the D1 trigger below fires. **014's
  refusal still governs EVERY CPU-only deploy shape, including CPU-only
  game hosts** — the ruling here widens the surface to GPU hosts only.
- NOT a flip of the 008 board by side effect: board rows move only through
  their own measured + published lanes.

## D1 — a TRIGGER, not a flat owner question (verdict round 1)

The board's comparison lanes already serve from their own GPU hosts
(openthai @4090-win, gliner @4090-win, paw hosted) — but those are
COMPARISON hosts, not precedent: Instinct's row claims OUR deploy shape,
and publishing `serve: ✓ @m5-metal` before such a host exists would be a
false claim, whatever the premise's energy (the cell would read
`serve: ✓ @<gpu-host>`, never a hardcoded box name). So — generalized
2026-09-30 on
the owner ask, faithful to the ratified reasoning ("Instinct's row claims
OUR deploy shape" — a text-lane GPU serve host IS our deploy shape):

**D1 flips to YES automatically when ALL of these hold:** (1) a GPU host
(Metal or CUDA) in a REAL serving deploy — the game prod host OR a
dedicated text-lane serve host — actually serves the encoder; (2) per-row
latency measured ON THAT BOX; (3) the artifact pin validates on that box. The T2
certification already exists (LB95 +0.0535), so nothing else is required —
the sst5 cell publishes as served `Instinct (encoder) @<host>` (0.5267 vs
the 0.4383 bar = +8.8) and 008's sst5 row flips through its own measured
lane. Until the trigger fires: the layer seating proceeds; the board
posture does not.

**(Owner round 3, 2026-09-30 — three clarifications folded:** the no-new-
product-lane note — nothing is named "Working"/"riir-working", L0–L5 are
048's tier vocabulary; D1's host generalized from "the game prod Metal
host" to ANY real GPU serving deploy — game prod OR text-lane serve host,
the board consumer's premise; the ladder-monotonicity note — each rung
strictly-additive via deadline fallback, L2 skipped by law (lookup-only),
L4 untasked because 048 deferred L4/L5 with Plasma reserved as the L4
salience-depth rung.**)**

## Staged plan (pending verdict round)

- [ ] T1 — **budget pricing (measured, no new lane)**: the demand model is
  **EVENT-ARRIVAL × ADMISSION**, not NPC population — the named consumers
  fire when someone speaks / a task or query arrives, so load scales with
  utterance/task arrival rate × the salience-admitted fraction. T1 derives
  from the measured per-row cost on the serving host: the admission cap,
  and **the G2 unit itself (GPU-seconds per second)** — limelight allocates
  depth TIERS today, not a GPU budget, so T1 derives the number G2 compares
  against. The WIRE RTT is counted inside the 500–1000 ms slot. Uses the
  existing `dump_encoder_states` / `instinct_encoder_eval` harnesses; no
  split reads. The M3-Max figure is the stand-in until the M5 host exists;
  the number is re-measured on arrival.
- [ ] T2 — **serve-side encoder lane (feature-gated)**: wire the landed
  arena reader (`src/encoder_arm.rs`, `arena-laya-metal`) into the serve
  path behind an opt-in feature (GPU hosts only; the default CF-shaped
  build compiles it to nothing). Gate: serve parity replays the frozen
  Bench-029 picks.
- [ ] T3 — **the decision_wire thin client (consumer #3, riir-ai side)**:
  the feature-gated client after the `riir-agents/decision_gates.rs`
  pattern; L3 call sites route encoder-class questions only for
  limelight-salient entities (the depth knob). **Deadline semantics:**
  every call carries a per-call deadline; on expiry — a down serve lane,
  or an encode + RTT overrunning the L3 slot — the decision falls back to
  the equipped bag class and the record notes WHICH class answered (a
  late reply must never stall the decision nor be applied to a later
  tick — that would break compute-once-and-record). SUBSTRATE-FIRST before
  any new System impl (the 048 T7 outbound-boundary row rides the same
  change).
- [ ] T4 — **arsenal vessel mint for the NLEH head** (riir-train,
  HOSTED-ONLY class) + the monotonic apply the arsenal already ships.
- [ ] T5 — **GOAT gate**: G1 pick-parity vs the frozen reads; **G1b a
  deadline-miss injection produces the bag decision, recorded as such**;
  G2 the budget share in T1's derived unit (GPU-s/s at the measured
  arrival rate ≤ the admission-capped allocation); G3 no-regression on
  the bag lanes (byte-identical when the feature is off); G4 alloc-free
  outside the encode call.
- [ ] T6 — D1 executes when the trigger fires (a real GPU serving deploy —
  game prod host or text-lane serve host — serving the encoder, latency
  measured on that box, pin validating) — the
  sst5 cell publishes as served and 008's row flips through its own lane.
- [ ] T7 — **the emotion screen (the ONE unscreened board-relevant encoder
  run; QUEUED, not run)** — a 008-track text-lane run of the same class,
  not a layer-seating task. Pre-registered per the 014 law BEFORE any run:
  the suite-generic trainer (`instinct_encoder_trainer`) over the emotion
  t20k pool, ONE extraction method (the NLEH class, same architecture,
  FRESH fit — never reused head weights), 5-head pool-side sweep,
  best-holdout selection, **ONE frozen read** on the emotion test split.
  Earn gate: the head must read **> 0.8850** — the armed-seat A0, T4's
  recorded bar for a REAL emotion arm; below it the posture-gap state
  stands and emotion stays A0-served. Cost ≈ 20–30 min GPU (M3-Metal
  stand-in or 4090), box state recorded beside the number. **On a PASS the
  arm is RECORD-ONLY (`serve: ✗` — the 014 class refusal still governs
  the CF lane): its value is securing the fragile emotion row (a
  posture-gap tie-in-waiting: a reflex republish at the armed posture
  collapses today's +11.5 to a tie) with a genuine arm for the
  D1-triggered posture. The board row itself does not move — the site's
  verdict bars exclude serve-refused arms by design. On a MISS: recorded
  negative, no serve change, no re-run (the read is spent).**
- [ ] T8 — **the retrieval-gated corpus ladder (DESIGN CANDIDATE — owner
  proposal 2026-09-30; PRE-REGISTRATION RATIFIED by verdict 2026-09-30,
  ready to run on the owner go)**: the pattern transfer from the
  MTCS/LEO/PUCT + quest_grammar lineage — frozen corpus = the vessel,
  live encode = the prior,
  head = the fusion. Instinct already does the CORPUS half (offline
  `dump_encoder_states` pools → frozen NLEH heads; the bag arms ARE
  corpus-is-the-model); the irreducible live half is the QUERY encode
  (one row: 8–36 ms GPU / ~157 ms CPU). The candidate adds the retrieval
  gate between them:
  - **L2 rung (V2, same-space retrieval-distill — verdict-ratified; V1
    REFUSED)**: KNN runs entirely in the STATIC space (query static embed
    vs the train pool's static embeds — same space by construction; no
    cross-space projection, the axis C2 measured as lost), and the
    encoder's knowledge enters through the neighbors' **FROZEN
    OUT-OF-FOLD head distributions** — K heads fit on K−1 folds of the
    pool, each held fold labelled by the heads that never saw it (fold
    seeds + head hashes sealed in the sidecar). In-sample teacher labels
    are FORBIDDEN — the certified `t6_s0` head was fit on this pool, so
    its in-sample predictions ≈ gold and the experiment would collapse
    to gold-label kNN, certifying nothing. Runtime = static KNN (SIMD
    matvec over an 8544×256 i8 index ≈ 2.2 MB + ~170 KB soft labels,
    single-digit µs) → similarity-weighted vote of the neighbors' OOF
    distributions → fixed monotone calibrator. The only CF-tier
    encoder-class rung — deployability UNMEASURED (wasm-simd128
    port-class; no wasm32 measurement of this path exists).
  - **Admission gate**: high corpus-similarity → L2 answer; OOD or
    salient → L3 live encode (GPU); CPU-only hosts stop at L2. The gate
    threshold is selected POOL-SIDE and its **gated accuracy + test
    coverage** at that threshold are declared IN ADVANCE as the read's
    secondary result — the partial-coverage rung is the real value case.
  - **L5 consolidation + L4 swap (verdict-corrected from "L4 rung" — a
    nightly batch job runs at ≪0.1 Hz, which is L5 curation)**: nightly/
    downtime batch encode of admitted utterances into a NEW sealed store
    version (encode+admit+seal = L5); the ATOMIC SWAP of the version is
    the L4 half (swap-policy, freeze/thaw), gated by a pool-side
    regression check vs the incumbent — **a certified number never
    carries over; every new store version is uncertified until
    re-gated.** Two invariants: (a) admitted utterances DISJOINT from
    every eval split (hash membership in the BLAKE3 lineage); (b)
    head-labelled admissions NEVER become training data for a future
    head (self-distillation drift) — pseudo-labels are for the retrieval
    store only.
  - **Run-time notes (verdict round 2, fold at execution):** (a) after
    the first consolidation the store mixes teacher provenance — pool
    rows carry K-fold head labels while new admissions carry a single
    never-saw-them head — so record a TEACHER HASH PER ROW in the
    sidecar, letting the regression check tell the two apart; (b)
    "train-side slice" means a subset of the pool DISJOINT from the
    OOF-labelling folds, so selection never tunes on rows whose teacher
    labels depended on the held fold; (c) **EXECUTION ORDER (closing
    round): the OOF labels are produced BEFORE k, the calibrator, and
    the admission threshold are selected** — selection running first,
    even on the disjoint slice, would be tuning against labels that are
    not yet final.
  - **The SIMD boundary, honest**: wasm-simd128 is a PORT-class lane
    (≈ SSE/NEON class via the gemm/KNN kernels), NOT a 540× multiplier —
    the full f32 encoder forward stays out of CF reach (~157 ms →
    100–200 ms class); the RETRIEVAL surrogate is the CF-shaped rung.
  - **Boundary + the stated prior**: UNMEASURED. C2's pure-static negative
    (0.3917) is the floor; **the honest prior is that V2 lands NEAR C2
    (≈0.38–0.43)** — kNN and C2's MLP estimate over the same static
    features, and 256-d kNN over 8.5k points carries the curse of
    dimensionality; the only real lever is soft-target smoothing from the
    OOF labels. A miss is an expected outcome, not a surprise.
  - **The ratified POC pre-registration** (supersedes "gates if ever run"):
    sst5, **the IDENTICAL n=600 rows as Bench 029's C1 read** (exact
    pairing vs A1 0.4217 and C2 0.3917; this is the split's THIRD read,
    disclosed). Offline (GPU, pool-side only): static-embed the pool
    (PCA-256 + i8), train K OOF heads, freeze per-row OOF distributions;
    leave-one-out KNN in all pool-side selection (a query never retrieves
    itself); k ∈ {4,8,16,32,64} + calibrator + gate threshold selected on
    a train-side slice (pool-side holdouts read ~10 pt inflated —
    SELECTION ONLY, never forecast); pool-side attribution diagnostics
    reported free (gold-kNN vs OOF-teacher-kNN vs C2 on the same holdout
    — separates the retrieval effect from the distillation effect).
    ONE frozen read. **G1: paired LB95 > 0 vs C2** (a point-only beat at
    n=600, SE ≈ 2 pt, is noise → "inconclusive, recorded negative for
    value"); **G2: multiplicity-adjusted** LB at 1−0.05/3 (≈LB98.3) vs
    A1 (or the uncorrected LB95 disclosed as uncorrected); **G3:
    FULL-ROW latency** — tokenize → static embed → PCA-256 → i8 quantize
    → KNN → top-k → vote → calibrate, p50/p99 + box state, measured on
    the SAME i8 index that would deploy (an f32 stand-in is not a
    measurement; a wasm32+simd128 number is owed before any CF claim);
    **G4: alloc count 0 per query after warm-up** (pre-allocated top-k
    heap + vote buffer as scratch). **One sealed bundle** in the BLAKE3
    sidecar: index, OOF soft-label table, PCA matrix, quant scales,
    calibrator, fold-head hashes, pool/test hash membership. Failure
    path: recorded negative, T8 stays a candidate, no re-run. Substrate
    to REUSE, not rebuild (substrate-first before coding):
    riir-neuron-db `dense_embed` (the retrieval index) + the i8 dot
    kernels in katgpt-moka-wasm `moka_int8.rs` / katgpt-types' int8 dot
    path — check BOUNDARY.md for dep legality (a sealed data artifact
    over the wire is L8-lawful either way).

## Scope notes

- The CPU fallback never routes the encoder (L3 falls back to the
  equipped bag/static class — the arsenal's equipped-set-per-host rule).
- Consumers named for the L3 depth op — **event-driven, not per-tick** (the
  rung needs a caller, and its inputs are texts): a player/GM utterance
  going to NPC sentiment or intent (fires when someone speaks — the
  quest_grammar corpus class), riir-agents task-reasoning encoding, RAG
  query encoding on the neuron-db read path. Salience gates ADMISSION
  (which utterances/tasks pay the encode), never the tick loop.
  **Certification scope (verdict round 2): the measured +0.0535 LB95
  evidence covers the UTTERANCE→SENTIMENT/INTENT consumer ONLY** (it IS
  the sst5 read). riir-agents task encoding and RAG query encoding are
  CANDIDATES, each needing its own G1 against its own frozen read —
  "encoder class seated" must never be read as "every encoder use
  certified". The RAG consumer is also NOT a game-layer L3 surface (it
  is a read-path caller that reuses the same serve lane).
- The 4090 CUDA posture (`laya-riir-cuda`) serves the same lane on that
  host class if ever needed (Issue 008 T7's riir-train 599/600 encoder
  artifacts are host-portable — the 014 M3-replay proved box-independence).
- **One serve lane, many consumers — the "game-only" reading is the
  TITLE's framing, not the scope.** The encoder serve binary is
  consumer-agnostic: the game layer (decision_wire consumer #3) is the
  CERTIFIED consumer; the text-lane cells (D1, any real GPU serving
  deploy) are the BOARD consumer; riir-agents task encoding and neuron-db
  RAG query encoding are recorded candidates (own-G1 each). The board's
  beaten rows are NOT abandoned to the game lane — they are MEASURED-dead
  for this class (the per-suite state below): pointing the encoder at
  xnli/massive/ag_news/code_fixtures again would not beat those bars on
  the recorded evidence (wave-2/3). What the board rows are waiting on is
  the D1 trigger, not a re-run.
- **The per-suite encoder-run state** (recorded so the site's `Instinct
  (encoder) — not run` cells are never misread as backlog — five of the
  eleven are measured or law-refuted NEGATIVES; "not run" ≠ "expected
  win"):
  - sst5 — DONE: 0.5267, +8.8 over the gliner bar (Bench 029,
    T2-certified; record-only).
  - emotion — UNSCREENED: the one worthwhile run (T7 above).
  - ag_news — screened NEGATIVE: the 0.9500 bar IS the laya reference at
    ceiling; the head adds nothing at ceiling (the class-relative law).
  - massive_intent_en — screened NEGATIVE: the head LOSES 4.9 pt to its
    own reference on test.
  - code_fixtures — NEGATIVE-by-law: the reference sits 26.7 pt under the
    paw bar vs the measured max head-lift of +14.7.
  - xnli_en — measured (wave-2): 0.8600 test < the 0.9000 openthai bar;
    pool-side holdout gains did not transfer.
  - typed_decisions — NOT screenable as-built: per-case option sets need
    a scoring-shape change first (CODE before any run).
  - banking77 / prompt_injections — unscreened, NO board need (already
    strictly best with the bag arms; curiosity only, ~30 min each).
  - thai_wisesight / thai_sib200 — need the MULTILINGUAL checkpoint (the
    english encoder is useless for Thai); 008 T5's DEFER stands until the
    vs-best gaps close.
  - No new DATA anywhere — the t20k pools exist; per-suite cost is encode
    (~36 ms/row M3-Metal stand-in) + sweep + one read ≈ 20–30 min/suite.
