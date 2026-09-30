# Issue 016 — the encoder class on GPU hosts: L3 think-depth seating via decision_wire (the promote-on-a-new-surface path)

**Status:** OPEN — filed 2026-09-30 (owner premise: "the prod is M5 ultra metal so we do
have gpu plan"). Carries the layer-fit ruling + the staged promotion path. The 014
serve-posture gate is NOT re-asked for the CF text lane; this issue is the different
decision surface the 014 close left open by construction (the extraction substrate
"stands reusable"). **Verdict round 1 (claude, 2026-09-30): REVISE — all seven reasons
incorporated** (arithmetic corrected to 80–1800 ms GPU/s + the admission cap made
required; resident-from-boot per L9 "never loads"; the sync-boundary bridge law; wire
RTT counted; M5-unmeasured premise; event-driven named consumers; D1 reframed as a
trigger; 014-still-governs-CPU line).

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
false claim, whatever the premise's energy. So:

**D1 flips to YES automatically when ALL of these hold:** (1) a Metal host
in the prod deploy shape actually serves the encoder; (2) per-row latency
measured ON THAT BOX; (3) the artifact pin validates on that box. The T2
certification already exists (LB95 +0.0535), so nothing else is required —
the sst5 cell publishes as served `Instinct (encoder) @<host>` (0.5267 vs
the 0.4383 bar = +8.8) and 008's sst5 row flips through its own measured
lane. Until the trigger fires: the layer seating proceeds; the board
posture does not.

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
- [ ] T6 — D1 executes when the trigger fires (a real prod-shape Metal host
  serving the encoder, latency measured on that box, pin validating) — the
  sst5 cell publishes as served and 008's row flips through its own lane.

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
