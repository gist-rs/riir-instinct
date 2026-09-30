# Issue 016 — the encoder class on GPU hosts: L3 think-depth seating via decision_wire (the promote-on-a-new-surface path)

**Status:** OPEN — filed 2026-09-30 (owner premise: "the prod is M5 ultra metal so we do
have gpu plan"). Carries the layer-fit ruling + the staged promotion path. The 014
serve-posture gate is NOT re-asked for the CF text lane; this issue is the different
decision surface the 014 close left open by construction (the extraction substrate
"stands reusable"). Verdict round filed same day.

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
| **L3 Working** | **1–2 Hz** | **500–1000 ms** | **✓ 1–7 % of slot on Metal (8–36 ms/row measured, 014 replay); the "think depth per decision" knob IS this op** |
| L4 Critical | 0.2–0.5 Hz | 2–5 s | ✓ batch encode of salient sets |

The fit is architectural, not just numeric: limelight's function is deciding
WHO gets deep cognition; the encoder class is the deep-cognition op spent on
the salient few. Salience-gated cardinality is the budget control: at
~1–5 % of entities spotlighted per 1–2 Hz window, a 1000-NPC world pays
~10–50 encodes/s ≈ 100–500 ms GPU/s on the measured M3-Metal class — an M5
Ultra has far more headroom than that.

## The seam (048's own laws — no new architecture)

- **L8**: layers are game-side; serving lanes serve decisions over the wire.
  The game side reaches the encoder class through `decision_wire` + a
  feature-gated thin client — **consumer #3, the pattern after
  `riir-agents/decision_gates.rs`** (consumer #2). No cargo dep either
  direction; riir-ai's L3 call sites never link instinct.
- **L9**: L3 ROUTES among equipped vessels (ns–µs — the routing, not the
  think); the think-depth op itself is the per-decision budget spend the
  cognition budget arbitrates. Swap policy stays L4/L5 game-side; the
  monotonic swap mechanism stays arsenal-side (`arsenal_ops.rs`, landed).
- **A1/A9 (arsenal laws)**: the trained NLEH head is a HOSTED-ONLY vessel
  (our trained IP; minted by riir-train, never runtime-minted); the laya
  encoder weights are the RUNTIME (the bag-count runtime's analogue — the
  serve host's capability, not a vessel payload).
- **L10**: no new katgpt-rs surface.

## What this issue is NOT

- NOT a re-ask of 014's text-lane gate: the reflex.gist.rs lane keeps its
  µs-class CF serve and its board rows unchanged; the `Instinct (encoder)`
  site lane keeps `serve: ✗` unless the owner separately amends it (the
  sub-decision below).
- NOT a flip of the 008 board by side effect: board rows move only through
  their own measured + published lanes.

## Owner sub-decision D1 (recorded here, not decided here)

The board's comparison lanes already serve from their own GPU hosts
(openthai @4090-win, gliner @4090-win, paw hosted). Does the M5 premise
extend to an **encoder-lane serve posture from an M5 host** — publishing
the sst5 cell as a served `Instinct (encoder) @m5-metal` (0.5267, vs the
0.4383 gliner bar = +8.8)? This would flip sst5's 008 board row through a
MEASURED, already-certified number (T2 LB95 +0.0535 over A1). It re-asks
014's scope only in the sense that the owner is supplying the exact premise
("a GPU tier in the deploy shape") the refusal said was absent. Default
posture if unanswered: the layer seating proceeds; the board posture does
not.

## Staged plan (pending verdict round)

- [ ] T1 — **budget pricing (measured, no new lane)**: encode cost/row on
  the M5-class host × limelight salience cardinality (the spotlight % and
  window) → the GPU-second share per 1000 NPCs at L3 cadence. Uses the
  existing `dump_encoder_states` / `instinct_encoder_eval` harnesses; no
  split reads.
- [ ] T2 — **serve-side encoder lane (feature-gated)**: wire the landed
  arena reader (`src/encoder_arm.rs`, `arena-laya-metal`) into the serve
  path behind an opt-in feature (GPU hosts only; the default CF-shaped
  build compiles it to nothing). Gate: serve parity replays the frozen
  Bench-029 picks.
- [ ] T3 — **the decision_wire thin client (consumer #3, riir-ai side)**:
  the feature-gated client after the `riir-agents/decision_gates.rs`
  pattern; L3 call sites route encoder-class questions only for
  limelight-salient entities (the depth knob). SUBSTRATE-FIRST before any
  new System impl (the 048 T7 outbound-boundary row rides the same
  change).
- [ ] T4 — **arsenal vessel mint for the NLEH head** (riir-train,
  HOSTED-ONLY class) + the monotonic apply the arsenal already ships.
- [ ] T5 — **GOAT gate**: G1 pick-parity vs the frozen reads; G2 the
  budget share (GPU-s/s at target NPC count ≤ the limelight allocation);
  G3 no-regression on the bag lanes (byte-identical when the feature is
  off); G4 alloc-free outside the encode call.
- [ ] T6 — D1 executes per the owner's answer.

## Scope notes

- The CPU fallback never routes the encoder (L3 falls back to the
  equipped bag/static class — the arsenal's equipped-set-per-host rule).
- Consumers named for the L3 depth op (honest, not exhaustive): NPC
  dialogue/quest intent over authored text (quest_grammar corpus),
  riir-agents task-reasoning encoding, RAG query encoding on the neuron-db
  read path.
- The 4090 CUDA posture (`laya-riir-cuda`) serves the same lane on that
  host class if ever needed (Issue 008 T7's riir-train 599/600 encoder
  artifacts are host-portable — the 014 M3-replay proved box-independence).
