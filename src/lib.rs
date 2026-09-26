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

pub use specialist::Specialist;
