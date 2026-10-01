# Issue 018 — Rethink encoder lane: lean + GOAT plan (lazy inheritance, shared worker, one-checkpoint typed, quantization)

**Status:** OPEN — filed 2026-10-01 (owner ask after the /#sizes Rethink row landed); Claude verdict CONVERGED AGREE at round 3 (2× REVISE, all reasons folded); plan ready to execute in branch order, nothing coded yet. Addendum 2026-10-01: two-case calibration branch wired (Δ-retention breach routes through D1's existing verdict, pre-registered refit allowed first; absolute-floor tripwire lands in Lane C's pre-declared loss options for the head C retrains, narrow filing for any other head) — katgpt-rs Research 562 addendum; nothing fired, execution order unchanged.

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

- [ ] One worker thread + one `RiirAgent` per (Checkpoint, device), shared
      by every ENC lane on that checkpoint; heads stay per-lane (KB-MB).
      The worker carries the CHECKPOINT DIGEST — the epoch stays per-lane
      with its head (swap/monotonicity must NOT attach to the shared
      worker).
- [ ] Per-suite warmup + template validation move to LANE ATTACH (the
      worker no longer knows a suite's template at boot); a template drift
      fails that lane's attach loudly, never the shared worker.
- [ ] Ship behind a GOAT feature flag; promote to default only after
      parity + RAM + mixed-load latency all pass (the promotion gates
      below).
- [ ] RAM gate: measured N × 848 MB → 1 × 848 MB for the 3-suite host.
- [ ] Parity gate — INTERLEAVED, not sequential: replay all served suites
      interleaved through the shared worker and compare bit-identical
      against isolated per-lane runs (catches state leaking between
      suites: agent scratch, the reused `y` buffer, device caches).
- [ ] Concurrency gate: mixed-suite p50/p99 under concurrent load, shared
      vs per-lane, interleaved A/B with `bench_preflight.sh` box state
      quoted (±6%/±21.7% envelope law). Serialization is the law WITHIN a
      lane; across lanes today's three workers can overlap on the device —
      one shared worker adds head-of-line blocking, and that cost is
      measured, never assumed.
- [ ] Fairness/queue gate: per-lane fairness and queue bounds (today each
      lane owns `sync_channel(64)`; one shared queue lets a burst on one
      suite back-pressure the others — bound it or shard it).
- [ ] FALLBACK (recorded, not silent): if p99 regresses beyond the
      envelope, keep per-lane workers and share only the WEIGHTS
      (mmap/Arc of the checkpoint) — RAM win kept, latency trade recorded.

## Lane A — L3 inherits L2's lazy + evict (the L9 revisit) — riir-instinct

Preserve L9's REAL intent (never a load ON the hot request path, never
per-request loads) while inheriting A7's semantics verbatim — with lazy's
failure surface closed at BOOT, not at first request:

- [ ] Allow `budget.load = "lazy"` on ENC rows: the lane boots `Unloaded`;
      the FIRST decision triggers the load (agent + head + one warmup
      encode paying the Metal/CUDA pipeline compile); the 503 `loading`
      window covers it (the bag-lane posture, state named, never a silent
      fallback); the lane is resident after.
- [ ] **Boot preflight runs for lazy rows too** — lazy must not move
      boot-time refusals onto the first request: weights present + BLAKE3
      digest check; head digest check; template-vs-head class-count check
      WITHOUT a forward pass (a property of template + tokenizer); a
      memory-budget check against device capacity — **counted per shared
      worker** (the sum over DISTINCT (Checkpoint, device) workers, never
      over lanes: three english lanes share one worker and must be charged
      848 MB once, or a host that fits gets refused). A config error is a
      boot refusal, never a production 503.
- [ ] **Readiness semantics**: healthz reports a lazy-Unloaded lane as
      `ready-cold`, distinct from `ready` — a load balancer must not route
      the first user into the compile window blind.
- [ ] `POST /arsenal/release` evicts back to Unloaded (epoch kept) — the
      existing wire, no new lineage code. Under the shared worker (Lane
      B), release drops the lane's SENDER; the worker and its 848 MB
      survive while another lane holds one; RAM is freed when the LAST
      holder releases — the disclosure says so.
- [ ] `eager` stays the DEFAULT (the production posture unchanged);
      `lazy` is the opt-in for memory-constrained / multi-checkpoint hosts.
- [ ] Disclosure: healthz + the decision receipt carry the lane's load
      state; a cold first decision names the load+compile cost it paid; a
      released-then-recalled lane re-pays the compile ONCE per activation
      (bounded, disclosed).
- [ ] The validator's ENC eager-only refusal becomes: `eager | lazy` both
      valid for ENC (lazy rows carry the one-time-load contract).

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

- [ ] Train typed v2 (per-option) head over the ENGLISH encode of the
      typed pool — the inputs exist (`typed_test_lenc_english.bin`, the
      2.4 GB english encode cache, riir-train `.raw/t608`).
- [ ] T2 gate: paired LB95 of the english-encoded head vs the seated arms
      on the frozen test read; the typed cell re-seats ONLY on a win (the
      third-posture cell-identity witness re-run — arena, then serve-path).
- [ ] NEGATIVE branch: if it fails the bar, typed stays measurement-lane
      only and this issue records the number — no posture rollback.
- [ ] Input-cache note (the D1-pass branch): C's training input on that
      branch is the adopted Q8 ENCODE of the typed pool — the existing
      2.4 GB english-encode cache is F16 and must be REGENERATED under
      the adopted quant (an encode pass over the pool, riir-train side).
      That regeneration cost is part of D2 → C, budgeted, never a
      surprise.
- [ ] D-interaction (recorded): heads are fit to F16 encodes — Lane D2's
      adopted quant invalidates these inputs and forces a C retrain +
      re-seat. The ORDER BRANCH below resolves this: on a D1 pass, C runs
      AFTER D2 and trains on the adopted quant's encode cache exactly
      once; C never trains on an encoding D2 is about to change.
- [ ] **Tripwire landing pad (562/576; wired 2026-10-01):** if the Lane D absolute-floor case ever fires on a trained head, C's retrain is where the fix lives — CE+λ·Brier becomes a PRE-DECLARED loss option for the Lane C head training (CE+Brier is Raschka's supervised form, katgpt-rs Research 576's PASS-Redirect line, author-caveated "modest gain, may not generalize"; the RL-family scoring-rule recipes are 576 §3's log/spherical/RPS — the loss form is chosen from that family at filing time; the reward-shaped twin already ships in riir-train `svr.rs::r_verify` `λ_cal`). Until a raw-FAILS reading exists, C trains plain CE — the refit-first posture is the shipped primary, now the independent consensus (562 addendum S2).

## Lane D — quantize the english checkpoint — riir-infer + riir-train

Split into the CHEAP PROBE (D1) and the full adoption (D2) — the probe
prices everything before any training or kernel work is spent:

- [ ] **D1 — the retention probe (cheap, first):** FAKE-QUANT instrument
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
- [ ] **D2 — full adoption** (only on a D1 pass): Q8 quant in riir-infer
      (house dequant machinery; the Bonsai PQ precedent), Q4/PQ2 second.
      English 848 MB → ~424 MB (Q8) / ~220 MB (Q4) → posture total
      ~460 MB / ~255 MB.
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
