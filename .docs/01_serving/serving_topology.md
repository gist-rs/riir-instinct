# Serving topology — one lane per suite, and where the think lane plugs in

How a decision is answered: the selection laws, the arm vocabulary, the lane
lifecycle that IS automatic, and the extension seam the private think lane
uses. This page is concept-level by design — measured records live in the
repos that own them.

## The one-sentence model

**One lane per suite, chosen by a validated manifest at boot; nothing routes
per-request.**

A reader arriving from a Mixture-of-Experts background should read this as the
deliberate opposite: there is no gating network, no per-input expert pick, and
no weighted mixing of outputs. The routing "intelligence" was spent OFFLINE —
paired GOAT benches decided which arm each suite serves — and the verdict was
baked into the manifest as data. The runtime is dumb-but-auditable on purpose.

## The selection laws

- **A5 — the manifest is the only selection surface.** Suite → vessel/artifact
  → posture → pins live in `arsenal.toml`; there are no hard-coded match
  tables and no filename conventions. Boot validation refuses drift loud,
  naming row + field.
- **A6 — posture rows are pinned in both media.** The manifest's digest is
  byte-pinned by the serve gates: a TOML edit reds exactly like a code edit.
- **A2 — never blend.** Selection is a monotonic atomic hot-swap of whole
  artifacts. A decision observes one whole server, never a weighted mixture.
- **A3 — cheap routing, no dyn on the decide path.** Per-decision work is
  modelless ns–µs class. Any consult (paying a heavier engine on some cases)
  must be GOAT-justified — the measured refusal precedent is banking77's H1
  consult, which paid accuracy at confidence and was refused.

## The arms

| arm | what answers | shape |
|---|---|---|
| `A0` | the modelless engine | the free floor; a tie sells nothing |
| `A1` | the specialist over the seat | the seated artifact alone |
| `H1` | cascade with top-k prune | gate abstains → prune → specialist over survivors (input-conditional within the suite) |
| `H2` | prior fusion | specialist margin folded into the modelless prior |
| `ENC` | the think-depth lane | served ONLY through the extension seam below; the open build never serves the class |

An ENC row in a build with no lane backend installed refuses **loud at boot**,
naming the install seam — never a silent degrade to a cheaper arm.

Every served decision also carries the seat engine's own fused-gate
abstention (`gate_abstained`) — distinct from the lane's `abstained`:
for `A0` they agree, while `A1`/`H2` never abstain even where the gate
flags the question. Additive and not on the wire JSON; it is the
escalation-set signal downstream lanes (Rethink's ESC) read.

## What IS automatic (lane lifecycle)

- **Lazy load** — `budget.load = "lazy"` rows boot `Unloaded`; the first
  decision triggers the load behind the 503 `loading` window. Eager rows are
  byte-identical to the pre-lazy behavior.
- **The hoard gate** — at load/swap time, a candidate suite is judged against
  the loaded set: a near-duplicate corpus (colinearity past the cap) is
  refused, and a rank-collapsed set fails the Vendi certificate. The corpus
  centroid is a signed simhash fold; serving decisions never touch it.
- **Monotonic swap** — `check_epoch_tag` accepts advance or idempotent
  replay; downgrade and fork are refused (force logs loud). Install happens
  whole under the slot lock — no torn reads, no blend, no dyn.
- **Wire eviction** — a release call evicts a lazy lane; the epoch tag is
  kept, so reloading a downgraded artifact still refuses.
- **The LRU residency bound** — lazy lanes are additionally bounded by an
  LRU policy: at/over the capacity knob (`INSTINCT_LRU_CAPACITY`, `0` =
  off), the least-recently-used Ready lazy row that has sat idle past the
  hysteresis floor is released at the next lazy trigger, best-effort — a
  resource bound, never an admission control (the hoard gate stays the
  only admission judge), eager rows exempt.

## The extension seam

The open grammar carries the `ENC` vocabulary; the execution is a lane
backend. A downstream crate installs its loaders via `install_ext_boots`
BEFORE any boot, and the suite then boots into the extension seat — a
first-class decide-path citizen with the same signature, the same serve edge,
and the same receipts as every other lane.

```mermaid
graph TD
    M[arsenal.toml<br/>one row per suite] --> V[boot validation<br/>digests · pins · class]
    V --> L{lane boots}
    L -->|A0 / A1 / H1 / H2| B[bag server<br/>sub-ms decide]
    L -->|ENC + backend installed| E[extension seat<br/>the think lane]
    L -->|ENC, no backend| X[loud boot refusal<br/>names the seam]
    R[request] --> D[decide: exactly one lane answers]
    B --> D
    E --> D
    D --> RC[receipt: lane id + input/decision digests]
    B -. loading .-> S[503 loud, state named]
    E -. error .-> S
```

### The no-fallback law

The decide path returns a typed result — an error propagates as a loud 503
with a machine-readable code. There is **no per-request fallback** from the
think lane to a cheaper arm, by construction. The reason is measurement
honesty: the receipt names the lane that answered. A silent fallback would
serve one engine's answer wearing another's label and corrupt every published
cell.

"Smarter falls back to the cheap flow" therefore happens at **seating time,
not request time**: if the heavier class does not strictly beat the incumbent
at the paired gate, the manifest simply keeps the cheaper arm, and the
heavier lane never boots for that suite.

## Where adaptivity lives today

The two-tier escalation is no longer hypothetical — the private lane
SHIPPED it through this seam: answer on the cheap leg, escalate the hard
cases to the think leg behind a worthiness gate, the tier disclosed in the
receipt. It was promoted after its paired GOAT (riir-rethink Issue 017 T5,
2026-10-04, Bench 0060) and is armed per the same laws this page carries:
the manifest `escalate` table is the only arming surface (A5 — no new env
switch), one row carries an escalate table today (`emotion_syn`), a
rolling rate guard latches a cheap-only demotion outside its band, and
`RIIR_RETHINK_ESC=0` (the exact literal) is the runtime demote. The
measured caution it was designed under still holds in the shipped shape:
per-suite margins, a bounded escalation rate, and a loud no-fallback
policy are load-bearing, not decoration. A report-only value-of-information
readout rides beside the guard (riir-rethink Issue 047, 2026-10-31) —
receipts only, so its precision leg stays blind until an outcome join
exists, and it gates nothing.

The grammar question — should the open `EscalateSpec` carry a static
precision floor / damp field? — was DECLINED by the owner on 2026-11-06
(Issue 025, closed; record in `HISTORY.md`): no gain is proven. The
served lane has no outcome capture, so a floor would have nothing to
enforce against, and the only measurement (rethink Bench 081, sst5
replay) shows escalation already pays (+0.254 ±0.147 per escalation),
so a floor has no failing case to fix. Re-open trigger: an outcome join
lands AND a suite's measured value per escalation reads non-positive. The
one-definition law is unchanged: the vocabulary lands HERE and rethink
byte-pins it — the two-repo lockstep the manifest law already enforces.
