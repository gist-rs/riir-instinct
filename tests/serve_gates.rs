//! The hosted serving gates (Plan 001 P5 / Issue 002; the arsenal
//! re-pin is Proposal 001 T2, law A6).
//!
//! Four faces:
//!
//! 1. **The manifest byte pin** — the embedded `arsenal.toml` is pinned
//!    byte-for-byte (BLAKE3): a TOML edit reds exactly like a code edit
//!    does (law A6 — the posture is DATA now, so the pin moved with it).
//! 2. **The posture pin** — the manifest's rows ARE the Bench-004 GOAT
//!    verdicts (the Issue-008 T1 re-baseline + T2 product gate), arm-by-arm
//!    against the frozen record. Runs everywhere.
//! 3. **The parity gate** — the served decision for committed test cases
//!    IS the frozen `predictions.json` pick for the same case (the serve
//!    path is the arena path, never a re-derivation). SKIPs loud when the
//!    t20k datasets / winner artifacts are absent (a bare clone) — a skip
//!    is a deferral, never a green.
//! 4. **The HTTP edge gates** — healthz shape, decide shapes, refusal
//!    codes, CORS. The dataless refusals (unknown suite, bad JSON,
//!    oversized body, CORS) run everywhere; the ready-lane shapes ride
//!    the data gate.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use riir_instinct::arsenal::{ArsenalManifest, ValidateCtx};
use riir_instinct::server::{Arm, AnySuiteServer};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn datasets_dir() -> PathBuf {
    repo_root().join("../riir-reflex/.raw/datasets_t20k")
}

fn winners_dir() -> PathBuf {
    repo_root().join("../riir-train/data/instinct_specialists")
}

fn predictions_path() -> PathBuf {
    repo_root().join(".benchmarks/004_rebaseline_current_reflex/predictions.json")
}

fn embedded_manifest() -> ArsenalManifest {
    ArsenalManifest::embedded_default().expect("the embedded arsenal manifest parses")
}

// ── face 1: the manifest byte pin (law A6) ─────────────────────────────

/// BLAKE3 over the embedded `arsenal.toml` bytes. Any edit to the
/// manifest — a posture tweak, a digest bump, a reordered row — changes
/// the digest and MUST re-pin here in the same change (with the GOAT
/// re-run + frozen-predictions parity update law A6 demands). This is
/// the TOML analogue of the compile-time posture table it replaced.
const PINNED_MANIFEST_DIGEST: &str =
    "blake3:57dc4f311ce43f0cf2b55c46dd4702dc4c75ccd7b7503b0ab703698b884b33f7";

#[test]
fn arsenal_manifest_bytes_are_pinned_byte_for_byte() {
    let actual = format!("blake3:{}", ArsenalManifest::embedded_manifest_digest());
    assert_eq!(
        actual, PINNED_MANIFEST_DIGEST,
        "the embedded arsenal.toml changed — re-pin PINNED_MANIFEST_DIGEST and carry the \
         GOAT re-run + frozen-predictions parity update law A6 demands"
    );
}

// ── face 2: the posture pin (the Bench-004 verdict, now as DATA) ─────

#[test]
fn manifest_posture_rows_are_the_rebaseline_goat_verdict() {
    let m = embedded_manifest();
    // The Bench-004 verdicts (Issue 008 T1's re-baseline at the CURRENT
    // published reflex posture + T2's strict-superiority gate): exactly
    // ONE hybrid arm certifies — massive's H2. The refusals are DATA
    // (each row's comment carries the paired LB95 that refused it).
    let expected: [(&str, Arm, &str); 6] = [
        ("ag_news", Arm::A0, "A0"),
        ("emotion", Arm::A0, "A0"),
        ("sst5", Arm::A0, "A0"),
        (
            "massive_intent_en",
            Arm::H2 { beta: 1.0, n_min: 2.0, tau_n: 8.0 },
            "H2(β=1,nmin=2,τ=8)",
        ),
        ("banking77", Arm::A0, "A0"),
        ("xnli_en", Arm::A0, "A0"),
    ];
    assert_eq!(m.rows().len(), 6, "the manifest carries exactly the six rows");
    for (suite, arm, name) in expected {
        let row = m
            .row(suite)
            .unwrap_or_else(|| panic!("{suite}: missing from the arsenal manifest"));
        let parsed = row
            .to_arm()
            .unwrap_or_else(|e| panic!("{suite}: posture refused: {e}"));
        assert_eq!(
            parsed, arm,
            "{suite}: the manifest posture drifted from the Bench-004 verdict"
        );
        assert_eq!(parsed.name(), name, "{suite}: arm display name drifted");
    }
    assert!(
        m.row("prompt_injections").is_none(),
        "a suite with no specialist gained a row"
    );
}

/// The deployment half of the manifest pin: validation against the real
/// artifact dirs. With the winner files present, this is the digest-drift
/// gate (a re-minted winner reds HERE, naming the row); without them the
/// file checks skip (the dataless posture) and the schema checks still
/// run — a skip is a deferral, never a green.
#[test]
fn arsenal_manifest_validates_against_the_deployment_dirs() {
    let m = embedded_manifest();
    let winners = winners_dir();
    let ctx = ValidateCtx::raw(&winners);
    m.validate(&ctx)
        .unwrap_or_else(|e| panic!("arsenal validation refused: {e}"));
}

// ── face 2: the parity gate (data-gated, skip loud) ──────────────────

fn data_present() -> bool {
    datasets_dir().join("ag_news").is_dir() && winners_dir().join("ag_news_winner_v1.bin").is_file()
}

/// Boot one suite's server — on a big-stack thread: the seat boot's stack
/// frames exceed a test thread's 2 MiB default (the serve binary gives
/// its loader threads the same headroom).
fn boot_suite(suite: &'static str) -> Result<riir_instinct::server::AnySuiteServer, String> {
    let datasets = datasets_dir();
    let winners = winners_dir();
    let manifest = embedded_manifest();
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || AnySuiteServer::boot(suite, &datasets, &winners, &manifest))
        .expect("spawn boot thread")
        .join()
        .expect("boot thread panicked")
}

/// The frozen predictions record: suite → (registered arm name, picks).
/// The frozen predictions record: suite → (registered arm name, picks,
/// the registered arm's per-case abstention flags — the SERVE's
/// first-class abstention contract, recorded since Bench 004; an older
/// record without the field reads as all-answered).
fn frozen_picks(suite: &str) -> Option<(String, Vec<usize>, Vec<bool>)> {
    let doc: serde_json::Value =
        serde_json::from_reader(std::fs::File::open(predictions_path()).ok()?).ok()?;
    for s in doc["frozen_test_predictions"].as_array()? {
        if s["suite"].as_str()? == suite {
            let registered = s["registered"].as_str()?.to_string();
            for arm in s["arms"].as_array()? {
                if arm["name"].as_str()? == registered {
                    let abstained = arm["abstained"]
                        .as_array()
                        .map(|a| a.iter().map(|b| b.as_bool().unwrap_or(false)).collect())
                        .unwrap_or_else(|| {
                            let n = arm["picks"].as_array().map(|p| p.len()).unwrap_or(0);
                            vec![false; n]
                        });
                    return Some((
                        registered,
                        arm["picks"]
                            .as_array()?
                            .iter()
                            .map(|p| p.as_u64().unwrap() as usize)
                            .collect(),
                        abstained,
                    ));
                }
            }
        }
    }
    None
}

/// The presented option keys of a case, in the case's own order — the
/// exact iteration `engine_request` feeds the engine (the bridge's
/// position space).
fn case_option_keys(case: &riir_reflex::harness::suites::SuiteCase) -> Vec<String> {
    let q = &case.questions[0];
    if let Some(obj) = q.criteria.as_object() {
        obj.keys().cloned().collect()
    } else if let Some(levels) = q.criteria.as_array() {
        levels
            .iter()
            .map(|v| match v {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .collect()
    } else {
        panic!("case {}: criteria must be object or array", case.id);
    }
}

#[test]
fn served_decisions_are_the_frozen_goat_picks() {
    if !data_present() {
        eprintln!(
            "SKIP loud: t20k datasets / winner artifacts absent (bare clone) — \
             the parity gate needs the bytes the published lanes carry"
        );
        return;
    }
    let seat = riir_reflex::harness::runner::seat::prepare_seat("ag_news", &datasets_dir())
        .expect("prepare ag_news seat");
    let mut server = boot_suite("ag_news").expect("boot ag_news server");
    let (registered, picks, abstained) = frozen_picks("ag_news").expect("frozen ag_news record");
    assert_eq!(
        registered,
        server.meta().arm.name(),
        "the boot arm must be the frozen registered arm"
    );

    // The FIRST 16 test cases, in seat order — the same order the frozen
    // run evaluated.
    let n = 16.min(seat.suite.cases.len()).min(picks.len());
    assert!(n >= 8, "parity sample too small: {n}");
    for (ci, case) in seat.suite.cases.iter().take(n).enumerate() {
        let options = case_option_keys(case);
        let d = server
            .decide(&seat.state_strs[ci], Some(&options))
            .unwrap_or_else(|e| panic!("case {ci}: decide failed: {e}"));
        // The abstention contract FIRST: A0's record follows reflex's
        // HARD convention (abstains forced to their argmax pick), while
        // the serve honors the fused gate's abstention — so an abstained
        // case's recorded pick is not a servable answer, and the serve
        // must abstain exactly where the record says it does.
        assert_eq!(
            d.abstained, abstained[ci],
            "case {ci}: served abstention drifted from the frozen record"
        );
        if !d.abstained {
            assert_eq!(
                d.pick_index,
                Some(picks[ci]),
                "case {ci}: served pick drifted from the frozen pick"
            );
        }
        assert!(d.us < 100_000, "case {ci}: decision took {} µs — outside the modelless tier", d.us);
    }
}

/// The massive sentinel face: the seat's t20k test split lacks
/// `cooking_query` (the artifact carries 60 intents, the seat offers 59);
/// a request PRESENTING it must still be scorable by the specialist (the
/// NaN-no-evidence mark) — never a bridge refusal at boot time, and the
/// default presentation (the seat universe) matches the frozen picks.
#[test]
fn massive_artifact_known_seat_unknown_option_stays_scorable() {
    if !data_present() {
        eprintln!("SKIP loud: datasets/winners absent");
        return;
    }
    let seat = riir_reflex::harness::runner::seat::prepare_seat(
        "massive_intent_en",
        &datasets_dir(),
    )
    .expect("prepare massive seat");
    let mut server = boot_suite("massive_intent_en").expect("boot massive server");
    let (registered, picks, _abstained) =
        frozen_picks("massive_intent_en").expect("frozen massive record");
    assert_eq!(registered, server.meta().arm.name());
    assert_eq!(
        server.meta().artifact_labels,
        60,
        "the artifact's 60-intent universe is the sentinel's source"
    );

    // Default presentation parity on the first 8 test cases.
    for (ci, case) in seat.suite.cases.iter().take(8).enumerate() {
        let options = case_option_keys(case);
        let d = server
            .decide(&seat.state_strs[ci], Some(&options))
            .unwrap_or_else(|e| panic!("case {ci}: {e}"));
        assert_eq!(d.pick_index, Some(picks[ci]), "case {ci}");
    }

    // The seat-unknown label is absent from the DEFAULT universe but
    // present in the key map (sentinel) — a cooked_query (sic, the
    // artifact's spelling) presentation answers instead of refusing.
    let seat_has_it = seat.labels.iter().any(|l| l == "cooking_query");
    let artifact_labels: Vec<String> = {
        // the winner artifact's label list, read through the loader
        let bytes = std::fs::read(
            winners_dir().join("massive_intent_en_winner_v1.bin"),
        )
        .expect("winner bytes");
        riir_instinct::specialist::decode_artifact(&bytes)
            .expect("decode")
            .labels
    };
    assert!(artifact_labels.iter().any(|l| l == "cooking_query"));
    assert!(!seat_has_it, "the t20k seat unexpectedly gained cooking_query — update the sentinel face");

    let case = &seat.suite.cases[0];
    let mut options = case_option_keys(case);
    options.truncate(59.min(options.len()));
    options.push("cooking_query".to_string());
    let d = server
        .decide(&seat.state_strs[0], Some(&options))
        .expect("sentinel presentation must be scorable");
    assert!(d.pick_index.is_some());
}

/// The bridge refusal: options that neither name seat labels nor match
/// the label count are REFUSED loud (the Issue-006 rule).
#[test]
fn undefined_bridge_refuses_loud() {
    if !data_present() {
        eprintln!("SKIP loud: datasets/winners absent");
        return;
    }
    let mut server = boot_suite("ag_news").expect("boot");
    let err = server
        .decide("hello world", Some(&["Nope".into(), "Also Nope".into()]))
        .unwrap_err();
    assert!(err.contains("the specialist bridge is undefined"), "{err}");
}

// ── face 3: the HTTP edge gates ──────────────────────────────────────

struct ServerProc {
    child: Child,
    port: u16,
}

impl Drop for ServerProc {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn spawn_server() -> ServerProc {
    let exe = assert_cmd_env("CARGO_BIN_EXE_serve");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("probe bind");
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let mut cmd = Command::new(&exe);
    cmd.arg("--bind").arg(format!("127.0.0.1:{port}"));
    cmd.arg("--suites").arg("ag_news");
    // The CORS arm needs an allow-list.
    cmd.env("RIIR_INSTINCT_ALLOWED_ORIGIN", "https://reflex.gist.rs");
    let child = cmd
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn serve");
    ServerProc { child, port }
}

fn assert_cmd_env(var: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| {
        // Fall back to the target dir's release/debug profile layout.
        let p = repo_root().join("target/debug/serve");
        p.to_string_lossy().into_owned()
    })
}

fn http(
    port: u16,
    req: &str,
    body: Option<&str>,
) -> (u16, String) {
    let mut s = None;
    for _ in 0..40 {
        match TcpStream::connect(("127.0.0.1", port)) {
            Ok(c) => {
                s = Some(c);
                break;
            }
            Err(_) => std::thread::sleep(std::time::Duration::from_millis(50)),
        }
    }
    let mut s = s.expect("connect: the server never accepted");
    s.set_read_timeout(Some(std::time::Duration::from_secs(5))).unwrap();
    let head = match body {
        Some(b) => format!(
            "{req}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{b}",
            b.len()
        ),
        None => format!("{req}\r\nConnection: close\r\n\r\n"),
    };
    s.write_all(head.as_bytes()).unwrap();
    let mut reader = BufReader::new(s);
    let mut status_line = String::new();
    reader.read_line(&mut status_line).unwrap();
    let status = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|c| c.parse::<u16>().ok())
        .unwrap_or(0);
    // Strip the response headers — the body only (the raw assertions in
    // the CORS face read the socket directly).
    let mut rest = String::new();
    let _ = reader.read_to_string(&mut rest);
    let body = rest
        .split_once("\r\n\r\n")
        .map(|(_, b)| b.to_string())
        .unwrap_or(rest);
    (status, body)
}

#[test]
fn http_edge_refusal_and_healthz_faces() {
    let srv = spawn_server();
    // Wait for the listener (the process binds before the loaders run).
    let mut ready = false;
    for _ in 0..100 {
        if TcpStream::connect(("127.0.0.1", srv.port)).is_ok() {
            ready = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    assert!(ready, "the server never bound");

    // 404 + code for an unknown suite.
    let (status, body) = http(
        srv.port,
        "POST /decide HTTP/1.1",
        Some(r#"{"suite":"nope","state":"x"}"#),
    );
    assert_eq!(status, 404, "{body}");
    assert!(body.contains("\"code\":\"unknown_suite\""), "{body}");

    // 400 + code for bad JSON.
    let (status, body) = http(srv.port, "POST /decide HTTP/1.1", Some("{not json"));
    assert_eq!(status, 400, "{body}");
    assert!(body.contains("\"code\":\"bad_json\""), "{body}");

    // 413 for an oversized body.
    let big = format!(r#"{{"suite":"ag_news","state":"{}"}}"#, "x".repeat(1024 * 1024 + 64));
    let (status, body) = http(srv.port, "POST /decide HTTP/1.1", Some(&big));
    assert_eq!(status, 413, "{body}");
    assert!(body.contains("\"code\":\"too_large\""), "{body}");

    // 405 for the wrong method.
    let (status, body) = http(srv.port, "GET /decide HTTP/1.1", None);
    assert_eq!(status, 405, "{body}");
    assert!(body.contains("\"code\":\"method_not_allowed\""), "{body}");

    // healthz: 200 + the build stamp + the suite slot.
    let (status, body) = http(srv.port, "GET /healthz HTTP/1.1", None);
    assert_eq!(status, 200, "{body}");
    assert!(body.contains("\"service\":\"riir-instinct\""), "{body}");
    assert!(body.contains("\"build\""), "{body}");
    assert!(body.contains("ag_news"), "{body}");
    assert!(
        body.contains("\"loading\"") || body.contains("\"ready\"") || body.contains("\"failed\""),
        "{body}"
    );

    // CORS: allow-listed origin echoes; a foreign one is refused.
    let mut s = TcpStream::connect(("127.0.0.1", srv.port)).unwrap();
    s.write_all(
        b"OPTIONS /decide HTTP/1.1\r\nOrigin: https://reflex.gist.rs\r\nConnection: close\r\n\r\n",
    )
    .unwrap();
    let mut resp = String::new();
    let _ = BufReader::new(s).read_to_string(&mut resp);
    assert!(resp.contains("204 No Content"), "{resp}");
    assert!(resp.contains("Access-Control-Allow-Origin: https://reflex.gist.rs"), "{resp}");

    let (status, _) = http(
        srv.port,
        "OPTIONS /decide HTTP/1.1\r\nOrigin: https://evil.example",
        None,
    );
    assert_eq!(status, 403, "a foreign origin must not receive a preflight grant");
}

/// The decide happy path over HTTP — needs the DATA (the lane must be
/// ready), so it waits for readiness and skips loud without it.
#[test]
fn http_decide_happy_path_with_data() {
    if !data_present() {
        eprintln!("SKIP loud: datasets/winners absent");
        return;
    }
    let srv = spawn_server();
    // Wait for the ag_news lane to become READY (the seat boot takes
    // seconds — plus the Bench-004 ridge ladders, ~10 s for ag_news and
    // ~65 s for banking77 in the other loader threads; 420 s ceiling — a
    // boot slower than that is a finding, contention included).
    let mut ready = false;
    for _ in 0..840 {
        let (status, body) = http(srv.port, "GET /healthz HTTP/1.1", None);
        if status == 200 && body.contains("ag_news\":{\"state\":\"ready\"") {
            ready = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    assert!(ready, "the ag_news lane never reached ready within 420s");

    // The decide state comes from the FROZEN record — the first test case
    // the arena measured the served arm answering (not an invented
    // sentence, whose gate outcome no record pins). The parity gate
    // covers the abstention contract case by case; this one pins the
    // ANSWERED shape end to end over HTTP.
    let seat = riir_reflex::harness::runner::seat::prepare_seat("ag_news", &datasets_dir())
        .expect("prepare ag_news seat");
    let (_, _, abstained) = frozen_picks("ag_news").expect("frozen ag_news record");
    let answered = abstained
        .iter()
        .position(|a| !a)
        .expect("the frozen record has at least one answered ag_news case");
    let case = &seat.suite.cases[answered];
    let q = &case.questions[0];
    let state = seat.state_strs[answered].clone();
    let options: Vec<String> = if let Some(obj) = q.criteria.as_object() {
        obj.keys().cloned().collect()
    } else {
        panic!("ag_news criteria must be an object");
    };
    let req = serde_json::json!({ "suite": "ag_news", "state": state, "options": options });
    let req_body = req.to_string();
    let (status, body) = http(
        srv.port,
        "POST /decide HTTP/1.1",
        Some(&req_body),
    );
    assert_eq!(status, 200, "{body}");
    let doc: serde_json::Value = serde_json::from_str(&body).expect("decide body parses");
    assert_eq!(doc["lane"], "hybrid");
    assert_eq!(doc["suite"], "ag_news");
    // ag_news serves A0 (the Bench-004 T2 refusal) — the modelless lane
    // answers: the response carries the full-arity PROBABILITIES (a real
    // distribution over the presented options) and NO specialist scores
    // (the specialist is never consulted).
    assert_eq!(doc["arm"], "A0", "{body}");
    assert_eq!(doc["abstained"], false, "{body}");
    assert!(doc["pick"].is_string(), "{body}");
    let probs = doc["probabilities"].as_array().expect("A0 answers with probabilities");
    assert!(probs.len() == 4, "{body}");
    assert!(
        probs.iter().all(|p| p.is_f64()),
        "A0's probabilities must be numbers: {body}"
    );
    assert!(doc["specialist_scores"].is_null(), "{body}");
    assert_eq!(doc["escalated"], false, "{body}");
    let receipt = &doc["receipt"];
    assert!(receipt["build"].as_str().unwrap().len() == 16, "{body}");
    assert!(receipt["input"].as_str().unwrap().len() == 64, "{body}");
    assert!(receipt["decision"].as_str().unwrap().len() == 64, "{body}");
    assert!(doc["us"].as_u64().is_some(), "{body}");
}

// ── face 4: the arsenal budget + swap gates (Proposal 001 T5+T6) ─────

/// The HTTP client with a tunable read timeout (the swap edge loads a
/// lane — seconds-class — before answering; the 5 s decide timeout would
/// cut the response off).
fn http_to(port: u16, req: &str, body: Option<&str>, timeout: u64) -> (u16, String) {
    let mut s = None;
    for _ in 0..40 {
        match TcpStream::connect(("127.0.0.1", port)) {
            Ok(c) => {
                s = Some(c);
                break;
            }
            Err(_) => std::thread::sleep(std::time::Duration::from_millis(50)),
        }
    }
    let mut s = s.expect("connect: the server never accepted");
    s.set_read_timeout(Some(std::time::Duration::from_secs(timeout))).unwrap();
    let head = match body {
        Some(b) => format!(
            "{req}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{b}",
            b.len()
        ),
        None => format!("{req}\r\nConnection: close\r\n\r\n"),
    };
    s.write_all(head.as_bytes()).unwrap();
    let mut reader = BufReader::new(s);
    let mut status_line = String::new();
    reader.read_line(&mut status_line).unwrap();
    let status = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|c| c.parse::<u16>().ok())
        .unwrap_or(0);
    let mut rest = String::new();
    let _ = reader.read_to_string(&mut rest);
    let body = rest
        .split_once("\r\n\r\n")
        .map(|(_, b)| b.to_string())
        .unwrap_or(rest);
    (status, body)
}

fn spawn_server_cfg(args: &[&str], envs: &[(&str, &str)]) -> ServerProc {
    let exe = assert_cmd_env("CARGO_BIN_EXE_serve");
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("probe bind");
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let mut cmd = Command::new(&exe);
    cmd.arg("--bind").arg(format!("127.0.0.1:{port}"));
    for a in args {
        cmd.arg(a);
    }
    for (k, v) in envs {
        cmd.env(k, v);
    }
    let child = cmd
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn serve");
    ServerProc { child, port }
}

/// Wait for the listener, then for one suite's lane to reach a healthz
/// state (the boot/lazy-load window). Returns the LAST healthz body on
/// failure — the diagnostic the assertion shows.
fn wait_suite_state(port: u16, suite: &str, want: &str, secs: u64) -> Result<(), String> {
    let needle = format!(r#""{suite}":{{"state":"{want}""#);
    let mut last = String::new();
    for _ in 0..(secs * 2) {
        let (status, body) = http_to(port, "GET /healthz HTTP/1.1", None, 10);
        last = format!("{status} {body}");
        if status == 200 && body.contains(&needle) {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    Err(last)
}

/// The ag_news row of the embedded manifest, flipped to the lazy posture
/// (the first `budget.load` in the file is ag_news's row).
fn lazy_ag_news_manifest() -> String {
    riir_instinct::arsenal::EMBEDDED_MANIFEST.replacen(
        "budget  = { load = \"eager\", max_payload_mb = 16 }",
        "budget  = { load = \"lazy\", max_payload_mb = 16 }",
        1,
    )
}

#[test]
fn arsenal_release_refuses_the_eager_posture() {
    // Data-independent: the refusal is manifest-level, before any lane
    // state matters.
    let srv = spawn_server_cfg(&["--suites", "ag_news"], &[]);
    assert!(
        wait_bind(srv.port),
        "the server never bound"
    );
    let (status, body) = http_to(
        srv.port,
        "POST /arsenal/release HTTP/1.1",
        Some(r#"{"suite":"ag_news"}"#),
        10,
    );
    assert_eq!(status, 409, "{body}");
    assert!(body.contains("\"code\":\"not_lazy\""), "{body}");
}

fn wait_bind(port: u16) -> bool {
    for _ in 0..100 {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    false
}

/// The lazy cycle (T5): boot NOT loaded → first decision triggers the
/// load (503 window) → ready → release evicts → re-decision re-triggers.
#[test]
fn arsenal_lazy_release_reload_cycle_over_http() {
    if !data_present() {
        eprintln!("SKIP loud: datasets/winners absent");
        return;
    }
    let manifest = lazy_ag_news_manifest();
    assert!(manifest.contains("load = \"lazy\""), "the surgery must flip ag_news's row");
    let dir = std::env::temp_dir();
    let path = dir.join(format!("instinct_arsenal_lazy_{}.toml", std::process::id()));
    std::fs::write(&path, &manifest).expect("write lazy manifest");
    let manifest_arg = path.to_string_lossy().into_owned();
    let srv = spawn_server_cfg(
        &["--suites", "ag_news"],
        &[("INSTINCT_ARSENAL", manifest_arg.as_str())],
    );
    assert!(wait_bind(srv.port), "the server never bound");

    // Boot did NOT load the lazy row.
    wait_suite_state(srv.port, "ag_news", "unloaded", 10)
        .unwrap_or_else(|b| panic!("the lazy row must sit unloaded at boot; last healthz: {b}"));

    // The first decision TRIGGERS the load and answers 503.
    let (status, body) = http_to(
        srv.port,
        "POST /decide HTTP/1.1",
        Some(r#"{"suite":"ag_news","state":"Apple unveils a new M-series chip"}"#),
        30,
    );
    assert_eq!(status, 503, "{body}");
    assert!(body.contains("\"code\":\"loading\""), "{body}");

    // The window closes; the lane serves (the lazy load re-derives the
    // published posture incl. the ag_news ridge ladder — 420 s ceiling).
    wait_suite_state(srv.port, "ag_news", "ready", 420)
        .unwrap_or_else(|b| panic!("the lazy load never completed; last healthz: {b}"));
    let (status, body) = http_to(
        srv.port,
        "POST /decide HTTP/1.1",
        Some(r#"{"suite":"ag_news","state":"Apple unveils a new M-series chip"}"#),
        30,
    );
    assert_eq!(status, 200, "{body}");

    // Release evicts; the tag survives; the next decision re-triggers.
    let (status, body) = http_to(
        srv.port,
        "POST /arsenal/release HTTP/1.1",
        Some(r#"{"suite":"ag_news"}"#),
        10,
    );
    assert_eq!(status, 200, "{body}");
    assert!(body.contains("\"status\":\"released\""), "{body}");
    wait_suite_state(srv.port, "ag_news", "unloaded", 10)
        .unwrap_or_else(|b| panic!("the released row must read unloaded; last healthz: {b}"));
    let (status, _) = http_to(
        srv.port,
        "POST /decide HTTP/1.1",
        Some(r#"{"suite":"ag_news","state":"Apple unveils a new M-series chip"}"#),
        30,
    );
    assert_eq!(status, 503, "the re-decision must re-trigger the lazy load");
    wait_suite_state(srv.port, "ag_news", "ready", 420)
        .unwrap_or_else(|b| panic!("the reload never completed; last healthz: {b}"));
    let (status, _) = http_to(
        srv.port,
        "POST /decide HTTP/1.1",
        Some(r#"{"suite":"ag_news","state":"Apple unveils a new M-series chip"}"#),
        30,
    );
    assert_eq!(status, 200);
    let _ = std::fs::remove_file(&path);
}

/// The swap gate (T6) over the wire: idempotent no-op / fork refused /
/// advance installed / downgrade refused — the epoch-tag contract at the
/// HTTP edge.
#[test]
fn arsenal_swap_monotonic_gate_over_http() {
    if !data_present() || !winners_dir().join("emotion_winner_v1.bin").is_file() {
        eprintln!("SKIP loud: datasets/winner artifacts absent");
        return;
    }
    let srv = spawn_server_cfg(&["--suites", "ag_news"], &[]);
    assert!(wait_bind(srv.port), "the server never bound");
    wait_suite_state(srv.port, "ag_news", "ready", 420)
        .unwrap_or_else(|b| panic!("the ag_news lane never reached ready; last healthz: {b}"));

    // Idempotent no-op: the boot artifact at epoch 0 IS the applied tag —
    // answered without any reload.
    let (status, body) = http_to(
        srv.port,
        "POST /arsenal/swap HTTP/1.1",
        Some(r#"{"suite":"ag_news","artifact":"ag_news_winner_v1.bin","epoch":0}"#),
        30,
    );
    assert_eq!(status, 200, "{body}");
    assert!(body.contains("\"status\":\"noop\""), "{body}");

    // Fork: epoch 0 with a DIFFERENT artifact's bytes — refused before
    // any load.
    let (status, body) = http_to(
        srv.port,
        "POST /arsenal/swap HTTP/1.1",
        Some(r#"{"suite":"ag_news","artifact":"emotion_winner_v1.bin","epoch":0}"#),
        30,
    );
    assert_eq!(status, 409, "{body}");
    assert!(body.contains("\"code\":\"fork\""), "{body}");

    // Advance: epoch 1, same artifact — the full load runs (the ridge
    // ladder included), then the atomic install.
    let (status, body) = http_to(
        srv.port,
        "POST /arsenal/swap HTTP/1.1",
        Some(r#"{"suite":"ag_news","artifact":"ag_news_winner_v1.bin","epoch":1}"#),
        600,
    );
    assert_eq!(status, 200, "{body}");
    assert!(body.contains("\"status\":\"advanced\""), "{body}");
    assert!(body.contains("\"epoch\":1"), "{body}");
    // healthz discloses the advanced tag; the lane still serves.
    let (_, body) = http_to(srv.port, "GET /healthz HTTP/1.1", None, 10);
    assert!(body.contains("ag_news\":{\"state\":\"ready\""), "{body}");
    let (status, body) = http_to(
        srv.port,
        "POST /decide HTTP/1.1",
        Some(r#"{"suite":"ag_news","state":"Apple unveils a new M-series chip"}"#),
        30,
    );
    assert_eq!(status, 200, "{body}");

    // Downgrade: epoch 0 is now behind the applied 1 — refused.
    let (status, body) = http_to(
        srv.port,
        "POST /arsenal/swap HTTP/1.1",
        Some(r#"{"suite":"ag_news","artifact":"ag_news_winner_v1.bin","epoch":0}"#),
        30,
    );
    assert_eq!(status, 409, "{body}");
    assert!(body.contains("\"code\":\"downgrade\""), "{body}");
}
