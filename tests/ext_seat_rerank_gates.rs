//! The RERANK seat gates (riir-refine Plan 202 R3a — the instinct half).
//!
//! The `Ext` seat serves the rerank half of the escalation chain: refine's
//! L0–L3 candidates arrive as the `/decide` request's `options`; the lane
//! backend picks one or abstains; the answer rides back with the options
//! echoed EXACTLY as presented (the client-side half — the echo-is-the-
//! trust-anchor contract — lives in riir-refine's `seat_rerank` client;
//! this gate pins the SERVER half on the open build).
//!
//! Three faces:
//!
//! 1. **The presented options reach the backend verbatim, in order** —
//!    the server half of "the options are EXACTLY the L0–L3 candidates".
//! 2. **The pick/abstain round-trip over the HTTP edge** — a pick maps
//!    back to the presented option at `pick_index`; an abstain is
//!    first-class (`pick: null`, `abstained: true`).
//! 3. **A rerank lane answers only presented options** — an options-less
//!    body on the rerank seat refuses `422` (generate traffic never
//!    reaches the seat: the seat's contract is pick-or-abstain, never a
//!    free-form answer).
//!
//! DELIBERATELY its own test binary: `install_ext_boots` is install-once
//! per process (the `EXT_BOOTS` OnceLock), and `serve_gates` pins the
//! no-backend refusal of the SAME open build — a stub installed in that
//! binary would flip its refusal into a boot. One process per posture.
//!
//! Skips loud without the t20k datasets (the seat's vehicle) — a skip is
//! a deferral, never a green.

use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use riir_instinct::arsenal::ArsenalManifest;
use riir_instinct::server::{Arm, ExtBoots, LaneBackend, ServedDecision, ServedQuestion, SuiteMeta};
use riir_instinct::serve_edge::{LoadCtx, LoadedLane, ServeConfig};

/// The datasets the seats load from (the serve_gates convention — a bare
/// clone skips loud).
fn datasets_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../riir-reflex/.raw/datasets_t20k")
}

fn data_present() -> bool {
    datasets_dir().is_dir()
}

/// The options every backend call has presented so far, in call order —
/// the recorded half of the "EXACTLY as presented" assertion.
static SEEN_OPTIONS: Mutex<Vec<Vec<String>>> = Mutex::new(Vec::new());

/// The rerank stub: a [`LaneBackend`] whose whole semantics are the seat
/// contract itself — pick the second presented option (index 1, the
/// presentation order IS the answer space), or abstain when the state
/// says so. It holds no model: the CONTRACT is the thing under gate.
struct StubRerank {
    suite: &'static str,
}

impl StubRerank {
    fn decide_inner(
        &mut self,
        state: &str,
        options: Option<&[String]>,
    ) -> Result<ServedDecision, String> {
        // The rerank seat answers only PRESENTED options — an options-less
        // call is a generate-shaped question and refuses (the edge maps
        // it to 422; the refusal text names the contract).
        let Some(opts) = options else {
            return Err(format!(
                "suite {}: the rerank seat answers only presented options — generate \
                 traffic never reaches the seat",
                self.suite
            ));
        };
        SEEN_OPTIONS
            .lock()
            .expect("seen options lock")
            .push(opts.to_vec());
        // A noul question may present NO options — the fixed [false,
        // true] rendering is the LANE's (the bag server's decide_multi
        // law; the stub mirrors the lane side, never the edge's — the
        // edge only validates the shape and passes the empty slice
        // through).
        let noul_fixed = opts.is_empty();
        let fixed = ["false".to_string(), "true".to_string()];
        let presented: &[String] = if noul_fixed { &fixed } else { opts };
        let abstained = state.starts_with("ABSTAIN");
        let pick_index =
            if abstained { None } else { Some(1.min(presented.len() - 1)) };
        Ok(ServedDecision {
            suite: self.suite,
            arm: "ENC".to_string(),
            options: presented.to_vec(),
            pick_index,
            pick: pick_index.map(|i| presented[i].clone()),
            // The specialist's own scores decided (honest about not being
            // a distribution — the ServedDecision doc's shape).
            probabilities: None,
            specialist_scores: pick_index.map(|_| vec![0.1; presented.len()]),
            confidence: if abstained { 0.0 } else { 0.9 },
            escalated: true,
            abstained,
            us: 42,
            gate_abstained: false,
        })
    }

    /// The stub meta, built once (the healthz disclosure reads it per
    /// request — leak one instance, never one per call).
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
            source: riir_instinct::server::WeightSource::RawWinner,
        })
    }
}

impl LaneBackend for StubRerank {
    fn decide(
        &mut self,
        state: &str,
        options: Option<&[String]>,
    ) -> Result<ServedDecision, String> {
        self.decide_inner(state, options)
    }

    /// The multi-question contract: one state, every question answered in
    /// order (the rerank lane is single-question, but the seat implements
    /// the whole trait — the edge dispatches both forms byte-identically).
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
        // The zero vector is the "no corpus direction" shape — the
        // hoarding gate admits it UNJUDGED (its own logged posture).
        [0.0; riir_instinct::arsenal_ops::DIM]
    }
}

/// The install-once wrapper: every test in this binary needs the SAME
/// stub installed, and `install_ext_boots` refuses a second install. The
/// first caller wins; the rest see the same stub either way.
fn install_stub_once() {
    static INSTALLED: OnceLock<()> = OnceLock::new();
    INSTALLED.get_or_init(|| {
        riir_instinct::server::install_ext_boots(ExtBoots {
            bytes: |suite, _seat, _artifact_bytes, _arm| {
                Ok(Box::new(StubRerank { suite }))
            },
        })
        .expect("the stub installs exactly once per process");
    });
}

/// The ENC manifest row (the serve_gates grammar): explicit file, a
/// digest, LAZY budget — the first decision triggers the load and the
/// 503 window covers it (issue 018's lazy-once-then-resident contract).
fn enc_manifest_text() -> String {
    format!(
        "[[vessel]]\nsuite   = \"sst5\"\ndigest  = \"blake3:{}\"\nclass   = \
         \"hosted_only\"\nfile    = \"rerank_stub_head.bin\"\nposture = {{ arm = \"ENC\" \
         }}\npin_keys = []\nbudget  = {{ load = \"lazy\", max_payload_mb = 16 }}\n",
        "0".repeat(64)
    )
}

/// The host loader seam: the raw route would `check_winner_file`-refuse an
/// ENC row on a bridged suite — a downstream host installs its own loader
/// (the Rethink-host precedent, plan 009 T2). Ours reads no artifact at
/// all: the stub's bytes are inert, and the digest is the row's lane tag
/// (the pub helper — the SAME value the slot's birth tag carries, so the
/// install lands Idempotent instead of refusing a fork).
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
        b"rerank-stub-head-bytes",
        lctx.manifest,
    )?;
    Ok(LoadedLane {
        server,
        artifact_digest: digest,
        on_install: None,
    })
}

/// Boot the edge on an ephemeral port with the stub lane — ONE edge per
/// process (the tests share it; the lazy load is also part of the gate).
fn shared_edge_port() -> u16 {
    static PORT: OnceLock<u16> = OnceLock::new();
    *PORT.get_or_init(|| {
        install_stub_once();
        let text = enc_manifest_text();
        let digest = ArsenalManifest::digest_of(&text);
        let manifest = std::sync::Arc::new(ArsenalManifest::parse(&text).expect("manifest parses"));
        // Probe-bind for an ephemeral port (the serve_gates convention —
        // the port is released before the edge binds it; the connect
        // retry below absorbs the tiny race).
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
            prevalidated_manifest: Some((manifest, digest, "rerank gate".into())),
            lane_loader: Some(stub_lane_loader),
        };
        std::thread::Builder::new()
            .name("rerank-edge".into())
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

/// One POST /decide exchange: the hand-rolled request (the
/// decision_client framing), the full body read to EOF. Returns the raw
/// HTTP response.
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

/// POST with the lazy 503 retry loop: the FIRST decision triggers the
/// load and the window answers 503 `loading`; retry until the lane is
/// ready (bounded — a lane that never loads fails the gate, never hangs).
fn post_decide_until_ready(port: u16, body: &str) -> String {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let raw = post_decide(port, body);
        if !raw.starts_with("HTTP/1.1 503") || Instant::now() >= deadline {
            return raw;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
}

fn body_of(raw: &str) -> &str {
    raw.split_once("\r\n\r\n").map(|(_, b)| b).unwrap_or("")
}

fn json(body: &str) -> serde_json::Value {
    serde_json::from_str(body).unwrap_or(serde_json::Value::Null)
}

// ── the gates ─────────────────────────────────────────────────────────

/// The server half of "the options are EXACTLY the L0–L3 candidates":
/// what the backend received is byte-identical to what the request
/// presented, in presentation order — and the reply echoes it.
#[test]
fn rerank_options_reach_the_backend_exactly_presented() {
    if !data_present() {
        eprintln!("SKIP loud: datasets absent (the seat is the gate's vehicle)");
        return;
    }
    let port = shared_edge_port();
    let candidates = ["fix A: inline format args", "fix B: collapsible if", "fix C: map flatter"];
    let body = format!(
        "{{\"suite\":\"sst5\",\"state\":\"the buggy span\",\"options\":[{}]}}",
        candidates
            .iter()
            .map(|c| format!("\"{c}\""))
            .collect::<Vec<_>>()
            .join(",")
    );
    let raw = post_decide_until_ready(port, &body);
    assert!(
        raw.starts_with("HTTP/1.1 200"),
        "the rerank lane must answer 200 once loaded: {raw}"
    );
    let v = json(body_of(&raw));

    // The backend saw EXACTLY the presented options, in order. The list
    // is shared across parallel tests, so the assertion is "this exact
    // presentation was recorded" — the candidates are unique strings, so
    // the match is unambiguous (a `last()` assertion would race a
    // sibling test's request between mine and the read).
    let seen = SEEN_OPTIONS.lock().unwrap();
    assert!(
        seen.iter().any(|o| o == &candidates),
        "the backend must see the candidates verbatim, in order; recorded: {seen:?}"
    );

    // The reply's echo is the presentation (the client's trust anchor).
    let echo: Vec<&str> = v["options"]
        .as_array()
        .expect("options array")
        .iter()
        .map(|o| o.as_str().expect("string option"))
        .collect();
    assert_eq!(echo, candidates, "the reply must echo the presented options");

    // The pick maps back into the presentation order.
    assert_eq!(v["pick_index"].as_u64(), Some(1), "the stub picks index 1");
    assert_eq!(v["pick"].as_str(), Some(candidates[1]));
    assert_eq!(v["abstained"].as_bool(), Some(false));
    assert_eq!(v["arm"].as_str(), Some("ENC"), "the ENC seat answered");

    // The v1 contract echo (the R7 freeze): the served version rides the
    // answer so a proxy/lane mismatch is visible at the client.
    assert_eq!(v["contract_version"].as_u64(), Some(1), "{v}");
    // The frozen v1 field set (reflex issue 074 T2): MEMBERSHIP — a
    // silent field removal reds here; new fields append without reding
    // (the additive-only law).
    let doc = v.as_object().expect("the decision is an object");
    for field in [
        "suite",
        "arm",
        "lane",
        "contract_version",
        "options",
        "pick",
        "pick_index",
        "probabilities",
        "specialist_scores",
        "confidence",
        "escalated",
        "abstained",
        "us",
        "lane_load",
        "receipt",
    ] {
        assert!(
            doc.contains_key(field),
            "the frozen v1 field {field:?} vanished from the response: {v}"
        );
    }
    assert!(
        v["receipt"]["decision_blake3"].as_str().unwrap_or_default().len() == 64,
        "the decision commitment rides the receipt (blake3 spelling): {v}"
    );
    assert!(
        v["receipt"]["input_blake3"].as_str().unwrap_or_default().len() == 64,
        "the input commitment rides the receipt (blake3 spelling): {v}"
    );
}

/// Abstention is first-class over the edge: the backend abstained → the
/// wire answer is `abstained: true`, `pick: null` — the modelless answer
/// stands on the caller's side (the client half of that contract is the
/// refine client's `SeatAnswer::Abstained`).
#[test]
fn rerank_abstain_is_first_class_over_the_edge() {
    if !data_present() {
        eprintln!("SKIP loud: datasets absent (the seat is the gate's vehicle)");
        return;
    }
    let port = shared_edge_port();
    let body =
        "{\"suite\":\"sst5\",\"state\":\"ABSTAIN: no confident pick\",\"options\":[\"a\",\"b\"]}";
    let raw = post_decide_until_ready(port, body);
    assert!(
        raw.starts_with("HTTP/1.1 200"),
        "an abstain is a 200 answer, never an error: {raw}"
    );
    let v = json(body_of(&raw));
    assert_eq!(v["abstained"].as_bool(), Some(true));
    assert!(v["pick"].is_null(), "an abstain carries no pick: {v}");
    assert!(v["pick_index"].is_null(), "an abstain carries no index: {v}");
}

/// The seat's contract is pick-or-abstain over PRESENTED options: an
/// options-less body is a generate-shaped question and refuses 422 —
/// generate traffic never reaches the seat.
#[test]
fn a_rerank_lane_answers_only_presented_options() {
    if !data_present() {
        eprintln!("SKIP loud: datasets absent (the seat is the gate's vehicle)");
        return;
    }
    let port = shared_edge_port();
    let raw =
        post_decide_until_ready(port, "{\"suite\":\"sst5\",\"state\":\"no options here\"}");
    assert!(
        raw.starts_with("HTTP/1.1 422"),
        "an options-less body must refuse 422: {raw}"
    );
    let v = json(body_of(&raw));
    let code = v["code"].as_str().unwrap_or_default();
    let err = v["error"].as_str().unwrap_or_default();
    // The edge's error vocabulary: a backend Err(String) that is not one
    // of the bag lanes' own shape prefixes maps to 422 `bad_request`, and
    // the REFUSAL TEXT carries the semantic — which is what names the
    // rerank contract (and, with it, the generate-never-reaches-the-seat
    // law).
    assert_eq!(code, "bad_request", "the caller-shape refusal code: {v}");
    assert!(
        err.contains("presented options") && err.contains("generate"),
        "the refusal names the rerank contract: {err}"
    );
}

/// The wire contract version refusal (the R7 v1 freeze — reflex issue
/// 074 T1): an unknown version is fail-closed LOUD — 400 naming the
/// supported set — and the refusal PRECEDES the suite lookup (suite
/// `nope`: a 400 here, never the 404 `unknown_suite`). Deliberately
/// UNGATED on data: the check fires before any lane exists, so it must
/// run on every box the edge boots on — a data-gated contract refusal
/// would be green-by-skip on exactly the fresh-clone boxes most likely
/// to carry a stale client.
#[test]
fn unknown_contract_version_refuses_fail_closed() {
    let port = shared_edge_port();
    for ver in ["2", r#""1""#] {
        let raw = post_decide(
            port,
            &format!(r#"{{"suite":"nope","state":"x","contract_version":{ver}}}"#),
        );
        assert!(
            raw.starts_with("HTTP/1.1 400"),
            "an unknown contract_version {ver} must refuse 400: {raw}"
        );
        let v = json(body_of(&raw));
        assert_eq!(
            v["code"].as_str(),
            Some("unsupported_contract_version"),
            "the refusal code: {v}"
        );
        let err = v["error"].as_str().unwrap_or_default();
        assert!(
            err.contains("supported") && err.contains('1'),
            "the refusal must name the supported set: {err}"
        );
    }
    // The absent field stays a v1 client by construction: the same body
    // WITHOUT the field passes the contract check and reaches the suite
    // lookup — the 404 `unknown_suite` spelling is byte-unchanged
    // (back-compat by construction, pinned).
    let raw = post_decide(port, r#"{"suite":"nope","state":"x"}"#);
    assert!(
        raw.starts_with("HTTP/1.1 404"),
        "the absent field must read as v1 and reach the suite lookup: {raw}"
    );
    assert!(
        body_of(&raw).contains("unknown_suite"),
        "the absent-field body must fail on the suite, never the contract: {raw}"
    );
}

/// The presentation-shape contract at the EDGE (the crash-class fix): a
/// <2 presented-option body — single form `"options":[]`/1-wide, or a
/// multi-form choice carrying <2 — refuses 422 `bad_field` BEFORE the
/// suite lookup. Before this pin the body reached the lane unvalidated
/// (the ext-seat dispatch) and a lane indexing the presentation panicked
/// the conn thread and POISONED the slot lock — one malformed body took
/// the whole suite down. Data-independent by construction (the refusal
/// precedes any lane work), so it runs ungated like the version gate.
#[test]
fn a_sub2_presentation_refuses_at_the_edge() {
    let port = shared_edge_port();
    for body in [
        r#"{"suite":"nope","state":"x","options":[]}"#,
        r#"{"suite":"nope","state":"x","options":["lonely"]}"#,
        r#"{"suite":"nope","state":"x","questions":[{"id":"q1","kind":"choice","options":[]}]}"#,
    ] {
        let raw = post_decide(port, body);
        assert!(
            raw.starts_with("HTTP/1.1 422"),
            "a <2 presentation must refuse 422 at the edge: {body} → {raw}"
        );
        let v = json(body_of(&raw));
        assert_eq!(v["code"].as_str(), Some("bad_field"), "{v}");
        assert!(
            v["error"].as_str().unwrap_or_default().contains("presented options"),
            "the refusal names the presentation law: {v}"
        );
    }
    // The noul exemption: an EMPTY noul presentation is the fixed
    // [false, true] rendering — legal at the edge (the lane renders
    // it; the multi-form echo gate answers it end to end).
    let raw = post_decide(
        port,
        r#"{"suite":"nope","state":"x","questions":[{"id":"q1","kind":"noul"}]}"#,
    );
    assert!(
        raw.starts_with("HTTP/1.1 404"),
        "the noul-empty presentation passes the shape check and reaches the suite lookup: {raw}"
    );
}

/// The multi-question form (the decision_wire law — one state, ALL
/// questions answered in one call) echoes the contract version at the
/// envelope AND on each decision element — the freeze covers both
/// answer shapes (reflex issue 074 T1).
#[test]
fn contract_version_echoes_on_the_multi_form() {
    if !data_present() {
        eprintln!("SKIP loud: datasets absent (the seat is the gate's vehicle)");
        return;
    }
    let port = shared_edge_port();
    let body = concat!(
        r#"{"suite":"sst5","state":"multi contract","contract_version":1,"questions":"#,
        r#"[{"id":"q1","kind":"choice","options":["a","b"]},{"id":"q2","kind":"noul"}]}"#
    );
    let raw = post_decide_until_ready(port, body);
    assert!(raw.starts_with("HTTP/1.1 200"), "{raw}");
    let v = json(body_of(&raw));
    assert_eq!(v["contract_version"].as_u64(), Some(1), "{v}");
    let elems = v["decisions"].as_array().expect("decisions array");
    assert_eq!(elems.len(), 2, "{v}");
    for d in elems {
        assert_eq!(d["contract_version"].as_u64(), Some(1), "{d}");
    }
}

// ── the R5 per-domain LRU policy (Plan 202; INSTINCT_LRU_CAPACITY) ────

/// The 2-suite lazy edge the LRU cycle gates against: two synthetic
/// suites ("lru-a", "lru-b") over the sst5 VEHICLE (the loader maps any
/// row's suite to the vehicle seat — the routing key is data, the R7
/// law), both rows lazy, stub backend, capacity 1 via the env knob the
/// edge reads LIVE at every lazy trigger. Own port (a second OnceLock
/// edge in this binary — install-once is per process, and the stub is
/// already the installed backend).
fn lru_edge_port() -> (u16, &'static str) {
    static PORT: OnceLock<(u16, &'static str)> = OnceLock::new();
    *PORT.get_or_init(|| {
        install_stub_once();
        let row = |suite: &str| {
            format!(
                "[[vessel]]\nsuite   = \"{suite}\"\ndigest  = \"blake3:{}\"\nclass   = \
                 \"hosted_only\"\nfile    = \"rerank_stub_head.bin\"\nposture = {{ arm = \"ENC\" \
                 }}\npin_keys = []\nbudget  = {{ load = \"lazy\", max_payload_mb = 16 }}\n",
                "0".repeat(64)
            )
        };
        let text = format!("{}{}", row("lru-a"), row("lru-b"));
        let digest = ArsenalManifest::digest_of(&text);
        let manifest = std::sync::Arc::new(ArsenalManifest::parse(&text).expect("manifest parses"));
        let probe = std::net::TcpListener::bind("127.0.0.1:0").expect("probe bind");
        let port = probe.local_addr().expect("probe addr").port();
        drop(probe);
        // The vehicle loader: every row seats through sst5 (the stub
        // never reads the seat — only the boot signature consumes it).
        fn vehicle_loader(
            lctx: &LoadCtx<'_>,
            suite: &'static str,
            _artifact: Option<&str>,
        ) -> Result<LoadedLane, String> {
            let seat = riir_instinct::serve_edge::prepare_lane_seat(lctx, "sst5")?;
            let row = lctx
                .manifest
                .row(suite)
                .ok_or_else(|| format!("suite {suite} is not in the arsenal manifest"))?;
            let digest = riir_instinct::serve_edge::lane_tag_digest(row, suite);
            let server = riir_instinct::server::AnySuiteServer::boot_bytes(
                suite,
                seat,
                b"rerank-stub-head-bytes",
                lctx.manifest,
            )?;
            Ok(LoadedLane { server, artifact_digest: digest, on_install: None })
        }
        let cfg = ServeConfig {
            bind: format!("127.0.0.1:{port}"),
            datasets_dir: datasets_dir().to_string_lossy().into_owned(),
            winners_dir: "unused-stub-loader-reads-no-artifact".to_string(),
            synth_corpus_dir: None,
            arsenal_path: None,
            suite_filter: None,
            cors_origin: Some(vec![]),
            prevalidated_manifest: Some((manifest, digest, "lru gate".into())),
            lane_loader: Some(vehicle_loader),
        };
        std::thread::Builder::new()
            .name("lru-edge".into())
            .spawn(move || {
                let _ = riir_instinct::serve_edge::run(cfg);
            })
            .expect("edge thread");
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if TcpStream::connect(("127.0.0.1", port)).is_ok() {
                break;
            }
            assert!(Instant::now() < deadline, "the lru edge never opened port {port}");
            std::thread::sleep(Duration::from_millis(50));
        }
        (port, "INSTINCT_LRU_CAPACITY")
    })
}

fn healthz_state(port: u16, suite: &str) -> String {
    // One GET /healthz exchange (the rerank gates' POST helper is
    // decide-shaped; this reads the suite states).
    let mut s = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    s.set_read_timeout(Some(Duration::from_secs(10))).ok();
    s.write_all(b"GET /healthz HTTP/1.1\r\nConnection: close\r\n\r\n")
        .expect("write");
    let mut raw = String::new();
    let _ = s.read_to_string(&mut raw);
    let body = raw.split_once("\r\n\r\n").map(|(_, b)| b).unwrap_or("");
    assert!(raw.starts_with("HTTP/1.1 200"), "{raw}");
    let v = json(body);
    v["suites"][suite]["state"]
        .as_str()
        .unwrap_or("absent")
        .to_string()
}

/// The R5 cycle over the wire: capacity 1, two lazy suites — a loads,
/// b's first decision EVICTS a (the LRU resident), a's next decision
/// re-triggers (503 window) and evicts b in turn. The eviction rides the
/// SAME release machinery (the tag kept, the reload installs
/// Idempotent) — the policy is a decision, not new machinery (the R5
/// law). Data-gated: the sst5 vehicle seats from the datasets.
#[test]
fn lru_capacity_evicts_the_least_recently_used_lane() {
    if !data_present() {
        eprintln!("SKIP loud: datasets absent (the seat is the gate's vehicle)");
        return;
    }
    let (port, cap_var) = lru_edge_port();
    // Set BEFORE the first lazy trigger — the edge reads the knob live,
    // but the FIRST decision is the cleanest observation point.
    std::env::set_var(cap_var, "1");

    // a loads (resident 0 → 1, nothing to evict).
    let raw = post_decide_until_ready(
        port,
        r#"{"suite":"lru-a","state":"a warm","options":["x","y"]}"#,
    );
    assert!(raw.starts_with("HTTP/1.1 200"), "a must load and answer: {raw}");
    assert_eq!(healthz_state(port, "lru-a"), "ready");

    // b's first decision evicts a (the only resident, a is LRU) then
    // loads — the 503 window covers the load, not the eviction.
    let raw = post_decide_until_ready(
        port,
        r#"{"suite":"lru-b","state":"b arrives","options":["x","y"]}"#,
    );
    assert!(raw.starts_with("HTTP/1.1 200"), "b must load and answer: {raw}");
    assert_eq!(
        healthz_state(port, "lru-a"),
        "unloaded",
        "the LRU resident (a) must be evicted for b"
    );
    assert_eq!(healthz_state(port, "lru-b"), "ready");

    // a's return: b is now the LRU resident — the cycle repeats. The
    // FIRST post is the 503 (re-trigger), the retry settles ready.
    let first = post_decide(
        port,
        r#"{"suite":"lru-a","state":"a returns","options":["x","y"]}"#,
    );
    assert!(
        first.starts_with("HTTP/1.1 503"),
        "the evicted lane must re-trigger the load window: {first}"
    );
    let raw = post_decide_until_ready(
        port,
        r#"{"suite":"lru-a","state":"a returns","options":["x","y"]}"#,
    );
    assert!(raw.starts_with("HTTP/1.1 200"), "{raw}");
    assert_eq!(
        healthz_state(port, "lru-b"),
        "unloaded",
        "b must now be the LRU eviction"
    );
    assert_eq!(healthz_state(port, "lru-a"), "ready");

    // Recency, not order, decides: touching b AGAIN (a second decision
    // while a is resident would evict... a is resident now — a decision
    // on b must NOT evict b itself and must evict a).
    let raw = post_decide_until_ready(
        port,
        r#"{"suite":"lru-b","state":"b returns","options":["x","y"]}"#,
    );
    assert!(raw.starts_with("HTTP/1.1 200"), "{raw}");
    assert_eq!(
        healthz_state(port, "lru-a"),
        "unloaded",
        "recency decides: a (older touch) must be the eviction, never the incoming b"
    );

    std::env::remove_var(cap_var);
}

/// The lazy posture itself: the FIRST decision on a lazy lane answers
/// 503 `loading` and TRIGGERS the load — the gate that the retry loop
/// above rides. Pinned directly so the rerank lane's load window stays
/// observable (the 503 is the client's retry signal, not an outage).
/// SEEN_OPTIONS is nonempty by the time this runs only if a sibling test
/// already loaded the lane — so the assertion is on the TRANSITION
/// (this test's first read is either the 503 or an already-ready 200),
/// and the lane must END ready either way.
#[test]
fn the_lazy_load_window_stays_observable() {
    if !data_present() {
        eprintln!("SKIP loud: datasets absent (the seat is the gate's vehicle)");
        return;
    }
    let port = shared_edge_port();
    let body = "{\"suite\":\"sst5\",\"state\":\"lazy trigger\",\"options\":[\"a\",\"b\"]}";
    let raw = post_decide(port, body);
    if raw.starts_with("HTTP/1.1 503") {
        let v = json(body_of(&raw));
        assert_eq!(v["code"].as_str(), Some("loading"), "the load window's code: {v}");
    } else {
        assert!(
            raw.starts_with("HTTP/1.1 200"),
            "either the 503 window or a ready answer: {raw}"
        );
    }
    let raw = post_decide_until_ready(port, body);
    assert!(raw.starts_with("HTTP/1.1 200"), "the lane must end ready: {raw}");
}
