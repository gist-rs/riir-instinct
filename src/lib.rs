//! riir-instinct — the private model-based/hybrid decision lane
//! ("Reflex · instinct"), the trained sibling of the public modelless
//! engine `../riir-reflex` (riir-ai Proposal 047).
//!
//! P3 T1 skeleton: the sealed specialist artifact reader. riir-train
//! trains (`Issue 576`), this repo consumes BYTES — the artifact is the
//! contract, weights never enter any repo.
//!
//! Boundary contract: `BOUNDARY.md` (deps: katgpt-core sigmoid primitive,
//! riir-reflex tokenizer law — both measured here; riir-infer +
//! reflexer-vessel rows land with their first consumer).

pub mod specialist;
pub mod hybrid;
pub mod stats;
pub mod arsenal;
pub mod arsenal_ops;
pub mod server;
/// The serve lane's receipt primitives (Plan 043 C0 — the
/// one-definition law): the build fingerprint + the input/decision
/// BLAKE3 halves + the manifest fingerprint, ONE home for the serve
/// binary, the decstat submitter, and the decstat verifier.
pub mod receipt;
/// The encoder-feature arm reader (instinct issue 014 C1): the NLEH v1
/// codec + the live laya-english encode replay. RECORD-ONLY — the encoder
/// class is refused at serve (issue 014 decision 1); the ungated half is
/// pure (codec + feature law), the seat runner rides `arena-laya`.
pub mod encoder_arm;
/// The serve-side encoder lane (instinct issue 016 T2): posture ENC — the
/// sealed NLEH head + the laya-english agent resident from boot, answering
/// the suite's canonical presentation through the SAME decide surface the
/// bag lanes serve. GPU-host posture; compiled to nothing at default
/// features (the 014 serve refusal governs every CPU deploy shape).
#[cfg(feature = "serve-encoder")]
pub mod encoder_serve;
/// The frozen-artifact staleness probe (Issue 012 / Plan 005): the soft
/// early-warning readout beside the hard pick-parity gate. Report-only —
/// nothing here serves, swaps, or writes state.
pub mod staleness;
#[cfg(feature = "vessel")]
pub mod vessel;
/// The decstat capture lane (Plan 002 / Issue 004 T1): consent-gated
/// decision-outcome stats over the riir-kat wire — Unset never pushes.
#[cfg(feature = "decstat")]
pub mod decstat;
/// The decstat receipt VERIFIER (Plan 043 Phase C1): boots the seat
/// engine over the frozen pool, resolves lease items against the
/// blake3 input map, re-runs the decide path, compares decision
/// hashes — toolchain/manifest skew refuses, corpus skew is counted.
#[cfg(feature = "decstat")]
pub mod decstat_verify;
/// The Tetris lane (Issue 009 T5+T6): the serving-matched teacher check +
/// the serving-input-contract measurement, over the katgpt-rs tetris
/// substrate (the shared engine; never a fourth one).
#[cfg(feature = "tetris")]
pub mod tetris_lane;
/// The Tetris value-critic arm (Issue 009 T8's modelless second arm + T7's
/// substrate): the Plan-308 KARC basis-ridge readout over the teacher's
/// search-root Q, plus the sample-collection lane loop.
#[cfg(feature = "tetris")]
pub mod tetris_critic;
/// The round-5 blended teacher (Issue 009 T7 round 5): chance_puct over the
/// champion evaluator + the trained critic's afterstate values blended at
/// the search's value seam. weight 0 is the plain teacher, bit-exact.
#[cfg(feature = "tetris")]
pub mod tetris_blend;

pub use hybrid::{
    A0Answer, Cascade, FusedPick, HybridDecision, HybridLane, MAX_TOP_K, NOUL_PAIR, PriorFusion,
    SeatJoin, SpecialistLane, prior_fusion_pick, seat_join,
};
pub use specialist::Specialist;
pub use receipt::{decision_blake3, fingerprint, fp8_from_hex, hash32_from_hex, input_blake3,
    manifest_fingerprint, manifest_fingerprint_hex};
pub use staleness::{PairReport, PairSide, ProbeItem, ProbeSet, SuiteProbe, FIRE_GOLD_DELTA, compare_pair};
pub use arsenal_ops::{
    EpochApply, EpochTag, HoardRefusal, HoardReport, InstallOutcome, LaneSlot, LaneState,
    ReleaseOutcome, ReleaseRefusal, SwapRefusal, check_epoch_tag, corpus_centroid, fold8,
    hoard_check, hoard_gate_armed, hoard_gate_armed_for,
};
pub use stats::{
    ArmStat, PairedDiff, delta_suite, paired_upper_bound, paired_upper_bound_f64, pareto_rank0,
    select_arm, wilson_bound, wilson_lb,
};
#[cfg(feature = "vessel")]
pub use vessel::{AppliedState, LoadedVessel, VesselLoadError, load_hosted, load_hosted_bytes};
#[cfg(feature = "decstat")]
pub use decstat::{DecStatSink, FlushConfig, consent_enabled, install, load_signing_key, primitive_tag, record, spawn_flusher};
