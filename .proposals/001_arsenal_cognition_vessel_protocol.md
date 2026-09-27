# Proposal 001 — the `arsenal` protocol: cognition-vessel selection, serving laws, and the capability axis

**Status:** PROPOSED 2026-09-27 — name `arsenal` OWNER-RATIFIED 2026-09-27; substrate census complete (substrate-first Mode 1, 2026-09-27, two-agent sweep); Claude verdict AGREE (round 3, 2026-09-27). Companion: `../riir-ai/.proposals/048_limelight_cognition_budget.md` (the depth axis; canonical layer-authority map + basic-instinct gate live there). Highwater: `.proposals/.highwater` measured **000** (empty dir) at allocation; bumped to 001 here.

## Why now

- Selection of decision artifacts is **hard-coded everywhere**: `serving_posture()` is a match table (`src/server.rs:103-125`), `REGISTERED_SUITES` a const gate, suite→vessel a filename convention (`{suite}_v1.vessel`, `src/bin/serve.rs:331-345`). The reflex game heads are `include_str!`-baked into every build and **re-fitted at every boot** (`../riir-reflex/src/game_heads.rs:66-88,883-896`) — the measured init-bloat class (lazy ANE bucket 926.8 ms, `../riir-reflex/.benchmarks/042/BENCH.md:19-26`; first-forward cliff p99 −64–67%, Bench 006; six lanes ready ≤14.7 s).
- Owner directive: Tetris/flappy/lanes are **not basic instincts** — they are product-specific fitted heads and must become plug-and-play vessels, selected like cargo features, without MoE-style routing latency or sharding bloat.

## Substrate check (substrate-first skill, Mode 1)

- **Searched:** vessel format/pins/classes, manifest/loadout/quiver/arsenal/kit, pin table, winners/vessel dirs, serving posture, pick_domain/set_admission/contrastive_scope/distance_abstain, hot-swap/blend/epoch, ValidatorManifest.
- **Found (consume):** reflexer-vessel format v1 (68 B header, ed25519, two classes, `PinTable` + empty `DEFAULT_PIN_KEYS`, monotonic `check_monotonic` refusing downgrade/fork, 1 MiB / 16 MiB class caps); instinct serving (listener-first, 503-while-loading, per-suite applied state, frozen-predictions parity gate, `Cascade` top-k prune, banking77 G3 FAIL → A0); four modelless routers in katgpt-core (`pick_domain` ns-class, `set_admission` Vendi certificate, `contrastive_scope` off-distribution refusal, `CorpusDistanceGate`/`fused_should_abstain`); Plan 333 AOI LRU GC (`neuron_vessel_runtime`); `ValidatorManifest`/`ValidatorPackage` (`../riir-ai/crates/riir-engine/src/arena/validator_package.rs:45-161`) — the workspace's one manifest precedent (TOML frontmatter + artifact + domain/version).
- **Missing (build):** the manifest itself; build-time capability features; a runtime swap driver; the basic-instinct gate (canonical in 048).
- **Rules checked:** reflexer law 1 (artifact hot-swap, **NEVER a weighted blend**); private-forever (HOSTED-ONLY never leaves controlled hardware); no-game-crates dep law; `riir-ai/BOUNDARY.md:144-148` (the GAME side is the consumer and reaches serving over the wire; zero deps INTO riir-ai — and none outbound from riir-ai either, enforcement owed per 048 L10/T7).

## The laws (hard rules)

1. **A1 — Bytes are runtime; capability is compile-time (VESSELS only).** Cargo features select readers/routers/classes; weights and vessels are never committed, never compiled in; build-time selection of vessel *bytes* is refused. Layer-0 substrate is EXEMPT by the basic-instinct gate (048 L7): compiled anchor sets and trained-but-generic artifacts behind features (`npc_brain_anchors`) are substrate, not vessels.
2. **A2 — Never blend.** Selection is monotonic atomic hot-swap of whole vessels (reflexer law 1 verbatim). Weighted blends / MoE-mixing of decision artifacts are refused — blends do not preserve move rankings.
3. **A3 — Coarse vessels, cheap routing, no dyn dispatch.** Vessels are suite/domain-grained; per-decision routing is modelless ns–µs (`pick_domain`/`set_admission`/`contrastive_scope`/`CorpusDistanceGate`); fine-grained MoE at the decision layer is refused. Bench 558's G2 fail was NOT the router — `pick_domain` measured ~10 ns — it was `Box<dyn ErasedCluster>` + `override_pi` virtual calls (~50 ns); so A3 additionally bans dyn dispatch on the per-decision path (monomorphize). Consult must be GOAT-justified (banking77 G3 FAIL precedent). Weight-space MoE stays where it already lives: dMoE/LatentMoE at the shard/kernel tier.
4. **A4 — Product-specific → vessel.** The basic-instinct gate (canonical: 048 L7) decides what may never become a vessel. Tetris/flappy/lanes heads are vessels, not instincts.
5. **A5 — The manifest is the only selection surface.** No new hard-coded match tables or filename conventions; suite→vessel→posture→pins live in a validated manifest; boot drift fails loud.
6. **A6 — Posture rows are pinned in both media.** A posture change without a GOAT re-run + frozen-predictions parity update is a review block. After T2 the posture is DATA, so `tests/serve_gates.rs` pins the manifest's BLAKE3 digest (or asserts its rows against a pinned table) — a TOML edit must red exactly like a code edit does today.
7. **A7 — Lazy + budgeted, evicted by wire signal.** Listener-first, 503-while-loading (exists); per-vessel load budget; LRU eviction triggered by a wire-level release/curation message from riir-ai's L5 curator — a wire-only server has no AOI to observe (AOI is game-side, 048); `set_admission`'s Vendi certificate bounds arsenal redundancy (the hoarding gate). Long init is a defect, not a posture.
8. **A8 — No runtime minting.** Vessels are minted train-time only (`../riir-train/crates/riir-train-engine/src/bin/vessel_mint.rs`); runtime swaps among minted artifacts; consolidation→vessel is a future plan with its own GOAT. Glacial consolidation produces NeuronShards (memory), not serving vessels.
9. **A9 — Same-engine-class only.** No cross-arch plug-and-play; compatibility = format version + reader capability + digest pin. (Analogy, not precedent: katgpt-rs Proposal 009 demoted cross-arch canonical hidden-state adapters — a different surface — and A9 is the conservative consequence of that negative for vessels.)
10. **A10 — Moat.** PUBLIC-RELEASE vessels carry no GAME-IP product content (no seal/MMORPG assets, tables, or worlds); the reflex arena demo heads (tetris/flappy/lanes) are public by design — they already ship in the public reflex repo — so their vessels may be PUBLIC-RELEASE. HOSTED-ONLY vessels never leave controlled hardware and have no writer in this repo (minting: riir-train). Game-specific cognition wiring lives in riir-ai + private repos (see 048 L10); katgpt-rs ships **primitives only** — the four routers + `decision_wire`, all already shipped, no new katgpt-rs surface.

## The manifest (sketch; `ValidatorPackage` precedent)

```toml
# arsenal.toml — one per HOST/deployment (zone keying is game-side, 048); digest-validated at boot; drift fails loud
[[vessel]]
suite    = "banking77"
digest   = "blake3:…"          # boot refuses drift, loud
class    = "hosted_only"       # public_release | hosted_only
posture  = { arm = "H2", beta = 1.0, n_min = 2, tau_n = 8 }   # serving_posture's arms carry params — not a bare string
pin_keys = ["mint-2026-09"]    # reflexer PinTable key ids
budget   = { load = "lazy", max_payload_mb = 16 }
```

Boot: manifest ↔ files (digests, pins, class-vs-reader-capability) validated before lanes resolve; unknown posture refused, never defaulted.

## Layer authority (serving face; canonical table: 048)

L2 looks up · **L3 routes** (ns–µs, never loads) · **L4 swaps** (POLICY + timing, epoch-tagged, state boundaries, never mid-combat) · **L5 curates** (pin/retire/GC/request mint, ≤0.1 Hz). Split of labor: swap POLICY is game-side (048 L9); the atomic monotonic MECHANISM is here (reflexer `check_monotonic` — the real precedent; the laya `(ptr,len,epoch)` chain-cache is an analogy only). INTERIM until L4/L5 ship: boot- and operator-driven swaps + `MindState` boundaries are the only callers.

## Noise-reduction refactor plan (scatter → one surface)

- `serving_posture()` match + `REGISTERED_SUITES` → manifest rows, re-pinned by the frozen-predictions parity gate (`tests/serve_gates.rs`).
- Game heads → pre-fitted PUBLIC-RELEASE vessels (task lands in **riir-reflex**; `head_digest` BLAKE3 pins map onto the signed payload commitment — strictly stronger); boot-fit cost leaves the public binary.
- Capability features across reflexer/instinct/reflex: `vessel_public_read` / `vessel_hosted_read` / router selection — the compile axis.
- Disambiguation tables into reflex/instinct/reflexer AGENTS (game layer names vs repo names; 047's entanglement).

## GOAT gate

`arsenal_budget_goat`: init p99 under a full manifest (lazy bounds held); routing ns–µs at k=dozens; swap atomicity (no torn reads, no blend) under concurrent decisions; armed-off = bit-identical. Feature-gated; promote only on GOAT.

## Plan

- [ ] T1 `ArsenalManifest` (TOML) + boot validation + loud drift refusals (riir-instinct; adds the `toml` dep — NOT in Cargo.toml today, which carries only serde/serde_json — so the BOUNDARY.md allowlist row lands in the same commit, beside the "Owns: selection protocol" row)
- [ ] T2 posture→manifest migration + parity re-pin (serving posture becomes data)
- [ ] T3 capability features (reflexer/instinct/reflex)
- [ ] T4 heads→vessels extraction in riir-reflex (PUBLIC-RELEASE class, lazy load)
- [ ] T5 budget legs: lazy / wire-triggered eviction (L5-curator release message — no AOI server-side) / Vendi hoarding gate
- [ ] T6 atomic monotonic swap MECHANISM (riir-instinct, reusing reflexer `check_monotonic` — the real precedent; the laya epoch is an analogy only) under the swap POLICY owned game-side (048 L9; interim callers: boot / operator / `MindState` boundaries). Shared epoch-tag contract with 048 T6
- [ ] T7 laws into reflex/reflexer/instinct AGENTS + disambiguation tables
- [ ] T8 `arsenal_budget_goat` + GOAT gate + promotion decision
- [-] Runtime minting (consolidation → vessel at Glacial) — deferred, needs its own GOAT; A8 stands until then
- [-] S-LoRA-scale multi-tenant batched serving — deferred (Plan 040 "far future" posture stands)
- [-] reflexer format v2 bundle/index — deferred; the manifest lives beside files, format v1 unchanged

## References

- `../riir-ai/.proposals/048_limelight_cognition_budget.md` (depth axis; canonical layer map + basic-instinct gate + moat split)
- reflexer AGENTS laws (never-blend); reflexer-vessel format v1; riir-instinct Bench 002 serving posture + banking77 G3 FAIL; katgpt-rs Proposal 009 (cross-arch canonical hidden-state directions — an analogy, not a precedent), Bench 558 (dyn-dispatch G2 fail; `pick_domain` itself ~10 ns), R161 dMoE, Benches 461/462 (router reconditioning at snapshot swap)
- S-LoRA/dLoRA (multi-adapter serving — deferred); FrugalGPT/RouteLLM (cascade/router prior art); options framework (initiation set = basic-instinct boundary)
