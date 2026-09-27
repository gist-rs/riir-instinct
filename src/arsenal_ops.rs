//! The arsenal runtime mechanisms (Proposal 001 T5+T6) — the library
//! half the serve binary wires to the HTTP edge:
//!
//! - **The Vendi hoarding gate (T5, law A7):** `set_admission`'s exact
//!   cosine-kernel Vendi certificate bounds arsenal redundancy — a load
//!   that pushes the set of loaded suites past the certificate is
//!   refused loud, never silently accepted. The per-suite vector is the
//!   suite's **corpus centroid** folded into the 8-dim admission space
//!   ([`DIM`]): the sum of the L2-normalized hashed-token bags of every
//!   train doc, folded bucket-wise (`bucket % 8`). Two suites serving
//!   near-identical corpora produce colinear centroids; a hoarded set of
//!   pairwise-distinct-but-rank-deficient centroids collapses the
//!   certificate. Both anchors come from katgpt-core `set_admission`
//!   itself — the colinearity cap at `SetAdmissionConfig::theta_coll`
//!   (the shipped 0.95 prior) and the certificate floor at
//!   `rho_vendi` (0.2) over `certify_set`. Nothing here re-implements
//!   Vendi; the `alpha_align`/`kappa_div` greedy weights are unused (the
//!   gate judges a whole set, it does not run the greedy).
//! - **The atomic monotonic swap gate (T6, law A2):** one
//!   [`check_epoch_tag`] over reflexer-vessel's epoch-tag contract —
//!   advance / idempotent no-op / fork refused / downgrade refused —
//!   and [`LaneSlot`], the whole-snapshot slot registry: a decision and
//!   a swap serialize on one mutex, so every decision observes ONE
//!   whole server (never a torn read, never a blend). No dyn dispatch
//!   on the per-decision path (law A3) — the lane handle is a plain
//!   generic parameter.
//!
//! The kill-switch discipline is the decstat consent literal: only the
//! exact string `"0"` disarms ([`hoard_gate_armed_for`]) — a typo must
//! never silently restore a gate.

use std::sync::Mutex;

use katgpt_core::set_admission::{CertificateReport, SetAdmissionConfig, certify_set};

pub use katgpt_core::set_admission::DIM;

use crate::specialist::bag_into;

// ── T5: the Vendi hoarding gate ─────────────────────────────────────────

/// Fold one sparse L2-normalized bag into the 8-dim admission space:
/// bucket-wise accumulation (`bucket % 8`). Deterministic; the caller
/// normalizes (the certificate and the colinearity check are
/// direction-only).
#[must_use]
pub fn fold8(bag: &[(u32, f32)]) -> [f32; DIM] {
    let mut acc = [0.0_f32; DIM];
    for &(bucket, w) in bag {
        acc[bucket as usize % DIM] += w;
    }
    acc
}

/// A suite's corpus centroid: the signed simhash fold of per-doc
/// `bag_into` vectors over the suite's train corpus, projected into the
/// admission space ([`DIM`]). Zero for an empty corpus (the gate then
/// refuses to judge — see [`hoard_check`]).
///
/// **The sign is load-bearing (the Bench-003 GOAT finding):** each bag
/// entry's weight is multiplied by a deterministic ±1 derived from its
/// own bucket id (a token-level simhash fold). The UNSIGNED fold this
/// replaced sums bucket proportions, and every English corpus's bucket
/// histogram reads ≈ uniform — every suite's centroid landed within the
/// 0.95 colinearity cap of every other's (measured: the armed gate
/// admitted 0/6 real suites, each refused as a near-duplicate of the
/// first loaded), which inverts the gate: mirrors and strangers are
/// indistinguishable. With the signed fold, the same corpus (a mirror)
/// reproduces the identical token set and stays colinear (cos → 1),
/// while distinct corpora produce independent ±1 sums that decorrelate
/// (cos ≈ 0) — the near-duplicate signal the gate's contract names, and
/// the construction that makes the colinearity anchor decidable.
pub fn corpus_centroid<'a>(texts: impl Iterator<Item = &'a str>) -> [f32; DIM] {
    let mut acc = [0.0_f32; DIM];
    let mut bag = Vec::new();
    let mut tok = Vec::new();
    for text in texts {
        bag_into(text.as_bytes(), &mut bag, &mut tok);
        for &(bucket, w) in &bag {
            // Balanced ±1 from a multiplicative mix of the bucket id —
            // the raw high bit is useless here (ids are uniform over
            // VOCAB = 2¹⁷, so bit 31 is always 0), and a mixed bit is
            // robust to structure in the hash's low bits.
            let sign = if ((bucket.wrapping_mul(0x9E37_79B1) >> 16) & 1) == 0 {
                1.0
            } else {
                -1.0
            };
            acc[(bucket as usize) % DIM] += sign * w;
        }
    }
    acc
}

/// Unit-normalize (the cosine shell). `None` for zero/non-finite vectors —
/// the same admission rule `certify_set` applies internally.
fn unit(v: &[f32; DIM]) -> Option<[f32; DIM]> {
    if v.iter().any(|x| !x.is_finite()) {
        return None;
    }
    let n = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if n <= 0.0 {
        return None;
    }
    let inv = 1.0 / n;
    let mut out = *v;
    for x in &mut out {
        *x *= inv;
    }
    Some(out)
}

fn dot(a: &[f32; DIM], b: &[f32; DIM]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// The kill-switch predicate: only the exact string `"0"` disarms (the
/// decstat consent-literal discipline — a loose truthy check would let a
/// typo re-arm, and an absent value is ARMED, never disarmed).
#[must_use]
pub fn hoard_gate_armed_for(value: Option<&str>) -> bool {
    value != Some("0")
}

/// The live kill-switch read: `RIIR_INSTINCT_HOARD_GATE`.
#[must_use]
pub fn hoard_gate_armed() -> bool {
    hoard_gate_armed_for(std::env::var("RIIR_INSTINCT_HOARD_GATE").ok().as_deref())
}

/// Why a load refused the hoarding gate. Both arms name the numbers so the
/// operator can adjudicate without re-running anything.
#[derive(Debug, Clone, PartialEq)]
pub enum HoardRefusal {
    /// The candidate's corpus centroid is colinear with a loaded suite —
    /// a near-duplicate arsenal entry (the set_admission colinearity
    /// anchor at `theta_coll`).
    NearDuplicate { against: String, cos: f32 },
    /// The set INCLUDING the candidate reads rank-collapsed against the
    /// Vendi certificate floor (`vendi < rho_vendi · min(K, DIM)`).
    Collapsed { vendi: f32, floor: f32, k: usize },
}

impl std::fmt::Display for HoardRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NearDuplicate { against, cos } => write!(
                f,
                "near-duplicate suite refused: corpus centroid colinear with loaded \
                 {against:?} (cos {cos:.4} > the 0.95 cap) — hoarding a redundant arsenal entry"
            ),
            Self::Collapsed { vendi, floor, k } => write!(
                f,
                "hoarded set refused: Vendi certificate collapsed ({vendi:.3} < floor \
                 {floor:.3} at K={k}) — the loaded suites are rank-redundant"
            ),
        }
    }
}

/// What a passing check read (logged at every gated load — the gate is
/// observable, never silent about its numbers).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HoardReport {
    /// Suites in the certified set (loaded + candidate).
    pub k: usize,
    /// The certificate verdicts (`certify_set`).
    pub vendi: f32,
    pub participation_ratio: f32,
    /// `vendi ≥ 0.95 · min(K, DIM)` — the d=8 depth ceiling reached (a
    /// note, never a refusal).
    pub saturated: bool,
    /// The floor the check compared against (`rho_vendi · min(K, DIM)`).
    pub floor: f32,
    /// `false` when the candidate had no usable centroid (empty corpus) —
    /// the set was certified but the candidate could not be judged
    /// against the colinearity anchor. Admitted, never silently.
    pub judged: bool,
}

/// The hoarding gate (Proposal 001 T5): does admitting `candidate`
/// (vector `cand_vec`) push the loaded set past the set_admission
/// certificate?
///
/// Exact predicate, in order:
/// 1. **Colinearity anchor** — `cos(candidate, loaded_i) > cfg.theta_coll`
///    (the shipped 0.95) for ANY loaded suite →
///    [`HoardRefusal::NearDuplicate`].
/// 2. **Vendi certificate** — `certify_set` over (loaded ∪ candidate);
///    `report.collapsed` (`vendi < cfg.rho_vendi · min(K, DIM)`, ρ = 0.2)
///    → [`HoardRefusal::Collapsed`]. `saturated` is a report note, never
///    a refusal.
///
/// A candidate with no usable centroid (zero/non-finite — an empty
/// corpus) cannot be judged; it is ADMITTED with `judged: false` — the
/// gate never silently drops a suite, it says so. `candidate`'s name is
/// the caller's context (log lines carry it) — the gate itself judges
/// vectors, not names.
pub fn hoard_check(
    _candidate: &str,
    cand_vec: [f32; DIM],
    loaded: &[(&str, [f32; DIM])],
    cfg: &SetAdmissionConfig,
) -> Result<HoardReport, HoardRefusal> {
    let mut set: Vec<[f32; DIM]> = Vec::with_capacity(loaded.len() + 1);
    for (_, v) in loaded {
        if let Some(h) = unit(v) {
            set.push(h);
        }
    }
    let Some(hat) = unit(&cand_vec) else {
        let rep = certify_set(cfg, &set);
        return Ok(report(rep, set.len(), cfg, false));
    };
    for (name, v) in loaded {
        if let Some(h) = unit(v) {
            let cos = dot(&hat, &h);
            if cos > cfg.theta_coll {
                return Err(HoardRefusal::NearDuplicate {
                    against: (*name).to_string(),
                    cos,
                });
            }
        }
    }
    set.push(hat);
    let rep = certify_set(cfg, &set);
    if rep.collapsed {
        return Err(HoardRefusal::Collapsed {
            vendi: rep.vendi,
            floor: cfg.rho_vendi * set.len().min(DIM) as f32,
            k: set.len(),
        });
    }
    Ok(report(rep, set.len(), cfg, true))
}

fn report(rep: CertificateReport, k: usize, cfg: &SetAdmissionConfig, judged: bool) -> HoardReport {
    HoardReport {
        k,
        vendi: rep.vendi,
        participation_ratio: rep.participation_ratio,
        saturated: rep.saturated,
        floor: cfg.rho_vendi * k.min(DIM) as f32,
        judged,
    }
}

// ── T6: the atomic monotonic swap gate ──────────────────────────────────

/// One arsenal slot's applied epoch tag — reflexer-vessel's epoch-tag
/// contract verbatim (`(epoch: u64, digest: [u8; 32])`, the same shape as
/// the format crate's `ApplyState { artifact_version, commitment }`).
/// Also documented game-side (riir-ai Proposal 048 T6) — the contract is
/// shared, never forked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EpochTag {
    pub epoch: u64,
    pub digest: [u8; 32],
}

impl EpochTag {
    /// Boot genesis: epoch 0 from the manifest (the row's pinned artifact
    /// digest — the caller supplies the bytes).
    pub const GENESIS: Self = Self {
        epoch: 0,
        digest: [0u8; 32],
    };
}

/// The accepted-apply verdicts: a strict advance, or the idempotent
/// re-apply of the artifact already serving.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpochApply {
    Advance,
    Idempotent,
}

/// Why an apply refused — the format crate's own `ApplyRefusal` taxonomy
/// (`OlderThanCurrent` / `VersionFork`), spelled over the arsenal tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapRefusal {
    /// Replay/downgrade: the offered epoch is older than the applied one.
    Downgrade { applied: u64, offered: u64 },
    /// Same epoch, different digest — a lineage fork, not a successor.
    Fork { epoch: u64 },
}

impl std::fmt::Display for SwapRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Downgrade { applied, offered } => write!(
                f,
                "downgrade refused: epoch {offered} < applied {applied} (force is an \
                 operator action and logs)"
            ),
            Self::Fork { epoch } => write!(
                f,
                "lineage fork refused: epoch {epoch} with a different digest than the \
                 applied tag"
            ),
        }
    }
}

/// The epoch-tag gate (Proposal 001 T6, law A2) — reflexer-vessel's
/// `VerifiedVessel::check_monotonic` contract applied to the arsenal tag:
///
/// - `new_epoch > applied.epoch` → [`EpochApply::Advance`];
/// - `new_epoch == applied.epoch && new_digest == applied.digest` →
///   [`EpochApply::Idempotent`] (the idempotent no-op);
/// - `new_epoch == applied.epoch && different digest` →
///   [`SwapRefusal::Fork`];
/// - `new_epoch < applied.epoch` → [`SwapRefusal::Downgrade`].
///
/// **Attribution (the `src/vessel.rs` `signed_message` precedent):** the
/// format crate's method takes a `VerifiedVessel` receiver, which is only
/// constructible via `decode` — and `decode` refuses the HOSTED-ONLY
/// class by its own accident-guard law, so the hosted lane can never hold
/// one. This adapter mirrors the gate's branch structure verbatim and
/// reuses the format crate's refusal SEMANTICS (same arms, same wording);
/// the semantic identity is pinned against the real `check_monotonic`
/// under the `vessel` feature (see the `vessel_identity_tests` module).
/// The FORCE path is the caller's operator act and must log — it lives at
/// the edge, never here. (The `Result` carries its own `#[must_use]`
/// through both arms — no attribute needed here.)
pub fn check_epoch_tag(
    new_epoch: u64,
    new_digest: [u8; 32],
    applied: &EpochTag,
) -> Result<EpochApply, SwapRefusal> {
    if new_epoch < applied.epoch {
        return Err(SwapRefusal::Downgrade {
            applied: applied.epoch,
            offered: new_epoch,
        });
    }
    if new_epoch == applied.epoch {
        if new_digest == applied.digest {
            return Ok(EpochApply::Idempotent);
        }
        return Err(SwapRefusal::Fork {
            epoch: applied.epoch,
        });
    }
    Ok(EpochApply::Advance)
}

// ── the whole-snapshot slot registry ────────────────────────────────────

/// One suite's lane state. Every non-Ready state carries the applied tag —
/// eviction (release) NEVER rolls the epoch back (a re-load of a
/// downgraded artifact must refuse, not silently rewind), and a load in
/// flight must not lose the tag it will install under.
pub enum LaneState<L> {
    /// A lazy row not yet loaded (or evicted by a release).
    Unloaded { applied: EpochTag },
    /// A load in flight (boot, or the first decision after `Unloaded`).
    Loading { applied: EpochTag },
    /// Serving. `centroid` is the suite's corpus centroid (the hoarding
    /// gate's vector for this slot).
    Ready {
        server: L,
        applied: EpochTag,
        centroid: [f32; DIM],
    },
    /// The last load refused (the reason names the gate or the error).
    Failed { error: String, applied: EpochTag },
}

impl<L> LaneState<L> {
    /// The healthz state word.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unloaded { .. } => "unloaded",
            Self::Loading { .. } => "loading",
            Self::Ready { .. } => "ready",
            Self::Failed { .. } => "failed",
        }
    }

    /// The applied tag, in every state.
    pub fn applied(&self) -> Option<&EpochTag> {
        match self {
            Self::Unloaded { applied }
            | Self::Loading { applied }
            | Self::Ready { applied, .. }
            | Self::Failed { applied, .. } => Some(applied),
        }
    }
}

/// One suite's registry slot: the unit of the lazy load (T5), the wire
/// release (T5), and the atomic monotonic swap (T6).
///
/// **Atomicity (A2):** the decision path and [`Self::install_ready`]
/// serialize on the one mutex, and `install_ready` writes the whole
/// [`LaneState::Ready`] snapshot in one assignment — a concurrent
/// decision observes either the old server WHOLE or the new server WHOLE,
/// never a blend. Pinned by the hammer test below.
pub struct LaneSlot<L> {
    pub suite: &'static str,
    pub state: Mutex<LaneState<L>>,
}

/// What [`LaneSlot::install_ready`] did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallOutcome {
    /// The tag advanced — the new artifact is serving.
    Advanced,
    /// The tag matched the applied one — re-installed the same artifact
    /// (the lazy reload after eviction takes this arm).
    Idempotent,
    /// The operator forced past a refusal — logged at the edge, never
    /// here.
    Forced,
}

/// What [`LaneSlot::release`] did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseOutcome {
    Released,
    /// Already unloaded — an idempotent release.
    Noop,
}

/// Why a release refused. The lazy-posture check (manifest row) lives at
/// the caller; these are the slot-state arms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseRefusal {
    /// A load is in flight — refusing beats racing the loader.
    InFlight,
    /// The lane failed — nothing loaded to release (the error stays
    /// visible for diagnosis).
    NotReady,
}

impl<L> LaneSlot<L> {
    pub fn new(suite: &'static str, state: LaneState<L>) -> Self {
        Self {
            suite,
            state: Mutex::new(state),
        }
    }

    /// The lazy trigger (T5): `Unloaded → Loading` exactly once. `false`
    /// when the slot is in any other state (the caller then answers from
    /// it). The loader thread is the caller's to spawn.
    pub fn begin_lazy_load(&self) -> bool {
        let mut st = self.state.lock().expect("slot lock");
        match &*st {
            LaneState::Unloaded { applied } => {
                let tag = *applied;
                *st = LaneState::Loading { applied: tag };
                true
            }
            _ => false,
        }
    }

    /// Mark the last load refused (the loader's failure path). Keeps the
    /// applied tag — a failed load never rolls the epoch back.
    pub fn fail(&self, error: String) {
        let mut st = self.state.lock().expect("slot lock");
        let applied = st.applied().copied().unwrap_or(EpochTag::GENESIS);
        *st = LaneState::Failed { error, applied };
    }

    /// The T6 install: gate the tag against the applied one (unless
    /// `force`), then swap the WHOLE snapshot in one assignment. Works
    /// from every state — the loader installs into `Loading`, a swap
    /// replaces `Ready`, a swap into an evicted slot enters `Unloaded`.
    /// A refused gate leaves the state untouched (the old server keeps
    /// serving).
    pub fn install_ready(
        &self,
        server: L,
        tag: EpochTag,
        centroid: [f32; DIM],
        force: bool,
    ) -> Result<InstallOutcome, SwapRefusal> {
        let mut st = self.state.lock().expect("slot lock");
        let applied = st.applied().copied().unwrap_or(EpochTag::GENESIS);
        let outcome = if force {
            InstallOutcome::Forced
        } else {
            match check_epoch_tag(tag.epoch, tag.digest, &applied)? {
                EpochApply::Advance => InstallOutcome::Advanced,
                EpochApply::Idempotent => InstallOutcome::Idempotent,
            }
        };
        *st = LaneState::Ready {
            server,
            applied: tag,
            centroid,
        };
        Ok(outcome)
    }

    /// The T5 wire release: drop the loaded server, keep the applied tag.
    /// The caller checks the manifest's lazy posture first (eager rows
    /// refuse there — posture-as-data).
    pub fn release(&self) -> Result<ReleaseOutcome, ReleaseRefusal> {
        let mut st = self.state.lock().expect("slot lock");
        match &*st {
            LaneState::Ready { applied, .. } => {
                let tag = *applied;
                *st = LaneState::Unloaded { applied: tag };
                Ok(ReleaseOutcome::Released)
            }
            LaneState::Unloaded { .. } => Ok(ReleaseOutcome::Noop),
            LaneState::Loading { .. } => Err(ReleaseRefusal::InFlight),
            LaneState::Failed { .. } => Err(ReleaseRefusal::NotReady),
        }
    }

    /// The applied epoch (healthz disclosure).
    pub fn applied_epoch(&self) -> Option<u64> {
        self.state
            .lock()
            .expect("slot lock")
            .applied()
            .map(|t| t.epoch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    const CFG: SetAdmissionConfig = SetAdmissionConfig {
        alpha_align: 0.6,
        kappa_div: 0.2,
        theta_coll: 0.95,
        rho_vendi: 0.2,
    };

    // ── fold8 / corpus_centroid ─────────────────────────────────────────

    #[test]
    fn fold8_accumulates_bucket_wise() {
        // DIM = 8: buckets 0 and 8 fold together; 1 and 9 fold together.
        let bag = vec![(0u32, 1.0f32), (8, 2.0), (1, 3.0), (9, 4.0)];
        let v = fold8(&bag);
        assert_eq!(v[0], 3.0, "buckets 0 and 8 fold together");
        assert_eq!(v[1], 7.0, "buckets 1 and 9 fold together");
        for (i, x) in v.iter().enumerate().skip(2) {
            assert_eq!(*x, 0.0, "dim {i} untouched");
        }
    }

    #[test]
    fn corpus_centroid_is_the_sum_of_doc_directions() {
        let docs = ["bank transfer declined", "wire funds to the account"];
        let a = corpus_centroid(docs.iter().copied());
        // Deterministic + order-stable.
        let b = corpus_centroid(docs.iter().copied());
        assert_eq!(a, b, "the centroid is deterministic");
        assert!(
            unit(&a).is_some(),
            "non-empty corpora carry a direction"
        );
        assert_eq!(
            corpus_centroid(std::iter::empty()),
            [0.0; DIM],
            "an empty corpus is the zero vector (the unjudged arm)"
        );
    }

    // ── the kill switch ─────────────────────────────────────────────────

    #[test]
    fn the_kill_switch_disarms_only_on_the_exact_literal() {
        assert!(hoard_gate_armed_for(None), "absent = armed");
        for v in ["1", "off", "0 ", " 0", "false", "O"] {
            assert!(hoard_gate_armed_for(Some(v)), "{v:?} must not disarm");
        }
        assert!(!hoard_gate_armed_for(Some("0")), "exactly \"0\" disarms");
    }

    // ── the hoarding gate arms ──────────────────────────────────────────

    #[test]
    fn a_diverse_set_admits_and_reports() {
        // Orthogonal basis directions — maximally diverse.
        let loaded: Vec<(&str, [f32; DIM])> = (0..4)
            .map(|i| {
                let mut v = [0.0_f32; DIM];
                v[i] = 1.0;
                (match i {
                    0 => "ag_news",
                    1 => "emotion",
                    2 => "sst5",
                    _ => "banking77",
                }, v)
            })
            .collect();
        let mut cand = [0.0_f32; DIM];
        cand[4] = 1.0;
        let rep = hoard_check("xnli_en", cand, &loaded, &CFG).expect("diverse set admits");
        assert!(rep.judged);
        assert_eq!(rep.k, 5);
        assert!(rep.vendi >= rep.floor, "vendi {} ≥ floor {}", rep.vendi, rep.floor);
        // An orthogonal 5-set is MAXIMALLY diverse — it saturates the d=8
        // depth ceiling (vendi = K = 5 ≥ 0.95·5). The note, never a
        // refusal.
        assert!(rep.saturated, "an orthogonal set rides the depth ceiling");
    }

    #[test]
    fn a_near_duplicate_refuses_on_the_colinearity_anchor() {
        let base = unit(&[0.5_f32; DIM]).expect("non-zero base normalizes");
        let loaded = vec![("ag_news", base)];
        let rep = hoard_check("ag_news_mirror", base, &loaded, &CFG)
            .expect_err("an identical centroid is a near-duplicate");
        match rep {
            HoardRefusal::NearDuplicate { against, cos } => {
                assert_eq!(against, "ag_news");
                assert!(cos > 0.999, "identical corpora read cos 1.0, got {cos}");
            }
            other => panic!("expected NearDuplicate, got {other:?}"),
        }
    }

    #[test]
    fn an_empty_corpus_is_admitted_unjudged() {
        let loaded = vec![("ag_news", [1.0_f32, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])];
        let rep = hoard_check("no_corpus", [0.0; DIM], &loaded, &CFG).expect("unjudged admits");
        assert!(!rep.judged, "a zero centroid cannot be judged");
    }

    #[test]
    fn a_rank_redundant_set_refuses_on_the_certificate_not_colinearity() {
        // Seven unit vectors with ALL pairwise cos = 0.949: past the 0.95
        // colinearity cap (so no NearDuplicate can fire) but spectrally
        // near-rank-1 (the certificate collapses at K=7, floor 1.4,
        // vendi ≈ 1.28). Construction: v_i = c·w + √(1−c²)·u_i with u_i
        // orthonormal and w the remaining basis direction — pairwise
        // cos = c² = 0.949 exactly.
        let c2 = 0.949_f32;
        let c = c2.sqrt();
        let s = (1.0 - c2).sqrt();
        let vec_of = |i: usize| {
            let mut v = [0.0_f32; DIM];
            v[DIM - 1] = c;
            v[i] = s;
            v
        };
        // Load six: each passes (colinearity 0.949 < 0.95; the partial
        // certificate stays above its floor at every K ≤ 6).
        let mut loaded: Vec<(&str, [f32; DIM])> = Vec::new();
        for i in 0..6usize {
            let name: &'static str = match i {
                0 => "s0",
                1 => "s1",
                2 => "s2",
                3 => "s3",
                4 => "s4",
                _ => "s5",
            };
            let rep = hoard_check(name, vec_of(i), &loaded, &CFG)
                .unwrap_or_else(|e| panic!("member {i} must pass: {e}"));
            assert!(rep.vendi >= rep.floor, "K={} vendi {} < floor {}", i + 2, rep.vendi, rep.floor);
            loaded.push((name, vec_of(i)));
        }
        // The seventh pushes the set past the certificate.
        let err = hoard_check("s6", vec_of(6), &loaded, &CFG).expect_err("K=7 collapses");
        match err {
            HoardRefusal::Collapsed { vendi, floor, k } => {
                assert_eq!(k, 7);
                assert!(vendi < floor, "vendi {vendi} must read under floor {floor}");
            }
            other => panic!("expected Collapsed, got {other:?}"),
        }
    }

    // ── the epoch-tag gate arms (the format crate's own test values) ───

    #[test]
    fn the_epoch_gate_refuses_downgrades_and_forks() {
        let applied = EpochTag {
            epoch: 5,
            digest: [7; 32],
        };
        // Strict advance.
        assert_eq!(
            check_epoch_tag(6, [9; 32], &applied),
            Ok(EpochApply::Advance)
        );
        // Exact re-apply — the idempotent no-op.
        assert_eq!(
            check_epoch_tag(5, [7; 32], &applied),
            Ok(EpochApply::Idempotent)
        );
        // Downgrade.
        assert_eq!(
            check_epoch_tag(4, [7; 32], &applied),
            Err(SwapRefusal::Downgrade {
                applied: 5,
                offered: 4
            })
        );
        // Fork: same epoch, different digest.
        assert_eq!(
            check_epoch_tag(5, [8; 32], &applied),
            Err(SwapRefusal::Fork { epoch: 5 })
        );
        // Genesis accepts any epoch ≥ 0 with a real digest as an advance
        // (epoch 0 + genesis digest would be the idempotent no-op).
        assert_eq!(
            check_epoch_tag(1, [1; 32], &EpochTag::GENESIS),
            Ok(EpochApply::Advance)
        );
    }

    // ── the slot lifecycle ──────────────────────────────────────────────

    fn tag(e: u64) -> EpochTag {
        EpochTag {
            epoch: e,
            digest: [e as u8; 32],
        }
    }

    #[test]
    fn lazy_lifecycle_triggers_once_and_reloads_after_release() {
        let slot: LaneSlot<u64> = LaneSlot::new(
            "ag_news",
            LaneState::Unloaded {
                applied: tag(0),
            },
        );
        // The first trigger wins; every other call is a no-op.
        assert!(slot.begin_lazy_load());
        assert!(!slot.begin_lazy_load(), "a second trigger must not re-enter");
        assert!(!slot.begin_lazy_load());
        // The loader installs (same tag → the idempotent arm — the reload
        // after eviction installs through the same gate).
        assert_eq!(
            slot.install_ready(1, tag(0), [1.0; DIM], false),
            Ok(InstallOutcome::Idempotent)
        );
        // Release drops the server, keeps the tag.
        assert_eq!(slot.release(), Ok(ReleaseOutcome::Released));
        assert_eq!(slot.release(), Ok(ReleaseOutcome::Noop));
        assert!(slot.begin_lazy_load(), "a released slot re-triggers");
        assert_eq!(
            slot.install_ready(2, tag(0), [1.0; DIM], false),
            Ok(InstallOutcome::Idempotent)
        );
        let guard = slot.state.lock().unwrap();
        match &*guard {
            LaneState::Ready { server, .. } => {
                assert_eq!(*server, 2, "the fresh instance is serving");
            }
            other => panic!("expected Ready, got {:?}", other.as_str()),
        }
        drop(guard);
    }

    #[test]
    fn release_refuses_in_flight_and_failed_states() {
        let slot: LaneSlot<u64> = LaneSlot::new(
            "x",
            LaneState::Loading { applied: tag(0) },
        );
        assert_eq!(slot.release(), Err(ReleaseRefusal::InFlight));
        slot.fail("hoarding gate refused load: test".into());
        assert_eq!(slot.release(), Err(ReleaseRefusal::NotReady));
        // The failed state kept the tag.
        assert_eq!(slot.applied_epoch(), Some(0));
    }

    #[test]
    fn install_gate_refusal_leaves_the_old_server_serving() {
        let slot: LaneSlot<u64> = LaneSlot::new(
            "x",
            LaneState::Ready {
                server: 11,
                applied: tag(3),
                centroid: [1.0; DIM],
            },
        );
        // Fork refused.
        assert!(matches!(
            slot.install_ready(12, EpochTag { epoch: 3, digest: [9; 32] }, [1.0; DIM], false),
            Err(SwapRefusal::Fork { epoch: 3 })
        ));
        // Downgrade refused.
        assert!(matches!(
            slot.install_ready(12, tag(2), [1.0; DIM], false),
            Err(SwapRefusal::Downgrade { applied: 3, offered: 2 })
        ));
        // The old server is untouched by both refusals.
        let guard = slot.state.lock().unwrap();
        match &*guard {
            LaneState::Ready { server, applied, .. } => {
                assert_eq!(*server, 11);
                assert_eq!(*applied, tag(3));
            }
            other => panic!("expected Ready, got {:?}", other.as_str()),
        }
        drop(guard);
        // Force overrides — and logs at the edge (the outcome says so).
        assert_eq!(
            slot.install_ready(12, tag(2), [1.0; DIM], true),
            Ok(InstallOutcome::Forced)
        );
        assert_eq!(slot.applied_epoch(), Some(2));
    }

    #[test]
    fn a_failed_load_never_rolls_the_epoch_back() {
        let slot: LaneSlot<u64> = LaneSlot::new(
            "x",
            LaneState::Ready {
                server: 5,
                applied: tag(4),
                centroid: [1.0; DIM],
            },
        );
        // Release, then a lazy reload of a DOWNGRADED artifact refuses at
        // the install gate — the slot goes Failed at epoch 4, not back to
        // the downgraded tag.
        assert_eq!(slot.release(), Ok(ReleaseOutcome::Released));
        assert!(slot.begin_lazy_load());
        let refused = slot.install_ready(6, tag(1), [1.0; DIM], false);
        assert!(matches!(refused, Err(SwapRefusal::Downgrade { .. })));
        slot.fail("swap gate refused install: downgrade refused".into());
        assert_eq!(slot.applied_epoch(), Some(4), "eviction + failed reload keep the epoch");
    }

    // ── the atomicity hammer (A2: never a torn read, never a blend) ────

    #[test]
    fn concurrent_swap_and_read_observe_whole_snapshots() {
        let slot: Arc<LaneSlot<(u64, u64)>> = Arc::new(LaneSlot::new(
            "hammer",
            LaneState::Ready {
                server: (0, 0),
                applied: tag(0),
                centroid: [1.0; DIM],
            },
        ));
        let stop = Arc::new(AtomicBool::new(false));
        let mut readers = Vec::new();
        for _ in 0..4 {
            let slot = Arc::clone(&slot);
            let stop = Arc::clone(&stop);
            readers.push(std::thread::spawn(move || {
                let mut last = 0u64;
                while !stop.load(Ordering::Relaxed) {
                    let st = slot.state.lock().expect("slot lock");
                    if let LaneState::Ready {
                        server: (epoch, payload),
                        applied,
                        ..
                    } = &*st
                    {
                        // WHOLE: the payload, the tag and the applied
                        // epoch always agree — a torn swap would split
                        // them.
                        assert_eq!(epoch, payload, "torn read: payload must match its epoch");
                        assert_eq!(applied.epoch, *epoch, "torn read: the applied tag rides the same snapshot");
                        assert!(
                            *epoch >= last,
                            "the observed epoch went backwards ({last} → {epoch})"
                        );
                        last = *epoch;
                    }
                }
            }));
        }
        // The swapper: 500 strict advances.
        let slot_w = Arc::clone(&slot);
        let swapper = std::thread::spawn(move || {
            for e in 1..=500u64 {
                slot_w
                    .install_ready((e, e), tag(e), [1.0; DIM], false)
                    .expect("strict advances must pass the gate");
            }
        });
        swapper.join().expect("swapper");
        stop.store(true, Ordering::Relaxed);
        for r in readers {
            r.join().expect("reader");
        }
    }
}

/// The semantic-identity pin (vessel feature): the adapter's decisions
/// match the format crate's REAL `VerifiedVessel::check_monotonic` on the
/// same (version, commitment) pairs — minted public vessels, decoded, and
/// compared arm by arm. This is what makes the adapter an adapter and not
/// a fork.
#[cfg(all(test, feature = "vessel"))]
mod vessel_identity_tests {
    use super::*;
    use ed25519_dalek::{SigningKey, VerifyingKey};

    fn mint(version: u64) -> Vec<u8> {
        let key = SigningKey::from_bytes(&[7u8; 32]);
        reflexer_vessel::encode_public(&key, 1, version, [0u8; 32], b"identity-payload")
    }

    fn pins() -> reflexer_vessel::PinTable {
        let key = SigningKey::from_bytes(&[7u8; 32]);
        let vk_bytes = key.verifying_key().to_bytes();
        let vk = VerifyingKey::from_bytes(&vk_bytes).expect("valid key");
        reflexer_vessel::PinTable::empty().with_wildcard(vk)
    }

    #[test]
    fn the_adapter_matches_check_monotonic_arm_for_arm() {
        let buf = mint(5);
        let vv = reflexer_vessel::decode(&buf, &pins()).expect("minted vessel decodes");
        let digest = vv.commitment();

        // The offered pair is FIXED at the vessel's own identity (the
        // format crate's gate always offers `self`); only the applied
        // state varies. My adapter receives the same pair.
        let cases: [reflexer_vessel::ApplyState; 4] = [
            // strict advance
            reflexer_vessel::ApplyState { artifact_version: 4, commitment: [9; 32] },
            // idempotent re-apply
            reflexer_vessel::ApplyState { artifact_version: 5, commitment: digest },
            // downgrade
            reflexer_vessel::ApplyState { artifact_version: 6, commitment: [7; 32] },
            // fork: same version, different commitment
            reflexer_vessel::ApplyState { artifact_version: 5, commitment: [8; 32] },
        ];
        for applied in cases {
            let mine = check_epoch_tag(
                5,
                digest,
                &EpochTag {
                    epoch: applied.artifact_version,
                    digest: applied.commitment,
                },
            );
            let theirs = vv.check_monotonic(&applied);
            match (mine, theirs) {
                (Ok(a), Ok(())) => {
                    let want = if 5 == applied.artifact_version {
                        EpochApply::Idempotent
                    } else {
                        EpochApply::Advance
                    };
                    assert_eq!(a, want, "applied v{}: verdict drift", applied.artifact_version);
                }
                (Err(SwapRefusal::Downgrade { applied: a, offered: o }), Err(e)) => {
                    assert_eq!(
                        e,
                        reflexer_vessel::ApplyRefusal::OlderThanCurrent {
                            current: a,
                            offered: o
                        },
                        "downgrade refusal drift"
                    );
                }
                (Err(SwapRefusal::Fork { epoch: e }), Err(other)) => {
                    assert_eq!(
                        other,
                        reflexer_vessel::ApplyRefusal::VersionFork { version: e },
                        "fork refusal drift"
                    );
                }
                (mine, theirs) => panic!("verdict shape drift: {mine:?} vs {theirs:?}"),
            }
        }
    }
}
