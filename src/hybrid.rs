//! The hybrid composition (riir-instinct Issue 003 T2 / Issue 005): the
//! modelless lane (riir-reflex, the seat posture) and the specialist
//! (the sealed RISP artifact) composed two ways —
//!
//! - **H1 cascade** (riir-ai Proposal 047's default): the modelless lane
//!   answers when its fused abstain gate passes; else the specialist
//!   decides among the top-k modelless survivors (the prune). The µs tier
//!   is kept on every question the modelless lane is confident about, and
//!   the specialist's spend is confined to the abstaining tail.
//! - **H2 prior fusion** (the katgpt-rs Proposal 013 shape, transplanted):
//!   the specialist prior sharpened by the modelless count-table evidence,
//!   `p'_i ∝ p_i · exp(g · β · m_i)` with `g = σ((n − n_min)/τ_n)` — the
//!   evidence gate a safety floor for the thin tail, the margin term the
//!   discriminating work (E0's verdict).
//!
//! Both arms are PURE over their inputs: no allocation, no RNG, ties to
//! the lowest label index (the engine argmax law). The G0 identity
//! postures are first-class values ([`HybridLane::ReflexOnly`]) and pinned
//! by the tests below. G4 (the alloc-free hot path) is
//! `tests/g4_alloc.rs` — a dedicated binary so the counting allocator
//! sees no concurrent test traffic.

use crate::specialist::Specialist;
use katgpt_core::exact_sigmoid;

/// Survivor cap of the H1 prune ([`Cascade::top_k`] is validated against
/// it). Fixed array, never a Vec — the hot path is allocation-free.
pub const MAX_TOP_K: usize = 32;

/// One hybrid decision, in label space. `escalated == false` with the
/// A0 abstain flag still set reproduces A0's OUTPUT exactly — that is the
/// G0 kill-switch contract (the runner emits abstain when
/// `a0.abstained && !escalated`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HybridDecision {
    /// The pick in label space (the seat's label order).
    pub pick: usize,
    /// Whether the cascade escalated to the specialist (H1) — H2 never
    /// escalates (it consults both lanes on every question).
    pub escalated: bool,
}

/// The modelless lane's answer for one question, in label space (the
/// seat's per-question eval output, borrowed).
#[derive(Debug, Clone, Copy)]
pub struct A0Answer<'a> {
    /// Forced probabilities over the label universe (noul is [no, yes]).
    pub probs: &'a [f64],
    /// The forced argmax pick (the seat's own law, ties-lowest).
    pub pick: usize,
    /// The fused abstain gate's verdict (true = the lane abstained).
    pub abstained: bool,
}

/// The H1 prune: keep the modelless lane's top-k options as the
/// specialist's candidate set. `top_k` ≥ the label count degenerates to
/// "the specialist decides freely" — legal, disclosed by the config.
#[derive(Debug, Clone, Copy)]
pub struct Cascade {
    pub top_k: usize,
}

impl Default for Cascade {
    fn default() -> Self {
        Self { top_k: 8 }
    }
}

impl Cascade {
    /// The deterministic top-k prune of `probs`: descending probability,
    /// ties keep the LOWER label index first (an equal later label never
    /// displaces an earlier one). Writes `(label_idx, prob)` into
    /// `survivors[..returned]`, best first. Zero-alloc.
    ///
    /// The running k-th threshold makes the common case ONE compare:
    /// once the window is full, an option at or below the threshold
    /// cannot enter (an EQUAL later label must not displace the earlier
    /// one it ties with — the tie law), so only genuine candidates pay
    /// the insertion walk. Issue 003's G2 remedy for the O(n·k) shape
    /// that breached the 100 ns fusion-only bar on the wide suites.
    /// Output-identical to the naive insertion scan (pinned against a
    /// sort-based reference by `prune_matches_the_sort_reference`).
    pub fn prune<'s>(
        &self,
        probs: &[f64],
        survivors: &'s mut [(usize, f64); MAX_TOP_K],
    ) -> &'s mut [(usize, f64)] {
        assert!(
            self.top_k >= 1 && self.top_k <= MAX_TOP_K,
            "top_k out of range"
        );
        let mut n = 0usize;
        // The enter threshold: −∞ until the window fills, then the k-th
        // survivor's prob. `!(p > thr)` is the ONE NaN-safe skip — a NaN
        // prob can never enter (it would poison the ordering and, in the
        // full window, walk past the array).
        let mut thr = f64::NEG_INFINITY;
        for (i, &p) in probs.iter().enumerate() {
            // NaN-safe skip: `!(p > thr)` (never `p <= thr`) so a NaN prob
            // fails the enter test and can neither poison the ordering
            // nor walk past the array. The negated form is the point —
            // the lint is allowed for exactly this.
            #[allow(clippy::neg_cmp_op_on_partial_ord)]
            if !(p > thr) {
                continue;
            }
            // Insertion position: first slot strictly worse than (i, p) —
            // an equal element stops the walk, so an equal LATER label
            // lands after it (ties keep the lower index first).
            let mut pos = n;
            while pos > 0 && survivors[pos - 1].1 < p {
                pos -= 1;
            }
            debug_assert!(pos < self.top_k);
            // Shift the tail right; when full the LAST element (the
            // smallest prob, highest index among equals) falls off.
            let last = n.min(self.top_k - 1);
            let mut s = last;
            while s > pos {
                survivors[s] = survivors[s - 1];
                s -= 1;
            }
            survivors[pos] = (i, p);
            if n < self.top_k {
                n += 1;
            }
            thr = if n == self.top_k {
                survivors[n - 1].1
            } else {
                f64::NEG_INFINITY
            };
        }
        &mut survivors[..n]
    }
}

/// The noul bridge's unified pair (Plan 003 / riir-train Issue 578): the
/// producer's label spelling for a noul question's two positions — gold
/// idx 0 (the `[false, true]` rendering's false side) is `"no"`, gold idx
/// 1 (true) is `"yes"` — shared verbatim by both 578 artifacts
/// (`prompt_injections` and `typed_decisions`, whose class universes
/// carry the pair as ordinary labels).
pub const NOUL_PAIR: [&str; 2] = ["no", "yes"];

/// The seat↔artifact join form (Plan 003's bridge — which of the three
/// seat label shapes a suite presents, decided from the two label sets
/// alone).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeatJoin {
    /// Join by the returned names — the seat labels verbatim (the six
    /// legacy suites, typed_decisions excluded) or the unified noul pair
    /// in place of the positional int spelling (prompt_injections; the
    /// harness's noul rendering is the fixed `[false, true]`, gold idx p
    /// speaks it, and the train side's dataset map label p →
    /// [`NOUL_PAIR`][p] is the same correspondence).
    Named(Vec<String>),
    /// The seat's labels are CONTEXT (typed_decisions' workflow names —
    /// the modelless engine's domain space): NO seat label resolves to a
    /// class row, `perm` carries sentinels, and every specialist answer
    /// flows through the per-question presented-option bridge
    /// (`fill_positions`' by-name rule) — never through `perm`. The
    /// identity-by-count resolve is DISABLED for these suites (the count
    /// of presented keys against the label count is meaningless when the
    /// two spaces are unrelated); an unmatched presented key refuses loud
    /// (the train side's distractor-universe law covers every
    /// train-presented key, so a miss is template drift).
    Context,
}

/// Decide the join form for a seat (Plan 003). Name-joinable seats and
/// the positional noul pair yield [`SeatJoin::Named`]; a seat whose labels
/// are ALL absent from the artifact yields [`SeatJoin::Context`]. Two
/// drift shapes deliberately fall through to [`SeatJoin::Named`] with the
/// seat labels unchanged, so [`SpecialistLane::join`] is the single
/// refusal site: a PARTIAL overlap (some seat labels resolve, some don't
/// — the join's "seat label not in the artifact" error carries the
/// context), and the positional int spelling whose artifact lacks the
/// unified pair (a producer-contract break — never read as context).
#[must_use]
pub fn seat_join(seat_labels: &[String], artifact_labels: &[String]) -> SeatJoin {
    let int_spelling = seat_labels
        .iter()
        .enumerate()
        .all(|(i, l)| l == &i.to_string());
    if seat_labels.iter().all(|l| artifact_labels.contains(l)) {
        return SeatJoin::Named(seat_labels.to_vec());
    }
    let positional = seat_labels.len() == NOUL_PAIR.len()
        && int_spelling
        && NOUL_PAIR
            .iter()
            .all(|n| artifact_labels.iter().any(|a| a == n));
    if positional {
        return SeatJoin::Named(NOUL_PAIR.iter().map(|s| (*s).to_string()).collect());
    }
    if !int_spelling && seat_labels.iter().all(|l| !artifact_labels.contains(l)) {
        return SeatJoin::Context;
    }
    SeatJoin::Named(seat_labels.to_vec())
}

/// A loaded specialist joined onto a seat's label order. `perm[label]` is
/// the specialist's class row for that label — a bijection asserted at
/// construction (the join pin; a silent permutation here would move every
/// class, the riir-train 576 writer-defect class) — or the
/// [`usize::MAX`] sentinel for a CONTEXT-joined seat
/// ([`SeatJoin::Context`]: the labels are the modelless engine's domain
/// space, not the specialist's answer space; nothing in the arm paths
/// reads `perm` for such a seat, and [`Self::scores_label_into`] maps the
/// sentinel to NaN where only timing reads it).
#[derive(Debug, Clone)]
pub struct SpecialistLane {
    pub spec: Specialist,
    pub perm: Vec<usize>,
    pub cascade: Cascade,
}

impl SpecialistLane {
    /// Join an artifact onto the seat's labels: every seat label must
    /// appear EXACTLY ONCE in the artifact's universe (the map is
    /// injective, asserted). The artifact may carry MORE labels than the
    /// seat offers (riir-train trains over the full intent set; the
    /// harness's test option union can be one narrower — measured,
    /// massive_intent_en 60 vs 59) — the extra classes are never
    /// survivors, because the arms only ever pick among the seat's
    /// offered options. `suite` is asserted too — loading the banking77
    /// winner into the emotion seat must be loud, never a permute.
    pub fn join(
        spec: Specialist,
        suite: &str,
        seat_labels: &[String],
        cascade: Cascade,
    ) -> Result<Self, String> {
        if spec.suite != suite {
            return Err(format!(
                "join: artifact suite {:?} != seat suite {suite:?}",
                spec.suite
            ));
        }
        if spec.labels.len() < seat_labels.len() {
            return Err(format!(
                "join: artifact has {} labels, seat needs {} — a seat option would \n                 be unscoreable",
                spec.labels.len(),
                seat_labels.len()
            ));
        }
        let mut perm = vec![usize::MAX; seat_labels.len()];
        let mut used = vec![false; spec.labels.len()];
        for (li, seat_label) in seat_labels.iter().enumerate() {
            let ci = spec
                .labels
                .iter()
                .position(|l| l == seat_label)
                .ok_or_else(|| format!("join: seat label {seat_label:?} not in the artifact"))?;
            if used[ci] {
                return Err(format!("join: artifact label {seat_label:?} matched twice"));
            }
            used[ci] = true;
            perm[li] = ci;
        }
        if perm.contains(&usize::MAX) {
            return Err("join: incomplete permutation".into());
        }
        let ignored = used.iter().filter(|&&u| !u).count();
        if ignored > 0 {
            eprintln!(
                "  [join] {suite}: artifact carries {ignored} label(s) the seat never \
                 offers — unmapped, never survivors"
            );
        }
        Ok(Self {
            spec,
            perm,
            cascade,
        })
    }

    /// The CONTEXT join ([`SeatJoin::Context`], Plan 003): the seat's
    /// labels are the modelless engine's domain space (typed_decisions'
    /// workflow names), never the specialist's answer space — `perm[li]`
    /// is the label's class row where one exists and the
    /// [`usize::MAX`] sentinel where none does (asserted: the seat must
    /// be FULLY disjoint from the artifact — a partial overlap is drift,
    /// and [`Self::join`]'s refusal is the loud answer). Every specialist
    /// answer flows through the per-question presented-option bridge; the
    /// seat-label pin deliberately does not apply.
    pub fn join_context(
        spec: Specialist,
        suite: &str,
        seat_labels: &[String],
        cascade: Cascade,
    ) -> Result<Self, String> {
        if spec.suite != suite {
            return Err(format!(
                "join_context: artifact suite {:?} != seat suite {suite:?}",
                spec.suite
            ));
        }
        if seat_labels
            .iter()
            .any(|l| spec.labels.iter().any(|a| a == l))
        {
            return Err(format!(
                "join_context: seat label(s) of {suite} resolve to artifact class rows — \
                 a partial overlap is drift, not context; use the name join"
            ));
        }
        eprintln!(
            "  [join] {suite}: context seat — {} label(s) carry sentinel class rows; \
             every answer resolves through the presented-option bridge",
            seat_labels.len()
        );
        Ok(Self {
            spec,
            perm: vec![usize::MAX; seat_labels.len()],
            cascade,
        })
    }

    /// The specialist's class scores for LABEL-order indices into `out`
    /// (len must equal the label count). A context-joined sentinel row
    /// ([`SeatJoin::Context`]) scores NaN — no class exists; the callers
    /// are timing paths (the fusion micro) and label-space readouts, and
    /// NaN's comparisons-as-false make the argmax law degrade to the
    /// first position rather than read a phantom class. Zero-alloc.
    pub fn scores_label_into(&self, bag: &[(u32, f32)], out: &mut [f32]) {
        assert_eq!(out.len(), self.perm.len(), "scores out of shape");
        for (o, &li) in out.iter_mut().zip(self.perm.iter()) {
            *o = if li == usize::MAX {
                f32::NAN
            } else {
                self.spec.score_class(bag, li)
            };
        }
    }

    /// Score ARBITRARY class rows into `out` (one class row per entry).
    /// The per-case presented-position path: the caller resolves each
    /// presented option to its artifact class row (the Issue-006 bridge —
    /// the seat's pick space is the question's presented options, not the
    /// label universe). Zero-alloc.
    pub fn scores_classes_into(&self, bag: &[(u32, f32)], class_rows: &[usize], out: &mut [f32]) {
        assert_eq!(out.len(), class_rows.len(), "scores out of shape");
        for (o, &c) in out.iter_mut().zip(class_rows.iter()) {
            *o = self.spec.score_class(bag, c);
        }
    }

    /// A1 — the specialist alone: argmax score over the full label
    /// universe, ties to the lowest LABEL index. Returns (pick, winning
    /// score). Zero-alloc (`scores` is the caller's scratch, len = label
    /// count).
    pub fn pick_alone(&self, bag: &[(u32, f32)], scores: &mut [f32]) -> (usize, f32) {
        self.scores_label_into(bag, scores);
        let mut best = 0usize;
        for (i, &s) in scores.iter().enumerate().skip(1) {
            if s > scores[best] {
                best = i;
            }
        }
        (best, scores[best])
    }

    /// H1 — the cascade decision over one question. The modelless answer
    /// passes through when its gate did not abstain; an abstaining
    /// question is pruned to the top-k modelless survivors and the
    /// specialist picks among them (ties among survivors → the lowest
    /// POSITION, the global argmax law). `class_of_pos` maps each of A0's
    /// option positions to the artifact class row that position denotes —
    /// the PER-CASE bridge between the seat's presented-option space
    /// (what A0's probs and the gold index speak) and the specialist's
    /// label space; indexing `perm` by position instead is the Issue-006
    /// instrument defect. The pick is a POSITION, directly comparable
    /// with gold. Zero-alloc.
    pub fn h1_decide(
        &self,
        a0: A0Answer<'_>,
        bag: &[(u32, f32)],
        class_of_pos: &[usize],
        survivors: &mut [(usize, f64); MAX_TOP_K],
        scores: &mut [f32; MAX_TOP_K],
    ) -> HybridDecision {
        if !a0.abstained {
            return HybridDecision {
                pick: a0.pick,
                escalated: false,
            };
        }
        debug_assert_eq!(class_of_pos.len(), a0.probs.len(), "class map out of shape");
        let kept = self.cascade.prune(a0.probs, survivors);
        let n = kept.len();
        for (i, &(pos, _)) in kept.iter().enumerate() {
            scores[i] = self.spec.score_class(bag, class_of_pos[pos]);
        }
        let mut best = 0usize;
        for i in 1..n {
            if scores[i] > scores[best] {
                best = i;
            }
        }
        HybridDecision {
            pick: kept[best].0,
            escalated: true,
        }
    }
}

/// The hybrid lane: the G0 kill switch is a first-class arm
/// ([`HybridLane::ReflexOnly`]), never a missing-file fallback.
#[derive(Debug, Clone)]
pub enum HybridLane {
    /// G0a — no specialist / hybrid disabled: the output is A0
    /// byte-identical (no escalation ever fires).
    ReflexOnly,
    /// The loaded hybrid (H1; H2 is a pure function over the same lane's
    /// scores — [`prior_fusion_pick`]).
    Specialist(SpecialistLane),
}

impl HybridLane {
    /// H1 decision. `ReflexOnly` never escalates — the kill switch.
    /// `class_of_pos` is the per-case position → class-row bridge (see
    /// [`SpecialistLane::h1_decide`]).
    pub fn h1_decide(
        &self,
        a0: A0Answer<'_>,
        bag: &[(u32, f32)],
        class_of_pos: &[usize],
        survivors: &mut [(usize, f64); MAX_TOP_K],
        scores: &mut [f32; MAX_TOP_K],
    ) -> HybridDecision {
        match self {
            HybridLane::ReflexOnly => HybridDecision {
                pick: a0.pick,
                escalated: false,
            },
            HybridLane::Specialist(lane) => {
                lane.h1_decide(a0, bag, class_of_pos, survivors, scores)
            }
        }
    }

    /// Score ARBITRARY class rows (see
    /// [`SpecialistLane::scores_classes_into`]).
    pub fn scores_classes_into(&self, bag: &[(u32, f32)], class_rows: &[usize], out: &mut [f32]) {
        match self {
            HybridLane::ReflexOnly => unreachable!("specialist scores without a specialist"),
            HybridLane::Specialist(lane) => lane.scores_classes_into(bag, class_rows, out),
        }
    }

    /// The specialist's per-label scores for one question's bag (H2's
    /// prior; A1's readout). `ReflexOnly` has no prior — the caller never
    /// asks (H2/A1 are specialist arms).
    pub fn scores_label_into(&self, bag: &[(u32, f32)], out: &mut [f32]) {
        match self {
            HybridLane::ReflexOnly => unreachable!("specialist scores without a specialist"),
            HybridLane::Specialist(lane) => lane.scores_label_into(bag, out),
        }
    }

    /// A1 — specialist alone (see [`SpecialistLane::pick_alone`]).
    pub fn pick_alone(&self, bag: &[(u32, f32)], scores: &mut [f32]) -> (usize, f32) {
        match self {
            HybridLane::ReflexOnly => unreachable!("specialist pick without a specialist"),
            HybridLane::Specialist(lane) => lane.pick_alone(bag, scores),
        }
    }
}

/// H2 — prior fusion (the Proposal 013 shape): `p'_i ∝ p_i ·
/// exp(g · β · m_i)` with `g = σ((n − n_min)/τ_n)` the evidence gate over
/// the count tables' seen-event count and `m_i` the PER-TOKEN in-scope
/// margin of option i (`in_scores[i] − best_other`, divided by the token
/// count). Argmax of the unnormalized product — the renormalizing
/// constant is per-question and never moves the pick. Ties → the lowest
/// label index. Zero-alloc, O(n) in options.
///
/// Identity properties (pinned by tests): `g ≡ 0` ⇒ the fused ordering IS
/// the specialist ordering (A1); with no count tables armed (`in_scores`
/// empty) the margin term is 0 ⇒ the same A1 collapse.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PriorFusion {
    pub beta: f32,
    pub n_min: f32,
    pub tau_n: f32,
}

impl Default for PriorFusion {
    fn default() -> Self {
        Self {
            beta: 1.0,
            n_min: 4.0,
            tau_n: 4.0,
        }
    }
}

/// The fused readout: (pick, confidence) — the winning product
/// renormalized over the options (a normalization of the fused PRODUCT,
/// never a softmax over logits).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FusedPick {
    pub pick: usize,
    pub conf: f64,
}

pub fn prior_fusion_pick(
    fusion: &PriorFusion,
    spec_scores: &[f32],
    nb_in_scores: &[f32],
    n_seen: usize,
    n_tokens: usize,
) -> FusedPick {
    let g = exact_sigmoid((n_seen as f32 - fusion.n_min) / fusion.tau_n);
    let inv = 1.0 / n_tokens.max(1) as f32;
    // The per-token margin of i against the best OTHER option: with the
    // global max + second max, margin_i = in[i] − (i == argmax ? second
    // : max). One pass, no per-option rescans.
    //
    // NaN = NO count-table evidence for that option (the bridge's
    // artifact-known, seat-unknown label — a cal-front option whose class
    // the specialist trained but the seat's tables never carried): its
    // margin term is muted to 0 (the prior stands) and it is never a
    // rival in this scan. The strict `>` comparisons already skip it;
    // the anchor is a MAX index, never NaN.
    let mut best = usize::MAX;
    let mut second: Option<usize> = None;
    for (i, &s) in nb_in_scores.iter().enumerate() {
        if s.is_nan() {
            continue;
        }
        if best == usize::MAX || s > nb_in_scores[best] {
            if best != usize::MAX {
                second = Some(best);
            }
            best = i;
        } else if second.is_none_or(|j| s > nb_in_scores[j]) {
            second = Some(i);
        }
    }
    let mut total = 0.0f64;
    let mut best_i = 0usize;
    let mut best_v = f64::NEG_INFINITY;
    for (i, &p) in spec_scores.iter().enumerate() {
        let margin = if nb_in_scores.is_empty() {
            0.0
        } else {
            let own = nb_in_scores[i];
            if own.is_nan() {
                0.0
            } else {
                let rival = if i == best {
                    second.map_or(f32::NEG_INFINITY, |j| nb_in_scores[j])
                } else if best != usize::MAX {
                    nb_in_scores[best]
                } else {
                    f32::NEG_INFINITY
                };
                let m = if rival.is_finite() {
                    own - rival
                } else {
                    own
                };
                m * inv
            }
        };
        let v = f64::from(p) * f64::from((g * fusion.beta * margin).exp());
        total += v;
        if v > best_v {
            best_v = v;
            best_i = i;
        }
    }
    FusedPick {
        pick: best_i,
        conf: if total > 0.0 { best_v / total } else { 0.0 },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::specialist::{ARTIFACT_MAGIC, ARTIFACT_VERSION, decode_artifact, VOCAB};

    /// Hand-build one RISP v1 artifact (the riir-train encoder's bytes;
    /// the format is the contract, the producer is over there).
    fn encode(suite: &str, labels: &[&str], b: &[f32], rows: &[Vec<f32>]) -> Vec<u8> {
        assert_eq!(b.len(), labels.len());
        let l = labels.len();
        let mut payload = Vec::new();
        payload.extend_from_slice(suite.as_bytes());
        payload.push(0);
        payload.extend_from_slice(&(l as u32).to_le_bytes());
        for label in labels {
            payload.extend_from_slice(label.as_bytes());
            payload.push(0);
        }
        payload.extend_from_slice(&(VOCAB as u32).to_le_bytes());
        payload.extend_from_slice(&b.iter().flat_map(|f| f.to_le_bytes()).collect::<Vec<u8>>());
        for row in rows {
            let max_abs = row.iter().fold(0.0f32, |m, &x| m.max(x.abs()));
            let scale = if max_abs == 0.0 { 1.0 } else { max_abs / 127.0 };
            payload.extend_from_slice(&scale.to_le_bytes());
            for &x in row {
                payload.push(((x / scale).round().clamp(-127.0, 127.0)) as i8 as u8);
            }
        }
        let mut out = Vec::with_capacity(payload.len() + 5 + 32);
        out.extend_from_slice(&ARTIFACT_MAGIC);
        out.push(ARTIFACT_VERSION);
        out.extend_from_slice(&payload);
        out.extend_from_slice(blake3::hash(&payload).as_bytes());
        out
    }

    /// Seat order ["s0","s1"]; artifact order ["s1","s0"] — class 0
    /// ("s1") strong on "beta", class 1 ("s0") strong on "alpha". The
    /// join must permute by name: "beta" → s1, "alpha" → s0 — a silent
    /// order-copy picks exactly the reverse.
    fn toy_lane_with(top_k: usize) -> SpecialistLane {
        let mut probe = Vec::new();
        let mut scratch = Vec::new();
        crate::specialist::bag_into(b"alpha", &mut probe, &mut scratch);
        let h_alpha = probe[0].0 as usize;
        crate::specialist::bag_into(b"beta", &mut probe, &mut scratch);
        let h_beta = probe[0].0 as usize;
        let mut w_s1 = vec![0.0f32; VOCAB];
        w_s1[h_beta] = 4.0;
        let mut w_s0 = vec![0.0f32; VOCAB];
        w_s0[h_alpha] = 4.0;
        let bytes = encode("toy", &["s1", "s0"], &[0.0, 0.0], &[w_s1, w_s0]);
        let spec = decode_artifact(&bytes).expect("decode");
        SpecialistLane::join(
            spec,
            "toy",
            &["s0".to_string(), "s1".to_string()],
            Cascade { top_k },
        )
        .expect("join")
    }

    fn toy_lane() -> SpecialistLane {
        toy_lane_with(2)
    }

    fn bag_of(text: &str) -> Vec<(u32, f32)> {
        let mut bag = Vec::new();
        let mut scratch = Vec::new();
        crate::specialist::bag_into(text.as_bytes(), &mut bag, &mut scratch);
        bag
    }

    /// G0a — the kill switch: ReflexOnly reproduces A0 exactly, INCLUDING
    /// on abstaining questions (no escalation, A0's own pick).
    #[test]
    fn kill_switch_is_byte_identical_to_a0() {
        let lane = HybridLane::ReflexOnly;
        let mut survivors = [(0usize, 0.0f64); MAX_TOP_K];
        let mut scores = [0.0f32; MAX_TOP_K];
        let a0 = A0Answer {
            probs: &[0.3, 0.7],
            pick: 1,
            abstained: true,
        };
        let d = lane.h1_decide(a0, &[], &[0, 1], &mut survivors, &mut scores);
        assert_eq!(d, HybridDecision { pick: 1, escalated: false });
        let a0 = A0Answer {
            probs: &[0.9, 0.1],
            pick: 0,
            abstained: false,
        };
        let d = lane.h1_decide(a0, &[], &[0, 1], &mut survivors, &mut scores);
        assert_eq!(d, HybridDecision { pick: 0, escalated: false });
    }

    /// The join is a name permutation, and the scores follow it:
    /// "beta" must pick s1 (the artifact's class 0), "alpha" must pick
    /// s0 (class 1) — a silent order-copy picks exactly the reverse.
    #[test]
    fn join_permutes_by_name_and_scores_follow() {
        let lane = toy_lane();
        let mut scores = [0.0f32; 2];
        let (pick, conf) = lane.pick_alone(&bag_of("beta"), &mut scores);
        assert_eq!(pick, 1, "beta → s1 through the join");
        assert!(conf > 0.5);
        let (pick, _) = lane.pick_alone(&bag_of("alpha"), &mut scores);
        assert_eq!(pick, 0, "alpha → s0 through the join");
    }

    /// The join refuses the wrong suite, the wrong width, and a missing
    /// seat label — loud, never a permute.
    #[test]
    fn join_refuses_drift() {
        let spec = decode_artifact(&encode(
            "other",
            &["zeta", "eta"],
            &[0.0, 0.0],
            &[vec![0.0; VOCAB], vec![0.0; VOCAB]],
        ))
        .unwrap();
        assert!(SpecialistLane::join(spec, "toy", &["s0".into(), "s1".into()], Cascade::default())
            .is_err());
        let spec = decode_artifact(&encode(
            "toy",
            &["zeta", "eta", "extra"],
            &[0.0, 0.0, 0.0],
            &[vec![0.0; VOCAB], vec![0.0; VOCAB], vec![0.0; VOCAB]],
        ))
        .unwrap();
        assert!(SpecialistLane::join(spec, "toy", &["s0".into(), "s1".into()], Cascade::default())
            .is_err());
        let spec = decode_artifact(&encode(
            "toy",
            &["zeta"],
            &[0.0],
            &[vec![0.0; VOCAB]],
        ))
        .unwrap();
        assert!(SpecialistLane::join(spec, "toy", &["s0".into(), "s1".into()], Cascade::default())
            .is_err());
    }

    /// The seat join forms (Plan 003): named passthrough; the positional
    /// int spelling over the unified pair; the context seat (fully
    /// disjoint labels); and the two drift shapes passing through
    /// unchanged so the join stays the single refusal site.
    #[test]
    fn seat_join_named_positional_context_and_refusal() {
        // Named passthrough: labels all present in the artifact.
        let seat = vec!["wf_a".to_string(), "wf_b".to_string()];
        let artifact = vec!["no".to_string(), "wf_a".to_string(), "yes".to_string(), "wf_b".to_string()];
        assert_eq!(seat_join(&seat, &artifact), SeatJoin::Named(seat.clone()));
        // Positional: prompt_injections' int spelling over the pair.
        let seat = vec!["0".to_string(), "1".to_string()];
        let artifact = vec!["no".to_string(), "yes".to_string()];
        assert_eq!(
            seat_join(&seat, &artifact),
            SeatJoin::Named(vec!["no".to_string(), "yes".to_string()])
        );
        // Context: typed_decisions' workflow seat — fully disjoint from
        // the option-key artifact.
        let seat = vec!["wf_a".to_string(), "wf_b".to_string()];
        let artifact = vec!["no".to_string(), "execute_refund".to_string(), "yes".to_string()];
        assert_eq!(seat_join(&seat, &artifact), SeatJoin::Context);
        // Partial overlap → unchanged (join refuses with drift context).
        let seat = vec!["wf_a".to_string(), "no".to_string()];
        let artifact = vec!["no".to_string(), "yes".to_string()];
        assert_eq!(seat_join(&seat, &artifact), SeatJoin::Named(seat));
        // Int spelling but NO pair in the artifact — a producer-contract
        // break, never context → unchanged (join refuses).
        let seat = vec!["0".to_string(), "1".to_string()];
        let artifact = vec!["a".to_string(), "b".to_string()];
        assert_eq!(seat_join(&seat, &artifact), SeatJoin::Named(seat));
    }

    /// H1: an abstaining question escalates; with top_k = 1 the survivor
    /// set is exactly A0's argmax, so the specialist can only confirm it.
    #[test]
    fn h1_topk_one_is_confined_to_the_a0_argmax() {
        let lane = HybridLane::Specialist(toy_lane_with(1));
        let bag = bag_of("alpha");
        let a0 = A0Answer {
            probs: &[0.25, 0.75],
            pick: 1,
            abstained: true,
        };
        let mut survivors = [(0usize, 0.0f64); MAX_TOP_K];
        let mut scores = [0.0f32; MAX_TOP_K];
        let d = lane.h1_decide(a0, &bag, &[0, 1], &mut survivors, &mut scores);
        assert!(d.escalated);
        assert_eq!(d.pick, 1, "top-1 prune confines the specialist to A0's argmax");
    }

    /// The prune: descending order, ties keep the lower index, the cap
    /// holds, and a wider cap than the option count keeps them all.
    #[test]
    fn prune_orders_ties_and_caps() {
        let cas = Cascade { top_k: 3 };
        let mut survivors = [(0usize, 0.0f64); MAX_TOP_K];
        let kept = cas.prune(&[0.1, 0.5, 0.5, 0.9, 0.2], &mut survivors);
        let idx: Vec<usize> = kept.iter().map(|&(i, _)| i).collect();
        assert_eq!(idx, vec![3, 1, 2], "desc prob; the 0.5 tie keeps 1 before 2");
        let cas = Cascade { top_k: 8 };
        let kept = cas.prune(&[0.1, 0.2], &mut survivors);
        assert_eq!(kept.len(), 2);
        assert_eq!(kept[0].0, 1);
    }

    /// The threshold path: an equal LATER option at the k-th threshold
    /// must land AFTER the earlier one it ties with, and the displaced
    /// element is the smallest prob (highest index among equals).
    #[test]
    fn prune_threshold_tie_keeps_the_earlier_index_first() {
        let cas = Cascade { top_k: 3 };
        let mut survivors = [(0usize, 0.0f64); MAX_TOP_K];
        let kept = cas.prune(&[0.9, 0.5, 0.3, 0.5], &mut survivors);
        let idx: Vec<usize> = kept.iter().map(|&(i, _)| i).collect();
        assert_eq!(
            idx,
            vec![0, 1, 3],
            "the later 0.5 (idx 3) enters above the threshold but lands AFTER the earlier 0.5 (idx 1) — the tie law; idx 2 (0.3) falls off"
        );
    }

    /// Output-identical to a naive sort-based reference over a
    /// deterministic pseudo-random spread (the threshold fast path must
    /// never reorder or mistie). `top_k` at both extremes included.
    #[test]
    fn prune_matches_the_sort_reference() {
        for top_k in [1usize, 3, 8, MAX_TOP_K] {
            for n in [1usize, 7, 59, 400] {
                let probs: Vec<f64> = (0..n)
                    .map(|i| ((i as f64 * 0.618_033_988_7).sin() * 1e4).fract().abs())
                    .collect();
                let mut ranked: Vec<(usize, f64)> =
                    probs.iter().copied().enumerate().collect();
                ranked
                    .sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
                let want: Vec<usize> = ranked[..top_k.min(n)].iter().map(|&(i, _)| i).collect();

                let cas = Cascade { top_k };
                let mut survivors = [(0usize, 0.0f64); MAX_TOP_K];
                let kept = cas.prune(&probs, &mut survivors);
                let got: Vec<usize> = kept.iter().map(|&(i, _)| i).collect();
                assert_eq!(got, want, "top_k {top_k} · n {n}");
            }
        }
    }

    /// H2's g ≡ 0 identity: with the evidence gate hard-zeroed (n_min far
    /// above any n, tiny τ) the fused pick IS the A1 pick — never a
    /// reflex read — even against a dominant nb margin.
    #[test]
    fn h2_zero_gate_collapses_to_a1() {
        let lane = toy_lane();
        let bag = bag_of("alpha");
        let mut spec_scores = [0.0f32; 2];
        lane.scores_label_into(&bag, &mut spec_scores);
        let (a1, _) = lane.pick_alone(&bag, &mut spec_scores);
        let fusion = PriorFusion {
            beta: 3.0,
            n_min: 1.0e9,
            tau_n: 1.0e-3,
        };
        let fused = prior_fusion_pick(&fusion, &spec_scores, &[-9.0, 9.0], 0, 5);
        assert_eq!(fused.pick, a1, "g ≡ 0 ⇒ the fused ordering is the A1 ordering, even against a dominant nb margin for the other option");
    }

    /// H1 scores the class each POSITION denotes (the per-case bridge),
    /// never perm[position] — the Issue-006 instrument law, pinned on a
    /// discriminating shape. The toy's artifact order is ["s1","s0"] so
    /// perm = [1, 0]; a case presenting (s1, s0) in THAT order has
    /// class_of_pos = [0, 1] (position 0 is the artifact's class 0). With
    /// bag "beta" (class 0 strong) the pick must be position 0 — the
    /// perm-indexed defect scores perm[0] = class 1 at position 0 and
    /// picks exactly the reverse.
    #[test]
    fn h1_scores_the_class_the_position_denotes() {
        let lane = HybridLane::Specialist(toy_lane_with(2));
        let bag = bag_of("beta");
        let a0 = A0Answer {
            probs: &[0.5, 0.5],
            pick: 0,
            abstained: true,
        };
        let mut survivors = [(0usize, 0.0f64); MAX_TOP_K];
        let mut scores = [0.0f32; MAX_TOP_K];
        let d = lane.h1_decide(a0, &bag, &[0, 1], &mut survivors, &mut scores);
        assert_eq!(
            d.pick, 0,
            "position 0 = class 0 = the beta-strong class; the perm-indexed bug picks 1"
        );
    }

    /// H2's margin term actually discriminates: the prior favors s0 on
    /// "alpha"; a dominant s1 nb margin flips the fused pick to s1 with
    /// the gate open — while the same margin at thin evidence (the gate
    /// nearly closed), β = 0, and an empty evidence table all stay at the
    /// A1 pick.
    #[test]
    fn h2_margin_term_moves_the_pick() {
        let lane = toy_lane();
        let bag = bag_of("alpha");
        let mut spec_scores = [0.0f32; 2];
        lane.scores_label_into(&bag, &mut spec_scores);
        assert!(spec_scores[0] > spec_scores[1], "the toy prior favors s0 on alpha");
        let fusion = PriorFusion {
            beta: 0.5,
            n_min: 4.0,
            tau_n: 4.0,
        };
        let s1_dominant = [-6.0f32, 6.0];
        let open = prior_fusion_pick(&fusion, &spec_scores, &s1_dominant, 99, 5);
        assert_eq!(open.pick, 1, "a dominant s1 margin, gate open, flips to s1");
        let thin = prior_fusion_pick(&fusion, &spec_scores, &s1_dominant, 0, 5);
        assert_eq!(thin.pick, 0, "the same margin at n < n_min stays at the prior");
        let beta0 = prior_fusion_pick(
            &PriorFusion { beta: 0.0, ..fusion },
            &spec_scores,
            &s1_dominant,
            99,
            5,
        );
        assert_eq!(beta0.pick, 0, "β = 0 mutes the margin term entirely");
        let no_table = prior_fusion_pick(&fusion, &spec_scores, &[], 99, 5);
        assert_eq!(no_table.pick, 0, "no count tables ⇒ no margin ⇒ A1");
    }

    /// NaN in the NB in-scores = NO evidence for that option (the
    /// artifact-known, seat-unknown label the aligned 052 protocol
    /// surfaced): its own margin term mutes to 0 — the option still
    /// competes via its prior, its weight is exactly the no-table weight
    /// — and it is NEVER a rival (a huge real score flips the pick; a
    /// NaN in the same slot must not).
    #[test]
    fn h2_nan_evidence_mutes_the_margin_and_is_never_a_rival() {
        let fusion = PriorFusion {
            beta: 1.0,
            n_min: 0.0,
            tau_n: 1.0,
        };
        let spec = [0.5f32, 0.3, 0.2];
        // (a) the masked option's weight is the NO-TABLE weight: with
        // known margins 9/−4 for options 0/2 and 0 for the masked 1, the
        // normalization is the closed form — the pick stands and the
        // confidence denominator includes the prior-weighted masked term.
        let f = prior_fusion_pick(&fusion, &spec, &[9.0, f32::NAN, 5.0], 99, 1);
        let v0 = 0.5f64 * 4.0f64.exp();
        let v1 = 0.3f64;
        let v2 = 0.2f64 * (-4.0f64).exp();
        assert_eq!(f.pick, 0);
        assert!(f.pick == 0 && (f.conf - v0 / (v0 + v1 + v2)).abs() < 1e-6,
            "the masked option's margin is exactly 0 and its prior weight counts in the denominator");
        // (b) never a rival: a real 1000 in slot 1 flips the pick to 1
        // (m0 = 9−1000); the NaN in the same slot must not.
        let rival = prior_fusion_pick(&fusion, &spec, &[9.0, 1000.0, 5.0], 99, 1);
        assert_eq!(rival.pick, 1, "sanity: a real dominant rival does flip");
        let masked = prior_fusion_pick(&fusion, &spec, &[9.0, f32::NAN, 5.0], 99, 1);
        assert_eq!(masked.pick, 0, "a NaN slot is never a rival — the pick stands");
        // (c) NaN never anchors the max: an all-NaN vector is the
        // no-evidence case (A1 stands, finite confidence).
        let all_nan = prior_fusion_pick(&fusion, &spec, &[f32::NAN, f32::NAN, f32::NAN], 99, 1);
        assert_eq!(all_nan.pick, 0, "all-NaN ⇒ no margins ⇒ A1");
        assert!(all_nan.conf.is_finite(), "no NaN may leak into the confidence");
    }

    /// H2's readout confidence is a proper [0,1] normalization of the
    /// fused product, highest for a dominant option.
    #[test]
    fn h2_confidence_is_normalized() {
        let fusion = PriorFusion::default();
        let one_hot = prior_fusion_pick(&fusion, &[0.99, 0.01, 0.01], &[], 9, 5);
        let uniform = prior_fusion_pick(&fusion, &[0.5, 0.5], &[], 9, 5);
        assert!(one_hot.conf > 0.9);
        assert!((uniform.conf - 0.5).abs() < 1e-6);
    }
}
