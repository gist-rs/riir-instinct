//! riir-instinct — the open trained-specialists decision lane
//! ("Reflex · instinct"), the trained sibling of the public modelless
//! engine `../riir-reflex` (riir-ai Proposal 047). The moat (the encoder
//! arm, the HOSTED-ONLY vessel reader, the economy client) lives in the
//! private Rethink lane — `moat/` here is the Rethink SEED, and the lib
//! carries the [`server::install_ext_boots`] extension point it plugs
//! into (riir-ai Proposal 052).
//!
//! P3 T1 skeleton: the sealed specialist artifact reader. riir-train
//! trains (`Issue 576`), this repo consumes BYTES — the artifact is the
//! contract, weights never enter any repo.
//!
//! Boundary contract: `BOUNDARY.md` (deps: katgpt-core sigmoid primitive,
//! riir-reflex tokenizer law — both measured here; the reflexer-vessel
//! row is the PUBLIC-RELEASE reader only).

pub mod specialist;
pub mod hybrid;
pub mod stats;
pub mod arsenal;
pub mod arsenal_ops;
pub mod server;
/// The shared hosted-serving HTTP edge (plan 009 T1 — the serve-edge
/// lift): the listener loop, routing, the lane registry + loading
/// machinery and every refusal shape, parameterized by
/// [`serve_edge::ServeConfig`]. The `serve` bin is a thin env/arg shell
/// over [`serve_edge::run`]; the Rethink lane consumes the SAME edge
/// over the extension point — re-shared, never forked (riir-ai
/// Proposal 052).
pub mod serve_edge;
/// The serve lane's receipt primitives (Plan 043 C0 — the
/// one-definition law): the build fingerprint + the input/decision
/// BLAKE3 halves + the manifest fingerprint, ONE home for the serve
/// binary's receipt surfaces.
pub mod receipt;
/// The frozen-artifact staleness probe (Issue 012 / Plan 005): the soft
/// early-warning readout beside the hard pick-parity gate. Report-only —
/// nothing here serves, swaps, or writes state.
pub mod staleness;
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
pub use receipt::{decision_blake3, fingerprint, input_blake3};
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
