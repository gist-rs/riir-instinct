//! The heal specialist seat server (Plan 225 T2.3 — the A1 candidate).
//!
//! Boots the REAL serve edge with suite `heal` and a [`LaneBackend`] that
//! scores every presented option with the trained 2-class RISP head
//! (`heal_head_v1.bin`, riir-instinct `instinct_heal_train`): the pair
//! (buggy module, candidate fix) composes into the edit-diff text — the
//! words the candidate adds and removes versus the module — the presence
//! bag of which the head scores per class; the seat picks the argmax
//! `gold` option or abstains (first-class) below the pinned thresholds.
//!
//! Contract: the same laws `tests/ext_seat_rerank_gates.rs` pins — options
//! echoed exactly as presented, `pick_index` into the presentation,
//! abstain first-class, options-less refused (a backend `Err` maps to a
//! 422 at the edge).
//!
//! ```text
//! HEAL_HEAD_PATH=../riir-refine/.refine/heal_seat/heal_head_v1.bin \
//!   cargo run --release --example heal_specialist_server -- 127.0.0.1:8092
//! ```
//!
//! DELIBERATELY a separate example binary (install-once posture rules):
//! the lane constructs the `Ext` server directly in its fn-pointer
//! `lane_loader` (the backend parked in a process `OnceLock`), so no
//! `install_ext_boots` and no dataset seat are needed — the backend
//! answers presented options, not a fixed label universe.

use std::sync::OnceLock;
use std::time::Instant;

use riir_instinct::arsenal::ArsenalManifest;
use riir_instinct::server::{
    AnySuiteServer, Arm, LaneBackend, ServedDecision, ServedQuestion, SuiteMeta, WeightSource,
};
use riir_instinct::serve_edge::{LoadCtx, LoadedLane, ServeConfig};
use riir_instinct::specialist::{self, Specialist};

const SUITE: &str = "heal";
const GOLD_LABEL: &str = "gold";

/// The abstention thresholds the trainer froze on the case-level holdout
/// (Bench 119 records them beside the read; env overrides exist for
/// re-reads at a different posture, never for silent drift).
const DEFAULT_TAU_PROB: f32 = 0.3;
const DEFAULT_TAU_MARGIN: f32 = 0.0;

/// The pair-composition law — MUST stay identical to the trainer's
/// `pair_text` (riir-instinct `examples/instinct_heal_train.rs`): the
/// edit-diff words (candidate adds / removes versus the buggy module),
/// tokenized as plain words so the shared `events_into` law sees
/// `add <w…> del <w…>`.
fn pair_text(state: &str, option: &str) -> String {
    let m: Vec<&str> = state.split_whitespace().collect();
    let c: Vec<&str> = option.split_whitespace().collect();
    let mut p = 0usize;
    while p < m.len() && p < c.len() && m[p] == c[p] {
        p += 1;
    }
    let mut s = 0usize;
    while s < m.len() - p && s < c.len() - p && m[m.len() - 1 - s] == c[c.len() - 1 - s] {
        s += 1;
    }
    let added = &c[p..c.len() - s];
    let removed = &m[p..m.len() - s];
    let mut out = String::from("add");
    for w in added {
        out.push(' ');
        out.push_str(w);
    }
    out.push_str("\ndel");
    for w in removed {
        out.push(' ');
        out.push_str(w);
    }
    out
}

struct HealRerank {
    spec: Specialist,
    gold_idx: usize,
    tau_prob: f32,
    tau_margin: f32,
    meta: &'static SuiteMeta,
    scratch: Vec<u32>,
}

impl HealRerank {
    /// One decode per process: the fn-pointer loader builds the backend
    /// from the parked head bytes at the lane's (lazy) first load.
    fn build(meta: &'static SuiteMeta, tau_prob: f32, tau_margin: f32) -> Result<Self, String> {
        let bytes = HEAD_BYTES.get().expect("head bytes parked");
        let spec = specialist::decode_artifact(bytes)?;
        assert_eq!(spec.suite, SUITE, "head suite is heal");
        let gold_idx = spec
            .labels
            .iter()
            .position(|l| l == GOLD_LABEL)
            .ok_or_else(|| "gold label missing from the head".to_string())?;
        Ok(Self {
            spec,
            gold_idx,
            tau_prob,
            tau_margin,
            meta,
            scratch: Vec::new(),
        })
    }

    fn decide_inner(
        &mut self,
        state: &str,
        options: Option<&[String]>,
    ) -> Result<ServedDecision, String> {
        let Some(opts) = options else {
            return Err(format!(
                "suite {SUITE}: the rerank seat answers only presented options — generate \
                 traffic never reaches the seat"
            ));
        };
        let t0 = Instant::now();
        let mut scores: Vec<f32> = Vec::with_capacity(opts.len());
        let mut bag: Vec<(u32, f32)> = Vec::new();
        for opt in opts {
            bag.clear();
            specialist::presence_bag_into(
                pair_text(state, opt).as_bytes(),
                &mut bag,
                &mut self.scratch,
            );
            scores.push(self.spec.score_class(&bag, self.gold_idx));
        }
        let (mut top1, mut top2) = (0usize, 0usize);
        for (i, &p) in scores.iter().enumerate() {
            if p > scores[top1] {
                top2 = top1;
                top1 = i;
            } else if i != top1 && p > scores[top2] {
                top2 = i;
            }
        }
        let margin = if scores.len() > 1 {
            scores[top1] - scores[top2]
        } else {
            scores[top1]
        };
        let abstained = scores[top1] < self.tau_prob || margin < self.tau_margin;
        let pick_index = (!abstained).then_some(top1);
        let confidence = scores[top1] as f64;
        Ok(ServedDecision {
            suite: SUITE,
            arm: "ENC".to_string(),
            options: opts.to_vec(),
            pick_index,
            pick: pick_index.map(|i| opts[i].clone()),
            probabilities: Some(scores.iter().map(|p| *p as f64).collect()),
            specialist_scores: Some(scores),
            confidence,
            escalated: true,
            abstained,
            us: t0.elapsed().as_micros() as u64,
            gate_abstained: false,
        })
    }
}

impl LaneBackend for HealRerank {
    fn decide(
        &mut self,
        state: &str,
        options: Option<&[String]>,
    ) -> Result<ServedDecision, String> {
        self.decide_inner(state, options)
    }

    fn decide_multi(
        &mut self,
        state: &str,
        questions: &[ServedQuestion<'_>],
    ) -> Result<Vec<ServedDecision>, String> {
        questions
            .iter()
            .map(|q| self.decide_inner(state, Some(q.options)))
            .collect()
    }

    fn meta(&self) -> &SuiteMeta {
        self.meta
    }

    fn centroid(&self) -> [f32; riir_instinct::arsenal_ops::DIM] {
        // Zero centroid = the hoarding gate admits UNJUDGED (its own
        // posture) — the heal lane holds no corpus direction.
        [0.0; riir_instinct::arsenal_ops::DIM]
    }
}

/// The ENC manifest row the lane installs under (the dogfood shape:
/// hosted-only class, lazy budget — the boot path the R3 gates pin) with
/// the REAL head digest, not a decorative zero.
fn enc_manifest_text(head_blake3: &str) -> String {
    format!(
        "[[vessel]]\nsuite   = \"{SUITE}\"\ndigest  = \"blake3:{head_blake3}\"\nclass   = \
         \"hosted_only\"\nfile    = \"heal_head_v1.bin\"\nposture = {{ arm = \"ENC\" \
         }}\npin_keys = []\nbudget  = {{ load = \"lazy\", max_payload_mb = 16 }}\n"
    )
}

/// The head bytes parked here by `main`; the fn-pointer loader decodes a
/// fresh backend at the lane's (lazy) first load — one decode per process,
/// no shared-state pointer games.
static HEAD_BYTES: OnceLock<Vec<u8>> = OnceLock::new();
static HEAD_DIGEST: OnceLock<[u8; 32]> = OnceLock::new();
static TAU: OnceLock<(f32, f32)> = OnceLock::new();

fn heal_lane_loader(
    _lctx: &LoadCtx<'_>,
    suite: &'static str,
    _artifact: Option<&str>,
) -> Result<LoadedLane, String> {
    assert_eq!(suite, SUITE);
    let meta = META.get().expect("meta parked");
    let (tau_prob, tau_margin) = TAU.get().copied().expect("tau parked");
    let backend = HealRerank::build(meta, tau_prob, tau_margin)?;
    let digest = HEAD_DIGEST.get().copied().expect("digest parked");
    Ok(LoadedLane {
        server: AnySuiteServer::Ext(Box::new(backend)),
        artifact_digest: digest,
        on_install: None,
    })
}

static META: OnceLock<&'static SuiteMeta> = OnceLock::new();

fn main() {
    let bind = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8092".to_string());
    let bind_display = bind.clone();
    let head_path = std::env::var("HEAL_HEAD_PATH").unwrap_or_else(|_| {
        eprintln!(
            "REFUSED loud: HEAL_HEAD_PATH is not set — the trained head is load-bearing \
             (instinct_heal_train exports it)"
        );
        std::process::exit(2);
    });
    let bytes = std::fs::read(&head_path).unwrap_or_else(|e| panic!("head {head_path}: {e}"));
    let spec = specialist::decode_artifact(&bytes).expect("head decodes (RISP)");
    assert_eq!(spec.suite, SUITE, "head suite is heal");
    let gold_idx = spec
        .labels
        .iter()
        .position(|l| l == GOLD_LABEL)
        .expect("gold label present in the head");
    let tau_prob = std::env::var("HEAL_TAU_PROB")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_TAU_PROB);
    let tau_margin = std::env::var("HEAL_TAU_MARGIN")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_TAU_MARGIN);
    let head_blake3 = blake3::hash(&bytes).to_hex().to_string();
    eprintln!(
        "[heal-seat] head={head_path} labels={:?} gold_idx={gold_idx} tau_prob={tau_prob} tau_margin={tau_margin} blake3={head_blake3}",
        spec.labels,
    );

    let meta: &'static SuiteMeta = Box::leak(Box::new(SuiteMeta {
        suite: SUITE,
        arm: Arm::Enc,
        labels: spec.labels.len(),
        artifact_labels: spec.labels.len(),
        effective_cap: 8,
        head_scale: 1.0,
        nb_scale: 1.0,
        score_threshold: tau_prob,
        distance_threshold: tau_margin,
        winner_blake3: head_blake3.clone(),
        source: WeightSource::RawWinner,
    }));
    let digest: [u8; 32] = blake3::hash(&bytes).into();
    HEAD_BYTES.set(bytes).expect("head parks once");
    HEAD_DIGEST.set(digest).expect("digest parks once");
    TAU.set((tau_prob, tau_margin)).expect("tau parks once");
    META.set(meta).expect("meta parks once");

    let manifest_text = enc_manifest_text(&head_blake3);
    let manifest_digest = ArsenalManifest::digest_of(&manifest_text);
    let manifest =
        std::sync::Arc::new(ArsenalManifest::parse(&manifest_text).expect("manifest parses"));
    let cfg = ServeConfig {
        suite_filter: Some(vec![SUITE.to_string()]),
        bind,
        datasets_dir: "unused-heal-lane-needs-no-dataset".to_string(),
        winners_dir: "unused-heal-lane-reads-HEAL_HEAD_PATH".to_string(),
        synth_corpus_dir: None,
        arsenal_path: None,
        cors_origin: Some(vec![]),
        prevalidated_manifest: Some((
            manifest,
            manifest_digest,
            "plan 225 heal specialist".into(),
        )),
        lane_loader: Some(heal_lane_loader),
    };
    eprintln!("[heal-seat] serving on {bind_display} — Ctrl-C to stop");
    if let Err(e) = riir_instinct::serve_edge::run(cfg) {
        eprintln!("[heal-seat] edge exited: {e}");
        std::process::exit(1);
    }
}
