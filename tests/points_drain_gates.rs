//! The points-drain seat gates (Plan 326 T8–T12; the design contract is
//! `../riir-refine/.proposals/027_points_drain_fair_use.md`). Whole-target
//! gated on `points_drain` (the `[[test]]` required-features row) — the
//! module does not exist without the feature.
//!
//! Gates, one per plan task:
//!
//! - **T10** — the replay window: out-of-order arrivals from concurrent
//!   runs ACCEPT (the pin); seen and below-floor REJECT; wrap = 128
//!   distinct seqs; above-max accepts and slides.
//! - **T9** — the attestation verify: a valid attestation joins and the
//!   seq is recovered from the VERIFIED payload; every failure class
//!   (tampered body/sig, wrong day, bad hex, missing header, hash
//!   mismatch, garbage seq) lands ANONYMOUS — never an error.
//! - **T8** — the report-only day counters: joined counts under
//!   `(account, day)`, unjoined under `(source_hash, day)`; the source
//!   key groups by /24 (IPv4) or /64 (IPv6) and rotates by day + salt;
//!   `drain_completed` hands each day's rows out exactly once.
//! - **T11** — config ABSENT never refuses (the shipped posture, G3);
//!   armed: joined allowance `A` → billable tier (`B` at balance ≥ `R`)
//!   → refuse; unknown joined/balance falls to the per-source posture;
//!   a counter-store outage fails per-source OPEN and the billable tier
//!   CLOSED; a per-source budget exhaustion refuses.
//! - **T12** — the plane POST carries the bearer + a parseable
//!   `DrainUsageExport`; without a plane (or on a plane failure) the
//!   rows land in the local daily JSONL dump; the usage-state TTL cache
//!   fetches once per window.
//!
//! Network: every stub is a 127.0.0.1:0 listener on its own thread —
//! no external network, no fixed ports (the shared-temp-path law).

use std::net::{IpAddr, Ipv4Addr, TcpListener};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ed25519_dalek::{Signer, SigningKey};
use riir_instinct::points_drain::{
    resolve_client_ip, source_key, AttestHeaders, Attested, Budgets, Meter, MeterConfig,
    PlaneCreds, ReplayWindow, SourceCount, Verdict,
};
use riir_ledger::attest::attestation_signing_bytes;
use riir_ledger::dto::{DrainUsageExport, DrainUsageRow, UsageState};
use tempfile::TempDir;

const DAY_A: &str = "2026-11-01";
const DAY_B: &str = "2026-11-02";

fn v4(a: u8, b: u8, c: u8, d: u8) -> IpAddr {
    IpAddr::V4(Ipv4Addr::new(a, b, c, d))
}

fn cfg(enforcement: Option<Budgets>, plane: Option<PlaneCreds>, dump_dir: PathBuf) -> MeterConfig {
    MeterConfig {
        enforcement,
        plane,
        usage_ttl: Duration::from_secs(300),
        dump_dir,
    }
}

fn temp_cfg() -> (MeterConfig, TempDir) {
    let dir = TempDir::new().expect("tempdir");
    let c = MeterConfig {
        enforcement: None,
        plane: None,
        usage_ttl: Duration::from_secs(300),
        dump_dir: dir.path().to_path_buf(),
    };
    (c, dir)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// A signing key from a fixed seed (deterministic; no RNG dep needed).
fn key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

/// The account hex a caller would send: `BLAKE3(pubkey)`, lowercase.
fn account_hex(sk: &SigningKey) -> String {
    hex(blake3::hash(sk.verifying_key().to_bytes().as_slice()).as_bytes())
}

/// Compose the three Appendix-A headers + the seq transport header for
/// one attested call (the client half's exact shape).
fn attested_headers(sk: &SigningKey, day: &str, seq: u64, body: &[u8]) -> AttestHeaders {
    let pubkey = sk.verifying_key().to_bytes();
    let account: [u8; 32] = *blake3::hash(pubkey.as_slice()).as_bytes();
    let payload = attestation_signing_bytes(&account, day, seq, blake3::hash(body).as_bytes());
    let sig = sk.sign(&payload);
    AttestHeaders {
        account: Some(hex(&account)),
        pubkey: Some(hex(&pubkey)),
        seq: Some(seq.to_string()),
        sig: Some(hex(&sig.to_bytes())),
    }
}

/// One captured request at the stub: (path, authorization, body).
type Req = (String, Option<String>, Vec<u8>);
/// The stub's answer closure: (path, authorization, body) → (status,
/// response body).
type StubHandler = dyn Fn(&str, Option<&str>, &[u8]) -> (u16, String) + Send + Sync;

/// A std-only stub server: one thread per connection, 127.0.0.1:0. The
/// handler answers (status, body); the test reads the captured requests.
fn spawn_stub(handler: Arc<StubHandler>) -> (String, Arc<Mutex<Vec<Req>>>) {
    use std::io::{Read as _, Write as _};
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind stub");
    let addr = listener.local_addr().expect("addr");
    let captured: Arc<Mutex<Vec<Req>>> = Arc::new(Mutex::new(Vec::new()));
    let cap = captured.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(stream) = stream else { continue };
            let cap = cap.clone();
            let handler = handler.clone();
            std::thread::spawn(move || {
                let mut stream = stream;
                // Read the head one byte at a time until CRLFCRLF.
                let mut buf = Vec::new();
                let mut byte = [0u8; 1];
                loop {
                    match stream.read(&mut byte) {
                        Ok(0) | Err(_) => return,
                        Ok(_) => {
                            buf.push(byte[0]);
                            if buf.ends_with(b"\r\n\r\n") {
                                break;
                            }
                        }
                    }
                }
                let head = String::from_utf8_lossy(&buf).to_string();
                let mut lines = head.split("\r\n");
                let request_line = lines.next().unwrap_or_default();
                let mut parts = request_line.split_whitespace();
                let _method = parts.next().unwrap_or_default();
                let path = parts.next().unwrap_or_default().to_string();
                let mut authorization = None;
                let mut content_length = 0usize;
                for line in lines {
                    if let Some((name, value)) = line.split_once(':') {
                        let (n, v) = (name.trim(), value.trim());
                        if n.eq_ignore_ascii_case("authorization") {
                            authorization = Some(v.to_string());
                        } else if n.eq_ignore_ascii_case("content-length") {
                            content_length = v.parse().unwrap_or(0);
                        }
                    }
                }
                let mut body = vec![0u8; content_length];
                if content_length > 0 && stream.read_exact(&mut body).is_err() {
                    return;
                }
                cap.lock().expect("captured lock").push((
                    path.clone(),
                    authorization.clone(),
                    body.clone(),
                ));
                let (status, resp_body) = handler(&path, authorization.as_deref(), &body);
                let reason = if status == 200 { "OK" } else { "Status" };
                let resp = format!(
                    "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{resp_body}",
                    resp_body.len()
                );
                let _ = stream.write_all(resp.as_bytes());
            });
        }
    });
    (format!("http://{addr}"), captured)
}

fn usage_stub(
    account: &str,
    joined: bool,
    balance: u64,
    status: u16,
) -> (String, Arc<Mutex<Vec<Req>>>) {
    let answer = serde_json::to_string(&UsageState {
        format_version: riir_ledger::FORMAT_VERSION,
        account_id_hex: account.to_string(),
        joined,
        balance,
    })
    .expect("serialize UsageState");
    spawn_stub(Arc::new(move |path, _auth, _body| {
        let _ = path;
        (status, answer.clone())
    }))
}

// ─── T10 — the replay window ─────────────────────────────────────────

#[test]
fn replay_window_accepts_out_of_order_concurrent_runs() {
    // THE pin: several concurrent `cargo refine` runs share one account
    // and day; their sequences interleave out of order and every unseen
    // one must ACCEPT — a strictly-increasing check would demote them
    // all to the anonymous posture.
    let mut w = ReplayWindow::new(500);
    assert!(w.check_and_set(507));
    assert!(w.check_and_set(503));
    assert!(w.check_and_set(505));
    assert!(w.check_and_set(501));
    assert!(w.check_and_set(502));
    // 503 was seen → replay.
    assert!(!w.check_and_set(503));
    // Unseen, inside the window, below max → accept.
    assert!(w.check_and_set(504));
}

#[test]
fn replay_window_rejects_seen_and_below_floor() {
    let mut w = ReplayWindow::new(200);
    assert!(!w.check_and_set(200), "the init seq is seen");
    // The window floor is max-127: 73 is the oldest accepted seq.
    assert!(w.check_and_set(73));
    assert!(!w.check_and_set(72), "below the window floor");
    assert!(!w.check_and_set(73), "seen");
    assert!(!w.check_and_set(1), "far below the floor");
}

#[test]
fn replay_window_wraps_at_128_distinct_seqs() {
    let mut w = ReplayWindow::new(1000);
    // 999 down to 873: 127 unseen seqs inside the window (873 is the
    // floor) — all accept, and with the init seq that is exactly the
    // 128 distinct values the window holds.
    for seq in (873..=999).rev() {
        assert!(w.check_and_set(seq), "seq {seq} must accept");
    }
    assert!(!w.check_and_set(872), "the window is full below");
}

#[test]
fn replay_window_slides_forward() {
    let mut w = ReplayWindow::new(1000);
    assert!(w.check_and_set(2000), "above max accepts and slides");
    // The slide dropped the old bits: 1001 is now below the floor.
    assert!(!w.check_and_set(1001));
    assert!(w.check_and_set(1874), "the new floor accepts");
}

// ─── T9 — the attestation verify ─────────────────────────────────────

#[test]
fn attestation_verify_happy_path_recovers_seq() {
    let sk = key(1);
    let body = b"{\"suite\":\"ag_news\",\"state\":\"x\"}";
    let headers = attested_headers(&sk, DAY_A, 42, body);
    let attested = riir_instinct::points_drain::verify_attestation(Some(&headers), body, DAY_A);
    assert_eq!(
        attested,
        Attested::Joined {
            account_hex: account_hex(&sk),
            seq: 42
        }
    );
}

#[test]
fn attestation_failures_land_anonymous_never_error() {
    let sk = key(2);
    let body = b"{\"suite\":\"ag_news\"}";
    let good = attested_headers(&sk, DAY_A, 7, body);
    let anon = |h: Option<&AttestHeaders>| {
        assert_eq!(
            riir_instinct::points_drain::verify_attestation(h, body, DAY_A),
            Attested::Anonymous
        );
    };
    // No headers at all.
    anon(None);
    // Each header missing in turn.
    let mut h = good.clone();
    h.account = None;
    anon(Some(&h));
    let mut h = good.clone();
    h.pubkey = None;
    anon(Some(&h));
    let mut h = good.clone();
    h.seq = None;
    anon(Some(&h));
    let mut h = good.clone();
    h.sig = None;
    anon(Some(&h));
    // Tampered body (the signature binds the body hash).
    let other_body = b"{\"suite\":\"ag_news\",\"state\":\"tampered\"}";
    assert_eq!(
        riir_instinct::points_drain::verify_attestation(Some(&good), other_body, DAY_A),
        Attested::Anonymous
    );
    // Tampered signature (one flipped hex nibble).
    let mut h = good.clone();
    let mut sig = h.sig.clone().expect("sig");
    sig.replace_range(0..2, if sig.starts_with('0') { "1" } else { "0" });
    h.sig = Some(sig);
    anon(Some(&h));
    // Wrong verification day (the seat's clock is authoritative).
    assert_eq!(
        riir_instinct::points_drain::verify_attestation(Some(&good), body, DAY_B),
        Attested::Anonymous
    );
    // Account/pubkey mismatch (a forged account header).
    let mut h = good.clone();
    h.account = Some(hex(&[0xee; 32]));
    anon(Some(&h));
    // Bad hex everywhere.
    let mut h = good.clone();
    h.account = Some("zz".repeat(32));
    anon(Some(&h));
    let mut h = good.clone();
    h.pubkey = Some("short".to_string());
    anon(Some(&h));
    let mut h = good.clone();
    h.sig = Some("zz".repeat(64));
    anon(Some(&h));
    // Garbage seq.
    let mut h = good.clone();
    h.seq = Some("not-a-number".to_string());
    anon(Some(&h));
}

// ─── T8 — the report-only day counters + the per-source key ──────────

#[test]
fn day_counters_count_by_class_and_drain_once() {
    let (config, _dir) = temp_cfg();
    let salt = [3u8; 32];
    let meter = Meter::with_parts(salt, config);
    let sk = key(3);
    let ip = v4(203, 0, 113, 7);
    let body = b"{\"suite\":\"ag_news\"}";

    let headers = attested_headers(&sk, DAY_A, 1, body);
    assert_eq!(
        meter.evaluate(Some(ip), Some(&headers), body, DAY_A),
        Verdict::Pass
    );
    let headers2 = attested_headers(&sk, DAY_A, 2, body);
    assert_eq!(
        meter.evaluate(Some(ip), Some(&headers2), body, DAY_A),
        Verdict::Pass
    );
    assert_eq!(meter.joined_count(&account_hex(&sk), DAY_A), Some((2, 0)));

    // Anonymous traffic counts under the per-source key for the day.
    let expected_key = source_key(&salt, DAY_A, ip);
    assert_eq!(meter.evaluate(Some(ip), None, body, DAY_A), Verdict::Pass);
    assert_eq!(meter.source_count(&expected_key, DAY_A), Some(1));

    // Replay of seq 1 → the anonymous posture (the window is T10's, the
    // count is T8's: joined map unchanged, per-source grows).
    let replay = attested_headers(&sk, DAY_A, 1, body);
    assert_eq!(
        meter.evaluate(Some(ip), Some(&replay), body, DAY_A),
        Verdict::Pass
    );
    assert_eq!(meter.joined_count(&account_hex(&sk), DAY_A), Some((2, 0)));
    assert_eq!(meter.source_count(&expected_key, DAY_A), Some(2));

    // Drain: DAY_A completed on DAY_B; the rows leave exactly once.
    let (rows, sources) = meter.drain_completed_for_test(DAY_B);
    assert_eq!(
        rows,
        vec![DrainUsageRow {
            account_id_hex: account_hex(&sk),
            utc_day: DAY_A.to_string(),
            free_calls: 2,
            billable_calls: 0,
        }]
    );
    assert_eq!(
        sources,
        vec![SourceCount {
            utc_day: DAY_A.to_string(),
            source_hash: expected_key.clone(),
            calls: 2,
        }]
    );
    assert!(meter.joined_count(&account_hex(&sk), DAY_A).is_none());
    assert!(meter.source_count(&expected_key, DAY_A).is_none());
}

#[test]
fn source_key_groups_by_prefix_and_rotates() {
    let salt = [7u8; 32];
    // Same /24, different hosts → the SAME key (the network, not the
    // address).
    assert_eq!(
        source_key(&salt, DAY_A, v4(203, 0, 113, 7)),
        source_key(&salt, DAY_A, v4(203, 0, 113, 99))
    );
    // Different /24 → different key.
    assert_ne!(
        source_key(&salt, DAY_A, v4(203, 0, 113, 7)),
        source_key(&salt, DAY_A, v4(203, 0, 114, 7))
    );
    // IPv6: same /64 → same key; different → different.
    let a: IpAddr = "2001:db8:1:2::5".parse().expect("v6");
    let b: IpAddr = "2001:db8:1:2::dead".parse().expect("v6");
    let c: IpAddr = "2001:db8:1:3::5".parse().expect("v6");
    assert_eq!(source_key(&salt, DAY_A, a), source_key(&salt, DAY_A, b));
    assert_ne!(source_key(&salt, DAY_A, a), source_key(&salt, DAY_A, c));
    // Day rotation: same network, different day → different key.
    assert_ne!(
        source_key(&salt, DAY_A, v4(203, 0, 113, 7)),
        source_key(&salt, DAY_B, v4(203, 0, 113, 7))
    );
    // Salt rotation (per-boot secret): same network + day, new salt.
    assert_ne!(
        source_key(&salt, DAY_A, v4(203, 0, 113, 7)),
        source_key(&[8u8; 32], DAY_A, v4(203, 0, 113, 7))
    );
    // The client-IP trust posture: X-Forwarded-For is honored ONLY from
    // a loopback peer (the edge), never from a remote one.
    let loopback: IpAddr = "127.0.0.1".parse().expect("lo");
    let remote = v4(10, 1, 2, 3);
    assert_eq!(
        resolve_client_ip(Some(loopback), Some("198.51.100.9, 10.0.0.1")),
        Some(v4(198, 51, 100, 9))
    );
    assert_eq!(
        resolve_client_ip(Some(remote), Some("198.51.100.9")),
        Some(remote)
    );
    assert_eq!(
        resolve_client_ip(Some(loopback), Some("garbage")),
        Some(loopback)
    );
    assert_eq!(resolve_client_ip(None, None), None);
}

// ─── T11 — the enforcement hooks ─────────────────────────────────────

#[test]
fn enforcement_config_absent_is_report_only_never_refuses() {
    let (config, _dir) = temp_cfg();
    let meter = Meter::with_parts([5u8; 32], config);
    let sk = key(6);
    let body = b"{}";
    let ip = v4(198, 51, 100, 1);
    // Far past any plausible allowance / per-source budget: with the
    // config ABSENT (the shipped posture) the meter counts and never
    // refuses — the default path's behavior is unchanged (G3).
    for seq in 0..500u64 {
        let headers = attested_headers(&sk, DAY_A, seq, body);
        assert_eq!(
            meter.evaluate(Some(ip), Some(&headers), body, DAY_A),
            Verdict::Pass
        );
    }
    for _ in 0..300 {
        assert_eq!(meter.evaluate(Some(ip), None, body, DAY_A), Verdict::Pass);
    }
    assert_eq!(meter.joined_count(&account_hex(&sk), DAY_A), Some((500, 0)));
}

#[test]
fn enforcement_joined_allowance_then_billable_then_refuse() {
    let sk = key(11);
    let acct = account_hex(&sk);
    let body = b"{}";
    let ip = v4(198, 51, 100, 2);
    let (base, captured) = usage_stub(&acct, true, 10, 200);
    let config = cfg(
        Some(Budgets {
            allowance_a: 2,
            billable_b: 1,
            rate_r: 5,
            per_source_p: 1000,
        }),
        Some(PlaneCreds {
            base_url: base,
            bearer: "svc".to_string(),
        }),
        PathBuf::from("drain_dump"),
    );
    let meter = Meter::with_parts([12u8; 32], config);

    let headers = attested_headers(&sk, DAY_A, 1, body);
    assert_eq!(
        meter.evaluate(Some(ip), Some(&headers), body, DAY_A),
        Verdict::Pass
    );
    let headers2 = attested_headers(&sk, DAY_A, 2, body);
    assert_eq!(
        meter.evaluate(Some(ip), Some(&headers2), body, DAY_A),
        Verdict::Pass
    );
    // Free allowance exhausted; balance 10 ≥ R 5 → one billable call.
    let headers3 = attested_headers(&sk, DAY_A, 3, body);
    assert_eq!(
        meter.evaluate(Some(ip), Some(&headers3), body, DAY_A),
        Verdict::Pass
    );
    assert_eq!(meter.joined_count(&acct, DAY_A), Some((2, 1)));
    // A + B exhausted → refuse (contribution vocabulary, never an error).
    let headers4 = attested_headers(&sk, DAY_A, 4, body);
    match meter.evaluate(Some(ip), Some(&headers4), body, DAY_A) {
        Verdict::Refuse(message) => assert!(message.contains("contribute")),
        other => panic!("expected a refusal, got {other:?}"),
    }
    assert_eq!(
        meter.joined_count(&acct, DAY_A),
        Some((2, 1)),
        "the refused call is not counted"
    );
    // The TTL cache fetched the usage answer exactly once for all calls.
    assert_eq!(captured.lock().expect("cap").len(), 1);
}

#[test]
fn enforcement_balance_below_rate_refuses_after_allowance() {
    let sk = key(13);
    let acct = account_hex(&sk);
    let body = b"{}";
    let ip = v4(198, 51, 100, 3);
    // Balance 1 < R 5: the billable tier never arms.
    let (base, _cap) = usage_stub(&acct, true, 1, 200);
    let config = cfg(
        Some(Budgets {
            allowance_a: 1,
            billable_b: 10,
            rate_r: 5,
            per_source_p: 1000,
        }),
        Some(PlaneCreds {
            base_url: base,
            bearer: "svc".to_string(),
        }),
        PathBuf::from("drain_dump"),
    );
    let meter = Meter::with_parts([14u8; 32], config);
    let headers = attested_headers(&sk, DAY_A, 1, body);
    assert_eq!(
        meter.evaluate(Some(ip), Some(&headers), body, DAY_A),
        Verdict::Pass
    );
    let headers2 = attested_headers(&sk, DAY_A, 2, body);
    match meter.evaluate(Some(ip), Some(&headers2), body, DAY_A) {
        Verdict::Refuse(_) => {}
        other => panic!("expected a refusal, got {other:?}"),
    }
}

#[test]
fn enforcement_unknown_join_or_balance_falls_to_per_source() {
    let sk = key(15);
    let acct = account_hex(&sk);
    let body = b"{}";
    let ip = v4(198, 51, 100, 4);
    let salt = [16u8; 32];
    let expected_key = source_key(&salt, DAY_A, ip);

    // (a) No plane configured → the usage lookup fails → the unjoined
    // posture (per-source), never an error.
    let config = cfg(
        Some(Budgets {
            allowance_a: 100,
            billable_b: 10,
            rate_r: 5,
            per_source_p: 1000,
        }),
        None,
        PathBuf::from("drain_dump"),
    );
    let meter = Meter::with_parts(salt, config);
    let headers = attested_headers(&sk, DAY_A, 1, body);
    assert_eq!(
        meter.evaluate(Some(ip), Some(&headers), body, DAY_A),
        Verdict::Pass
    );
    assert!(
        meter.joined_count(&acct, DAY_A).is_none(),
        "not counted as joined"
    );
    assert_eq!(meter.source_count(&expected_key, DAY_A), Some(1));

    // (b) The plane says NOT joined → the same per-source posture.
    let (base, _cap) = usage_stub(&acct, false, 999, 200);
    let config = cfg(
        Some(Budgets {
            allowance_a: 100,
            billable_b: 10,
            rate_r: 5,
            per_source_p: 1000,
        }),
        Some(PlaneCreds {
            base_url: base,
            bearer: "svc".to_string(),
        }),
        PathBuf::from("drain_dump"),
    );
    let meter2 = Meter::with_parts(salt, config);
    let headers2 = attested_headers(&sk, DAY_A, 1, body);
    assert_eq!(
        meter2.evaluate(Some(ip), Some(&headers2), body, DAY_A),
        Verdict::Pass
    );
    assert!(meter2.joined_count(&acct, DAY_A).is_none());
    assert_eq!(meter2.source_count(&expected_key, DAY_A), Some(1));

    // (c) The plane 404s → the failed read is not cached and demotes.
    let (base, _cap2) = usage_stub(&acct, true, 999, 404);
    let config = cfg(
        Some(Budgets {
            allowance_a: 100,
            billable_b: 10,
            rate_r: 5,
            per_source_p: 1000,
        }),
        Some(PlaneCreds {
            base_url: base,
            bearer: "svc".to_string(),
        }),
        PathBuf::from("drain_dump"),
    );
    let meter3 = Meter::with_parts(salt, config);
    let headers3 = attested_headers(&sk, DAY_A, 1, body);
    assert_eq!(
        meter3.evaluate(Some(ip), Some(&headers3), body, DAY_A),
        Verdict::Pass
    );
    assert!(meter3.joined_count(&acct, DAY_A).is_none());
}

#[test]
fn tiered_fail_store_down_fails_per_source_open_and_billable_closed() {
    let sk = key(17);
    let acct = account_hex(&sk);
    let body = b"{}";
    let ip = v4(198, 51, 100, 5);
    let salt = [18u8; 32];
    let expected_key = source_key(&salt, DAY_A, ip);
    let config = cfg(
        Some(Budgets {
            allowance_a: 1,
            billable_b: 1,
            rate_r: 5,
            per_source_p: 2,
        }),
        None,
        PathBuf::from("drain_dump"),
    );
    let meter = Meter::with_parts(salt, config);
    let (joined_store, source_store) = meter.debug_counter_stores();

    // Poison the JOINED store (a panic under the lock = the outage
    // model): the joined posture FAILS CLOSED — the caller is demoted
    // to the per-source posture and SERVES (per-source budget intact).
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = joined_store.lock().expect("hold");
        panic!("poison the joined store");
    }));
    let headers = attested_headers(&sk, DAY_A, 1, body);
    assert_eq!(
        meter.evaluate(Some(ip), Some(&headers), body, DAY_A),
        Verdict::Pass
    );
    assert!(meter.joined_count(&acct, DAY_A).is_none());
    assert_eq!(meter.source_count(&expected_key, DAY_A), Some(1));

    // Poison the SOURCE store too: per-source traffic FAILS OPEN —
    // served, uncounted, never refused, never an error.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = source_store.lock().expect("hold");
        panic!("poison the source store");
    }));
    for _ in 0..10 {
        assert_eq!(meter.evaluate(Some(ip), None, body, DAY_A), Verdict::Pass);
    }
    assert!(meter.source_count(&expected_key, DAY_A).is_none());
}

#[test]
fn per_source_budget_refuses_when_exhausted() {
    let config = cfg(
        Some(Budgets {
            allowance_a: 100,
            billable_b: 10,
            rate_r: 5,
            per_source_p: 3,
        }),
        None,
        PathBuf::from("drain_dump"),
    );
    let meter = Meter::with_parts([19u8; 32], config);
    let ip = v4(198, 51, 100, 6);
    let body = b"{}";
    for _ in 0..3 {
        assert_eq!(meter.evaluate(Some(ip), None, body, DAY_A), Verdict::Pass);
    }
    match meter.evaluate(Some(ip), None, body, DAY_A) {
        Verdict::Refuse(message) => {
            assert!(
                message.contains("contribute"),
                "the contribution vocabulary"
            );
            assert!(
                message.contains("local healing is unaffected"),
                "local law intact"
            );
        }
        other => panic!("expected a refusal, got {other:?}"),
    }
    // A DIFFERENT /24 draws its own budget (the coarse allowance family
    // serves there).
    assert_eq!(
        meter.evaluate(Some(v4(198, 51, 101, 9)), None, body, DAY_A),
        Verdict::Pass
    );
}

// ─── T12 — the aggregate export ──────────────────────────────────────

#[test]
fn export_posts_drain_usage_export_with_bearer() {
    let (base, captured) = spawn_stub(Arc::new(|_path, _auth, _body| (200, "{}".to_string())));
    let config = cfg(
        None,
        Some(PlaneCreds {
            base_url: base,
            bearer: "svc-tok".to_string(),
        }),
        PathBuf::from("drain_dump"),
    );
    let meter = Meter::with_parts([20u8; 32], config);
    let rows = vec![DrainUsageRow {
        account_id_hex: "aa".repeat(32),
        utc_day: DAY_A.to_string(),
        free_calls: 5,
        billable_calls: 2,
    }];
    assert!(meter.export_rows(&rows, &[]), "the plane accepted");
    let reqs = captured.lock().expect("cap");
    assert_eq!(reqs.len(), 1);
    let (path, auth, body) = &reqs[0];
    assert_eq!(path, "/points/usage");
    assert_eq!(auth.as_deref(), Some("Bearer svc-tok"));
    let export: DrainUsageExport = serde_json::from_slice(body).expect("parse export");
    assert_eq!(export.format_version, riir_ledger::FORMAT_VERSION);
    assert_eq!(export.rows, rows);
}

#[test]
fn export_without_plane_dumps_local_jsonl() {
    let (config, dir) = temp_cfg();
    let meter = Meter::with_parts([21u8; 32], config);
    let rows = vec![DrainUsageRow {
        account_id_hex: "bb".repeat(32),
        utc_day: DAY_A.to_string(),
        free_calls: 1,
        billable_calls: 0,
    }];
    let sources = vec![SourceCount {
        utc_day: DAY_A.to_string(),
        source_hash: "cafe".to_string(),
        calls: 9,
    }];
    assert!(!meter.export_rows(&rows, &sources), "no plane — local only");
    let dump = std::fs::read_to_string(dir.path().join(format!("drain_usage_{DAY_A}.jsonl")))
        .expect("the dump file");
    let mut lines = dump.lines();
    let row: DrainUsageRow =
        serde_json::from_str(lines.next().expect("row line")).expect("parse row");
    assert_eq!(row, rows[0]);
    let source: SourceCount =
        serde_json::from_str(lines.next().expect("source line")).expect("parse source");
    assert_eq!(source, sources[0]);
    assert!(lines.next().is_none(), "no extra lines");
}

#[test]
fn export_plane_failure_falls_back_to_local_dump() {
    let (base, _cap) = spawn_stub(Arc::new(|_path, _auth, _body| (500, "boom".to_string())));
    let dir = TempDir::new().expect("tempdir");
    let config = cfg(
        None,
        Some(PlaneCreds {
            base_url: base,
            bearer: "svc-tok".to_string(),
        }),
        dir.path().to_path_buf(),
    );
    let meter = Meter::with_parts([22u8; 32], config);
    let rows = vec![DrainUsageRow {
        account_id_hex: "cc".repeat(32),
        utc_day: DAY_A.to_string(),
        free_calls: 2,
        billable_calls: 0,
    }];
    assert!(!meter.export_rows(&rows, &[]), "the plane refused");
    let dump = std::fs::read_to_string(dir.path().join(format!("drain_usage_{DAY_A}.jsonl")))
        .expect("the fallback dump");
    let row: DrainUsageRow = serde_json::from_str(dump.lines().next().expect("line")).expect("row");
    assert_eq!(row, rows[0], "counts are never lost");
}

#[test]
fn usage_state_ttl_cache_fetches_once_per_window() {
    let sk = key(23);
    let acct = account_hex(&sk);
    let body = b"{}";
    let ip = v4(198, 51, 100, 7);
    let (base, captured) = usage_stub(&acct, true, 999, 200);
    let config = MeterConfig {
        enforcement: Some(Budgets {
            allowance_a: 100,
            billable_b: 10,
            rate_r: 5,
            per_source_p: 1000,
        }),
        plane: Some(PlaneCreds {
            base_url: base,
            bearer: "svc".to_string(),
        }),
        usage_ttl: Duration::ZERO, // every lookup re-fetches
        dump_dir: PathBuf::from("drain_dump"),
    };
    let meter = Meter::with_parts([24u8; 32], config);
    for seq in 1..=2u64 {
        let headers = attested_headers(&sk, DAY_A, seq, body);
        assert_eq!(
            meter.evaluate(Some(ip), Some(&headers), body, DAY_A),
            Verdict::Pass
        );
    }
    assert_eq!(
        captured.lock().expect("cap").len(),
        2,
        "ttl 0 = refetch each call"
    );
}

#[test]
fn http_client_parses_statuses_and_rejects_foreign_schemes() {
    // 404 through the stub: the status parse is exact, not "any 4xx".
    let (base, _cap) = spawn_stub(Arc::new(|_path, _auth, _body| (404, "nope".to_string())));
    let (status, body) = riir_instinct::points_drain::http_get_json(
        &format!("{base}/x"),
        None,
        Duration::from_secs(5),
    )
    .expect("request");
    assert_eq!(status, 404);
    assert_eq!(body, "nope");
    let err = riir_instinct::points_drain::http_post_json(
        "https://plane/points/usage",
        None,
        "{}",
        Duration::from_secs(5),
    )
    .expect_err("https is refused by design");
    assert!(err.contains("http://"), "the refusal names the posture");
}
