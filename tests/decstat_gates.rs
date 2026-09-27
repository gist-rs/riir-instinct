//! The decstat capture gates (Plan 002 / Issue 004 T1). Whole-file
//! gated on `decstat`, with the `required-features` row in Cargo.toml —
//! the repo-birth gate discipline: a plain `cargo test` must never run
//! this as a green zero, and the row is what names the feature set the
//! reader needs.
//!
//! Pinned here: the consent exact-literal law (the gate that keeps
//! Unset-never-pushes honest), the sink aggregate/drain/cap arithmetic,
//! the hex-seed key load, the primitive-tag mapping, and the full
//! client-side wire path (row → signed body → mock transport → ack
//! parse → signature verifies against the protocol composer) driven
//! through the SAME seam the ureq agent serves.

#![cfg(feature = "decstat")]

use std::sync::Mutex;

use riir_instinct::decstat::{
    DecStatSink, consent_enabled, load_signing_key, primitive_tag,
};
use riir_kat::kat_decstat_client::{DecStatRow, DecstatTransport, push_decstat};
use riir_kat::kat_lease_client::HttpResponse;
use riir_kat::kat_protocol_decstat::DECSTAT_COUNTERS_MAX;

#[test]
fn consent_is_the_exact_literal_on_never_a_loose_truthy() {
    assert!(consent_enabled(Some("on")));
    for off in [None, Some(""), Some("1"), Some("ON"), Some("true"), Some(" yes"), Some("on\n")] {
        assert!(!consent_enabled(off), "consent leaked for {off:?}");
    }
}

#[test]
fn primitive_tags_map_the_four_serving_arms() {
    assert_eq!(primitive_tag("A0"), "a0");
    assert_eq!(primitive_tag("A1"), "a1");
    assert_eq!(primitive_tag("H1(top_k=4)"), "h1");
    assert_eq!(primitive_tag("H2(β=0.25,nmin=2,τ=2)"), "h2");
    assert_eq!(primitive_tag("SOME future arm"), "arm");
}

#[test]
fn sink_aggregates_the_composed_counters_and_drains_clean() {
    let sink = DecStatSink::new();
    for _ in 0..3 {
        sink.record("ag_news", "h2", false);
    }
    sink.record("ag_news", "h2", true);
    sink.record("banking77", "a0", false);
    let row = sink.take_row("m3", "rustc x").expect("non-empty window");
    assert_eq!(row.decisions, 5);
    assert_eq!(row.abstains, 1);
    let count = |name: &str| {
        row.counters
            .iter()
            .find(|c| c.name == name)
            .map(|c| c.count)
    };
    assert_eq!(count("ag_news:h2:pick"), Some(3));
    assert_eq!(count("ag_news:h2:abstain"), Some(1));
    assert_eq!(count("banking77:a0:pick"), Some(1));
    assert_eq!(
        row.suites
            .iter()
            .find(|s| s.name == "ag_news")
            .map(|s| s.count),
        Some(4)
    );
    assert!(row.suites.iter().all(|s| s.name != "banking77:a0:pick"));
    assert!(row
        .suites
        .iter()
        .any(|s| s.name == "banking77" && s.count == 1));
    // The window drained.
    assert!(sink.take_row("m3", "rustc x").is_none());
    assert_eq!(sink.pending(), 0);
}

#[test]
fn over_ceiling_windows_cap_deterministically_never_panic() {
    let sink = DecStatSink::new();
    for i in 0..400 {
        sink.record(&format!("suite{i:03}"), "a0", false);
    }
    let row = sink.take_row("m", "t").expect("non-empty");
    assert_eq!(row.counters.len(), DECSTAT_COUNTERS_MAX);
    // Tie on counts → the name-asc prefix survives (deterministic).
    assert_eq!(row.counters[0].name, "suite000:a0:pick");
    assert!(row.counters.last().is_some());
}

#[test]
fn signing_key_loads_from_hex_seed_and_refuses_everything_else() {
    let dir =
        std::env::temp_dir().join(format!("instinct_decstat_gates_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("tmp dir");
    let good = dir.join("seed.hex");
    std::fs::write(&good, "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
        .expect("write");
    let key = load_signing_key(good.to_str().unwrap()).expect("loads");
    // RFC 8032 test-vector pubkey — pins the hex byte order.
    let expect: [u8; 32] = [
        0xd7, 0x5a, 0x98, 0x01, 0x82, 0xb1, 0x0a, 0xb7, 0xd5, 0x4b, 0xfe, 0xd3, 0xc9, 0x64,
        0x07, 0x3a, 0x0e, 0xe1, 0x72, 0xf3, 0xda, 0xa6, 0x23, 0x25, 0xaf, 0x02, 0x1a, 0x68,
        0xf7, 0x07, 0x51, 0x1a,
    ];
    assert_eq!(key.verifying_key().to_bytes(), expect);

    for (name, body) in [
        ("short", "abcd"),
        ("odd", "abc"),
        ("nonhex", "zz61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"),
        ("empty", ""),
    ] {
        let p = dir.join(format!("{name}.hex"));
        std::fs::write(&p, body).expect("write");
        assert!(
            load_signing_key(p.to_str().unwrap()).is_err(),
            "{name} seed must refuse"
        );
    }
    assert!(load_signing_key(dir.join("missing.hex").to_str().unwrap()).is_err());
    let _ = std::fs::remove_dir_all(&dir);
}

// ── the client-side wire path, through the transport seam ──────────────────

struct MockTransport {
    bodies: Mutex<Vec<String>>,
    response: Vec<u8>,
}

impl DecstatTransport for MockTransport {
    fn post_decstat(&self, json: &str) -> HttpResponse {
        self.bodies.lock().expect("mock lock").push(json.to_string());
        HttpResponse::Ok(self.response.clone())
    }
}

fn test_row() -> DecStatRow {
    let sink = riir_instinct::decstat::DecStatSink::new();
    sink.record("ag_news", "h2", false);
    sink.record("ag_news", "h2", true);
    sink.record("emotion", "a1", false);
    sink.take_row("m3-gate", "rustc gate").expect("non-empty")
}

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect::<Result<_, _>>()
        .expect("hex")
}

#[test]
fn push_row_through_the_transport_seam_and_the_ack_parses() {
    let mock = MockTransport {
        bodies: Mutex::new(Vec::new()),
        response: br#"{"accepted":true,"recorded":"new","epoch":42}"#.to_vec(),
    };
    let key = ed25519_dalek::SigningKey::from_bytes(&[9u8; 32]);
    let row = test_row();
    let ack = push_decstat(&mock, &key, &row).expect("push ok");
    assert!(ack.recorded_new);
    assert_eq!(ack.epoch, 42);
    let bodies = mock.bodies.lock().expect("lock");
    assert_eq!(bodies.len(), 1);
    let v: serde_json::Value = serde_json::from_str(&bodies[0]).expect("body is json");
    assert_eq!(v["lane"].as_str(), Some("dec"));
    assert_eq!(v["machine"].as_str(), Some("m3-gate"));
    assert_eq!(v["decisions"].as_u64(), Some(3));
    assert_eq!(v["abstains"].as_u64(), Some(1));
    // The signature verifies against the protocol composer (the same
    // bytes the dapps ingest recomposes) — the cross-repo contract read
    // from THIS side.
    use ed25519_dalek::Verifier as _;
    let pk = unhex(v["account_pubkey_hex"].as_str().unwrap());
    let vk = ed25519_dalek::VerifyingKey::from_bytes(&pk[..32].try_into().unwrap()).unwrap();
    let sig = unhex(v["signature_hex"].as_str().unwrap());
    let counters: Vec<riir_kat::kat_protocol_decstat::DecStatCount> = v["counters"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            riir_kat::kat_protocol_decstat::DecStatCount::new(
                p[0].as_str().unwrap(),
                p[1].as_u64().unwrap() as u32,
            )
        })
        .collect();
    let suites: Vec<riir_kat::kat_protocol_decstat::DecStatCount> = v["suites"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            riir_kat::kat_protocol_decstat::DecStatCount::new(
                p[0].as_str().unwrap(),
                p[1].as_u64().unwrap() as u32,
            )
        })
        .collect();
    let msg = riir_kat::kat_protocol_decstat::decstat_signing_message(
        &row.run_id,
        "m3-gate",
        "rustc gate",
        row.decisions,
        row.abstains,
        &counters,
        &suites,
    );
    assert!(
        vk.verify(&msg, &ed25519_dalek::Signature::from_bytes(&sig[..64].try_into().unwrap()))
            .is_ok()
    );
}

#[test]
fn a_refused_push_surfaces_the_error_and_drops_the_window_posture() {
    let mock = MockTransport {
        bodies: Mutex::new(Vec::new()),
        response: br#"{"error":"ceiling"}"#.to_vec(),
    };
    let key = ed25519_dalek::SigningKey::from_bytes(&[9u8; 32]);
    let row = test_row();
    let err = push_decstat(&mock, &key, &row).expect_err("refused");
    assert_eq!(err, "ceiling");
    // And a non-JSON body is a parse refusal, never a panic (the mock
    // rides HttpResponse::Ok directly — the transport's `{`-probe is
    // riir-kat's own test surface; through THIS seam the unparseable ack
    // is the parse layer's single arbiter speaking).
    let opaque = MockTransport { bodies: Mutex::new(Vec::new()), response: b"nope".to_vec() };
    let err = push_decstat(&opaque, &key, &row).expect_err("refused");
    assert!(err.contains("unparseable"), "{err}");
}
