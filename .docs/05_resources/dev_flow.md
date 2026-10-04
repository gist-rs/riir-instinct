# The Instinct dev build flow

How a specialist gets made and put on duty — the development counterpart
of the [composition figure](../03_decision_flow/instinct_flow.md), which
shows what happens at answer time. One figure, rendered from the
` ```gfflow ` block below by reflex-site's `scripts/render_flows.py`
(Plan 620; family design guide §8) into `instinct_dev_flow.svg` (desktop
swimlane), `instinct_dev_flow_m.svg` (390 px card list) and
`instinct_dev_flow.walk.json` beside this doc, mirrored byte-identically
into `reflex-site/assets/`.

1. **make corpus** — the training corpus is assembled from public data.
2. **train specialist** — one model per domain, trained on that corpus.
3. **stratified holdout + winner law** — the GOAT gate: the candidate
   earns a row only by strictly beating the free engine on one frozen,
   label-stratified test read.
4. **BLAKE3-lock** — a passing artifact's BLAKE3 digest is pinned in its
   manifest row and checked at every load, so any change to the file is
   caught; nothing trains at serve time. (Hashed, not signed: the signed
   vessel format exists but is not the serving path yet.)
5. **manifest row** — the locked winner is registered in `arsenal.toml`,
   the ONE selection surface (the composition figure's hot-swap lane).
6. **serve + receipts** — the lane answers with a decision receipt:
   build fingerprint, BLAKE3(input), BLAKE3(canonical decision), lane id.

Lanes are who runs what, all in the open lane's own accent: the corpus,
training, the locked artifact, serving.

```gfflow
file  = "instinct_dev_flow.svg"
title = "Instinct: how a specialist is made, locked and put on duty"
accent = "instinct"
intro = "Press play to walk the build path one step at a time, or click a dot to jump."

[[lane]]
id = "data"; label = "The corpus";  note = "public data · frozen splits"; color = "instinct"
[[lane]]
id = "train"; label = "Training";   note = "one per domain"; color = "instinct"
[[lane]]
id = "artifact"; label = "The artifact"; note = "locked · registered"; color = "instinct"
[[lane]]
id = "serve"; label = "Serving";    note = "receipts on answers"; color = "instinct"

[[step]]
id = "corpus"; n = "1"; lane = "data"; col = 0
title = "Make the corpus"; body = "assembled from public data, splits frozen"
status = "live"
[[step]]
id = "spec"; n = "2"; lane = "train"; col = 1
title = "Train a specialist"; body = "one small model per domain"
status = "live"
[[step]]
id = "holdout"; n = "3"; lane = "train"; col = 2
title = "The winner law"; body = "serve only a strict win over the free floor"
status = "live"
[[step]]
id = "lock"; n = "4"; lane = "artifact"; col = 3
title = "Lock it"; body = "the digest pinned, checked at every load"
status = "live"
[[step]]
id = "row"; n = "5"; lane = "artifact"; col = 4
title = "Register the row"; body = "the manifest — the one selection surface"
status = "live"
[[step]]
id = "srv"; n = "6"; lane = "serve"; col = 5
title = "Serve with receipts"; body = "build, input hash, decision hash, lane id"
status = "live"

[[edge]]
from = "corpus"; to = "spec"
[[edge]]
from = "spec"; to = "holdout"
[[edge]]
from = "holdout"; to = "lock"; label = "a strict win"
[[edge]]
from = "lock"; to = "row"
[[edge]]
from = "row"; to = "srv"

[[walk]]
title = "Make the corpus"
text  = "The training corpus is assembled from public data, with the splits frozen before anything trains — fit on one slice, choose on another, read test once for the verdict."
steps = ["corpus"]
[[walk]]
title = "Train a specialist"
text  = "One small model per domain trains on its slice of the corpus — a specialist for exactly the cases the free engine finds hard."
steps = ["spec"]
[[walk]]
title = "The winner law decides"
text  = "The candidate earns a manifest row only by strictly beating the free engine on one frozen, label-stratified test read. A tie or a loss sells nothing — the free floor keeps answering."
steps = ["holdout"]
[[walk]]
title = "Lock it"
text  = "A passing artifact's digest is pinned and checked at every load, so any change to the file is caught — and nothing trains at serve time."
steps = ["lock"]
[[walk]]
title = "Register the row"
text  = "The locked winner is registered in the manifest — the one selection surface. It is swapped whole, never blended, and a decision always observes one whole server."
steps = ["row"]
[[walk]]
title = "Serve with receipts"
text  = "The lane answers with a decision receipt: the build fingerprint, hashes of the input and the canonical decision, and the lane id — an audit trail on every answer."
steps = ["srv"]
```

## Status per step (the honesty table)

| # | Step | Status | Grounding |
|---|---|---|---|
| 1 | make corpus | live | the open datasets, frozen splits |
| 2 | train specialist | live | the per-domain trainers |
| 3 | the winner law | live | the GOAT gate, one frozen test read |
| 4 | BLAKE3-lock | live | digests pinned, checked at load |
| 5 | manifest row | live | arsenal.toml, the one selection surface |
| 6 | serve + receipts | live | the serving lane's decision receipts |

The figure's step-through walk has no IN/OUT payloads: the build path has
no public request wire, and the numbers law forbids typing an example.
When a status changes, edit the gfflow block above (then re-render with
reflex-site `python3 scripts/render_flows.py --only instinct_root`) and the
honesty table in the same commit.
