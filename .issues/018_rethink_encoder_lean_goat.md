# Issue 018 — Rethink encoder lane: lean + GOAT plan (lazy inheritance, shared worker, one-checkpoint typed, quantization)

**Status:** OPEN — filed 2026-10-01 (owner ask after the /#sizes Rethink row landed); Claude verdict CONVERGED AGREE at round 3 (2× REVISE, all reasons folded). **Lanes B + A LANDED 2026-10-02 (Bench 0050, the 4090 box) — the daylight session executed; lazy ENC rows legal with `eager` still the embedded default. Update 10 (2026-10-02, owner-approved menu items A+B): the shared worker is PROMOTED to the serve-encoder default (the serving soak completed GREEN — the 0054 confirmation addendum; the runtime demote switch is the recorded hedge), option (iii) re-priced and DECLINED at today's shapes, and the Q4 retention probe (Lane D4) pre-registered + running (Bench 0055).** Addendum 2026-10-01: two-case calibration branch wired (Δ-retention breach routes through D1's existing verdict, pre-registered refit allowed first; absolute-floor tripwire lands in Lane C's pre-declared loss options for the head C retrains, narrow filing for any other head) — katgpt-rs Research 562 addendum; nothing fired, execution order unchanged.

## Context — the numbers that triggered this

The reflex-site `/#sizes` chart now reports the Rethink encoder serving
posture (corrected same-day from an over-count that folded the arena's
measurement lane in):

| Piece | Bytes |
|---|---|
| laya-english checkpoint (third-party, 421M params, F16) | 848,195,504 |
| 3 sealed NLEH heads (sst5 / xnli_en / ag_news v1) | 7,994,976 |
| serve binary (`--features serve-encoder-metal`) | 7,940,160 |
| six t20k dataset suites (the seats) | 20,808,524 |
| **total disk** | **884,939,164** |

Two structural facts behind the number (both in code today):

1. **`encoder_serve.rs` hardcodes `Checkpoint::English`** at lane boot —
   every ENC suite encodes with the english checkpoint. The typed v2 head
   is paired to the TYPED checkpoint ("a head trained on the typed cache
   is meaningless over the english encoder", `encoder_arm.rs`) and rides
   the arena's record-only measurement lane.
2. **Each ENC lane spawns its OWN worker thread + its own `RiirAgent`**
   (`EncoderLane::from_parts`, `sync_channel(64)` per lane). Three english
   suites seated = 3 × 848 MB resident RAM, plus 3 worker threads.

## The owner's ruling (the inheritance principle)

> "other layer should inherit goat thing from previous layer isn't it?"

The tier stack: **L1** Reflex (modelless, free floor) · **L2** Instinct
hybrid (bag lanes with `budget.load = "lazy"` + `POST /arsenal/release`
evict — law A7) · **L3+** Rethink (the encoder lane). L3's L9 law
("resident from boot, never loads") blanket-bans what L2 ships as a GOAT
serving feature. Feature inversion up the stack — unless a measured trade
says otherwise. None is on record: the L9 validator refusal
("posture ENC must be eager — the encoder weights are resident from boot")
is a ruling, not a benchmark.

## Lane B — one encode worker per (Checkpoint, device) — riir-instinct

The unit of residency moves from the LANE to the shared worker. Every
semantics change below is written against that unit.

- [x] One worker thread + one `RiirAgent` per (Checkpoint, device), shared
      by every ENC lane on that checkpoint; heads stay per-lane (KB-MB).
      The worker carries the CHECKPOINT DIGEST — the epoch stays per-lane
      with its head (swap/monotonicity must NOT attach to the shared
      worker). **→ LANDED 2026-10-02 (Bench 0050):** the registry keys the
      worker by checkpoint under `serve-encoder-shared` (`ckpt:english`) or
      lane+head in the default build (`lane:<suite>` — byte-identical
      per-lane behavior, same code path); weak-handle residency (lanes own
      the worker; last release frees the RAM); the worker carries the
      streamed-BLAKE3 checkpoint digest. In-process the device is
      process-global (`LAYA_DEVICE`), so (Checkpoint, device) reduces to
      Checkpoint — recorded, not lost.
- [x] Per-suite warmup + template validation move to LANE ATTACH (the
      worker no longer knows a suite's template at boot); a template drift
      fails that lane's attach loudly, never the shared worker.
      **→ LANDED: attach validation job with the lane's real template
      fields; a drift fails the lane, the worker keeps serving.**
- [x] Ship behind a GOAT feature flag; promote to default only after
      parity + RAM + mixed-load latency all pass (the promotion gates
      below). **→ PROMOTED 2026-10-02 (Update 10): `serve-encoder`
      implies `serve-encoder-shared`; the soak completed GREEN (the 0054
      addendum) and the runtime demote switch is the recorded hedge.**
- [x] RAM gate: measured N × 848 MB → 1 × 848 MB for the 3-suite host.
      **→ MEASURED (Bench 0050): 3 lanes / 1 checkpoint / CPU / F16 —
      per-lane 4938 MB private vs shared 1690 MB = 2.92×.**
- [x] Parity gate — INTERLEAVED, not sequential: replay all served suites
      interleaved through the shared worker and compare bit-identical
      against isolated per-lane runs (catches state leaking between
      suites: agent scratch, the reused `y` buffer, device caches).
      **→ PASS both modes; cross-build fingerprints byte-identical (sst5
      c35a50b77602c063 · xnli_en 7c237555b7ab47fd; Bench 0050).**
- [x] Concurrency gate: mixed-suite p50/p99 under concurrent load, shared
      vs per-lane, interleaved A/B with `bench_preflight.sh` box state
      quoted (±6%/±21.7% envelope law). Serialization is the law WITHIN a
      lane; across lanes today's three workers can overlap on the device —
      one shared worker adds head-of-line blocking, and that cost is
      measured, never assumed. **→ PARTIAL, honestly scoped: the lockstep
      interleaved replay is the mixed-load instrument and it is
      bit-identical (wall 1216 s per-lane vs 1194 s shared, 6 boots
      included — no head-of-line regression visible at that shape); the
      full concurrent-HTTP p50/p99 A/B DEFERS to the serving soak.**
- [x] Fairness/queue gate: per-lane fairness and queue bounds (today each
      lane owns `sync_channel(64)`; one shared queue lets a burst on one
      suite back-pressure the others — bound it or shard it). **→
      `LaneGate` (per-lane in-flight cap 64) unit-tested to block at the
      cap and release on drop.**
- [x] FALLBACK (recorded, not silent): if p99 regresses beyond the
      envelope, keep per-lane workers and share only the WEIGHTS
      (mmap/Arc of the checkpoint) — RAM win kept, latency trade recorded.
      **→ recorded, dormant — no regression observed.**

## Lane A — L3 inherits L2's lazy + evict (the L9 revisit) — riir-instinct

Preserve L9's REAL intent (never a load ON the hot request path, never
per-request loads) while inheriting A7's semantics verbatim — with lazy's
failure surface closed at BOOT, not at first request:

- [x] Allow `budget.load = "lazy"` on ENC rows: the lane boots `Unloaded`;
      the FIRST decision triggers the load (agent + head + one warmup
      encode paying the Metal/CUDA pipeline compile); the 503 `loading`
      window covers it (the bag-lane posture, state named, never a silent
      fallback); the lane is resident after. **→ LANDED (Bench 0050): the
      validator accepts `eager | lazy` for ENC; `eager` stays the embedded
      default; the bag machinery (Unloaded→Loading→Ready, the 503 window,
      the release wire) already generic — consumed, never duplicated.**
- [x] **Boot preflight runs for lazy rows too** — lazy must not move
      boot-time refusals onto the first request: weights present + BLAKE3
      digest check; head digest check; template-vs-head class-count check
      WITHOUT a forward pass (a property of template + tokenizer); a
      memory-budget check against device capacity — **counted per shared
      worker** (the sum over DISTINCT (Checkpoint, device) workers, never
      over lanes: three english lanes share one worker and must be charged
      848 MB once, or a host that fits gets refused). A config error is a
      boot refusal, never a production 503. **→ LANDED: substrate fn
      `verify_checkpoint_present` (pin verification, never downloads), the
      ACTIVE-variant sidecar resolution + streamed checkpoint digest,
      `INSTINCT_ENCODER_MEM_BUDGET_MB` counted per distinct checkpoint
      (unset = disclosed, not gated — no portable GPU-memory query; the
      operator declares the ceiling), template-vs-head class count from
      the seat + head parse. The head digest check already ran at
      manifest validation for every row incl. lazy.**
- [x] **Readiness semantics**: healthz reports a lazy-Unloaded lane as
      `ready-cold`, distinct from `ready` — a load balancer must not route
      the first user into the compile window blind. **→ LANDED (healthz
      `readiness`: ready | ready-cold | loading | failed; /decide
      responses disclose `lane_load`).**
- [x] `POST /arsenal/release` evicts back to Unloaded (epoch kept) — the
      existing wire, no new lineage code. Under the shared worker (Lane
      B), release drops the lane's SENDER; the worker and its 848 MB
      survive while another lane holds one; RAM is freed when the LAST
      holder releases — the disclosure says so. **→ LANDED: the weak-handle
      registry IS those semantics (the release-free/recall unit tests pin
      them); the wire needed zero changes.**
- [x] `eager` stays the DEFAULT (the production posture unchanged);
      `lazy` is the opt-in for memory-constrained / multi-checkpoint hosts.
- [x] Disclosure: healthz + the decision receipt carry the lane's load
      state; a cold first decision names the load+compile cost it paid; a
      released-then-recalled lane re-pays the compile ONCE per activation
      (bounded, disclosed). **→ LANDED: healthz `readiness` + `load`,
      /decide `lane_load`, the boot/attach/warm log lines; the hashed
      receipt envelope stays the decision's, the load state rides beside
      it.**
- [x] The validator's ENC eager-only refusal becomes: `eager | lazy` both
      valid for ENC (lazy rows carry the one-time-load contract). **→
      LANDED (the serve_gates arm flipped; unknown load words still
      refuse).**

**GOAT gates (Lane A):**
- G1: post-load answers BIT-IDENTICAL eager vs lazy (same head bytes, same
  agent state, same frozen picks — the serve parity gate extended).
- G2: hot decision latency parity (interleaved A/B, `bench_preflight.sh`
  box state quoted; the ±6%/±21.7% envelope law).
- G3: at most ONE successful load per activation per shared worker — a
  load triggered by lane X never counts as a reload for lane Y, and two
  lanes waking cold at the same moment cause exactly ONE load (the
  lane-level once-only trigger in `serve.rs` generalized to the worker).
- G3-failure bound: a FAILED load (OOM, device lost) enters a named
  `Failed` state with backoff — never a per-decision retry storm.
- G4: alloc bounds unchanged (the counting allocator gate).

## Lane C — typed head retrained over english encodes — riir-train

One checkpoint serves ALL FOUR seated cells; the serving lane's hardcoded
english becomes correct-by-completeness instead of a typed-cell blocker.

- [x] Train typed v2 (per-option) head over the ENGLISH encode of the
      typed pool — **EXECUTED 2026-10-02 — NEGATIVE (Bench 0048):** the
      adopted-encoding (q8, english) encode pass ran clean (train 4000 /
      test 2000 rows, d 1024, Metal, 296 s + 170 s), the v2 trainer refused
      the earn bar in the worst direction (head holdout 0.3113 BELOW its
      own reference 0.3600; train acc flat 0.31–0.37 across all 15 epochs),
      the frozen test read stayed RESERVED (the trainer's no-cheat law; the
      read was never spent), and the gate never ran (no candidate head).
      The english checkpoint is AT OR BELOW CHANCE on every width (ref-only
      floor 0.3595; 2-wide 0.4733 vs chance 0.5, 4-wide 0.3182 vs 0.25,
      5-wide 0.2833 vs 0.2) — corroborated by Bench 039's F16 english floor
      0.3575 (itself a wire-fidelity witness vs reflex .issues/029) and
      preserved at the adopted encoding (Δ +0.0020). The typed cell keeps
      `typed_encoder_v2.bin` (typed ckpt) — the issue's negative branch,
      no rollback. The CE+λ·Brier landing pad deliberately NOT spent: the
      pad is a calibration-shaped fix for a head that trained; this head
      never learned (representational deficit, ~45 pts vs the typed
      floors). Lane B's value RAISED (the two-resident-checkpoint posture
      stands — exactly Lane B's RAM concern); Lane E NOT triggered (no D1/D2
      retention failure). Re-open only on a NEW substrate (an english
      fine-tune carrying typed-shaped data — riir-train work, 039
      screen-first).
- [x] T2 gate: paired LB95 of the english-encoded head vs the seated arms
      on the frozen test read; the typed cell re-seats ONLY on a win (the
      third-posture cell-identity witness re-run — arena, then serve-path).
      **→ NOT RUN — no candidate head existed to pair (the earn bar refused
      before any test read; the gate requires a head).**
- [x] NEGATIVE branch: if it fails the bar, typed stays measurement-lane
      only and this issue records the number — no posture rollback.
      **→ TAKEN (Bench 0048): typed keeps the typed-checkpoint head as
      seated; the number is the bench record.**
- [x] Input-cache note (the D1-pass branch): C's training input on that
      branch is the adopted Q8 ENCODE of the typed pool — the existing
      2.4 GB english-encode cache is F16 and must be REGENERATED under
      the adopted quant (an encode pass over the pool, riir-train side).
      That regeneration cost is part of D2 → C, budgeted, never a
      surprise. **→ Done as declared: both sides regenerated under
      `LAYA_WEIGHTS_VARIANT=q8` (Bench 0048 R1, ~8 min Metal total); the
      F16 english cache was never read by C.**
- [x] D-interaction (recorded): heads are fit to F16 encodes — Lane D2's
      adopted quant invalidates these inputs and forces a C retrain +
      re-seat. The ORDER BRANCH below resolves this: on a D1 pass, C runs
      AFTER D2 and trains on the adopted quant's encode cache exactly
      once; C never trains on an encoding D2 is about to change. **→ Held
      exactly: C trained once, on the adopted encoding, after D2a.**
- [ ] **Tripwire landing pad (562/576; wired 2026-10-01):** if the Lane D absolute-floor case ever fires on a trained head, C's retrain is where the fix lives — CE+λ·Brier becomes a PRE-DECLARED loss option for the Lane C head training (CE+Brier is Raschka's supervised form, katgpt-rs Research 576's PASS-Redirect line, author-caveated "modest gain, may not generalize"; the RL-family scoring-rule recipes are 576 §3's log/spherical/RPS — the loss form is chosen from that family at filing time; the reward-shaped twin already ships in riir-train `svr.rs::r_verify` `λ_cal`). Until a raw-FAILS reading exists, C trains plain CE — the refit-first posture is the shipped primary, now the independent consensus (562 addendum S2).

## Lane D — quantize the english checkpoint — riir-infer + riir-train

Split into the CHEAP PROBE (D1) and the full adoption (D2) — the probe
prices everything before any training or kernel work is spent:

- [x] **D1 — the retention probe (cheap, first):** FAKE-QUANT instrument
      (quantize-then-dequantize the weights, run the existing F16 forward
      unchanged — no riir-infer kernel work; measures weight-quantization
      error only, kernel numerics being D2's per-device determinism
      gate's subject): Q8-encode the test reads, score the EXISTING heads.
      Per-suite per-family pick retention + non-inferiority vs F16 (the
      rule below). This is the measured go/no-go for the whole lane —
      aggregate-flat-while-families-flip is the recorded failure shape
      (the Orthrus lesson), so per-family is the only read that counts.
      COVERAGE LIMIT (stated): D1 scores the heads that exist — the
      typed-over-english head does not exist yet, so the typed cell's
      retention is NOT covered by D1; it is measured when C trains on the
      encoding D1's verdict selected.
      **→ EXECUTED 2026-10-02 — PASS (Bench 0046): 7 suites × {fresh F16,
      fake-quant Q8_0}, paired per pre-registration (`33d02ed`, margins
      BEFORE any read); every fresh F16 read reproduced its frozen cell
      (per-row picks on 6, accuracy on sst5). 5 PASS · 2 UNDECIDED
      (massive: acc LB95 −0.0213 vs δ 0.02 at n=300; banking77: cal
      UB95 +0.0190 vs margin 0.01 on the F16-uncalibrated head, ECE 0.494
      at F16 — its cell's own disclosure), 0 FAIL. Retention 95.6–100%;
      Δacc −1.0…+0.67pt; ΔECE ≤ 0.0051; quant surface 421.2M elements /
      842.4 MB F16 → ~424 MB Q8 (the D2 estimate confirmed by
      construction); max weight err 0.0163, mean 3.0e-4. Instrument:
      riir-infer `294999e` + instinct `2a61105`. BRANCH ⇒ D2 (the two
      UNDECIDED rows ride D2's own re-seat — a real-kernel read re-measures
      them at higher resolution; the issue's lane rule never folds them).
- [-] **D2 — full adoption** (only on a D1 pass): Q8 quant in riir-infer
      (house dequant machinery; the Bonsai PQ precedent), Q4/PQ2 second.
      English 848 MB → ~424 MB (Q8) / ~220 MB (Q4) → posture total
      ~460 MB / ~255 MB. **CHECKBOX 2026-10-02: the Q8 half is ADOPTED
      end-to-end (D2a storage + D2b device residency + the no-change
      re-seat — Updates 2/4/5/6 below); the remainder DEFERRED by its own
      law — Q4 retention is D1-priced separately before `q4` serves
      anything, PQ2 rides the same seam.**
      **→ D2a (the STORAGE tier) ADOPTED 2026-10-02 (Bench 0047,
      riir-infer `10e33de`): Q8_0 artifact format + `laya-quant8`
      converter + `LAYA_WEIGHTS_VARIANT=q8` loader seam (sidecar-verified,
      never auto-derived at load; double-quantization refusals). english +
      typed converted, byte-equality proofs GREEN on the real 421.2M
      elements: 842.6 MB → 447.7 MB (53.1% — the honest number: the 80
      skipped 1D norms/biases stay F16 by the house law). The adoption
      re-seat is satisfied by the no-numerics-change proof (converter
      read-back == fake-quant, bit-exact) + the end-to-end spot-check
      (sst5 via artifact: 0.5333 = the probe read, picks byte-identical,
      `weight_posture: q8-artifact`). REMAINS: D2b — device-resident Q8
      buffers + dequant-fused kernels (the real device-memory tier;
      per-device determinism re-seats apply THERE), then Q4/PQ2 second,
      then C on the adopted encoding. **→ C EXECUTED (negative, Bench
      0048 — above). D2b SCOPED:
      `../riir-infer/.plans/616_laya_q8_device_residency.md` (Plan 616 —
      Phase 0 measure → Phase 1 host residency → Phase 2 the
      fused-staging kernels + the MPS tension priced (MPS binds raw F32;
      default is MPS-off-under-q8 with the cost A/B'd) → Phase 3 adoption
      + the Q4 format seam; bit-identity by staging-value identity is the
      central claim every gate proves). ✅ PHASE 0 + PHASE 1 DONE
      2026-10-02 (Update 4 below): host residency 2.85× (1489 vs 4238
      MiB RSS), byte-identical everywhere, Phase 2 next.**
- [ ] **The accuracy gate is NON-INFERIORITY, never "CI contains zero"**:

      | Requirement | Rule |
      |---|---|
      | Margin | δ declared per cell BEFORE measuring |
      | Pass | paired LB95 of (quant − F16) is above **−δ** |
      | Small families | any per-family n too small to resolve δ is **UNDECIDED**, never PASS |

- [ ] Calibration gate: the receipts/heads expose confidence — per-family
      Brier/ECE under the quant (accuracy can hold while calibration
      breaks); the Report-the-Floor rule applies wherever confidence is
      claimed (conformal-naive floor). **The pass rule, same treatment as
      accuracy:** a per-family calibration margin declared BEFORE
      measuring; pass = the paired UPPER bound of Δ(Brier) (quant − F16)
      sits below that margin; thin families UNDECIDED, never PASS.
      Without a pre-declared margin this is a report, not a gate.
- [ ] **Calibration-failure branch (wired 2026-10-01; two cases kept distinct — katgpt-rs Research 562 addendum):**
  (a) **Δ-retention breach** — a per-family Δ(Brier)/Δ(ECE) read above its pre-declared margin is a QUANT retention failure, not a head defect (in D1 the heads are the existing F16-trained ones; only the encoder is fake-quantized). Response: the per-family temperature refit may run FIRST only if pre-registered in the margin declaration (argmax-preserving, the Issue-810 track-b posture; external warrant: one fitted temperature on 50–300 labels fixed most calibration error everywhere tried, but refit temps ranged 0.65–4.45 across domains → fit per family). A refit that clears the margins = PASS-with-refit (recorded); an unrefit-able breach counts as a D1/D2 calibration FAIL and takes the EXISTING branch (D1-fail → C on F16 → E distill). No new trigger beside D1's verdict.
  (b) **Absolute-floor failure** — a G1 read of raw-FAILS-the-floor (below the conformal-naive floor — an absolute check, NOT a Δ read) on ANY gradient-trained head, F16 or quantized, is the standing 562/576 tripwire. For the head Lane C retrains, the fix is C's pre-declared loss option (Lane C's tripwire landing-pad bullet, above) — no separate filing. For any OTHER seated head (C never retrains it), the tripwire applies verbatim: the narrow riir-train calibration-aware-loss issue files THAT DAY for that head's own retrain.
- [ ] Determinism gate: quantized kernels may not be bit-identical across
      Metal and CUDA — the frozen-read determinism re-seat runs PER DEVICE.
- [ ] Adoption re-seats every cell (frozen reads invalidated by any
      numerics change — by design, never grandfathered) and forces the
      Lane C retrain (heads see shifted inputs).

## Lane E — distill the encoder (conditional; on D1 OR D2 retention failing)

- [ ] Student encoder (layer truncation or tiny student) over laya teacher
      encodes; heads retrain cheap on top. Biggest potential (sub-300 MB
      posture) but a full train cycle. TRIGGER: D1's retention verdict
      FAILS (fake-quant retention below the non-inferiority bar — the
      probe that gates D2's existence), or D2's full adoption fails its
      gates. D1 pass ⇒ D2 before E; D1 fail ⇒ E directly (D2 never
      exists to wait for).

## Non-goals

- No per-request loads, ever (L9's residue survives as G3 + the failure
  bound).
- No HOSTED-ONLY artifact leaves controlled hardware (the moat law, A10).
- No manifest default change: `eager` remains the embedded default; `lazy`
  is a per-host deployment choice.
- No public-release encoder artifacts (laya's license allows derivatives;
  the class stays hosted regardless).

## Order of execution — a BRANCH, decided by D1's verdict

**B → A → D1, then D1 chooses C's input encoding:**

- **D1 PASSES:** → **D2** (adopt Q8, re-seat) → **C** (train the typed
  head ONCE, on the adopted Q8 encode) — C never trains on F16 in this
  branch; training it there first would be a double spend (D2 would force
  a second C retrain).
- **D1 FAILS:** → **C** (on the F16 encodes, typed cell served on one
  checkpoint or recorded negative) → **E** (distill — the quant road is
  closed by measurement, not by preference).

- B first: pure plumbing, gated by interleaved bit-identical parity —
  gives A a regression floor before A touches validator semantics.
- A second: the inheritance law, with boot preflight + ready-cold +
  worker-unit release semantics.
- D1 third (the fake-quant probe prices everything downstream), C/D2
  fourth per the branch, E only on the fail arm.
- The /#sizes row + its note re-measure and update after each landing.

## Verdict record (Claude)

- Round 1 (2026-10-01): **REVISE** — reasons, all folded in above: (1)
  lazy moves boot-time refusals onto the first request unless a boot
  preflight runs for lazy rows (weights BLAKE3, head digest, template
  class-count without a forward, memory budget) + healthz needs
  `ready-cold` ≠ `ready`; (2) Lane A's gates must be written against the
  shared worker, not the lane (release drops a sender, the worker's 848 MB
  survives until the last holder releases; G3 counted per worker; epoch
  per-lane, checkpoint digest on the worker); (3) "serialization costs
  nothing" is false ACROSS lanes (head-of-line blocking) → mixed-load
  latency + fairness/queue gates + the share-weights-only fallback;
  interleaved (not sequential) parity replay; warmup/template validation
  moves to lane attach; ship behind a GOAT flag; (4) Lane D's "LB95
  contains zero" is not equivalence → non-inferiority with pre-declared δ
  and UNDECIDED small families + calibration (Brier/ECE) + per-device
  determinism gates; (5) D invalidates the heads' F16 inputs → D1 probe
  before C, order now B → A → D1 → C → D2 → E; (6) G3 needed a failure
  bound (named `Failed` + backoff, never a retry storm).
- Final round (2026-10-01): **AGREE** (round 3 of 3; the reviewer grep-
  verified every fold by line). Non-blocking notes both applied: the
  status line now names the final verdict + round count, and Lane C
  carries the Q8 encode-cache regeneration cost (the 2.4 GB cache is
  F16; the D1-pass branch regenerates it under the adopted quant as part
  of D2 → C). The verdict's own summary: the L9 inheritance argument
  stands (lazy-once-then-resident + boot preflight + ready-cold +
  per-worker G3 + failure bound), B-before-A gives A its regression
  floor, every gate names its instrument, the fallbacks are recorded.
  Execution is owner-triggered, in branch order: B → A → D1, then
  (D2 → C) on a pass or (C → E) on a fail.

## Execution addendum (2026-10-02, overnight — the owner-triggered session)

**Owner trigger recorded:** the owner referenced this issue directly
("did this change the plan?") and delegated owner-calls for the night
("any owner call you can ask Claude... expect all done when i woke up"),
with the posture note "prod higher tier is gpu so i dont bother cpu that
much, only simd is fine enough". Claude verdict round 1 (REVISE, folded):
board-first; Lane B/A plumbing deferred to a daylight session (they are
RAM/latency plumbing — exactly what the GPU-tier note deprioritizes); D1
after board work if the night allows.

**The bekko question (the owner's "it better than laya? or fusion?"):**
NO plan change to this issue. Bekko is an external subprocess oracle
(hotchpotch/bekko-system-one, reflex Bench 103) — comparison-lane only,
not a sealed artifact (A8/A10), unservable, and cannot enter the H1/H2
fusion lanes (fusion fuses sealed specialist artifacts). Recorded as
SUPPORTING INTEL for Lane E's premise: bekko-68m (68M params) beat the
reflex modelless floor on 4/7 general suites and lost to the seated
Rethink cells on every shared suite except massive (0.8667 vs ENC
0.6567) — a 68M model being competitive on general suites strengthens
the sub-300M-student case Lane E prices. On S1MB (card-reported,
unreproduced — riir-train 607): laya-typed 15.00 / laya-english 13.36 /
laya-multilingual 9.28 vs bekko-68m 40.46 — the laya family leads the
quality axis by a wide margin.

**⚠ Two D1s exist — naming collision clarified:** instinct Issue 016's
D1 ("the sst5 cell publishes as served") stays TRIGGER-BLOCKED (needs a
real GPU serving deploy — owner-adjacent creds, NOT covered by tonight's
delegation). THIS issue's Lane D1 (the fake-quant retention probe) is
owner-triggered and measured-precondition-free; per the amended order it
runs after the board work.

**Tonight's executed state:** the Rethink board is COMPLETE (reflex-site
`2e26f84`, CF `ef773e79`): 9 record-only encoder cells seated (4 prior +
massive 0.6567 / banking77 0.4420 / prompt 0.8017 / thai 0.4075+0.7843),
emotion disclosed ("no head earned" — the 6th fresh-seed fit refused on
holdout, the test-cherry-picked 0.62 never published, per the verdict
fold), code_fixtures + the 6 harness families disclosed (law-excluded).
Lane D1 NOT yet run (the night went to the board + the 603 multilingual
screen — DEAD BY LAW, 0.8433 ≤ 0.86, riir-train 603 closed); D1 remains
NEXT in the amended order, followed by the branch (D2 → C on a pass, C →
E on a fail). Lane B/A: daylight session.

**Update (2026-10-02, the follow-on session): D1 EXECUTED — PASS.**
Bench 0046 (`.benchmarks/0046_d1_fakequant_retention/` — pre-registration
`33d02ed` BEFORE the reads, instrument riir-infer `294999e` + instinct
`2a61105`, record + gate arithmetic in the bench dir): 5 PASS · 2
UNDECIDED (massive, banking77 — named, never folded) · 0 FAIL ⇒
**the branch fires D2**: real Q8 adoption in riir-infer, re-seating every
cell at the adopted numerics (the frozen reads invalidate BY DESIGN), the
per-device determinism gate, THEN C on the adopted Q8 encode. D2 is a
multi-session kernel job (riir-infer owns the format; the probe's block
layout is the storage format D2 lands); C follows immediately after. Lane
B/A remain the daylight plumbing session.

**Update 2 (same session): D2a EXECUTED — the storage tier ADOPTED.**
Bench 0047 + riir-infer `10e33de`: the Q8_0 artifact lane (converter with
a built-in byte-equality refusal, sidecar-verified loader variant,
`laya-quant8` bin) — english + typed converted and proven (842.6 →
447.7 MB, 53.1%); the sst5 end-to-end spot-check reproduced the probe's
read byte-identically through the real artifact, so Bench 0046's gate
table IS the adopted numerics' re-seat. NEXT: D2b (device-resident Q8 +
fused kernels — the per-device determinism gate's real subject) or Lane C
(the typed head retrain on the adopted encoding — trainable on this box;
the D2b kernels are the multi-session job). Lane B/A: daylight session.

**Update 3 (2026-10-02, the follow-on session): Lane C EXECUTED —
NEGATIVE (Bench 0048, pre-reg `8a3b721`).** The adopted-encoding (q8,
english) encode pass ran clean; the v2 trainer refused the earn bar
(head holdout 0.3113 < its own reference 0.3600; train acc flat across
all 15 epochs); the frozen test read stayed RESERVED (never spent); the
re-seat gate never ran. The english checkpoint is AT OR BELOW CHANCE on
every presented width (ref-only floor 0.3595 vs the F16 twin 0.3575,
Bench 039's recorded "substrate the lane would NOT ride") — a
representational ~45-pt deficit vs the typed floors, not a calibration
one, so the CE+λ·Brier landing pad was deliberately NOT spent (reasoning
in the bench record). Typed keeps `typed_encoder_v2.bin` (typed ckpt) as
seated; Lane B's value is RAISED (the two-resident-checkpoint posture
stands); Lane E NOT triggered. reflex-site untouched (no re-seat, no
board change). **AMENDED ORDER NOW: C closed (negative) → D2b kernels →
Q4/PQ2; B/A daylight.**

**Update 4 (2026-10-02, the D2b session): Plan 616 Phase 1 DONE — host
residency landed (riir-infer, all gates green).** The q8 GEMM weights now
carry RAW (never widened host-side under Metal): `Weight2D { Dense | Q8 }`
through Encoder/Head, the Backend trait's q8 family with widen-once
defaults (CPU/CUDA/CubeCL resolve byte-identically), Metal's `q8_widen_t`
load kernel dequant-transposing into the SAME device F32 Wᵀ (MPS
unchanged). Bit-identity proven at three levels — synthetic (CPU + Metal,
packed/fold/unfused/first-miss/k-tail), op-level (`metal_ops_smoke` q8
arms), and the LIVE english artifact A/B: byte-identical answers,
**RSS 1489 vs 4238 MiB (2.85× whole-process host cut**, the ~2.7 GB delta
== Phase 0's derived 1.685 GB widened-f32). Kill-switch
`LAYA_Q8_HOST_F32=1` (bit-restoring, the probe's control arm); the F16
gates (reflex G5 parity green against the in-flight
substrate). Layout lesson recorded in the plan (flat-block Q8 order, not
per-row). No bit moved ANYWHERE → the re-seat is the no-change proof when
Phase 2 lands. Phase 2 (device-resident Q8 + fused staging + the MPS A/B)
is the next session; Q4/PQ2 rides its format seam.

**Update 5 (2026-10-02, the D2b Phase 2 session): Plan 616 Phase 2 DONE —
device-resident Q8 + the fused staging kernels landed (riir-infer
`8c250a2`, all gates green).** The weights now hold ONE raw-bytes buffer
on the device (`weights_q8`, 1.0625 B/elt) and the three fused kernels
(`sgemm_q8`/`sgemm_xwide_q8`/`sgemm_splitk_q8`) stage from the native
blocked bytes with k-fastest lanes — one block per warp, warp-uniform
scale, the same tile math as the f32 instances (bit-identity by
construction, proven at all 24 dense A/B cells + 7 shared-tree gate
shapes + the fold arms). Option (i) shipped: MPS off on the weight shapes
under q8 (the pre-T13 split rule, one loud disclosure, fused/widen reach
counters); `LAYA_Q8_DEVICE_F32=1` restores Phase 1. **Measured: device
residency 1654.9 → 348.8 MiB (4.74×); whole-process RSS 1492 vs 1881
(Phase 1) vs 4826 MiB (Phase 0); the fused posture deterministic ×2
byte-identical; the Phase 1 tree byte-identical (device-f32 == host-widen);
the option-(i) dispatch delta 5.4e-7 probs / 7.2e-7 conf / act exactly 0 —
two orders under G5 (the T13-class accumulation-order change, measured and
bounded).** THE PRICED COST, now with per-cell numbers: fused/mps
1.22–2.07× on the dense cells (m-scaling; geo 1.42 @ m106 → 1.96 @
m1700), fused/narrow ~1.15 (the in-staging dequant ALU — the T11 L2
finding means the halved B bytes buy nothing); whole-forward single-question
paired median **1.002×** (the dense cells are a small share at the serving
shape; PROVENANCE power=AC load 4.17 powermode 2-high). **No F16 bit
moved (Dense → matmul_w → MPS untouched) — the seated cells stand; the
q8 posture's re-seat is Phase 3's adoption record.** The re-pricing
condition for option (iii) is MET with numbers: long-prefill q8 serving is
where an MSL MPS-replacement would earn its bench — owner call.
REMAINS: Phase 3 (adoption + re-seat + the Q4/PQ2 seam — rides the
staging's format constant); Lane B/A daylight.

**Update 6 (2026-10-02, the D2b Phase 3 session): Plan 616 Phase 3 DONE —
D2b ADOPTED (the no-change re-seat), the /#sizes row re-measured at the
adopted posture, and the Q4 seam LANDED (riir-infer, all gates green).**
- **The adoption re-seat is the no-change proof, recorded like 0047's:**
  no bit moved at any seated cell's surface. The F16 path (Dense →
  `matmul_w` → MPS) is byte-untouched — G5 parity green at BOTH postures
  against the in-flight substrate (CPU 27.5 s / Metal 10.7 s), so sst5
  0.5267 / xnli 0.8600 / ag_news 0.9475 / typed 0.7550 STAND (the third-
  posture witnesses carry). The q8 posture's own evidence: the live probe
  re-run at HEAD reproduced Phase 2 exactly (fused deterministic ×2
  byte-identical; the option-(i) dispatch delta 5.364e-7 probs / act
  exactly 0, two orders under G5; RSS 1491/1880/4825 MiB; device
  residency 348.8 vs 1654.9 MiB = 4.74×). D2b is the q8 serving posture.
- **/#sizes re-measured** (reflex-site `78488db`): the rethink_encoder
  row's model = the adopted q8 artifact 447,729,135 B + heads 7,994,976 B
  = 455,724,111 B (total 484,472,795 with the engine — 44% under the F16
  row); the note leads with the served posture's memory line (348.8 MiB
  device / 4.74×, RSS 1,492 vs 4,825 MiB); F16 stays as the legacy
  reference line.
- **The Q4 seam landed — the FORMAT tier only; Q4 RETENTION IS D1-PRICED
  SEPARATELY before `LAYA_WEIGHTS_VARIANT=q4` serves anything.** The
  staging decode's format constant is the seam, exactly as scoped:
  decoder (`RawQ4`/`widen_q4_0` — the house Q8 law's 4-bit shape: block
  32, scale f16(amax/7), signed grid [-7,+7], GGML nibble order,
  0.5625 B/elt), converter (`laya-quant4` over the NEW shared
  `blocked_artifact` container machinery — a third format lands as a
  decode + an encode loop), device tier through `QFmt` (the q4 MSL
  kernels DERIVED from the shipped q8 texts by exact token replacement —
  the decode block swapped, the tile math verbatim; `q4_widen_t` the
  load-kernel twin; per-format reach counters), and the bit-identity
  battery at Q4's OWN fidelity ALL GREEN (`q4_widen_identity` 6/6 + the
  metal_ops_smoke q4 arms at 7 shapes + folds + kill-switch; the q8
  battery byte-unchanged; lib 56/56; clippy -D ×4 postures; G5 both
  postures green). PQ2 rides the same seam as the next format row.
- **The cell-identity law stays**: the q8/Q4 postures carry their OWN
  batteries at their own grids; no seated F16 cell is re-read at a
  quant grid without its own D1 pass (the D1 branch law, unchanged).
  REMAINS: option (iii) re-price (owner call, the numbers stand); Lane
  B/A daylight; Q4 retention probe (D1-shaped) when the Q4 tier is
  wanted for serving.

**Update 7 (2026-10-02, the daylight session — Lane B + Lane A LANDED,
Bench 0050):** the checkboxes above carry the per-gate evidence; the
record is `.benchmarks/0050_lane_b_a_serving/RESULTS.md`. Headlines:
**RAM 4938 → 1690 MB private (2.92×)** at 3 ENC lanes / 1 checkpoint
(CPU/F16 posture); **cross-build parity byte-identical** (per-lane vs
shared-worker fingerprints sst5 `c35a50b77602c063` · xnli_en
`7c237555b7ab47fd`); the six weight-free registry gates (exactly-once
load, release-frees, recall-one-load, fairness cap, attach drift,
failed-boot retry) all pass; the lazy ENC posture + boot preflight +
`ready-cold` landed; the substrate gained `verify_checkpoint_present`
(riir-infer — the verify-only half of ensure_checkpoint, never
downloads). Concurrency gate honestly PARTIAL (the lockstep replay is
the instrument and it is bit-identical; the concurrent-HTTP p50/p99 A/B
defers to the serving soak — the fairness cap and the share-weights-only
fallback are the recorded hedges). REMAINS: promotion of
`serve-encoder-shared` to the serve-encoder default (rides the soak);
the concurrent-load p50/p99 A/B at the soak; option (iii) re-price and
the Q4 retention probe stay owner-gated as before.

**Update 8 (2026-10-02, the duplicate session — TWIN LANDING, disclosed):**
an M3 session implemented Lane B + Lane A INDEPENDENTLY in parallel with
the 4090 session and pushed second — origin's commit `41234d4` is
canonical, the duplicate (`dbf790d`, flag name `shared-encoder-worker`,
own pool module) died in the reflog per the twin-landing law. What
survives from it, disclosed as evidence rather than re-landed:

- **The deferred concurrency cell, MEASURED at the Metal posture** (M3
  Max, AC, F16, dev profile BOTH sides, like-for-like; 6 clients × 12
  decisions round-robining 3 lanes, n=72, tail support 1/72):
  per-lane p50 51,227 µs / p99 195,727 µs; shared p50 93,296 µs
  (**1.82× — the head-of-line cost**) / p99 236,456 µs
  (**1.208× ≤ the 1.217× loaded-box envelope — HELD, ~0.9 pt margin**);
  RSS 10,207 → 4,153 MiB whole-process (**2.46×**) at 3 lanes / 1
  worker. ⚠ The p99 margin is THIN — a busier box can breach it; the
  share-weights-only fallback is the recorded remedy. **Instrument
  disclosure: the probe targeted the DUPLICATE's internals (its own
  pool API) and died with it — these numbers are a one-session reading,
  not a landed gate; the soak owns the decision cell.**
- **The frozen-read parity through a shared lane**: the Bench-029 sst5
  replay (316/600 EXACT, arena leg 32/32) passed through the DUPLICATE's
  shared-worker lane at Metal — cross-posture bit-identity evidence of
  the same design shape 0050 proved by fingerprints.
- **TWO landed fixes found en-route** (both on top of `41234d4`):
  (1) `load_lane`'s `check_winner_file` applied the BAG winner coupling
  to ENC rows — a banking77 ENC row (head ≠ the bridged
  `banking77_nbsvm_v2.bin`) could never boot through the serve binary;
  the parity gates bypass `load_lane`, which is why the class survived
  both 016 and 0050 (fixed: the ENC skip, mirroring server.rs's route);
  (2) the reflex sibling's plan-011 `RunOptions.clef` — arena.rs's A0
  drift pin took the one-field ripple (`clef: false`; origin/develop
  was RED without it).
- **FOLLOW-UP (explicit, not silently dropped):** a runnable
  concurrent-load A/B instrument on the LANDED implementation (public
  surface only — the registry is private by design) so the soak cell
  re-measures without re-deriving; plus the promotion pull of
  `serve-encoder-shared` after the soak.

**Update 9 (2026-10-02, the follow-on M3 session — the Update-8 follow-up DISCHARGED,
Bench 0054):** the runnable A/B instrument landed on the LANDED implementation and the
concurrency cell was MEASURED through the public surface. Instrument:
`scripts/encoder_load_ab.py` (stdlib-only; builds both postures, generates the ENC manifest
from the heads' `.blake3` sidecars — the boot's drift gate validates for real — boots the
serve binary, polls /healthz readiness, drives N clients × M decisions round-robining the
suites over REAL test states from the frozen pool rendered in each suite's serialized-state
envelope, reads RSS, and emits `.benchmarks/0054_encoder_load_ab/{RESULTS.md,results.json}`
with the preflight PROVENANCE line embedded). The soak's decision cell is now ONE command:
`python3 scripts/encoder_load_ab.py --clients 8 --rounds 40`.

**The cell, at the heavy shape (8 clients × 40 rounds = n=320, tail support 4, release,
Metal/F16, lanes sst5/xnli_en/ag_news on the english checkpoint — exactly 0050's 3-lane /
1-checkpoint shape):**

| | per-lane | shared | shared ÷ per-lane |
|---|---|---|---|
| p50 | 60,173 µs | 93,178 µs | 1.549× |
| p99 | 361,609 µs | **132,204 µs** | **0.366×** |
| RSS steady | 10,192 MiB | 4,100 MiB | **2.49×** |

**The reading INVERTS Update 8's thin-cell hedge at the tail.** At real contention the
per-lane p99 BLOWS OUT (326k/362k across two heavy runs — three concurrent Metal forwards
per checkpoint contend for the GPU; the tail is spread across 10+ of 40 rounds, server-side,
not start skew) while the shared worker's queue makes the tail deterministic (126.6k/132.2k,
p99 ≈ 1.4× its own p50). The share-weights-only fallback is NOT needed for the tail — the
tail is where Lane B WINS. Update 8's 1.208×-within-envelope reading was under-powered
(n=72, dev profile, the duplicate's private internals) — its own disclosure said the soak
owns the cell. The p50 cost of sharing is real and stable: 1.38–1.62× across five runs.
**RSS: 2.49× at five independent replications** (Update 8's 2.46× + this session's 2.47–
2.49× across runs, orders, profiles).

**Decision parity across postures: byte-level, 4/4 instrument runs** — the warmup
decisions' `receipt.decision` digests equal across the two builds over the public surface
(the witness 0050 proved via test fingerprints, now proven through HTTP). A divergence
exits 2; the instrument is therefore also a standing posture-parity gate for the soak.

**Instrument lessons paid for en-route** (all in the script, all measured): the ENC lane
consumes the suite's serialized-state ENVELOPE (pyjson Python separators — sst5 `{"text"}`,
ag_news `{"article"}`, xnli_en `{"premise","hypothesis"}`; a raw string 422s at the edge);
each client's FIRST decision is a thread-start+connect-storm throwaway (the heavy cell's
4.3 s outlier sat ENTIRELY in round 0) — excluded, recorded as `client_warm`; the A/B ratio
line resolves the postures BY NAME (an order flip must not invert the label).

**Disclosures:** the canonical run carries a box-noise PAIR in the shared arm (two walls
3,645,289/3,645,531 µs, 242 µs apart — a simultaneous release = a system-wide stall from
the concurrent sibling sessions; above the reported p99, steady max outside the pair 134k);
the p99 tail support is 4/320; the light n=72 cells carried tail support 1 — why the heavy
cell is the canonical one. PROVENANCE embedded in the record
(`power=AC load=3.05 powermode=2(high)` at launch, sibling agents active).

**REMAINS:** promotion of `serve-encoder-shared` to the serve-encoder default — still
owner-gated, but the soak's evidence cell is now one command, and the evidence FLIPPED
favorable at the tail (p50 1.55× bounded cost vs p99 0.37× + RSS 2.49×); option (iii)
re-price + Q4/PQ2 retention probe stay owner-gated as before; Lane E not triggered.

**Numbering disclosure:** this record was born `0052_encoder_load_ab` off the session-start
highwater (0051) and RENUMBERED to `0054` the same session — the sibling session sharing
this worktree allocated 0052 (`0052_hybrid_quotable_ladder`) + 0053
(`0053_massive_synth_0052`) mid-session and pushed `d66db05` while this record was being
written; the write-time re-read caught it (the AGENTS.md `ls` + re-read law), zero inbound
mentions moved, the highwater counter restored then advanced to 0054. The dual-allocation
gate's lesson reproduced exactly: `.highwater` read at session START is a different timeline
by write time in a shared worktree.

**Update 10 (2026-10-02, the owner-approvals session — menu items A + B executed):**
the owner approved exactly A (promote `serve-encoder-shared` after a green soak) and B
(option (iii) re-price + the Q4/PQ2 retention probe); C–K stand as recorded (C and D
explicitly not approved).

**A — THE PROMOTION LANDED.** The soak ran at HEAD pre-promotion (the 0054 cell,
`--clients 8 --rounds 40`): **GREEN** — p50 1.438× · p99 **0.368×** · RSS **3.12×** ·
decision parity OK (PROVENANCE `power=AC load=4.94 powermode=2(high)`, the same disclosed
class as the canonical run). The 0054 RESULTS.md now carries the ADDENDUM with both runs
side by side (the instrument regenerates the file per run; the canonical numbers live at
`b6d40de` and in Update 9). Every promotion gate measured green: parity (0050 cross-build
fingerprints + through-HTTP receipt digests, twice), RAM (2.92× private at 0050 · 2.49–
3.12× whole-process across three heavy-shape runs), mixed-load latency (p50 bounded
1.38–1.62× across five runs — the recorded cost; p99 favorable 0.366–0.368× across FOUR
heavy-shape runs — the product claim: the tail is what users feel under load), fairness
(LaneGate), soak (canonical + this confirmation).

The promotion itself: **`serve-encoder = ["arena-laya", "serve-encoder-shared"]`** (cargo
unifies the two-feature cycle — `cargo metadata` green; both spellings remain valid
closures of the same posture). The **runtime demote switch
`RIIR_INSTINCT_ENCODER_SHARED=0`** (the exact literal, `encoder_serve.rs`) bit-restores
the per-lane keys — byte-identical serving decisions (the 0050/0054 parity law), so the
recorded share-weights-only FALLBACK stays dormant (the tail needs nothing) and the RSS
cost line reads: shared is 2.5–3.1× CHEAPER, the p50 1.44–1.55× is the accepted price.
`encoder_topology_label()` is the one label home ("shared-worker"/"per-lane"); the parity
gate's label now reads the effective topology (feature AND env). Post-promotion gates:
13 weight-free registry tests green at the promoted posture · clippy `-D` clean at the
promoted, per-lane and default postures · the A/B instrument's per-lane arm sets the
demote switch (a no-op pre-promotion, the construction post-promotion).

**B — the re-price RECORDED, the probe LAUNCHED.**

- **Option (iii) — the MSL MPS-replacement dense GEMM — RE-PRICED and DECLINED at
today's shapes** (the owner-approved call; the numbers stand): Phase 2 measured
fused/mps 1.22–2.07× on the dense cells (geo 1.42 @ m106 → 1.96 @ m1700) but
**whole-forward 1.002× at the single-question serving shape** — a full MPS-class kernel
rewrite buys ≈0.2% whole-forward in every lane the product serves today. T13's history
is the risk record: we MEASURED LOSING the dense-GEMM race to MPS at F16 (−26…−44% p50);
option (iii) is that same race with a harder kernel (fused dequant staging). The earning
condition the plan named — long-prefill q8 serving at big m on a memory-rich host —
exists in NO product lane today. **RE-ARM TRIGGER:** any real serving posture running q8
at long-prefill (m ≥ ~1024 dense) on a memory-rich host re-opens (iii) with its own
bench; until then `LAYA_Q8_DEVICE_F32=1` is the documented escape at exactly those
postures (Plan 616's own wording — mirrored there).
- **PQ2 — recorded as a DEPENDENCY, not a defer:** no PQ2 grid exists (the Q4 seam's
"next format row" is unlanded), and a retention probe cannot precede its format — the
issue already ordered it that way ("PQ2 rides the same seam as the next format row").
Its probe is D4-shaped the day the grid lands.
- **The Q4 retention probe (Lane D4) is PRE-REGISTERED and running** (Bench 0055):
riir-infer gained `WeightPosture::FakeQuantQ4` (`41f7e67` — the agent arm over
`fake_quant_q4_map`, the grid the Q4 converter already proved; values byte-identical to
the Q4_0 artifact's decode, so the probe's verdict IS the artifact tier's retention) and
the arena gained `--fake-quant-q4` (mutually exclusive with `--fake-quant`). Margins =
D1's VERBATIM, declared flat before any read (the serving bar is a product constant —
a looser Q4 bar would make the two format verdicts incomparable). The declared prior:
Q4's grid is ~16× coarser; the likeliest failure candidates are the fine-grained heads
(banking77, massive) — written before the read. The verdict gates ONLY the Q4 tier's
serving eligibility; it never re-opens the Q8 adoption.
