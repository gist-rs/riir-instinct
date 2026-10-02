# Plan 007 — Rethink product site + access-plane e2e (site, API lane, CLI billing, the ai.gist.rs link)

**Status:** PLANNED — filed 2026-10-02 (owner ask: finish the Rethink e2e — a prod-grade product website with `why`/`how`/`try`/`buy`/agents sections, the ai.gist.rs link owner-gated after testing, and Proposal 051 Phase 5 refined same-day: the API lane product, the binary CLI payment lane via Cloudflare pay, and the GitHub + email sign-in identity plane). Plan-only — nothing implements until the owner waves.

Branch: `develop` (per global rule — no feature branches)
Owner: katopz (site sections; **$5.00/mo**; pay handle **`katopz.cloudflare.pay`**; every credential, constant, deploy, and the umbrella link)
Expands: [Proposal 051](../../../riir-ai/.proposals/051_rethink_all_tier_adaptive_serving_family.md) Phase 5 (refined in this filing: T5.4 amended + T5.5–T5.8) × [riir-reflex Research 005 + Plan 011](../../../riir-reflex/.plans/011_clef_lane_jdi_protocol.md) (the `why` benchmark feed) × the reflex.gist.rs section patterns (hero / three-steps / playground / measured-with-losses / skill) × the riir-dapps precedents (OAuth lane, money-route error codes, SIWR CORS, fail-closed posture, offline view)

House laws this plan obeys (inherited, restated once): published numbers are GENERATED, never hand-typed; a lane/feature without credentials REFUSES loud, never half-runs; every latency number carries box state (bench_preflight law); no Python sidecars at RUNTIME in product repos (the C1 generator is a BUILD-TIME tool, the `publish_bench.py` precedent — never a serving-time dependency); zero new token ledgers (014's one-ledger law); money/posture surfaces fail closed; deploys are MANUAL (the Actions spending law); secrets never enter a repo.

## 0. What the owner ask did not cover — filled (the deltas this plan adds)

| # | Gap | Fill |
|---|---|---|
| 1 | "$5/mo" needs verification, activation, renewal, revocation — not a static QR | the access plane (Phase F): entitlement store + Ed25519-signed entitlement tokens + the claim flow |
| 2 | Identity for accounts + API keys | GitHub OAuth (the dapps `KAT_WALLET_OAUTH` lane pattern) + email magic link (Cloudflare Email Service native `send_email` binding; SPF/DKIM) |
| 3 | The API lane product shape | Phase F: keyed hosted decisions, per-tier quotas, key rotation, stable error-code table, CORS closed-by-default |
| 4 | Binary CLI payment | Phase G: `rethink login` / `rethink pay` / `rethink activate <token>` — offline-verifiable, the vessel-signature shape |
| 5 | Billing hygiene | refund policy, renewal reminders (the email lane), currency/tax note, dispute contact |
| 6 | Legal/trust surfaces | ToS, privacy (PII minimization + deletion path), acceptable use (agents included), `security.txt`, status/health capsule, changelog |
| 7 | The honesty law for `why` | the JDI crosswalk caveat rendered VERBATIM (011 B5); numbers only from the lane-doc/bench feeds; the Clef row appears when 011 Phase C lands — never a placeholder, never a zero |
| 8 | Two payment planes must not fuse | consumer USD subscription = an ACCESS-PLANE entitlement store; KAT/TUNA burn metering for fleet binaries = unchanged (051 T5.2). One-ledger law holds: no new token ledger, no new chain tier — the USD plane is entitlements, never a currency |
| 9 | Discoverability | SEO meta, OG/social cards, sitemap/robots, favicon, the ai.gist.rs umbrella card (owner-gated, Phase H), reflex↔rethink reciprocal links |
| 10 | Abuse + ops | signup kill switch, email-verified trial, rate limits, spend ceilings (email lane + hosted trial), secret leak scan, manual-deploy runbook, staging env first |
| 11 | a11y + polish | responsive pass, keyboard/contrast pass, 404 page, offline/503 view (the dapps offline-view precedent), cookieless CF Web Analytics (fits the no-telemetry brand) |
| 12 | Rename dependency | 051 Phase 1 rebrands site lane labels — NON-blocking: `LANE_DISPLAY` aliases keep history landing, so the site launches on current lane names and rebrands as a data refresh when the rename wave lands. The rename ALSO breaks `../../../<repo>/` relative links (labels-aliases do not cover paths) — the rename plan owns a link-sweep row (Dependencies) |

## Phase A — decisions + constants (owner-gated, one commit)

- [ ] A1 domain **`rethink.gist.rs`** — custom domain attached via the CF dashboard, NEVER a `routes` block in wrangler.toml (the reflex-site recorded law: a routes block makes every future deploy need zone-route permission no deploy token has; workers.dev stays as the raw URL)
- [ ] A2 repo **`gist-rs/rethink-site`** (public): the static site + ONE Worker (the access plane, Phase F); the product's own Rust never lives here (source-free public-surface discipline)
- [ ] A3 constants module with owner numbers, never literals at use sites (the µ/KAT lesson): `PRICE_USD_MICRO = 5_000_000` ($5.00/mo), `PAY_HANDLE = "katopz.cloudflare.pay"`, tier caps (hosted decisions/day per tier — owner numbers), hosted-trial budget, **refund window (owner/legal decision — A6; left blank here, never an agent-typed default)**, grace period
- [ ] A4 payment postures pinned (availability-gated, each state fail-closed): **P0a (now)** — the buy card renders the **"waitlist / coming soon"** state (the F3 email-fallback copy class: Wallets transfers are "soon" per the 2026-08-04 blog — nothing collectable, nothing claimable); **P0b (the day transfers land)** — pay-handle display + QR + the payment-reference claim flow; **P1 (design-only)** — x402 headless via Monetization Gateway with Virtual-Wallet guardrails + `cloudflare.pay` agent identity
- [ ] A5 brand: wordmark, palette slot (the reflex-site LANES-palette precedent), copy tone — capabilities-only until settleable (the P013 law); no accuracy claim without its measured cell + disclosure
- [ ] A6 **owner decisions (tokenomics + legal — NO default implied, the constants stay blank until answered)**: (a) **plane precedence** — a request carrying BOTH an `rk_` USD entitlement and a staked identity is metered by WHICH plane; what a USD entitlement maps to on the stake→domain tier ladder (a tier of its own? a cap class?); quota-exhaustion behavior — `429` with an upsell, or fall-through to TUNA/KAT burn, and in which order (this is the gap where $5/mo could quietly undercut the stake ladder — owner-priced before B5 copy exists); (b) the **refund window**; (c) **tax/VAT handling** for a global consumer subscription (owner/legal — register-to-collect vs price-includes vs geo-gate); (d) whether **Phase H launches on the free-lane walk alone** or waits for the paid walk (Phase I splits them)

## Phase B — rethink-site skeleton + static sections

- [ ] B1 hero: the one-paragraph product line (the adaptive decision-serving family: the free Reflex floor below, Rethink rungs above), install/try/buy CTAs, the measured-facts strip (G-gate numbers only, sourced)
- [ ] B2 `why` section shell (renderer only; data lands in Phase C): latency + accuracy cards, the "your data stays yours" line (loopback posture, abstain-first), the hosted-vs-local latency disclosure
- [ ] B3 `how` section shell: the SVG slot + the tier-rung table (from 051's rung map: Reflex floor / L1 bags / L2 surrogate / L3 encoder think / L4 batch / L5 curation) + the link to the /bench/ board
- [ ] B4 `try` section: the free floor FIRST — link back to **reflex.gist.rs** (playground + install commands), then the Rethink hosted-trial CTA (TUNA-credit lane, the 051 T5.2 posture; kill-switchable; email-verified)
- [ ] B5 `buy` section: the $5.00/mo card (what it includes: the hosted API tier + premium domains as they seat — **copy pending A6(a)**), the pay handle + QR (P0b), the claim link, the renewal/refund lines; **renders the P0a waitlist state until transfers are live** (fail-closed, the F3 copy class)
- [ ] B6 agents section (the reflex `#skill` pattern): curl-install blocks for `.claude/skills/rethink-integration/` + `.agents/skills/rethink-integration/`, the two FAQ folds ("which agents does it work with?" / "what does it change?"), the agent-payment note (Virtual Wallets + `cloudflare.pay` identity; capabilities-only until P1)
- [ ] B7 prod-grade static set: pricing/FAQ, docs index, security & privacy, terms, acceptable use, changelog, status page (health capsule), 404, offline/503 view, `security.txt`, sitemap.xml, robots.txt, favicon + OG/social cards, cookieless CF Web Analytics, responsive + a11y pass
- [ ] B8 CI: build + link check + secret leak scan + the no-hand-typed-numbers guard on `why`/`try` (the renderer reads data files; a literal number in those sections reds — the docs-gate class); deploy is MANUAL (wrangler)
- [ ] B9 staging posture first (the workers.dev preview env, devnet-class discipline), custom domain only after the checklist passes

## Phase C — the `why` benchmark feed (data-driven; 011-dependent)

- [ ] C1 consume the existing feeds: the arena `hybrid_lane_doc.json` (auto-emitted, reflex Issue 017 T5) + the reflex-site `publish_bench.py` flow; a `build_why_data.py`-class generator (a BUILD-TIME tool, the publish_bench precedent — not a runtime sidecar) emits `data/why.json` — numbers byte-derived from the source records, never typed
- [ ] C2 the Clef crosswalk rows land when reflex Plan 011 Phase C lands (`--clef` lane cells + the case-identity pin); the honesty caveat renders VERBATIM from 011 B5 ("JDI-comparable crosswalk, not a JDI board row — different corpus, protocol caps, hardware; board membership requires their full frozen suite") beside every crosswalk cell
- [ ] C3 the headline cell: typed_decisions — Clef-hosted vs Rethink (encoder; record-only until 016's D1 seats it) vs the laya-typed floor vs AgentJev (det ✗ disclosed); latency columns carry serving posture + box-state lines (bench_preflight law); hosted rows disclose the network round-trip (the JDI's own hosted-row precedent)
- [ ] C4 refresh flow: a republish script + a dated `data/provenance.json` (source records, blake3 pins, box states, generation time); the site renders the generation date beside the tables
- [ ] C5 gates: renderer-determinism fixtures + the no-placeholder law (a missing cell renders "measured pending", never a fabricated number, never a zero)

## Phase D — the `how` SVG + docs (the mirror law)

- [ ] D1 the SVG: the tier-rung ladder (Reflex floor → L1 → L2 → L3 → L4 → L5) + the adaptive-selection flow (salience/deadline/abstain → rung choice → receipt) — source of truth in the rethink repo `.docs/` (ungated), MIRRORED to the site (the reflex sync_mirror law; `--check` mode in CI fails on drift and skips loud without the sibling checkout — a skip is a deferral, never a green)
- [ ] D2 docs index: quickstart (hosted API + binary + skill), the decision_wire shape link (the katgpt-rs catalog note), the receipts page (what a receipt contains; consent-gated decstat)
- [ ] D3 every number in prose carries its source-record link; no orphan claims (the bench_doc_audit class)

## Phase E — the agents section content

- [ ] E1 `rethink-integration` SKILL.md (source of truth in the rethink repo `.docs/04_agent_skill/`, mirrored): the decision_wire calls, abstention handling, thresholds-from-your-data, hosted-endpoint auth (API-key header), rate-limit/error codes, the traps
- [ ] E2 the agent identity/payment page: `cloudflare.pay` handle declarations, Virtual-Wallet guardrail summary, the x402 posture (capabilities-only until P1 lands)

## Phase F — the access plane (identity + entitlement + the API lane)

- [ ] F1 Worker routes + the error-code table (the dapps money-route law: ONE code per failure class, stable forever, additive-only): `/auth/github/start|callback`, `/auth/email/start|verify`, `/account`, `/account/keys` (create/rotate/revoke), `/claim` (payment reference → **owner-approved** entitlement, G1), `/v1/decide` (keyed proxy; quota enforced), `/healthz`, `/status`. **The `/v1/decide` origin is named**: 014 Phase 4's hosted-serve routes (the rethink serve binary behind the plane) — the Worker authenticates to the origin with a shared secret header (mTLS/Tunnel if the owner prefers; one mechanism, pinned); **origin-down is a stable `503 host_unavailable`-class code** from the table and the status capsule reflects it (never a silent fallback)
- [ ] F2 GitHub OAuth: per-env app (client id in vars, secret via `wrangler secret put` — the dapps `KAT_WALLET_OAUTH` posture), state-cookie CSRF, device flow deliberately OFF; sessions signed, short-TTL, HttpOnly
- [ ] F3 email sign-in: Cloudflare Email Service native `send_email` binding (no API keys); single-use 10-min signed tokens (hashed at rest), send-rate limits + a spend ceiling, SPF/DKIM on the sending domain; **build-time check: the account's Email Service must permit arbitrary recipients — absent that, email sign-in ships FAIL-CLOSED (the button renders "waitlist") and GitHub sign-in carries the lane alone; never a half-verified email flow**
- [ ] F4 PII law: the email address stored purpose-bound (sign-in + renewal notices only), a deletion path at `/account`, no analytics join on email, receipts carry no PII (the decstat consent law)
- [ ] F5 entitlement store: KV/D1 rows `{account, plan, renews_at, status, payment_ref UNIQUE}`; entitlement TOKENS are Ed25519-signed (the vessel-signature shape) and **embed `plan`, `renews_at`, and `expires_at`** — offline verification carries its own time bound, since online revocation only reaches clients that check in; revocation = a denylist epoch the client checks online; **`payment_ref` is UNIQUE — one reference maps to ONE entitlement, ever (the claim-replay guard)**
- [ ] F6 API keys: `rk_` prefix, blake3 hash at rest (never the key), per-key quotas (tier constants), rotation with grace, `401/403/429` codes; CORS allow-list closed-by-default (the dapps SIWR CORS law)
- [ ] F7 gates: route contract tests per route (the dapps discipline), magic-link single-use + expiry tests, key-hash verification tests, quota tests, the signup kill-switch test (fail-closed), **the forged-reference test (a made-up payment reference NEVER issues an entitlement) + the replayed-reference test (a second claim on a used reference is refused, `already_settled` class)**

## Phase G — buy + the CLI billing lane

- [ ] G1 buy flow P0b: the user pays `katopz.cloudflare.pay` $5.00/mo → submits the payment reference at `/claim` → **the claim enters a MANUAL OWNER-APPROVAL QUEUE — an entitlement is never auto-issued from an unverified reference (no Wallets receipt/verification API exists at filing time; until a verifiable receipt source lands, a human approves)** → the entitlement issues on approval; the claim page names the exact handle, amount, cycle; honest copy: manual renewal notice until Wallets recurring lands
- [ ] G2 renewal/dunning: the email lane sends the renewal notice + a grace window (constant); expiry → plan reads `expired`, the served tier downgrades to free (never a hard stop on the free floor)
- [ ] G3 CLI (the `rethink` binary, Phase 5 dist): `rethink login` (GitHub OAuth loopback/web flow), `rethink pay` (opens the handle pay flow / prints QR + reference instructions), `rethink activate <token>` (verifies the Ed25519 entitlement OFFLINE, stores it in the account dir 0600), `rethink account` (status readout — the cargo-refine `--info` pattern)
- [ ] G4 P1 design only (deferred wiring): x402 headless purchase via Monetization Gateway — the API prices the subscription as a resource; agents buy with Virtual Wallets; identity = the `cloudflare.pay` handle
- [ ] G5 boundary pins: the USD plane writes NO token-ledger rows; KAT/TUNA burn metering (051 T5.2) untouched; the one-ledger law holds — an access-plane entitlement store, never a currency
- [ ] G6 gates: CLI↔plane contract tests (login → claim → activate → serve e2e on staging), offline-verification test (token validates with no network, and EXPIRES offline), expiry/downgrade test, tampered-token refusal test, **the forged-reference and replayed-reference refusals re-asserted at the CLI boundary**

## Phase H — the ai.gist.rs umbrella link (OWNER GATE)

- [ ] H1 after B–G are green AND the owner has tested the site: the dapps `umbrella.rs` gains the Rethink card/link (the third product beside Reflex/Refine); deploy manual; the umbrella's product-origin CORS stays
- [ ] H2 the reflex.gist.rs reciprocal: a small Rethink pointer on the reflex bench page (the reverse of the rethink→reflex try link) — owner-gated the same way
- [ ] H3 **the owner question from A6(d), answered at H time**: does the umbrella link launch on the FREE-lane walk alone (the paid walk being gated on Wallets transfers), or wait for the paid walk? — an explicit owner call, never implied

## Phase I — launch checklist (the prod-grade gate)

- [ ] DNS + cert on rethink.gist.rs; workers.dev preview green first
- [ ] secrets: every var through `wrangler secret put` (never repo, never echoed — the .env law); leak scan green
- [ ] rate limits + abuse: email-send ceiling, claim-rate ceiling, trial kill switch, signup kill switch — all TESTED failing closed
- [ ] status/health: the `/healthz` capsule rendered on the site (the dapps page-capsule pattern); the uptime note honest
- [ ] SEO/OG: meta + cards validated, sitemap submitted, favicon
- [ ] a11y: keyboard + contrast + labels pass; responsive at 375 px
- [ ] legal: ToS + privacy + acceptable use + refund policy live; the support route (inbound email routing to the owner) live
- [ ] analytics: CF Web Analytics (cookieless) only; no decision telemetry beyond consent-gated decstat receipts
- [ ] **the FREE-lane e2e walked by a human (completes NOW, no external dependency): install → login (GitHub; email if F3's posture allows) → API key issued → hosted call with quota → receipt → status capsule**
- [-] **the PAID-lane e2e walked by a human (DEFERRED — gated on Wallets transfers being live): pay (staging handle) → claim → owner approval → activate → entitlement-bound call → renewal notice** — the `- [-]` defer is honest: "finish Rethink e2e" is availability-gated on Cloudflare shipping transfers, and the plan says so instead of pretending

## Dependencies + sequencing

- reflex Plan 011 Phase C feeds C2/C3 — the site ships its `why` renderer first; the Clef row arrives by data (never blocked, never faked)
- 051 Phase 1 (the rename) rebrands lane labels — aliases make it non-blocking (launch on current names; rebrand is a data refresh). ⚠ The rename ALSO breaks the `../../../<repo>/` relative links this plan and 051 use (`riir-instinct/` → `riir-rethink/` directory move) — `LANE_DISPLAY` aliases cover lane LABELS, never file paths; **the rename plan (006) owns a link-sweep row for these** — recorded here so it is not lost
- Cloudflare Wallets transfers + Monetization Gateway availability gate P0b/G1-recurring and G4 — P0a (waitlist) and the free lane ship without them
- Email Service arbitrary-recipient posture gates F3's full lane — the fail-closed fallback is pinned
- A6(a) plane-precedence gates B5 copy and the F6 quota semantics — owner-priced before either ships
- The rename does NOT gate the site: `rethink-site` is a new public repo and lands before `riir-instinct` → `riir-rethink` moves (the dist naming follows the rename)

## Out of scope

- x402 wiring until Monetization Gateway joins (G4 design-only)
- KAT-denominated consumer pricing; crypto onramp UX
- Mobile apps; i18n; a second brand system
- Any change to the reflex/reflexer repos beyond the reciprocal link (H2)
- Mainnet anything (the owner ceremony)

## Validation

Per phase: B7/B8 static gates + link check; C5 renderer fixtures; D1 mirror check in CI; F7 route contracts; G6 CLI contract tests; I the human e2e. No phase ships without its gate; the umbrella link (H) is the LAST gate and owner-waved.
