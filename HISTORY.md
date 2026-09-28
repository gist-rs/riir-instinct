# HISTORY — riir-instinct

## 2026-09-28 — the 581 lane lands end to end: typed_decisions seats, certifies, and SERVES (Bench 015 + Issue 011)

The one-day arc, three repos, every gate measured:

- **Root cause (riir-train `d59d3984`, Bench 614)**: Bench 014's
  "template drift" was a FETCH-CAP ARTIFACT — reflex's fetch took 800 of
  typed_decisions' 1200 train rows; the 300 security_incidents train
  rows sat behind the cap. The owner-gated T1 shape call was adjudicated
  by a Claude verdict round (2 rounds, REVISE→AGREE): shape (b) — REAL
  rows — with the reviewer's additions adopted (snapshot byte-identity
  12/12, disjointness 0 collisions over three hashes, an A0′ arm on the
  same data, and registration gated on max(A0, A0′)). Retrained at
  610's config over 1200 rows / 49 classes: case 0.5813 vs majority
  0.4875, event 0.6350 vs lr0 0.0200 — re-minted
  `typed_decisions_armA_v1.bin` (blake3 `7f7a39e1935f8665…`). Issue 581
  closed; reflex's own cap fix filed (riir-reflex Issue 052 — a +10.7 pt
  MODELESS A0 gain for their lane).
- **The consumer read (Bench 015, `745d157`)**: control run first — 014
  reproduced byte-identically. Then the extended dir: A0′ 0.5725 · A1
  0.6300 (registered) · H1 0.6335 (paired noise vs A1, LB95 −0.0048).
  T2-certified against BOTH A0 legs (vs A0′ +0.0373; vs the published
  0.4655 +0.1432); G1 PASS. The honest breakdown is in the record: the
  pooled win is not mainly the new workflow (invoice A0′ BEATS A1), and
  rare keys collapse to the majority class. Latency rows taken at load
  8.06 — disclosed, directional only.
- **The serving extension (Issue 011, `e599ecc`)**: the serve path's
  one-question shape guard could not boot the suite (and `--suites`
  defaults to all manifest rows — a row would have broken the default
  boot). The `decision_wire` law ("one state, ALL questions answered in
  one call") decided the shape: `SuiteServer::decide_multi` (one
  multi-question case through the same eval_seat path; state bagged
  once; the noul resolve JOIN-FORM-aware — Named keeps the positional
  law, a Context join resolves NOUL_PAIR by name, an empty presentation
  takes the fixed rendering), the ServeContract axis on `winner_bridge`,
  the HTTP `questions` form beside the byte-compatible legacy body, and
  the manifest's EIGHTH row (A1) with the digest/dig/posture/deploy
  pins moved together. Validation: serve_gates 14/14 (the typed parity
  gate replays 12×5 questions against the 015 frozen picks), lib 49/49,
  all-features 60/60, clippy clean both postures, live smoke green
  (5 decisions in one call; the single-question form refuses 422
  bridge_undefined).
- **Issue 011 removed per noise-reduction** — this record + the code
  pins carry it.

## 2026-09-28 — issue-file hygiene: 001 / 002 / 004 removed (work verifiably landed, records durable)

One action per the fleet noise-reduction rule; the files were the last
copies of records already carried elsewhere:

- **001 (HOSTED-ONLY vessel reader)** — CLOSED 09-27, all T1–T5 landed
  (`src/vessel.rs` + `tests/vessel_gates.rs` 7/7, opt-in feature `vessel`).
  Durable record: AGENTS.md "P4 — the HOSTED-ONLY vessel reader".
- **002 (hosted instinct lane via riir-deployer)** — T1–T4 landed 09-27
  (`deploy.yaml` cf-container with the 12 `files:` rows, zigbuild x86_64,
  the local e2e verified end to end: plan → stage → docker run → six lanes
  ready → live decisions with receipts; the real CF push is the
  owner-adjacent step BY DESIGN — devnet first, mainnet the owner
  ceremony). Durable record: AGENTS.md "P5 — the hosted serving lane".
- **004 (decstat flywheel)** — T1–T3 landed 09-27 (wire `9da822b` →
  capture `3a1eb09` → store/route `c285435`; live-pushed to devnet epoch
  2960 the same day); T4 (riir-train intake) is tracked at its canonical
  home, riir-train Issue 577 — nothing here pointed at it that 577 does
  not. Durable record: AGENTS.md "P6 landed T1–T3".

Full narrative of each removed file: `git log --follow -- .issues/<file>`.

## 2026-09-28 — Issue 579 T3: the arena bridge — banking77 serves the nbsvm v2 winner (Bench 012)

Landed the consumer half of riir-train Issue 579 / Bench 612:
`specialist::winner_bridge` (the per-suite winner FILE + BAG-CONVENTION
table — the 578 coupling made structural, one home), `presence_bag_into`
(the train-side `instinct_nbsvm::presence_bag_into(norm=true)` mirror over
the same reflex tokenizer law), `BagConvention::bag_into` as the single
dispatch at every bag site (arena `SuiteCtx`, serve `SuiteServer::decide`,
the hoard-gate `corpus_centroid_with`), and `check_winner_file` — a raw
boot/swap naming any other artifact for a bridged suite refuses loud.
banking77 bridges to `banking77_nbsvm_v2.bin` (digest = Bench 612's mint,
byte-verified here); the five v1 suites stay `<suite>_winner_v1.bin` +
count bags. The frozen read (n=500,
`.benchmarks/012_banking77_nbsvm_v2_bridge/`): A0 0.8260 (pinned == the
published reflex row) · A1 0.8280 · H1 0.8320 · **H2(β=2,nmin=8,τ=8)
0.8540** — T2 REFUSED at LB95 −0.0013 (advertising law; the same
uncertified class as ag_news H2 −0.0100 / sst5 A1 −0.0129), G1 face FAIL
(the fused readout, the ag_news class), G3 PASS. **Serves H2(2,8,8) under
the owner's best-measured law**: the manifest row (digest + `file`
override + posture), `PINNED_MANIFEST_DIGEST`, the serve-gates posture
table, and the deploy.yaml shipping rows all moved in the same change;
all 11 serve-gate tests green. The train-side holdout edge transferred
(+2.8 pt on test). The refused v1 file stays on disk for reproduction but
no longer ships; a banking77 HOSTED-ONLY vessel must be re-minted from v2
by riir-train before the vessel lane serves this posture. Pin disclosure:
the harness FAMILIES' site check reds under the reflex sibling's
in-flight Issue-045 tree (its published bench.json predates the 045
posture moves) — all NINE dataset suites pinned green (arena == reflex
`run()` == site) in the same session; the records were written under
`--skip-pin-a0` with the run-1 log as the pin evidence. Pre-existing at
HEAD, not this lane: `tests/tetris_critic_parity.rs` carries two doc-list
clippy errors under `--all-features` (the tetris lane's file).

## 2026-09-27 — Proposal 001 T8: `arsenal_budget_goat` GOAT PASS — and the gate caught the unsigned-fold defect (Bench 003)

Landed `benches/arsenal_budget_goat.rs` (opt-in `arsenal_goat`,
harness=false; data-absent refuses exit 1) and ran it to a **7/7 GOAT
PASS** (`.benchmarks/003_arsenal_budget_goat/`; box state in the
PROVENANCE lines: M3 Max, AC, load ~5 — every bar carries ≥140× p99
headroom). The four legs: init p99 under the full manifest (validate 13.9
ms vs 1 s; lazy registry 0 µs; the eager six-lane boot DISCLOSED at 19.6
s — banking77's seat fit alone is 13.2 s, exactly what the lazy posture
defers); routing ns–µs at k=dozens (hoard k=5 p99 4.3 µs, k=64 7.4 µs vs
100 µs; slot Ready-read p99 42 ns); swap atomicity under concurrent
decisions (3 reader threads deciding under the slot lock vs 12
production-shape installs — **335,063 decisions, 0 torn reads, 0
errors**, epochs monotone); armed-off identity (kill-switch literal
re-pinned; the armed gate admits 6/6 real centroids, verdicts
bit-identical across two passes). Disclosed: swap install p50 2.5 s /
p99 21 s (fresh seat fit + decode — operator-priced, never per-decision);
contended decide p50 146 µs; banking77 quiescent p99 366 µs (the served
ceiling — Bench 002's 9–119 ns gated the fusion axis only).

**The finding the gate caught: the first run GOAT-FAILED at 0/6 real
admits.** The T5 hoarding gate's centroid used the UNSIGNED `fold8`
(bucket-wise sum of L2-normalized token bags). Token-hash histograms are
≈ uniform over the 8 admission buckets for EVERY English corpus —
content-blind at bucket granularity — so every suite's centroid sat
within the 0.95 colinearity cap of every other's and the armed gate
refused each candidate as a `NearDuplicate` of the first loaded neighbor
(the 83 ns check was the early-exit firing). The gate's own contract
("near-identical corpora ⇒ colinear centroids") could not hold for any
pair; mirrors and strangers were indistinguishable. The unit tests
missed it because they feed synthetic orthogonal vectors — no test ever
fed REAL corpora; the bench was the first instrument that did, which is
the argument for gating infrastructure with real-data benches and not
fixture-only units.

**Fix (same landing):** `corpus_centroid` is now a SIGNED simhash fold —
each bag entry's weight multiplied by a balanced ±1 derived from a
multiplicative mix of its own bucket id (`bucket.wrapping_mul(0x9E37_79B1)
>> 16 & 1`; token-level signs — per-axis signs cancel out of the cosine
and would be a no-op). A mirror corpus reproduces the identical token
set → identical signed sum → cos → 1 (still refused); distinct corpora
produce independent ±1 sums → cos ≈ 0. The six real suites now read
vendi 2.878 vs floor 1.2 (healthy diversity) and admits are 600/600.
All 13 arsenal_ops tests pass unchanged (the synthetic-vector tests are
sign-agnostic by construction); 45 lib tests default / 54 all-features
green; clippy clean at both postures; the frozen-predictions serving
parity gate passes unchanged — the centroid feeds ONLY the hoarding
gate (boot/swap policy), never a served decision. Promote verdict: the
budget/swap legs STAND as shipped; the bench is the standing re-run gate
for any centroid/gate/manifest change.

Validation caveat (carried from T5+T6): built against
riir-reflex@`9dc2e02` (the consumer-green snapshot) — develop still
carries the sibling's consumer-posture break; a re-run against the fixed
sibling is owed when they land it.

Also this session: Issue 007's claimed T4 was NOT landed —
`scripts/build_hybrid_doc.py` carried the scope law only as a docstring;
landed as `--self-test` (5 known-answer fixtures: scope inference, the
typed field overriding the name both directions, the A0 skip, and the
replicated harness metrics against hand-computed values), and the
committed Bench-002 lane doc — generated by the PRE-T2 builder — was
regenerated in place with the scope fields, provenance preserved
(`0959928` / the original arena timestamp); 003 closed+removed (record =
the Bench-002 entry), 005 status refreshed (Bench-002 aligned verdict,
the laya face PASS, the site publish landed; H3 stays the data-gated
open face).

## 2026-09-27 — Issue 007 closed: the missing builder self-test landed, and the committed Bench-002 lane doc predates the scope field

Closing 007 surfaced a checked-but-not-landed task: **T4's builder
self-test did not exist** — `scripts/build_hybrid_doc.py` carried the
scope law only as a docstring and the `latency_scope` assignment, with
no known-answer pin. Landed as `--self-test` (5 fixtures): name
inference (H2/A1 → `arm-only`, H1 → `seat+arm`), the typed
`contains_seat_solve` field overriding the name in BOTH directions, the
A0-registered suite skipped and disclosed, and the replicated harness
metrics pinned against hand-computed values on a deterministic fixture
(accuracy 0.75, ece 0.35, acc@50cov 1.0, p50 0.02 ms / p99 0.04 ms /
tail_support 1, consult_rate per arm). Regression-checked against the
real Bench-002 `predictions.json`: cell metrics byte-identical to the
committed doc.

That regression check surfaced the second finding: **the committed
`.benchmarks/002_hybrid_052_protocol/hybrid_lane_doc.json` was generated
by the PRE-T2 builder and carries no `latency_scope` at all** — the
published-site source doc had no scope disclosure to render, so T3's
site rendering had nothing to show for the hybrid lane until a
republish. The committed doc is regenerated in place with the CURRENT
builder, provenance preserved (`--git-sha 0959928 --date-utc
2026-09-26T23:09:05Z` — the arena build that produced
`predictions.json`, not the doc-builder commit); the diff is exactly the
five `latency_scope` fields. The reflex-site republish itself stays with
the site lane (deferred to the gate-clean whole-run, per that lane's own
note) — it will now pick the scope up from this doc.

## 2026-09-27 — Bench 002: the aligned Bench-052-protocol read + the reflex-site hybrid lane publish (Issue 003 T4 closed)

The site publish's comparability blocker is discharged with a measurement,
not an argument. The arena re-ran at the **Bench-052 protocol on the same
`datasets_t20k` bytes** the published lanes carry (the 4090 T5 run verified
977/977), and the comparability proof is **A0 == the published 052 modelless
rows 6/6** (ag_news 0.8825, emotion 0.7375, sst5 0.3967, xnli 0.5233,
massive 0.7800, banking77 0.8260) plus the in-run xnli drift pin. Record:
`.benchmarks/002_hybrid_052_protocol/` (ALIGNMENT.md carries the table,
the box state, and the gate notes).

Registered arms at the aligned protocol: ag_news H2(0.25,2,2) 0.8975
(G1 FAIL — the raw fused readout is already calibrated at 0.0133, the
Platt refit hurt; the raw readout stands), emotion A1 0.8550, sst5 A1
0.4217 (both G1+G3 PASS), massive H2(1,2,8) 0.8267 vs A0 0.7800 (+4.7
pt — the v2 "flips from A0 0.42" story was the old first-N sample's
unrepresentative 30-of-60 label prefix), banking77 H1 0.8060 @ 46.8%
consult (G3 FAIL — H1 pays up to ~4.7 pt at 95% confidence; no
promotable hybrid arm, A0 stands), xnli A0 stands. The v2 numbers are
superseded as the publishable record.

**The bridge gap the aligned protocol surfaced** (and v2 could not, on
the old bytes): the t20k massive test split carries 59 of the artifact's
60 intents (`cooking_query` has zero test rows), so the train-derived
cal front legitimately presents an artifact-known, seat-unknown option —
the Issue-006 bridge refused that shape and the run died at case 0. The
`cooking_query` panic's first diagnosis was WRONG (the seat's test cases
are clean; a python replication of the sampler "proved" it) — the
offending case was CAL-side, invisible to any test-split probe. Resolution:
`fill_positions`/`key_map` admit artifact-only labels (sentinel seat
index), and `prior_fusion_pick` gained the NaN-no-evidence mark — the
margin term mutes to 0 (the prior stands) and the option is never a
rival in the best/second scan; pinned by
`h2_nan_evidence_mutes_the_margin_and_is_never_a_rival` (the first test
draft's expectation was also wrong twice — the second-known rival is
the rival, and f32 `exp` rounding put the honest tolerance at 1e-6).

**The reflex-site publish** (`instinct (hybrid)` lane): publish_bench
gained the `hybrid` lane class (merge carry with the device-variant
skip, LANE_DISPLAY, the wholesale-replace inventory, both rename
surfaces), the bench page gained the lane in table/filter/charts + the
explainer + a magenta palette slot, and the det cell went three-state
(a lane that does not claim a repeat check renders "—", never a lying
✗). `scripts/build_hybrid_doc.py` packages the frozen read (registered
arm per suite; A0-registered suites honestly absent — xnli carries no
hybrid lane) with the reflex metric laws re-derived exactly, including
the `cooking_query`-class cal-front extension in the arena. Site tests
22/22 incl. the new hybrid case; bench_page + chart_render smokes PASS.
Published from `hybrid_lane_doc.json`; `lane_sources.git_sha` names the
landing commit.

## 2026-09-27 — Issue 006 RESOLVED: the arena's position-vs-index instrument defect (Bench 001 v2)

Fix `3a12a70`. The v1 massive anomaly (A1 0.0267 vs Bench 609's .670) was
the ARENA, never the specialist. Two instrument bugs, one class:

1. **Space mismatch**: A1/H1/H2 scored and picked in the seat's LABEL
   space (`perm` is label→class) while A0's probs and every `gold.idx`
   speak the question's PRESENTED-option space; the arena compared them
   directly. Expected ≈ 1/59 ≈ .017 — the observed .0150/.0267. Masked on
   every full-universe suite (identity); only massive's 20-of-59 sampled
   + shuffled options could expose it.
2. **Double indirect**: `cands[rank0_sorted[select_arm(..)]]` —
   `select_arm` already returns the candidate index. v1's banking77
   "A0 stands" registration was an artifact of this; v2 it crashed loud
   (rank0 [0,2,16]) and is fixed.

All three filed hypotheses refuted by measurement (the probe:
`examples/massive_anomaly_probe.rs`): the teacher dump join is asserted
at train (KeyMap permutation cannot pass); the pyjson state envelope
costs ~0.7 pp (state 0.7559 vs raw 0.7626 on the seat's own rows);
t20k and datasets massive test rows are identical, and the artifact
(seal `7bc3ee385f81abc0` = Bench 609's recorded winner) reads 0.86
train / 0.76 test through the SERVING reader.

The fix: `h1_decide` takes a per-case `class_of_pos` bridge;
`fill_positions` mirrors the engine's own two resolution rules (by name
when every presented key is a seat label — massive/banking77; identity
under k == N — the fixed-criteria suites, score-array included; else
refuse loud). A1/H2 pick among presented positions; the H2 margin's
rivals are the presented options. Pinned by
`h1_scores_the_class_the_position_denotes`.

**Bench 001 v2 supersedes v1 wholesale** (corrected frozen read):
massive A1 **0.8167**, registered H2(β=0.25,nmin=4,τ=4) **0.8300** vs A0
0.4200 — the suite flips from A0-stands to a hybrid win; banking77
registers **A1 0.7960** (v1's A0 was the bug); ag_news H2(0.25,2,2)
0.9000; emotion/sst5 A1 (0.8550/0.4217); xnli A0 stands. Laya paired
face PASS ×6. The lesson generalizes: **a pick/gold pair is a SPACE
contract — when a seat samples its option set, every scorer must speak
the presented-option space, and an instrument index that survives on
coincidence is an OOB panic waiting for the first suite that breaks the
coincidence.**

## 2026-09-27 — Bench 001: the hybrid GOAT run (P3 + P3a through the single test read)

The Reflex · instinct hybrid measured end to end. Commits: reflex seat
`979dd90`+`8d725d3`, hybrid `d4caaa6`, arena `dea91d5`, join-contract fix
`96fe890`, micro-assert fix (pre-run). The run: `.benchmarks/001_hybrid_goat/`
(RESULTS.md + predictions.json + registration.json).

- The seat seam lives in REFLEX (one-way): `harness::runner::seat` —
  `prepare_seat` / `fit_posture` (the deployed Bench 051 posture through
  the SAME code reflex's runner uses) / `eval_seat` (+
  `laya_escalation_latency_us` behind `laya-riir`). The boundary check
  forced the shape: reflex consuming instinct would be a cycle. The A0
  drift pin (arena A0 xnli_en == reflex `run()` hard accuracy,
  byte-exact 0.5167) proves the seat path on every run.
- Verdict: ag_news promotes **H2(β=1,nmin=2,τ=4)** — 0.8975 vs A0
  0.8625 / A1 0.8875, p50 2 µs vs A0's 150 µs, G1+G5 PASS. emotion +
  sst5 promote **A1** (the specialist alone: 0.8550 / 0.4217 vs A0's
  0.5750 / 0.2017; H1's reflex half DRAGS the cascade down — the fused
  gate passes reflex answers that are wrong more often than the
  specialist's). xnli + massive + banking77 keep **A0** (G5 refused
  honestly). H3 excluded with reason (no chain-shaped specialist
  suite).
- Honest fails on record: G2's H1 fusion-only bar (100 ns/q) is
  breached on wide suites by the O(n·k) prune (massive 136 ns/q,
  banking77 181 ns/q) — the O(n) survivor heap is the named remedy;
  this box's `.raw/datasets` differ from
  Bench 051's bytes (the A0 rows shifted accordingly — the arena is
  internally consistent per the drift pin; 051 cross-references are
  indicative, never comparable numbers). ⚠ SUPERSEDED the same day by
  Issue 006's resolution (the entry above): the massive chance reading
  and the xnli/massive/banking77 A0 registrations were the instrument's
  position-vs-index defect — read Bench 001 v2 for the corrected
  numbers.
- Instrument lessons paid for en route: the sigmoid-gate calibrator's
  `apply()` is the identity until `refit()` (the first run read Platt ==
  raw to 4 decimals — G1 false-failed everywhere until the face called
  refit; now pinned by tests/calibrator_probe.rs); the G5 budget face
  must REFUSE (not pass) when the registered arm is A0/A1; a micro-loop
  liveness assert on accumulated picks is wrong-headed (picks can all
  be label 0 — black_box is the liveness guarantee); the label join's
  contract is seat⊆artifact, not equality (massive: 60 artifact labels
  vs the seat's 59 offered options).

## 2026-09-27 — Bench 004: the re-baseline + the strict-superiority product gate (Issue 008 T1+T2)

Executed in one session, `feat:` commit of the same day. **Trigger:** Issue
008's root causes measured — Bench 002 registered every arm against the
Bench-052 Reflex while Reflex armed its cal-selected heads (Bench 057:
emotion ridge@8 → 0.8850), and banking77's refused H1 still showed on the
site row. Two changes:

1. **The seat posture is the CURRENT PUBLISHED reflex posture** (T1):
   `head+nb+ridge` select on (the reflex dep gained
   `nb_ridge`+`option_cond` — the latter only because `nb_ridge` alone
   does not compile upstream, reflex Issue 050 `7a99431`), genome off.
   The ridge ladder arms only where reflex's arming bar clears — emotion
   @8, every other arena suite 0 (byte-identical to off, reflex's own
   full-workspace measurement) — so ONE knob set reproduces the published
   per-suite postures. The A0 drift pin widened from xnli-only to ALL SIX
   suites, and gained the **site pin**: reflex's rows must equal the
   published `reflex-site/data/bench.json` numbers when that checkout
   stands beside the workspace (absent → loud skip). Measured 6/6 + site
   ✓ twice (at reflex `ad45067` and again at `7b5f0ba`, byte-identical
   verdicts — the sibling's intervening commits were measurement-only
   arms, pinned off in the drift pin: `nli_feature_ab`,
   `cascade_worthiness_lcb`).
2. **The registration IS the product gate** (T2): an arm not STRICTLY
   above the current Reflex row — paired (pick − A0) LB95 > 0 on the
   frozen test read (`stats::PairedDiff::lb95`, known-answer-tested both
   directions) — is REFUSED; A0 serves. registered == served from now on;
   there is no unserved-arm disclosure gap left (Issue 008 T3's original
   wording is moot; the remaining half is the site re-sync).

**Verdicts:** massive H2(1,2,8) 0.8267 vs A0 0.7800 — LB95 +0.0124, the
ONE certified arm. ag_news H2 (+0.0150/−0.0100), sst5 A1
(+0.0250/−0.0129), banking77 H1 (−0.0200/−0.0466) REFUSED. Emotion: the
ridge@8 base outruns A1 (0.8850 vs 0.8550) — the instrument picked A0
outright; the specialist must now OUT-GROW reflex, not trail it. The
sold set is exactly one suite; the site ✓ law (strictly ahead on every
suite it sells) holds on it.

**The abstention contract** (found by the parity gate the same day):
reflex's hard accuracy is the FORCED convention (abstains forced to
argmax — the published numbers) while the serve honors abstention. With
ag_news serving A0 (gate abstains 196/400 = 49% of its test questions),
the gate compared a served `None` against a recorded forced pick. Repair:
the arena's `ArmOut` records `abstained` per arm (A0 = the seat's flag;
H1 = the passthrough half; A1/H2 never), `predictions.json` carries it,
and the parity gate asserts served-abstention == recorded-abstention
before picks.

**Boot cost, honestly:** the ridge ladders are per-boot derivation now
(banking77 ≈ 65 s standalone) — serve readiness ceilings raised 120 →
420 s; a frozen-derived-posture seam (selected values pinned as data,
ladder skipped at serve time) is the recorded follow-up if it ever
binds.

**Validation:** full `cargo test` green (46 lib incl. the two new lb95
known-answers + 11 serve_gates incl. the abstention-aware parity gate
and the A0-shaped HTTP happy path over a frozen answered case); clippy
`-D`-grade clean at default + all-features. Committed with the re-pinned
manifest digest + face-2 rows + the 004 record (predictions, registration,
run.log, RESULTS, ALIGNMENT).

## 2026-09-26 — birth

Created per riir-ai Proposal 047 (owner decision: the model-based/hybrid
lane lives in a PRIVATE repo, riir-clippy-shaped, because hybrid wiring and
weights are product moat and `riir-reflex` is public). Trainer:
riir-train Issue 576. Motivation: riir-reflex Issue 038 / Bench 051 — the
modelless lane plateaus below laya on xnli / typed_decisions / ag_news.
