//! The Lane B shared-worker parity gate (instinct issue 018): the SERVE
//! surface under interleaved load. Two ENC lanes on the SAME checkpoint
//! (the shared-worker posture when `serve-encoder-shared` is compiled;
//! the per-lane default when not) replay the SAME frozen case sequence —
//! ISOLATED per-lane runs vs runs INTERLEAVED across the lanes in
//! lockstep — and the answers must be BIT-IDENTICAL. The defect class:
//! state leaking between suites through a shared worker (agent scratch,
//! the reused encode buffer, device caches).
//!
//! Within-build determinism (interleaved == isolated) is what one binary
//! proves. The CROSS-build arm (shared vs per-lane binaries
//! byte-identical) is the runner's job: build both feature sets, run this
//! gate twice, diff the printed `PARITY:` fingerprints — the bench record
//! carries the pair.
//!
//! SKIPs loud (a deferral, never a green) unless
//! `INSTINCT_ENCODER_SHARED_PARITY=1` and the material exists — the same
//! discipline as `serve_encoder_parity` (the sealed heads
//! `INSTINCT_ENCODER_ART`'s dir, the t20k datasets, the laya weights,
//! `LAYA_DEVICE` for the posture).

use std::path::PathBuf;

use riir_instinct::arsenal::{ArsenalManifest, ValidateCtx};
use riir_instinct::server::{AnySuiteServer, ServedQuestion};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn datasets_dir() -> PathBuf {
    std::env::var("INSTINCT_DATASETS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root().join("../riir-reflex/.raw/datasets_t20k"))
}

fn heads_dir() -> PathBuf {
    std::env::var("INSTINCT_ENCODER_ART")
        .map(|p| {
            PathBuf::from(p)
                .parent()
                .expect("ART has a parent")
                .to_path_buf()
        })
        .unwrap_or_else(|_| repo_root().join("../riir-train/.raw/t599"))
}

/// The two seated encoder suites (one checkpoint, two heads — exactly the
/// shared worker's sharing shape). The head FILES carry the trainer's
/// names (t6_s0 = the frozen Bench-029 sst5 head; the xnli_en v1 head),
/// so each suite's file resolves explicitly, env-overridable.
const SUITES: [(&str, &str); 2] = [("sst5", "t6_s0.bin"), ("xnli_en", "xnli_en_encoder_v1.bin")];
/// Cases per lane per replay (the frozen read's population slice).
const CASES: usize = 24;

fn head_file(suite: &str) -> String {
    let env_key = format!("INSTINCT_ENC_HEAD_{}", suite.to_uppercase());
    if let Ok(f) = std::env::var(&env_key) {
        return f;
    }
    SUITES
        .iter()
        .find(|(s, _)| *s == suite)
        .map(|(_, f)| f.to_string())
        .unwrap_or_else(|| panic!("suite {suite} has no head-file row in this gate"))
}

fn enc_manifest(suite: &'static str, winners: &std::path::Path) -> ArsenalManifest {
    let name = head_file(suite);
    let bytes = std::fs::read(winners.join(&name)).unwrap_or_else(|e| panic!("read {name}: {e}"));
    let digest = blake3::hash(&bytes).to_hex();
    let text = format!(
        "[[vessel]]\nsuite   = {suite:?}\ndigest  = \"blake3:{digest}\"\nclass   = \
         \"hosted_only\"\nfile    = {name:?}\nposture = {{ arm = \"ENC\" }}\npin_keys = \
         []\nbudget  = {{ load = \"eager\", max_payload_mb = 16 }}\n"
    );
    let m = ArsenalManifest::parse(&text).expect("the ENC manifest parses");
    m.validate(&ValidateCtx::raw(winners))
        .unwrap_or_else(|e| panic!("the ENC manifest must validate: {e}"));
    m
}

fn presented_options(suite: &str, criteria: &serde_json::Value) -> Vec<String> {
    match criteria {
        serde_json::Value::Object(m) => m.keys().cloned().collect(),
        serde_json::Value::Array(a) => a
            .iter()
            .map(|v| match v {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .collect(),
        _ => panic!("suite {suite}: the ENC template lost its criteria"),
    }
}

/// Fold one decision's answer (pick + score bits + confidence bits) into
/// a running fingerprint.
fn fold(running: &mut [u8; 32], d: &riir_instinct::server::ServedDecision) {
    let mut h = blake3::Hasher::new();
    h.update(running);
    h.update(d.pick.as_deref().unwrap_or("").as_bytes());
    if let Some(scores) = &d.specialist_scores {
        for s in scores {
            h.update(&s.to_bits().to_le_bytes());
        }
    }
    h.update(&d.confidence.to_bits().to_le_bytes());
    running.copy_from_slice(h.finalize().as_bytes());
}

fn decide_case(
    srv: &mut AnySuiteServer,
    suite: &'static str,
    seat: &riir_reflex::harness::runner::seat::Seat,
    case_i: usize,
) -> riir_instinct::server::ServedDecision {
    let case = &seat.suite.cases[case_i];
    let q = &case.questions[0];
    let options = presented_options(suite, &q.criteria);
    let sq = [ServedQuestion {
        qid: &q.qid,
        kind: q.kind,
        instructions: &q.instructions,
        options: &options,
    }];
    let out = srv
        .decide_multi(&seat.state_strs[case_i], &sq)
        .unwrap_or_else(|e| panic!("suite {suite} case {case_i}: {e}"));
    out.into_iter().next().expect("one decision per question")
}

fn replay_isolated(suite: &'static str) -> [u8; 32] {
    let datasets = datasets_dir();
    let winners = heads_dir();
    let manifest = enc_manifest(suite, &winners);
    let mut srv = AnySuiteServer::boot(suite, &datasets, &winners, &manifest)
        .unwrap_or_else(|e| panic!("boot {suite}: {e}"));
    let seat = riir_reflex::harness::runner::seat::prepare_seat(suite, &datasets)
        .unwrap_or_else(|e| panic!("prepare {suite}: {e}"));
    let mut fp = [0u8; 32];
    for case_i in 0..CASES {
        let d = decide_case(&mut srv, suite, &seat, case_i);
        fold(&mut fp, &d);
    }
    fp
}

#[test]
fn shared_worker_interleaved_replay_is_bit_identical_to_isolated() {
    if std::env::var("INSTINCT_ENCODER_SHARED_PARITY").as_deref() != Ok("1") {
        eprintln!(
            "SKIP loud: INSTINCT_ENCODER_SHARED_PARITY=1 not set — the gate needs the sealed \
             heads, the t20k datasets and the laya weights (a laya run); a skip is a deferral, \
             never a green"
        );
        return;
    }
    let datasets = datasets_dir();
    let winners = heads_dir();
    let missing: Vec<&str> = SUITES
        .iter()
        .map(|(s, f)| if winners.join(f).is_file() { "" } else { *s })
        .filter(|s| !s.is_empty())
        .collect();
    if !missing.is_empty() || !datasets.join("sst5").is_dir() {
        eprintln!(
            "SKIP loud: material absent — heads dir {} missing {missing:?} · datasets {}",
            winners.display(),
            datasets.display()
        );
        return;
    }
    let mode = riir_instinct::encoder_serve::encoder_topology_label();
    eprintln!("PARITY posture: build mode = {mode}");

    // The ISOLATED fingerprints (each lane answers alone).
    let iso: Vec<[u8; 32]> = SUITES.iter().map(|(s, _)| replay_isolated(s)).collect();

    // The INTERLEAVED replay: both lanes alive at once (under the shared
    // feature: ONE worker), answering case-by-case in lockstep.
    let mut servers: Vec<AnySuiteServer> = SUITES
        .iter()
        .map(|(s, _)| {
            let manifest = enc_manifest(s, &winners);
            AnySuiteServer::boot(s, &datasets, &winners, &manifest)
                .unwrap_or_else(|e| panic!("boot {s}: {e}"))
        })
        .collect();
    let seats: Vec<_> = SUITES
        .iter()
        .map(|(s, _)| {
            riir_reflex::harness::runner::seat::prepare_seat(s, &datasets_dir())
                .unwrap_or_else(|e| panic!("prepare {s}: {e}"))
        })
        .collect();
    let mut inter: Vec<[u8; 32]> = vec![[0u8; 32]; SUITES.len()];
    for case_i in 0..CASES {
        for (i, srv) in servers.iter_mut().enumerate() {
            let suite = SUITES[i].0;
            let d = decide_case(srv, suite, &seats[i], case_i);
            fold(&mut inter[i], &d);
        }
    }

    for (i, s) in SUITES.iter().enumerate() {
        let s = s.0;
        assert_eq!(
            inter[i], iso[i],
            "suite {s}: interleaved answers diverge from isolated — shared-worker state leak"
        );
        eprintln!(
            "PARITY: {s} interleaved == isolated · {} · {mode}",
            hex16(&iso[i])
        );
    }
    // The concurrency gate's disclosure: the lockstep replay's wall clock
    // (per-decision latencies are in the decisions; the p50/p99 quote is
    // the bench record's job — this line anchors the run).
    eprintln!(
        "PARITY: {CASES} cases × {} lanes in lockstep — bit-identical",
        SUITES.len()
    );
}

fn hex16(b: &[u8; 32]) -> String {
    b.iter()
        .map(|x| format!("{x:02x}"))
        .collect::<Vec<_>>()
        .join("")[..16]
        .to_string()
}
