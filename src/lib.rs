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

pub use hybrid::{
    A0Answer, Cascade, FusedPick, HybridDecision, HybridLane, MAX_TOP_K, PriorFusion,
    SpecialistLane, prior_fusion_pick,
};
pub use specialist::Specialist;
pub use stats::{
    ArmStat, PairedDiff, delta_suite, paired_upper_bound, paired_upper_bound_f64, pareto_rank0,
    select_arm, wilson_bound, wilson_lb,
};
