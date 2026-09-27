//! The hosted instinct decision server (Plan 001 P5 / Issue 002 — the
//! servable binary the cf-container target ships). Answers classification
//! decisions through the hybrid lane over a small std-only HTTP edge:
//!
//! - `GET  /healthz` — liveness + per-suite readiness (lanes load in
//!   background threads; the listener binds FIRST so the container
//!   HEALTHCHECK sees a live process during the seat boot).
//! - `POST /decide` — `{"suite", "state", "options"?}` → the served
//!   decision + the verifiable receipt (Proposal 014 §4: build
//!   fingerprint + feature set + BLAKE3(input) + lane id + decision; a
//!   client running the SAME release can re-derive and check).
//! - `GET  /` — the endpoint index.
//!
//! Lane readiness mirrors reflex's own serve postures (`loading` /
//! `ready` / `failed`): a loading or failed lane answers 503 with the
//! state named — never a silent modelless fallback, never a fabricated
//! answer. Every refusal carries a machine-readable `code`.
//!
//! The edge borrows reflex's serve SHAPE (borrow-pattern-not-dep) rather
//! than its module, which carries the game-head + laya machinery this
//! lane must not drag in.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use riir_instinct::server::{AnySuiteServer, REGISTERED_SUITES};

include!(concat!(env!("OUT_DIR"), "/build_stamp.rs"));

const MAX_BODY: usize = 1024 * 1024;
/// Error paths that do not consume the announced body drain up to this
/// many bytes before responding — closing on an unread body sends RST
/// and the client loses the refusal (the response it needs most).
const DRAIN_CAP: usize = 4 * 1024 * 1024;
/// The seat boot allocates big stack frames (the arena boots on the main
/// thread; loader threads need real headroom).
const LOADER_STACK: usize = 64 * 1024 * 1024;
const CONN_STACK: usize = 8 * 1024 * 1024;
const READ_TIMEOUT: Duration = Duration::from_secs(10);
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut datasets_dir = std::env::var("INSTINCT_DATASETS_DIR")
        .unwrap_or_else(|_| "../riir-reflex/.raw/datasets_t20k".into());
    let mut winners_dir = std::env::var("INSTINCT_WINNERS_DIR")
        .unwrap_or_else(|_| "../riir-train/data/instinct_specialists".into());
    let mut bind = std::env::var("INSTINCT_BIND").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let mut suites: Vec<&'static str> = REGISTERED_SUITES.to_vec();
    let mut i = 1;
    let args: Vec<String> = std::env::args().collect();
    while i < args.len() {
        match args[i].as_str() {
            "--datasets-dir" => {
                i += 1;
                datasets_dir = args[i].clone();
            }
            "--winners-dir" => {
                i += 1;
                winners_dir = args[i].clone();
            }
            "--bind" => {
                i += 1;
                bind = args[i].clone();
            }
            "--suites" => {
                i += 1;
                suites = args[i]
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .filter_map(|s| REGISTERED_SUITES.iter().copied().find(|r| *r == s))
                    .collect();
                if suites.is_empty() {
                    die("--suites named no registered suite");
                }
            }
            other => die(&format!("unknown arg {other}")),
        }
        i += 1;
    }

    let listener = match TcpListener::bind(&bind) {
        Ok(l) => l,
        Err(e) => die(&format!("bind {bind}: {e}")),
    };
    eprintln!(
        "[riir-instinct] serving on http://{bind} (hosted hybrid lane v{VERSION}, build {})",
        fingerprint()
    );
    eprintln!("datasets: {datasets_dir} · winners: {winners_dir}");
    let allow = allowed_origins();
    if allow.is_empty() {
        eprintln!("[riir-instinct] CORS: closed (no RIIR_INSTINCT_ALLOWED_ORIGIN)");
    } else {
        eprintln!("[riir-instinct] CORS: allowed origins — {}", allow.join(", "));
    }

    // The registry: one slot per requested suite. Loader threads fill it;
    // the accept loop answers from it.
    let slots: Arc<Vec<Slot>> = Arc::new(
        suites
            .iter()
            .map(|s| Slot {
                suite: s,
                state: Mutex::new(LaneState::Loading),
            })
            .collect(),
    );
    for idx in 0..slots.len() {
        let slots = Arc::clone(&slots);
        let datasets_dir = datasets_dir.clone();
        let winners_dir = winners_dir.clone();
        let loader = std::thread::Builder::new()
            .name(format!("boot-{}", slots[idx].suite))
            .stack_size(LOADER_STACK)
            .spawn(move || {
                let slot = &slots[idx];
                let t0 = std::time::Instant::now();
                match AnySuiteServer::boot(slot.suite, std::path::Path::new(&datasets_dir), std::path::Path::new(&winners_dir)) {
                    Ok(server) => {
                        let meta = server.meta().clone();
                        *slot.state.lock().expect("slot lock") = LaneState::Ready(server);
                        eprintln!(
                            "[riir-instinct] lane {} ready in {:.1}s (arm {}, {} labels, cap {})",
                            slot.suite,
                            t0.elapsed().as_secs_f32(),
                            meta.arm.name(),
                            meta.labels,
                            meta.effective_cap
                        );
                    }
                    Err(e) => {
                        *slot.state.lock().expect("slot lock") = LaneState::Failed(e.clone());
                        eprintln!("[riir-instinct] lane {} FAILED: {e}", slot.suite);
                    }
                }
            });
        if let Err(e) = loader {
            die(&format!("spawn boot thread: {e}"));
        }
    }

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let slots = Arc::clone(&slots);
                let allow = allow.clone();
                let conn = std::thread::Builder::new()
                    .name("conn".into())
                    .stack_size(CONN_STACK)
                    .spawn(move || {
                        if let Err(e) = handle_conn(s, &slots, &allow) {
                            eprintln!("[riir-instinct] conn error: {e}");
                        }
                    });
                if let Err(e) = conn {
                    eprintln!("[riir-instinct] spawn conn: {e}");
                }
            }
            Err(e) => eprintln!("[riir-instinct] accept error: {e}"),
        }
    }
}

fn die(msg: &str) -> ! {
    eprintln!("⛔ serve: {msg}");
    std::process::exit(1);
}

struct Slot {
    suite: &'static str,
    state: Mutex<LaneState>,
}

enum LaneState {
    Loading,
    Ready(AnySuiteServer),
    Failed(String),
}

impl LaneState {
    fn as_str(&self) -> &'static str {
        match self {
            LaneState::Loading => "loading",
            LaneState::Ready(_) => "ready",
            LaneState::Failed(_) => "failed",
        }
    }
}

fn allowed_origins() -> Vec<String> {
    std::env::var("RIIR_INSTINCT_ALLOWED_ORIGIN")
        .map(|v| {
            v.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

/// The build fingerprint — BLAKE3 over the toolchain + feature-set stamp
/// (the receipt's build half; Proposal 014 §4).
fn fingerprint() -> String {
    static FP: OnceLock<String> = OnceLock::new();
    FP.get_or_init(|| {
        let mut h = blake3::Hasher::new();
        h.update(RUSTC_RELEASE.as_bytes());
        h.update(b"\0");
        h.update(RUSTC_COMMIT.as_bytes());
        h.update(b"\0");
        h.update(RUSTC_HOST.as_bytes());
        h.update(b"\0");
        for f in COMPILED_FEATURES {
            h.update(f.as_bytes());
            h.update(b"\0");
        }
        h.update(VERSION.as_bytes());
        h.finalize().to_hex().to_string()[..16].to_string()
    })
    .clone()
}

struct Req {
    method: String,
    path: String,
    content_length: usize,
    origin: Option<String>,
}

fn read_request(reader: &mut BufReader<TcpStream>) -> std::io::Result<Option<Req>> {
    let mut line = String::new();
    if reader.read_line(&mut line)? == 0 {
        return Ok(None); // clean EOF between requests
    }
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("").to_ascii_uppercase();
    let path = parts.next().unwrap_or("/").to_string();
    let mut content_length = 0usize;
    let mut origin = None;
    loop {
        let mut h = String::new();
        if reader.read_line(&mut h)? == 0 {
            break;
        }
        let h = h.trim_end();
        if h.is_empty() {
            break;
        }
        if let Some((name, value)) = h.split_once(':') {
            let (name, value) = (name.trim(), value.trim());
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.parse().unwrap_or(0);
            } else if name.eq_ignore_ascii_case("origin") {
                origin = Some(value.to_string());
            }
        }
    }
    Ok(Some(Req {
        method,
        path,
        content_length,
        origin,
    }))
}

fn cors_echo(origin: Option<&str>, allow: &[String]) -> Option<String> {
    let o = origin?;
    allow
        .iter()
        .any(|a| a.eq_ignore_ascii_case(o))
        .then(|| o.to_string())
}

/// Best-effort body drain for error paths that never read it — closing a
/// connection with unread request bytes sends RST and destroys the
/// response the client needs. Bounded: an absurd body loses its response
/// (the client chose to send garbage).
fn drain_body(reader: &mut BufReader<TcpStream>, n: usize) {
    if n == 0 || n > DRAIN_CAP {
        return;
    }
    let mut remaining = n;
    let mut buf = [0u8; 16 * 1024];
    while remaining > 0 {
        let want = buf.len().min(remaining);
        match reader.read(&mut buf[..want]) {
            Ok(0) | Err(_) => break,
            Ok(got) => remaining -= got,
        }
    }
}

fn respond(
    stream: &mut TcpStream,
    status: &str,
    body: &str,
    cors: Option<&str>,
) {
    let bytes = body.as_bytes();
    let cors_hdr = cors
        .map(|o| format!("Access-Control-Allow-Origin: {o}\r\nVary: Origin\r\n"))
        .unwrap_or_default();
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n{cors_hdr}Connection: close\r\n\r\n",
        bytes.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(bytes);
    let _ = stream.flush();
}

/// The one JSON error shape every refusal shares — a quoted `error` plus a
/// machine-readable `code` (the money-route discipline, additive).
fn json_error(stream: &mut TcpStream, status: &str, code: &str, text: &str, cors: Option<&str>) {
    respond(
        stream,
        status,
        &format!(
            "{{\"error\":{},\"code\":{code:?}}}",
            serde_json::to_string(text).unwrap_or_default()
        ),
        cors,
    );
}

fn handle_conn(
    stream: TcpStream,
    slots: &[Slot],
    allow: &[String],
) -> std::io::Result<()> {
    stream.set_read_timeout(Some(READ_TIMEOUT))?;
    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(stream);
    let Some(req) = read_request(&mut reader)? else {
        return Ok(());
    };
    let cors = cors_echo(req.origin.as_deref(), allow);
    match (req.method.as_str(), req.path.as_str()) {
        ("OPTIONS", _) => match cors {
            Some(o) => {
                let head = format!(
                    "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: {o}\r\nVary: Origin\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nAccess-Control-Max-Age: 86400\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                );
                let _ = writer.write_all(head.as_bytes());
                let _ = writer.flush();
            }
            None => {
                let head = "HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                let _ = writer.write_all(head.as_bytes());
                let _ = writer.flush();
            }
        },
        ("GET", "/healthz") => {
            respond(&mut writer, "200 OK", &healthz(slots), cors.as_deref());
        }
        ("GET", "/") => {
            respond(
                &mut writer,
                "200 OK",
                &format!(
                    "{{\"service\":\"riir-instinct\",\"version\":{VERSION:?},\"build\":{:?},\"endpoints\":[\"GET /healthz\",\"POST /decide\"]}}",
                    fingerprint()
                ),
                cors.as_deref(),
            );
        }
        ("POST", "/decide") => {
            if req.content_length > MAX_BODY {
                drain_body(&mut reader, req.content_length);
                json_error(&mut writer, "413 Payload Too Large", "too_large", "body too large", cors.as_deref());
                return Ok(());
            }
            let mut body = vec![0u8; req.content_length];
            reader.read_exact(&mut body)?;
            decide_edge(&mut writer, slots, &body, cors.as_deref());
        }
        ("GET", "/decide") | ("POST", "/healthz") => {
            drain_body(&mut reader, req.content_length);
            json_error(
                &mut writer,
                "405 Method Not Allowed",
                "method_not_allowed",
                "wrong method for this endpoint",
                cors.as_deref(),
            );
        }
        _ => {
            drain_body(&mut reader, req.content_length);
            json_error(&mut writer, "404 Not Found", "not_found", "not found", cors.as_deref());
        }
    }
    Ok(())
}

fn healthz(slots: &[Slot]) -> String {
    let mut suites = serde_json::Map::new();
    for slot in slots {
        let state = slot.state.lock().expect("slot lock");
        let mut entry = serde_json::Map::new();
        entry.insert("state".into(), serde_json::Value::String(state.as_str().into()));
        if let LaneState::Ready(server) = &*state {
            let meta = server.meta();
            entry.insert("arm".into(), serde_json::Value::String(meta.arm.name()));
            entry.insert("labels".into(), serde_json::json!(meta.labels));
            entry.insert("artifact_labels".into(), serde_json::json!(meta.artifact_labels));
            entry.insert("effective_cap".into(), serde_json::json!(meta.effective_cap));
            entry.insert("head_scale".into(), serde_json::json!(meta.head_scale));
            entry.insert("nb_scale".into(), serde_json::json!(meta.nb_scale));
            entry.insert("score_threshold".into(), serde_json::json!(meta.score_threshold));
            entry.insert("distance_threshold".into(), serde_json::json!(meta.distance_threshold));
            entry.insert("winner_blake3".into(), serde_json::json!(meta.winner_blake3));
        }
        if let LaneState::Failed(e) = &*state {
            entry.insert("error".into(), serde_json::Value::String(e.clone()));
        }
        suites.insert(slot.suite.to_string(), serde_json::Value::Object(entry));
    }
    let doc = serde_json::json!({
        "status": "ok",
        "service": "riir-instinct",
        "version": VERSION,
        "build": fingerprint(),
        "features": COMPILED_FEATURES,
        "rustc": {"release": RUSTC_RELEASE, "commit": RUSTC_COMMIT, "host": RUSTC_HOST},
        "suites": serde_json::Value::Object(suites),
    });
    serde_json::to_string(&doc).unwrap_or_else(|_| "{\"status\":\"ok\"}".into())
}

/// The /decide edge: parse → lane lookup → decide → receipt.
fn decide_edge(stream: &mut TcpStream, slots: &[Slot], body: &[u8], cors: Option<&str>) {
    #[derive(serde::Deserialize)]
    struct DecideReq {
        suite: String,
        state: String,
        options: Option<Vec<String>>,
    }
    let parsed: Result<DecideReq, _> = serde_json::from_slice(body);
    let req = match parsed {
        Ok(r) => r,
        Err(e) => {
            json_error(stream, "400 Bad Request", "bad_json", &e.to_string(), cors);
            return;
        }
    };
    let Some(slot) = slots.iter().find(|s| s.suite == req.suite) else {
        let registered: Vec<&str> = slots.iter().map(|s| s.suite).collect();
        json_error(
            stream,
            "404 Not Found",
            "unknown_suite",
            &format!(
                "suite {:?} is not served here (served: {registered:?}); the full registry is at GET /healthz",
                req.suite
            ),
            cors,
        );
        return;
    };
    let mut state = slot.state.lock().expect("slot lock");
    let server = match &mut *state {
        LaneState::Ready(server) => server,
        LaneState::Loading => {
            json_error(
                stream,
                "503 Service Unavailable",
                "loading",
                &format!("suite {:?} is still loading (seat boot) — retry shortly", req.suite),
                cors,
            );
            return;
        }
        LaneState::Failed(e) => {
            json_error(
                stream,
                "503 Service Unavailable",
                "lane_failed",
                &format!("suite {:?} failed to load: {e}", req.suite),
                cors,
            );
            return;
        }
    };
    match server.decide(&req.state, req.options.as_deref()) {
        Ok(d) => {
            let doc = serde_json::json!({
                "suite": d.suite,
                "arm": d.arm,
                "lane": "hybrid",
                "options": d.options,
                "pick": d.pick,
                "pick_index": d.pick_index,
                "probabilities": d.probabilities,
                "specialist_scores": d.specialist_scores,
                "confidence": d.confidence,
                "escalated": d.escalated,
                "abstained": d.abstained,
                "us": d.us,
                "receipt": {
                    "build": fingerprint(),
                    "features": COMPILED_FEATURES,
                    "input": input_blake3(&req.state, &d.options),
                    "decision": decision_blake3(&d),
                    "lane": "hybrid",
                },
            });
            respond(
                stream,
                "200 OK",
                &serde_json::to_string(&doc).unwrap_or_else(|_| "{\"error\":\"serialize\"}".into()),
                cors,
            );
        }
        Err(e) => {
            // The bridge refusal + empty state + duplicate options — the
            // caller's shape errors (422); everything else is lane state.
            let (status, code) = if e.starts_with("presented options neither") {
                ("422 Unprocessable Entity", "bridge_undefined")
            } else if e == "empty state" {
                ("400 Bad Request", "empty_state")
            } else {
                ("422 Unprocessable Entity", "bad_request")
            };
            json_error(stream, status, code, &e, cors);
        }
    }
}

/// BLAKE3 over the request's decision inputs — the receipt's input half.
fn input_blake3(state: &str, options: &[String]) -> String {
    let mut h = blake3::Hasher::new();
    h.update(state.as_bytes());
    h.update(b"\0");
    for o in options {
        h.update(o.as_bytes());
        h.update(b"\0");
    }
    h.finalize().to_hex().to_string()
}

/// BLAKE3 over the canonical decision (sans receipt) — the receipt's
/// decision half.
fn decision_blake3(d: &riir_instinct::server::ServedDecision) -> String {
    let canonical = serde_json::json!({
        "suite": d.suite,
        "arm": d.arm,
        "pick": d.pick,
        "pick_index": d.pick_index,
        "probabilities": d.probabilities,
        "specialist_scores": d.specialist_scores,
        "confidence": d.confidence,
        "escalated": d.escalated,
        "abstained": d.abstained,
    });
    blake3::hash(serde_json::to_string(&canonical).unwrap_or_default().as_bytes())
        .to_hex()
        .to_string()
}
