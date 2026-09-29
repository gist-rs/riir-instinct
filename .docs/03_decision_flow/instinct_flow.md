# The Instinct composition flow

How the trained add-on composes with the free engine. One figure, two bands
(the Reflex hero's flywheel shape — the question in on top, the answer out
under it) rendered from the mermaid block below to BOTH mirrors by
`reflex-site`'s `scripts/render_tetris_flows.py` (same palette + postprocess
as the Reflex hero `decision_flow.svg`; `--check` verifies the two mirrors
byte-identical):

- `instinct_flow.svg` — beside this doc (the repo's record)
- `reflex-site/assets/instinct_flow.svg` — what `/bench/#instinct` embeds
  (the site is Reflex's; Instinct stays out of its home page until it beats
  every published lane — riir-instinct `.issues/008`, the PoC framing)

The laws the figure carries (full text: the repo root `AGENTS.md` +
`.proposals/001_arsenal_cognition_vessel_protocol.md`):

- **Reflex always answers first, free.** The specialist is paid only where
  Reflex's calibrated fused gate abstains — never as a replacement lane.
- **A2 — never blend.** Selection is a monotonic atomic hot-swap of whole
  artifacts from `arsenal.toml`, the ONE selection surface (A5); a decision
  observes one whole server, never a torn read.
- **Registered arms only.** A specialist joins the manifest solely where it
  beat free Reflex pre-registration, on one frozen test read (the GOAT
  gate, `stats::PairedDiff::lb95`).
- **The receipt.** Every answer carries the decision receipt: build
  fingerprint, BLAKE3(input), BLAKE3(canonical decision), lane id.

```mermaid
%% file: instinct_flow.svg
%% aria: The Instinct composition flow in two bands: the top band is the question in — the same typed question runs Reflex's modelless engine first, and a confident fused-gate answer returns in microseconds with the specialist never paid; on abstain the bottom band takes over — top-k prune keeps the candidates, the sealed per-domain specialist scores them, an H1 cascade or H2 prior fusion calibrates the pick, and the answer ships with a decision receipt — while arsenal.toml, the only selection surface, hot-swaps registered arms monotonically
flowchart TB
  subgraph IN["the question in — Reflex · modelless, always first, free"]
    direction LR
    Q["the same question in<br/>choice · score · yes/no"] --> E["embed → route → score<br/>corpus-is-the-model"] --> G{"fused gate<br/>confident?"}
    G -- "signal" --> A["answer + confidence<br/>microseconds · zero-alloc<br/>the specialist is never paid"]
  end
  subgraph OUT["on abstain — the trained add-on, paid only here"]
    direction LR
    P["top-k prune<br/>keep the candidates"] --> S["Instinct specialist scores<br/>trained · per-domain · BLAKE3-sealed"] --> F["fuse<br/>H1 cascade pick · H2 prior fusion<br/>p′ᵢ ∝ pᵢ·exp(g·β·mᵢ)"] --> A2["calibrated answer<br/>+ decision receipt<br/>BLAKE3 in · BLAKE3 out"]
  end
  T["thresholds + gate fitted offline<br/>from YOUR labeled data · one-off"]
  M["arsenal.toml — the only selection surface<br/>registered arms only (beat Reflex pre-registration)"]
  T -.-> G
  G -- "abstain — no signal" --> P
  M -. "monotonic hot-swap" .-> S
```
