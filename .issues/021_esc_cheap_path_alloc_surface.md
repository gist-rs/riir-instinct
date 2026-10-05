# Issue 021 — the ESC cheap path's allocation surface: profiled-hot classes with no mechanical heal (riir-refine Issue 146 T0.6 leads)

**Status:** OPEN (lead intake — measured, profile-guided; no heal applied here)

Filed from the profile-guided-healing real-workload arm
(riir-refine Issue 146 T0.6, executed 2026-10-05, landed at
riir-rethink `4cd8dcf`): the arm profiled the ESC wrapper decide loop
(sst5, the G4 posture, 1205 decides, Metal) and its dependency-span
rule DECLINED every hot span resolving into riir-instinct code — this
file is the decline's landing spot, per the rule.

## The measured hot map (rethink side, for context)

- `EscalatingBackend::decide_multi_armed` (rethink): 8.8 KB
  alloc/decision, 14.1% of measured time — the BELOW-it share is this
  repo's code: the cheap lane's `decide_multi` internals + the synth
  case + the gate's `eval_seat`.
- `EscGate::flags` (rethink fn): 5.7 KB/decision — the allocs are
  `eval_seat`'s (this repo, `server.rs`).

## The leads (this repo's classes, measured through the wrapper)

1. **`synth_served_case`** (`src/server.rs:71`): the per-request
   `SuiteCase` construction — `id: "served".into()`, `state.clone()`,
   per-question `qid.to_string()` + `instructions.to_string()` + the
   criteria `Value` clone. All exact-size (no waste), but every cheap
   decision pays the full rebuild. Lead: a `Cow`/arc'd case for the
   served shape, or reuse across the gate + cheap legs (the wrapper
   builds the case, then `flags()` re-derives the same eval inputs).
2. **`eval_seat`'s result Vecs** (the seat engine's eval path): the
   per-question probs Vecs + the decisions Vec are rebuilt per call.
   Lead: scratch-buffer reuse on the serve path (the `y` scratch
   pattern encoder_serve already uses).
3. **The cheap lane's `decide_multi` prelude** (`AnySuiteServer`):
   all_labels/all_classes/all_options Vecs + per-question pos Vecs +
   the A1 arm's `pos_spec` clone + `options.clone()` + pick/arm-name
   Strings — inventoried by rethink's G4 doc (190-allocation pin).
   Lead: the same scratch/ownership review, after 1–2.

**The measured floor**: the ESC cheap decision allocates ~190
allocations/decision (rethink G4 pin) of which the wrapper's OWN share
is ~10–15 — the rest is this repo's surface above. A 30–50% cut on the
cheap path is plausibly worth 2–4 µs/decision at the serve p50 (the
cheap lane runs ~6.3 ms/decision WALL today only because the profile
loop interleaves think-tier Metal encodes; the pure cheap path is
µs-class — measure before any edit).

**Anti-goal**: none of these are growth-doubling or capacity waste
(the profile found NONE on the measured path — every allocation is
exact-size). This is an ownership/scratch-refactor lead, not a
`vec-with-capacity` fix. GOAT-gate any change: the G4 pin is a
CEILING — a reduction must stay bit-identical on picks (the frozen
parity replays pin that).
