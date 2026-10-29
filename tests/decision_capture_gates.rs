//! The decision-capture seam's gates (rethink issue 045 — the decstat
//! producer half's return). The class that died: every capture COMPONENT
//! passed its gates while the edge wiring was absent — the count-pinned
//! green-zero one level up. These gates prove the WIRE: a served decision
//! reaches the capture fn, per [`ServeConfig::on_decision`], on the real
//! edge (`serve_edge::run` + a stub lane — no datasets needed).
//!
//! Shape: the `origin_gate_gates` precedent — own file per seam, the edge
//! booted on an ephemeral port, a fn-pointer capture incrementing a
//! static (fn pointers cannot capture; the static IS the harness).

use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use riir_instinct::arsenal::ArsenalManifest;
use riir_instinct::server::{
    Arm, ExtBoots, LaneBackend, ServedDecision, ServedQuestion, SuiteMeta, WeightSource,
};
use riir_instinct::serve_edge::{LoadCtx, LoadedLane, ServeConfig};

fn datasets_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../riir-reflex/.raw/datasets_t20k")
}

/// The capture count (the gate's readout — a static because fn pointers
/// cannot capture).
static CAPTURED: AtomicUsize = AtomicUsize::new(0);

fn capture_count() -> usize {
    CAPTURED.load(Ordering::SeqCst)
}

/// The capture fn under test — the exact signature the edge consults.
fn counting_capture(d: &ServedDecision) {
    assert!(
        !d.suite.is_empty(),
        "the captured decision must name its suite"
    );
    CAPTURED.fetch_add(1, Ordering::SeqCst);
}

/// The stub backend (the rerank gates' shape): a lane whose decisions are
/// fixtures — the edge's shape validation + dispatch is what's under test.
struct StubBag {
    suite: &'static str,
}

impl StubBag {
    fn decide_inner(
        &mut self,
        _state: &str,
        options: Option<&[String]>,
    ) -> Result<ServedDecision, String> {
        let presented: Vec<String> = match options {
            Some(opts) if !opts.is_empty() => opts.to_vec(),
            _ => vec!["a".to_string(), "b".to_string()],
        };
        Ok(ServedDecision {
            suite: self.suite,
            arm: "ENC".to_string(),
            pick_index: Some(0),
            pick: Some(presented[0].clone()),
            options: presented.clone(),
            probabilities: None,
            specialist_scores: Some(vec![0.1; presented.len()]),
            confidence: 0.9,
            escalated: true,
            abstained: false,
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
            winner_blake3: "stub".to_string(),
            source: WeightSource::RawWinner,
        })
    }
}

impl LaneBackend for StubBag {
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
        [0.0; riir_instinct::arsenal_ops::DIM]
    }
}

/// The stub loader (the rerank gates' shape verbatim — reads no artifact).
fn stub_lane_loader(
    lctx: &LoadCtx<'_>,
    suite: &'static str,
    _artifact: Option<&str>,
) -> Result<LoadedLane, String> {
    let seat = riir_instinct::serve_edge::prepare_lane_seat(lctx, suite)?;
    let row = lctx
        .manifest
        .row(suite)
        .ok_or_else(|| format!("suite {suite} is not in the arsenal manifest"))?;
    let digest = riir_instinct::serve_edge::lane_tag_digest(row, suite);
    let server = riir_instinct::server::AnySuiteServer::boot_bytes(
        suite,
        seat,
        b"capture-stub-head-bytes",
        lctx.manifest,
    )?;
    Ok(LoadedLane {
        server,
        artifact_digest: digest,
        on_install: None,
    })
}

fn install_stub_once() {
    static INSTALLED: OnceLock<()> = OnceLock::new();
    INSTALLED.get_or_init(|| {
        riir_instinct::server::install_ext_boots(ExtBoots {
            bytes: |_suite, _seat, _artifact_bytes, _arm| {
                Ok(Box::new(StubBag { suite: "sst5" }))
            },
        })
        .expect("the stub installs exactly once per process");
    });
}

/// The ENC manifest row (the rerank gates' grammar): explicit file, a
/// digest, LAZY budget — the first decision triggers the load.
fn enc_manifest_text() -> String {
    format!(
        "[[vessel]]\nsuite   = \"sst5\"\ndigest  = \"blake3:{}\"\nclass   = \
         \"hosted_only\"\nfile    = \"capture_stub_head.bin\"\nposture = {{ arm = \"ENC\" \
         }}\npin_keys = []\nbudget  = {{ load = \"lazy\", max_payload_mb = 16 }}\n",
        "0".repeat(64)
    )
}

/// Boot the edge ONCE per process with the capture armed (the rerank
/// gates' one-edge convention; the port is probed ephemeral).
fn shared_edge_port() -> u16 {
    static PORT: OnceLock<u16> = OnceLock::new();
    *PORT.get_or_init(|| {
        install_stub_once();
        let text = enc_manifest_text();
        let digest = ArsenalManifest::digest_of(&text);
        let manifest = std::sync::Arc::new(ArsenalManifest::parse(&text).expect("manifest parses"));
        let probe = std::net::TcpListener::bind("127.0.0.1:0").expect("probe bind");
        let port = probe.local_addr().expect("probe addr").port();
        drop(probe);
        let cfg = ServeConfig {
            bind: format!("127.0.0.1:{port}"),
            datasets_dir: datasets_dir().to_string_lossy().into_owned(),
            winners_dir: "unused-stub-loader-reads-no-artifact".to_string(),
            synth_corpus_dir: None,
            arsenal_path: None,
            suite_filter: Some(vec!["sst5".to_string()]),
            cors_origin: Some(vec![]),
            prevalidated_manifest: Some((manifest, digest, "capture gate".into())),
            lane_loader: Some(stub_lane_loader),
            origin_gate: None,
            on_decision: Some(counting_capture),
        };
        std::thread::Builder::new()
            .name("capture-edge".into())
            .spawn(move || {
                let _ = riir_instinct::serve_edge::run(cfg);
            })
            .expect("edge thread");
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if TcpStream::connect(("127.0.0.1", port)).is_ok() {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "the edge never opened port {port}"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        port
    })
}

/// One POST /decide exchange (the decision_client framing).
fn post_decide(port: u16, body: &str) -> String {
    let mut s = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    s.set_read_timeout(Some(Duration::from_secs(10))).ok();
    let req = format!(
        "POST /decide HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nContent-Type: \
         application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    s.write_all(req.as_bytes()).expect("write");
    let mut raw = String::new();
    let _ = s.read_to_string(&mut raw);
    raw
}

/// POST with the lazy 503 retry loop (the first decision triggers the
/// load; the window answers 503 `loading` until the lane is ready).
fn post_decide_until_ready(port: u16, body: &str) -> String {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let raw = post_decide(port, body);
        if raw.contains("200 OK") || raw.contains("\"loading\"") {
            if raw.contains("200 OK") {
                return raw;
            }
            if Instant::now() > deadline {
                panic!("the lane never became ready; last response: {raw}");
            }
            std::thread::sleep(Duration::from_millis(200));
        } else {
            panic!("unexpected edge response: {raw}");
        }
    }
}

/// THE gate: a served decision reaches the capture fn — the wire the
/// carve lost. One single-question decide ⇒ exactly one capture; the
/// multi-question contract ⇒ one capture PER decision.
#[test]
fn a_served_decision_reaches_the_capture_fn() {
    let before = capture_count();
    let port = shared_edge_port();
    let body = r#"{"contract_version":1,"suite":"sst5","state":"capture gate probe","options":["a","b"]}"#;
    let raw = post_decide_until_ready(port, body);
    assert!(
        raw.contains("200 OK"),
        "the decide must answer 200 (got: {raw})"
    );
    let after = capture_count();
    assert_eq!(
        after,
        before + 1,
        "exactly ONE capture per served decision (before {before}, after {after})"
    );

    // The multi-question contract (the Many arm's per-decision loop).
    let body_multi = r#"{"contract_version":1,"suite":"sst5","state":"capture gate probe","questions":[{"id":"q1","kind":"choice","instructions":"pick","options":["a","b"]},{"id":"q2","kind":"choice","instructions":"pick","options":["a","b"]}]}"#;
    let raw_multi = post_decide(port, body_multi);
    assert!(
        raw_multi.contains("200 OK"),
        "the multi decide must answer 200 (got: {raw_multi})"
    );
    let after_multi = capture_count();
    assert_eq!(
        after_multi,
        after + 2,
        "exactly ONE capture PER decision in the multi contract (after {after}, after_multi {after_multi})"
    );
}

/// The `None` posture pins the DEFAULT's compilation: the seam field
/// exists everywhere, the open callers carry `None`, and building a
/// config without a capture is well-formed (the byte-identical-boots
/// law — the field can never silently become REQUIRED).
#[test]
fn the_default_posture_compiles_without_a_capture() {
    let cfg = ServeConfig {
        bind: "127.0.0.1:0".to_string(),
        datasets_dir: String::new(),
        winners_dir: String::new(),
        synth_corpus_dir: None,
        arsenal_path: None,
        suite_filter: None,
        cors_origin: None,
        prevalidated_manifest: None,
        lane_loader: None,
        origin_gate: None,
        on_decision: None,
    };
    assert!(cfg.on_decision.is_none());
    assert_eq!(capture_count(), 0, "no capture without an edge run");
}
