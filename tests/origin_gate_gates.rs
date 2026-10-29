//! The origin-gate seam's wire gates (rethink issue 037 T1) — the edge
//! consults `ServeConfig::origin_gate` after the head parses and before
//! any route arm. Native, data-free: the gate runs against the embedded
//! teaching manifest (artifact-less A0 rows — no datasets, no winners),
//! so the whole battery runs on any box.
//!
//! What is pinned here:
//! 1. the gate REFUSES POST /decide without / with a wrong bearer —
//!    `401` + the gate's stable code, and the refusal body NEVER echoes
//!    the expected secret (the leak the gate exists to prevent);
//! 2. the gate PASSES POST /decide carrying the right bearer (the
//!    request reaches the route arms — the answer is whatever the lane
//!    plane says, never the gate's refusal);
//! 3. the gate passes GET /healthz and OPTIONS preflights (the gate
//!    decides which method+path it covers — the edge calls it on
//!    everything);
//! 4. the `None` config = ungated: POST /decide without a bearer
//!    reaches the lane plane and answers the UNGATED shape (no 401).

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

use riir_instinct::serve_edge::{GateRefusal, GateReq, OriginGateFn, ServeConfig};

const SECRET: &str = "origin-gate-test-secret-0192837465fafafafa";

/// The test gate: covers POST /decide ONLY; requires
/// `Authorization: Bearer <SECRET>` (constant-time compare, the
/// `mint_token_matches` law); the refusal message never names the
/// secret. Everything else passes — healthz, OPTIONS, unknown paths
/// (the 404 arm still owns those, by design).
fn test_gate(r: &GateReq<'_>) -> Option<GateRefusal> {
    if r.method != "POST" || r.path != "/decide" {
        return None;
    }
    let provided = r
        .authorization
        .and_then(|h| h.strip_prefix("Bearer "))
        .unwrap_or("");
    if !constant_time_eq(provided, SECRET) {
        return Some(GateRefusal {
            status: "401 Unauthorized",
            code: "unauthorized",
            message: "origin bearer required".into(),
        });
    }
    None
}

/// The `mint_token_matches` law (dapps kat-service routes.rs): digest
/// both sides, XOR-fold — never an early-exit byte compare on the raw
/// values.
fn constant_time_eq(a: &str, b: &str) -> bool {
    let dig = |s: &str| *blake3::hash(s.as_bytes()).as_bytes();
    let (x, y) = (dig(a), dig(b));
    let mut diff = 0u8;
    for (p, q) in x.iter().zip(y.iter()) {
        diff |= p ^ q;
    }
    diff == 0
}

fn boot_with_gate(gate: Option<OriginGateFn>) -> u16 {
    // The ephemeral-port probe (the ext_seat_rerank_gates shape): bind
    // :0, learn the port, drop, and hand the port to the edge — the
    // connect-retry below absorbs the tiny race.
    let probe = std::net::TcpListener::bind("127.0.0.1:0").expect("probe bind");
    let port = probe.local_addr().expect("probe addr").port();
    drop(probe);
    let cfg = ServeConfig {
        bind: format!("127.0.0.1:{port}"),
        // The gate battery reads no lane data: the teaching manifest is
        // artifact-less A0 rows, and the probe arms below never reach
        // the lane plane with a valid envelope.
        datasets_dir: "unused-gate-gates-read-no-data".into(),
        winners_dir: "unused-gate-gates-read-no-data".into(),
        synth_corpus_dir: None,
        arsenal_path: None,
        suite_filter: None,
        cors_origin: Some(vec![]),
        prevalidated_manifest: None,
        lane_loader: None,
        origin_gate: gate,
        on_decision: None,
    };
    std::thread::Builder::new()
        .name("origin-gate-edge".into())
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
            "the gated edge never came up on port {port}"
        );
        std::thread::sleep(Duration::from_millis(50));
    }
    port
}

/// One request, whole response read back (Connection: close — the edge
/// closes after every answer).
fn roundtrip(port: u16, req: &str) -> String {
    let mut s = TcpStream::connect(("127.0.0.1", port)).expect("connect");
    s.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
    s.write_all(req.as_bytes()).expect("write");
    let mut buf = Vec::new();
    s.read_to_end(&mut buf).expect("read");
    String::from_utf8_lossy(&buf).into_owned()
}

fn decide_request(authorization: Option<&str>) -> String {
    let auth = authorization
        .map(|a| format!("Authorization: {a}\r\n"))
        .unwrap_or_default();
    format!(
        "POST /decide HTTP/1.1\r\nHost: 127.0.0.1\r\n{auth}Content-Type: application/json\r\nContent-Length: 2\r\n\r\n{{}}"
    )
}

#[test]
fn the_gate_refuses_decide_without_or_with_a_wrong_bearer_and_never_echoes_the_secret() {
    let port = boot_with_gate(Some(test_gate));
    for auth in [None, Some("Bearer wrong"), Some("wrong-scheme")] {
        let resp = roundtrip(port, &decide_request(auth));
        assert!(
            resp.starts_with("HTTP/1.1 401 "),
            "auth {auth:?}: expected the gate's 401, got: {resp}"
        );
        assert!(
            resp.contains("\"code\":\"unauthorized\""),
            "auth {auth:?}: the stable code must ride the body: {resp}"
        );
        assert!(
            !resp.contains(SECRET),
            "auth {auth:?}: the refusal echoed the expected secret — the leak the gate exists to prevent: {resp}"
        );
    }
}

#[test]
fn the_gate_passes_decide_with_the_right_bearer_and_passes_everything_it_does_not_cover() {
    let port = boot_with_gate(Some(test_gate));
    // Right bearer: the gate passes; the lane plane answers (the empty
    // body is whatever the lane resolution says — never the gate's 401).
    let resp = roundtrip(port, &decide_request(Some(&format!("Bearer {SECRET}"))));
    assert!(
        !resp.starts_with("HTTP/1.1 401 "),
        "the right bearer was refused by the gate: {resp}"
    );
    // healthz: not covered — 200 with the edge's own shape.
    let resp = roundtrip(port, "GET /healthz HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n");
    assert!(
        resp.starts_with("HTTP/1.1 200 "),
        "healthz must pass the gate: {resp}"
    );
    // OPTIONS preflight with an allow-listed CORS echo: the allow list
    // is empty here so the preflight 403s at the CORS arm — either way
    // it must NOT read as the gate's 401 (the gate passed it).
    let resp = roundtrip(
        port,
        "OPTIONS /decide HTTP/1.1\r\nHost: 127.0.0.1\r\nOrigin: https://rethink.gist.rs\r\nAccess-Control-Request-Method: POST\r\n\r\n",
    );
    assert!(
        !resp.starts_with("HTTP/1.1 401 "),
        "OPTIONS must pass the gate (the gate covers POST /decide only): {resp}"
    );
}

#[test]
fn the_none_config_is_ungated_and_healthz_never_names_the_secret() {
    let port = boot_with_gate(None);
    // No gate: a bearer-less /decide reaches the lane plane — the
    // answer is the edge's own (4xx lane-plane shape), NEVER a 401
    // bearer refusal, and never the secret.
    let resp = roundtrip(port, &decide_request(None));
    assert!(
        !resp.contains("\"code\":\"unauthorized\""),
        "the ungated posture invented a bearer refusal: {resp}"
    );
    let resp = roundtrip(port, "GET /healthz HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n");
    assert!(resp.starts_with("HTTP/1.1 200 "), "healthz: {resp}");
    assert!(
        !resp.contains(SECRET),
        "healthz leaked the gate secret: {resp}"
    );
}
