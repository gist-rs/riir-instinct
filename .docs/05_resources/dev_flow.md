# The Instinct dev build flow

How a specialist gets made and put on duty — the development counterpart
of the [composition figure](../03_decision_flow/instinct_flow.md), which
shows what happens at answer time. One figure:

1. **make corpus** — the training corpus is assembled from public data.
2. **train specialist** — one model per domain, trained on that corpus.
3. **stratified holdout + winner law** — the GOAT gate: the candidate
   earns a row only by strictly beating the free engine on one frozen,
   label-stratified test read.
4. **BLAKE3-lock** — a passing artifact is locked into a tamper-evident,
   checksum-verified artifact; nothing trains at serve time.
5. **manifest row** — the locked winner is registered in `arsenal.toml`,
   the ONE selection surface (the composition figure's hot-swap lane).
6. **serve + receipts** — the lane answers with a decision receipt:
   build fingerprint, BLAKE3(input), BLAKE3(canonical decision), lane id.

```mermaid
%% file: dev_flow.svg
%% aria: The Instinct development build flow: a corpus is made from public data, a per-domain specialist is trained on it, a stratified holdout read applies the winner law so only a strict win over the free engine survives, the passing artifact is BLAKE3-locked, registered as a manifest row, and served with decision receipts
flowchart LR
  C["make corpus<br/>public data"] --> T["train specialist"] --> H["stratified holdout<br/>winner law — the GOAT gate"] --> S["BLAKE3-lock<br/>tamper-evident artifact"] --> M["manifest row<br/>arsenal.toml"] --> V["serve + receipts"]
```
