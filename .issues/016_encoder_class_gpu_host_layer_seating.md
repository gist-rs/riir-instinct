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
utterance→sentiment/intent consumer; measured-vs-cited table wording). **T1 DONE at
the stand-in posture + T2 LANDED (2026-09-30, Bench 031): the serve-side encoder lane
(green: the frozen Bench-029 read replayed 316/600 EXACT through the serve surface,
arena-runner parity 32/32, e2e HTTP smoke green; the DEFAULT manifest untouched —
ENC is a GPU-host deployment posture). T3 LANDED (riir-ai decision_client: the
deadline-fallback law + the G1b arms + Layer 1.33; the game call-site routing rides
the prod host). T5 partially armed (the parity gate carries G1's pick-parity leg +
the L3 slot bound; G1b lives in the client's arms; G2/G4 land with the consumer).**
T4 LANDED (the HOSTED-ONLY head vessel: riir-instinct 82011af reader+routes+round-trip
gate, riir-train f3ce8973 NLEH minter; the production mint is the owner's key
ceremony). **T7 EXECUTED 2026-09-30 — NEGATIVE BY THE SCREEN (Bench 032): the
reference floor on the emotion test split reads 0.5950 vs the 0.8850 earn bar; the
class's measured-max lift (+14.7 pt) tops out at 0.742 — the lane is dead by the
pre-registered screen-first law, the train run never earned. Emotion stays
A0-served; the posture-gap state stands (the row's security waits on a different
model class or the D1 posture). CROSS-CHECKED 2026-10-01 (Bench 035, the
Issue-825 double-run): the second session's independent 5-head sweep refused
every head (holdout 0.459–0.4835 vs the reference's 0.4975; best head 0.6200 on
test = 26.5 under the bar) and the Bench-032 cells reproduce EXACTLY at the 4090
CUDA posture — the verdict is double-measured; the frozen read was never spent
in either session.** T5–T6 open (T5's G2/G4 land with the consumer;
T6 = D1, trigger-blocked). **T8 EXECUTED 2026-10-01 — the three unscreened
board suites screened ([Bench 039](../.benchmarks/039_encoder_screens_bank_prompt_typed/RESULTS.md)):
banking77 DEAD (floor 0.4980 + max lift = 0.645 < 0.8540), prompt_injections
DEAD (floor 0.6983 + 0.147 = 0.845 < 0.8534), typed_decisions ALIVE — the
typed-checkpoint floor reads 0.7445 on the frozen t20k wire, +9.7 pt OVER
the 0.6475 seating bar before any lift → **the head run is EARNED**, blocked
only on the NLEH v2 per-option scoring shape (T9). Three wire-fidelity
witnesses: the floors reproduce published board reads byte-exactly (0.4980
laya-english bank / 0.7445 laya-typed / 0.3575 laya-english typed).**
**Owner round 3 (2026-09-30): three clarifications folded — the no-new-product-lane note
(nothing named "Working"/"riir-working"; L0–L5 are 048's tier vocabulary), D1's host
generalized to ANY real GPU serving deploy (game prod OR text-lane serve host — the
board consumer's premise), and the ladder-monotonicity note (each rung strictly-additive
via deadline fallback; L2 skipped by law; L4 untasked because 048 deferred L4/L5, Plasma
reserved as the L4 salience-depth rung).**

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
| L2 Surface | 5–10 Hz | 100–200 ms | ✗ lookup only |
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
salience-depth rung.**)

**SUPERSEDED 2026-09-30 (owner call, same session): round-3's "no new
product lane" clause is refused — the product consolidates as `riir-rethink`
(all-tier adaptive serving; the rename of this repo; the name evolved
same-session thinker → Rethink — the Reflex→Rethink pairing, endorsed by the
`rethinks` sweep), per
[riir-ai Proposal 051](../../riir-ai/.proposals/051_rethink_all_tier_adaptive_serving_family.md).
The staged plan T1–T8 stands unchanged; D1's wording becomes "publishes as
served Rethink (encoder) @<host>" once the rename lands. The
"Instinct (encoder)" site lane rebrands with aliases (Phase 1).****

## Staged plan (pending verdict round)

- [-] T1 — **budget pricing (measured, no new lane)**: the demand model is
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
  **DONE at the stand-in posture (Bench 031, 2026-09-30)**: the G2 unit
  derived (`gpu_s_per_s = admitted/s × per_row_s(host)`; `cap = budget_share
  / per_row_s`) + the stand-in cells (M3-Metal 14.7 ms cited, this box CPU
  228 ms measured, GPU 8–15 ms cited) in
  `.benchmarks/031_encoder_serve_t2_parity/RESULTS.md`. The serving host's
  cell lands with D1 — the gate prints p50/p99 with the device label every
  run, so the re-measure is one gate run. The M5 remains UNMEASURED.
- [x] T2 — **serve-side encoder lane (feature-gated)**: wire the landed
  arena reader (`src/encoder_arm.rs`, `arena-laya-metal`) into the serve
  path behind an opt-in feature (GPU hosts only; the default CF-shaped
  build compiles it to nothing). Gate: serve parity replays the frozen
  Bench-029 picks.
  **DONE 2026-09-30 (Bench 031)**: `src/encoder_serve.rs` — the EncoderLane
  behind `posture ENC` + feature `serve-encoder` (the agent on a dedicated
  worker thread per the reflex !Send law; resident from boot with the
  boot-time warmup encode per L9; the C1 canonical-presentation contract
  enforced — template criteria order + the suite's serialized-state form,
  both bugs the gate's first red run caught). Grammar + validator in
  `arsenal.rs` (ENC rows: explicit `file`, eager-only — a lazy ENC row is
  an L9 violation, refused); routing in `server.rs` (`Arm::Enc`; the bag
  server refuses ENC by construction; vessel boots refuse ENC until T4).
  Gates: `tests/serve_encoder_parity.rs` (two legs — arena-runner pick
  parity 32/32 in-process + the frozen-count 316/600 EXACT at the CPU
  posture; determinism; the 1 s L3 slot bound) + the default-build gate
  arm (an ENC row on a build without the feature refuses loud naming
  `--features serve-encoder`) + the ENC grammar/validator arms in
  `serve_gates.rs`. E2E smoke: the serve binary over HTTP answers 200 with
  `arm: "ENC"` + the score vector crossing (the sync boundary holds). The
  DEFAULT manifest is UNTOUCHED (the embedded byte pin holds; sst5 keeps
  serving A1 byte-identically) — ENC rows are a GPU-host deployment
  surface, never a default. Clippy clean at both postures; lib 58 +
  serve_gates 17 green. Full record:
  `.benchmarks/031_encoder_serve_t2_parity/RESULTS.md`.
- [x] T3 — **the decision_wire thin client (consumer #3, riir-ai side)**:
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
  **DONE 2026-09-30 (the client half; the game call-site routing rides the
  prod host)**: `riir-ai/crates/riir-agents/src/decision_client.rs`
  (feature `decision_client` = one optional serde_json parse dep; the
  request side hand-serialized — one writer, one parser). std TCP POST to
  the serve edge under a per-call deadline: every failure path collapses
  into `DecisionClass::BagFallback(reason)` — the record names WHICH class
  answered (Encoder / Served{arm} / BagFallback{DeadlineExceeded,
  ServeLaneDown(code), LaneAbstained}); a late-but-good response is
  dropped ON ARRIVAL (compute-once-and-record); the connect phase is
  capped separately at min(remaining, 250 ms) — measured Windows behavior
  (a closed port surfaces the refusal only at connect_timeout's own
  expiry) would otherwise eat the slot and misrecord a dead lane as a
  slow one. Sync boundary holds structurally (the client never reads a
  vector field). The depth knob is the CALLER's (limelight salience
  admission is game-layer routing; documented in-module). WIRE-ONLY per
  the BOUNDARY.md outbound-forbidden law (no dep on the serving repos).
  Gates: 9 module tests (the G1b deadline-miss injection, the late-reply
  arm, refused lane, error-code surfacing, bag-arm-served noted-not-
  hidden, abstention first-class, JSON-escape wire round-trip, the
  text_envelope C1 render) + a ci_feature_guard Layer 1.33 row (count
  floor 9, the tests are invisible to every other layer — feature is
  default-off). Clippy clean at default + decision_client + all-features;
  default lib 17 green (the module compiles out, G3).
- [x] T4 — **arsenal vessel mint for the NLEH head** (riir-train,
  HOSTED-ONLY class) + the monotonic apply the arsenal already ships.
  **DONE 2026-09-30**: the reader half —
  `riir-instinct/src/vessel.rs::load_hosted_head_bytes` (the same
  authenticity / class / monotonic walk as the bag vessels, the RAW
  decrypted NLEH payload out — the parse stays the lane's own law, zero
  change to the specialist decoder); the boot routes — `boot_vessel` /
  `boot_vessel_bytes` ENC arms under `vessel` + `serve-encoder` (the facts
  flow to the arsenal's existing epoch install — the monotonic apply ships
  unchanged, no new lineage code); a build without either feature refuses
  the ENC vessel route loud. The mint half — riir-train `vessel-mint`
  accepts the NLEH magic beside RISP (live-verified: the t6_s0 head minted
  rc 0, 3.2 MB payload vessel + sidecar, test key; the PRODUCTION mint is
  the owner's key ceremony, never a repo act) + the never-compiled Windows
  entropy fallback fixed en-route. Gate: the T4 round-trip arm in
  `tests/serve_encoder_parity.rs` (both features) — vessel-booted lane ==
  raw lane decision-for-decision (32 cases), the lineage facts carry the
  minted version, v1-over-applied-v1 refused as a downgrade. Commits:
  riir-instinct 82011af, riir-train f3ce8973.
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
- [x] T8 — **the three unscreened board suites screened (the 600 law,
      screen-first)** — banking77 / prompt_injections / typed_decisions, one
      frozen ref-only read each on the wire-faithful t20k TEST split
      (case-makers mirroring the reflex builders verbatim; the typed maker's
      suite shape matches the board exactly — 400 cases → 2000 questions,
      choice 600 / noul 600 / score 800). Enabler: the eval's ref screen
      gained the **per-row-k** mode (mixed presented widths score each row's
      own options against its own gold-as-presented-index; the v1 winner
      path stays fixed-class and refuses a mixed cache loudly).
      **Verdicts (Bench 039)**: bank/prompt DEAD (above); typed ALIVE — the
      head run is earned. Artifacts: `../riir-train/.raw/t608/` (regenerable
      makers + LENC caches; gitignored data).
- [ ] T9 — **the earned typed head run (NLEH v2 per-option scoring)** —
      CODE then train then ONE frozen read: (1) the v2 artifact shape —
      `score_i = MLP([marker_i ; pooled])` → per-option sigmoid (the
      Specialist law), pick = argmax over the presented options; handles any
      presented width and any per-case option text; (2) the trainer arm over
      the typed-ckpt TRAIN cache (800 t20k cases ≈ 4000 questions, gold-only
      first per the 600 wave-3 finding); (3) the single frozen test read +
      T2-style paired LB95 vs the 0.6475 displayed arm; (4) if it beats the
      bar: the arena `--encoder-art` read + seat + lane-doc emit (the 017 T5
      auto-emitter) + the site publish — the fourth Rethink cell and the
      strongest (floor alone +9.7 over the displayed arm).
- [x] T7 — **the emotion screen (the ONE unscreened board-relevant encoder
  run)** — a 008-track text-lane run of the same class,
  not a layer-seating task. Pre-registered per the 014 law BEFORE any run:
  the suite-generic trainer (`instinct_encoder_trainer`) over the emotion
  t20k pool, ONE extraction method (the NLEH class, same architecture,
  FRESH fit — never reused head weights), 5-head pool-side sweep,
  best-holdout selection, **ONE frozen read** on the emotion test split.
  Earn gate: the head must read **> 0.8850** — the armed-seat A0, T4's
  recorded bar for a REAL emotion arm; below it the posture-gap state
  stands and emotion stays A0-served.
  **EXECUTED 2026-09-30 — DEAD BY THE SCREEN, the train never earned
  ([Bench 032](../.benchmarks/032_emotion_encoder_screen/RESULTS.md)):
  the screen-first law (issue 600) ran first — the emotion-wire cases
  (the reflex `build_emotion` wire verbatim, 400 test + 15,969 unique
  train) → the test LENC cache on M3 Metal (15.3 s) → `--ref-only`:
  the reference floor reads **0.5950** vs the 0.8850 bar, and the
  class's measured-max lift (+14.7 pt) tops out at 0.742 — 14+ points
  under the bar, no training run can close it. The 5-head sweep + the
  frozen read were NOT spent (the screen exists to prevent exactly
  that). Emotion stays A0-served; the posture-gap risk stands (the
  row's security waits on a different model class or D1). Box state +
  artifacts in the record (M3 Metal, AC; `riir-train/.raw/t607/` —
  the case-maker is wire-faithful + regenerable).** The original MISS
  clause holds verbatim: negative, no serve change, no re-run.
  **CROSS-CHECK 2026-10-01 ([Bench
  035](../.benchmarks/035_emotion_encoder_t7_crosscheck/RESULTS.md)) —
  the Issue-825 shape, resolved the measured way: the second session ran
  the FULL pre-registered sweep independently before fetching, then
  cross-ran instead of landing a second negative blind. The Bench-032
  cells reproduce EXACTLY on the 4090 at CUDA (fresh encode of the same
  case bytes in 8.6 s: floor 0.5950 byte-exact, all six per-class recalls
  match, zero Metal-vs-CUDA cell flips — the lane's third
  device-independence witness), AND the 5-head fresh-seed sweep
  (0x016E016..A, gold-only, the bench-615 quarantine, this box's CPU
  cache) reads holdout 0.4590/0.4745/0.4685/0.4730/**0.4835 (best)** vs
  the reference's 0.4975 — every head refused BOTH gate halves
  (floor-robust: even a 0.0 floor refuses); train-acc ~0.71 vs holdout
  ~0.47 = the features overfit; the best head reads **0.6200 on test**
  (+2.5 over the floor, **26.5 under the bar** — the +14.7 ceiling was
  optimistic for emotion; the verdict holds with 4× the margin). The
  frozen read was NEVER SPENT in either session (both budgets stay
  RESERVED); artifacts `../riir-train/.raw/t599/emotion_*` (blake3
  sidecars).**

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
  - typed_decisions — **screened ALIVE 2026-10-01 (Bench 039, T8)**: the
    typed-checkpoint reference floor reads **0.7445** (1489/2000, per-width
    2/4/5 = 0.785/0.716/0.767) on the wire-faithful frozen test read — +9.7
    pt OVER the 0.6475 seating bar. The head run is EARNED; the blocker is
    CODE (the NLEH v2 per-option scoring shape — presented widths vary
    2/4/5 and the option TEXTS vary per case; v1 is fixed-class by
    construction). T9 below.
  - banking77 / prompt_injections — **screened DEAD 2026-10-01 (Bench 039,
    T8)**: english-ckpt floors 0.4980 / 0.6983 vs bars 0.8540 / 0.8534 —
    floor + the class's measured-max lift (+14.7 pt) tops out 0.645 / 0.845,
    both under. Measured negatives, never backlog.
  - thai_wisesight / thai_sib200 — need the MULTILINGUAL checkpoint (the
    english encoder is useless for Thai); 008 T5's DEFER stands until the
    vs-best gaps close.
  - No new DATA anywhere — the t20k pools exist; per-suite cost is encode
    (~36 ms/row M3-Metal stand-in) + sweep + one read ≈ 20–30 min/suite.
