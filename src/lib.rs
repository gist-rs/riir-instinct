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
#[cfg(feature = "vessel")]
pub mod vessel;
/// The decstat capture lane (Plan 002 / Issue 004 T1): consent-gated
/// decision-outcome stats over the riir-kat wire — Unset never pushes.
#[cfg(feature = "decstat")]
pub mod decstat;
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

pub use hybrid::{
    A0Answer, Cascade, FusedPick, HybridDecision, HybridLane, MAX_TOP_K, NOUL_PAIR, PriorFusion,
    SeatJoin, SpecialistLane, prior_fusion_pick, seat_join,
};
pub use specialist::Specialist;
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
