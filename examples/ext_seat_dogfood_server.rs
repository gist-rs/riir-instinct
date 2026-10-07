//! The dogfood seat server (riir-refine Plan 202 R2+R3 spike — the seat half).
//!
//! Boots the REAL serve edge with the rerank stub seat (the same contract
//! the `ext_seat_rerank_gates` binary pins: pick among presented options,
//! abstain first-class, refuse options-less) and serves until killed, so
//! the refine `SeatRerankClient` can drive REAL cross-process traffic at
//! it — the exchange the in-process gates cannot produce (each side in its
//! own binary, real sockets, real latencies, real receipt bytes).
//!
//! The backend logs every call to stderr (state size, option count,
//! per-option sizes, pick) — the server-side half of the spike ledger.
//!
//! ```text
//! cargo run --release --example ext_seat_dogfood_server -- 127.0.0.1:8092
//! ```
//!
//! Datasets resolve like the serve binary (`INSTINCT_DATASETS_DIR`, else
//! `../riir-reflex/.raw/datasets_t20k`); a missing dir refuses loud — the
//! seat vehicle is load-bearing, a silent empty boot proves nothing.
//!
//! DELIBERATELY a separate example binary, for the same reason the gates
//! are their own test binary: `install_ext_boots` is install-once per
//! process. One process per posture.

use std::sync::OnceLock;
use std::time::Instant;

use riir_instinct::arsenal::ArsenalManifest;
use riir_instinct::server::{
    Arm, ExtBoots, LaneBackend, ServedDecision, ServedQuestion, SuiteMeta, WeightSource,
};
use riir_instinct::serve_edge::{lane_tag_digest, prepare_lane_seat, LoadCtx, LoadedLane, ServeConfig};

const SUITE: &str = "sst5";

/// The rerank stub — byte-identical CONTRACT to the gates' `StubRerank`
/// (pick the second presented option; abstain on an `ABSTAIN`-prefixed
/// state; refuse options-less) plus the per-call wire log the spike's
/// server-side ledger reads.
struct LoggingRerank {
    suite: &'static str,
}

impl LoggingRerank {
    fn decide_inner(
        &mut self,
        state: &str,
        options: Option<&[String]>,
    ) -> Result<ServedDecision, String> {
        let Some(opts) = options else {
            return Err(format!(
                "suite {}: the rerank seat answers only presented options — generate \
                 traffic never reaches the seat",
                self.suite
            ));
        };
        let t0 = Instant::now();
        let sizes: Vec<usize> = opts.iter().map(String::len).collect();
        let abstained = state.starts_with("ABSTAIN");
        let pick_index = if abstained { None } else { Some(1.min(opts.len() - 1)) };
        eprintln!(
            "[seat-call] state_bytes={} options={} option_sizes={:?} abstained={} pick_index={:?} us={}",
            state.len(),
            opts.len(),
            sizes,
            abstained,
            pick_index,
            t0.elapsed().as_micros(),
        );
        Ok(ServedDecision {
            suite: self.suite,
            arm: "ENC".to_string(),
            options: opts.to_vec(),
            pick_index,
            pick: pick_index.map(|i| opts[i].clone()),
            probabilities: None,
            specialist_scores: pick_index.map(|_| vec![0.1; opts.len()]),
            confidence: if abstained { 0.0 } else { 0.9 },
            escalated: true,
            abstained,
            us: 42,
            gate_abstained: false,
        })
    }

    fn stub_meta(suite: &'static str) -> &'static SuiteMeta {
        static META: OnceLock<SuiteMeta> = OnceLock::new();
        META.get_or_init(|| SuiteMeta {
            suite,
            arm: Arm::Enc,
            labels: 5,
            artifact_labels: 5,
            effective_cap: 8,
            head_scale: 1.0,
            nb_scale: 1.0,
            score_threshold: 0.0,
            distance_threshold: 0.0,
            winner_blake3: "dogfood-stub".to_string(),
            source: WeightSource::RawWinner,
        })
    }
}

impl LaneBackend for LoggingRerank {
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
        Self::stub_meta(self.suite)
    }

    fn centroid(&self) -> [f32; riir_instinct::arsenal_ops::DIM] {
        // Zero centroid = the hoarding gate admits UNJUDGED (its own
        // logged posture) — the stub holds no corpus direction.
        [0.0; riir_instinct::arsenal_ops::DIM]
    }
}

/// The ENC manifest row the stub lane installs under (the gates' shape:
/// hosted-only class, lazy budget — the boot path the R3 gates pin).
fn enc_manifest_text() -> String {
    format!(
        "[[vessel]]\nsuite   = \"{SUITE}\"\ndigest  = \"blake3:{}\"\nclass   = \
         \"hosted_only\"\nfile    = \"rerank_stub_head.bin\"\nposture = {{ arm = \"ENC\" \
         }}\npin_keys = []\nbudget  = {{ load = \"lazy\", max_payload_mb = 16 }}\n",
        "0".repeat(64)
    )
}

/// The stub loader: reads no artifact (the Rethink-host seam, plan 009
/// T2 — a downstream host installs its own loader; ours boots the stub
/// backend on the digest the slot's birth tag carries, so the install is
/// `Idempotent`).
fn stub_lane_loader(
    lctx: &LoadCtx<'_>,
    suite: &'static str,
    _artifact: Option<&str>,
) -> Result<LoadedLane, String> {
    let seat = prepare_lane_seat(lctx, suite)?;
    let row = lctx
        .manifest
        .row(suite)
        .ok_or_else(|| format!("suite {suite} is not in the arsenal manifest"))?;
    let digest = lane_tag_digest(row, suite);
    let server = riir_instinct::server::AnySuiteServer::boot_bytes(
        suite,
        seat,
        b"rerank-stub-head-bytes",
        lctx.manifest,
    )?;
    Ok(LoadedLane {
        server,
        artifact_digest: digest,
        on_install: None,
    })
}

fn main() {
    let bind = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8092".to_string());
    let datasets_dir = std::env::var("INSTINCT_DATASETS_DIR")
        .unwrap_or_else(|_| "../riir-reflex/.raw/datasets_t20k".to_string());
    if !std::path::Path::new(&datasets_dir).is_dir() {
        eprintln!(
            "REFUSED loud: datasets dir {datasets_dir} absent — the seat vehicle is \
             load-bearing; set INSTINCT_DATASETS_DIR"
        );
        std::process::exit(2);
    }

    riir_instinct::server::install_ext_boots(ExtBoots {
        bytes: |suite, _seat, _artifact_bytes, _arm| Ok(Box::new(LoggingRerank { suite })),
    })
    .expect("install-once in a fresh process");

    let text = enc_manifest_text();
    let digest = ArsenalManifest::digest_of(&text);
    let manifest = std::sync::Arc::new(ArsenalManifest::parse(&text).expect("manifest parses"));
    let cfg = ServeConfig {
        suite_filter: Some(vec![SUITE.to_string()]),
        bind,
        datasets_dir,
        winners_dir: "unused-stub-loader-reads-no-artifact".to_string(),
        synth_corpus_dir: None,
        arsenal_path: None,
        cors_origin: Some(vec![]),
        prevalidated_manifest: Some((manifest, digest, "plan 202 dogfood spike".into())),
        lane_loader: Some(stub_lane_loader),
    };
    eprintln!("[dogfood-seat] serving — Ctrl-C to stop");
    if let Err(e) = riir_instinct::serve_edge::run(cfg) {
        eprintln!("[dogfood-seat] edge exited: {e}");
        std::process::exit(1);
    }
}
