# The Instinct composition flow

How the trained add-on composes with the free engine. One figure — the
question in on the free floor, the trained rung paid only on abstain —
rendered from the ` ```gfflow ` block below by reflex-site's
`scripts/render_flows.py` (Plan 620; family design guide §8) to BOTH
mirrors:

- `instinct_flow.svg` + `instinct_flow_m.svg` + `instinct_flow.walk.json`
  — beside this doc (the repo's record)
- `reflex-site/assets/…` — what `/bench/#instinct` and `/resources` embed

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

Lanes are who runs what: the free floor on **your machine** (Reflex
orange), the hybrid composition in **the open lane** (Instinct pink — it
serves live), and the encoder rung on **our GPU hosts** (Rethink violet —
measured on the board, record-only; hosted serving unlaunched since the
2026-10-03 Instinct/Rethink split).

The main path counts 1→2; the floor's two outcomes share 3 — **3a** the
sure answer (the specialist is never paid), **3b** the abstained question
on the open lane (**3b.1** prune → **3b.2** the specialist scores →
**3b.3** fuse). Still unsure escalates one rung deeper to the encoder
(**4**). The manifest (**5**) hot-swaps registered arms — a back edge,
never a number.

```gfflow
file  = "instinct_flow.svg"
title = "Instinct: the trained add-on composes on top of the free floor"
accent = "reflex"
intro = "Press play to walk the composition one step at a time, or click a dot to jump. Each step shows what goes in and what comes out — captured from the real release binary where the step is live."

[[lane]]
id = "me";   label = "Your machine";  note = "the Reflex floor · private · free"; color = "reflex"
[[lane]]
id = "lane"; label = "The open lane"; note = "Instinct hybrid · CPU · serves live"; color = "instinct"
[[lane]]
id = "host"; label = "Our hosts";     note = "the encoder rung · record-only"; color = "rethink"
[[lane]]
id = "ops";  label = "The manifest";  note = "the one selection surface"; color = "instinct"

[[step]]
id = "ask"; n = "1"; lane = "me"; col = 0
title = "Your question"; body = "the same typed question — choice, score or yes-no"
status = "live"
[[step]]
id = "floor"; n = "2"; lane = "me"; col = 1
title = "Reflex answers first"; body = "embed, route, score; the fused gate checks confidence"
status = "live"
[[step]]
id = "ans"; n = "3a"; lane = "me"; col = 2
title = "Answer + receipt"; body = "microseconds, calibrated — the specialist is never paid"
status = "live"
[[step]]
id = "prune"; n = "3b.1"; lane = "lane"; col = 2
title = "Abstain, prune"; body = "only abstains go on; top-k keeps the candidates"
status = "live"
[[step]]
id = "score"; n = "3b.2"; lane = "lane"; col = 3
title = "Specialist scores"; body = "locked per-domain artifacts — never blended"
status = "live"
[[step]]
id = "fuse"; n = "3b.3"; lane = "lane"; col = 4
title = "Fuse and calibrate"; body = "a cascade pick or prior fusion returns one calibrated answer"
status = "live"
[[step]]
id = "enc"; n = "4"; lane = "host"; col = 4
title = "The encoder rung"; body = "one rung deeper where bags read too coarse"
status = "planned"; note = "record-only"
[[step]]
id = "manifest"; n = "5"; lane = "ops"; col = 3
title = "Registered arms"; body = "each beat free Reflex on one frozen read"
status = "live"

[[edge]]
from = "ask"; to = "floor"
[[edge]]
from = "floor"; to = "ans"; label = "sure"
[[edge]]
from = "floor"; to = "prune"; label = "not sure"
[[edge]]
from = "prune"; to = "score"
[[edge]]
from = "score"; to = "fuse"
[[edge]]
from = "fuse"; to = "enc"; label = "still unsure"
[[edge]]
from = "fuse"; to = "ans"; back = true; via = "gutter"
[[edge]]
from = "enc"; to = "ans"; back = true; via = "gutter"; label = "the think returns — planned"
[[edge]]
from = "manifest"; to = "score"; back = true; label = "swapped whole"

[[walk]]
title = "The same typed question"
text  = "Your app sends the same typed question it would send the free engine: a state plus questions — choice, score or yes-no. This spot question is one the game heads know."
steps = ["ask"]
in    = { lang = "json", src = "data/flows/instinct_flow/01_in.json", label = "POST /decide" }
[[walk]]
title = "The free floor answers first"
text  = "Reflex runs before anything paid: embed, route to the corpus, score every option, and the fused gate checks confidence. On this harder triage question the gate abstains — every outcome comes back null — and that abstain is what pays the rung above."
steps = ["floor"]
in    = { lang = "json", src = "data/flows/instinct_flow/02_in.json", label = "POST /decide · a harder question" }
out   = { lang = "json", src = "data/flows/instinct_flow/02_out.json", label = "not sure: the abstain that pays the rung" }
[[walk]]
title = "Sure — the specialist is never paid"
text  = "When the floor is sure, the answer goes straight back with calibrated confidence and a receipt, and no specialist is consulted. That is the rung model: you pay for thinking, never for re-answering what the free floor already knows."
steps = ["ans"]
in    = { lang = "json", src = "data/flows/instinct_flow/01_in.json", label = "POST /decide" }
out   = { lang = "json", src = "data/flows/instinct_flow/03a_out.json", label = "sure: outcome yes" }
[[walk]]
title = "Prune to the candidates"
text  = "Only the abstained question goes on. The survivors are pruned to the top candidates so the rung above scores a handful of options, not the whole list."
steps = ["prune"]
[[walk]]
title = "The specialist scores"
text  = "A per-domain specialist — a locked artifact whose digest is checked at every load — scores the candidates. Selection is a monotonic hot-swap of whole artifacts from the manifest: never a weighted blend, because blends do not preserve rankings."
steps = ["score"]
[[walk]]
title = "Fuse into one calibrated answer"
text  = "The composition fuses the floor's read with the specialist's: a cascade pick, or prior fusion lifting the calibrated favourite. A confident pick returns as the answer, receipt attached."
steps = ["fuse"]
[[walk]]
title = "Still unsure — one rung deeper"
text  = "Where the bag view reads too coarse, a trained encoder head thinks — GPU-hosted, weights on our hosts only. It is measured on the benchmark board today, record-only: the hosted lane opens later, and this is the public request it would carry."
steps = ["enc"]
illustrative = true
in    = { lang = "json", src = "data/flows/instinct_flow/04_in.json", label = "the abstained question, sent up" }
[[walk]]
title = "The manifest hot-swaps arms"
text  = "arsenal.toml is the one selection surface: a specialist joins only where it beat free Reflex on one frozen test read, and it is swapped whole — a decision always observes one whole server."
steps = ["manifest"]
```

## Status per step (the honesty table — re-check before every edit)

| # | Step | Lane | Status | Grounding |
|---|---|---|---|---|
| 1–2 | question, Reflex floor first | your machine | live | the release binary |
| 3a | sure answer + receipt, specialist never paid | your machine | live | the fused gate, first-class abstain |
| 3b.1–3b.3 | prune, specialist scores, fuse | the open lane | live — the hybrid serves | the serving manifest rows |
| 4 | the encoder rung | our GPU hosts | measured on the board · record-only · hosted serving unlaunched | the bench encoder cells |
| 5 | the manifest (registered arms only) | the open lane | live | arsenal law A5 |

When a status changes, edit the gfflow block above (then re-render with
reflex-site `python3 scripts/render_flows.py --only instinct_root`) and the
honesty table in the same commit. Walk payloads are derived by reflex-site
`scripts/build_flow_walks.py` from the captured release-binary wire — never
typed.
