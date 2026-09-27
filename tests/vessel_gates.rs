//! The vessel gate taxonomy (Issue 001 T5) — one arm per refusal CLASS,
//! each proven to fire against a REAL minted vessel.
//!
//! The fixtures assemble the documented vessel wire format directly
//! (68-byte header ‖ ed25519-STRICT sig ‖ payload): the format crate's
//! HOSTED-ONLY writer is deliberately `pub(crate)` — no shipped path
//! writes class 1 — but a hostile-fixture test's whole job is to
//! produce exactly the bytes a real mint would, so the reader's
//! refusals are proven against the true wire shape, not a mock.
//! `tests/iss` marks the fixture minter; the reader itself consumes
//! ONLY `riir_instinct::vessel` + `reflexer_vessel` (no fixture code in
//! the shipped path).

#![cfg(feature = "vessel")]

use std::path::PathBuf;

use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use reflexer_vessel::{Class, PinTable, MAGIC, HEADER_LEN, SIG_LEN};

use riir_instinct::vessel::{AppliedState, VesselLoadError, encrypt_payload, load_hosted_bytes};
use riir_instinct::specialist::{ARTIFACT_MAGIC, ARTIFACT_VERSION, VOCAB};
use riir_instinct::Specialist;

/// A fixture mint: the signing key, its key-id, and the pin table that
/// trusts it.
struct Mint {
    signing: SigningKey,
    key_id: u32,
    pins: PinTable,
}

impl Mint {
    fn new(key_id: u32) -> Self {
        let signing = SigningKey::from_bytes(&[key_id as u8; 32]);
        let pins = PinTable::with_key(key_id, VerifyingKey::from(&signing));
        Self { signing, key_id, pins }
    }
}

/// One HOSTED-ONLY vessel: `encrypt_payload`'s envelope (nonce ‖
/// ciphertext) signed per the documented wire format. `version` must
/// exceed the mint's last (the lineage discipline is the minter's).
fn mint_hosted(
    mint: &Mint,
    version: u64,
    parent: [u8; 32],
    risp_bytes: &[u8],
    key: &[u8; 32],
) -> Vec<u8> {
    let payload = encrypt_payload(key, &nonce_for(version), risp_bytes);
    mint_signed(mint, Class::HostedOnly, version, parent, &payload)
}

fn nonce_for(version: u64) -> [u8; 16] {
    let mut n = [0u8; 16];
    n[..8].copy_from_slice(&version.to_le_bytes());
    n
}

/// Sign `header ‖ payload` per the wire format (header.to_bytes is
/// private, so the fixture lays out the 68 documented bytes itself).
fn mint_signed(
    mint: &Mint,
    class: Class,
    version: u64,
    parent: [u8; 32],
    payload: &[u8],
) -> Vec<u8> {
    let mut header = [0u8; HEADER_LEN];
    header[0..8].copy_from_slice(&MAGIC);
    header[8..12].copy_from_slice(&1u32.to_le_bytes()); // format_version
    let flags: u32 = match class {
        Class::HostedOnly => 1,
        Class::PublicRelease => 0,
    };
    header[12..16].copy_from_slice(&flags.to_le_bytes());
    header[16..20].copy_from_slice(&mint.key_id.to_le_bytes());
    header[20..28].copy_from_slice(&version.to_le_bytes());
    header[28..60].copy_from_slice(&parent);
    header[60..68].copy_from_slice(&(payload.len() as u64).to_le_bytes());
    let mut signed = Vec::with_capacity(HEADER_LEN + payload.len());
    signed.extend_from_slice(&header);
    signed.extend_from_slice(payload);
    let sig = mint.signing.sign(&signed);
    let mut out = Vec::with_capacity(HEADER_LEN + SIG_LEN + payload.len());
    out.extend_from_slice(&header);
    out.extend_from_slice(&sig.to_bytes());
    out.extend_from_slice(payload);
    out
}

/// A minimal valid RISP v1 artifact (the same shape
/// `src/specialist.rs`'s own tests mint — no shared encoder to drift;
/// the live cross-check is `examples/load_winners`).
fn risp_fixture(suite: &str, labels: &[&str]) -> Vec<u8> {
    let l = labels.len();
    let mut payload = Vec::new();
    payload.extend_from_slice(suite.as_bytes());
    payload.push(0);
    payload.extend_from_slice(&(l as u32).to_le_bytes());
    for label in labels {
        payload.extend_from_slice(label.as_bytes());
        payload.push(0);
    }
    payload.extend_from_slice(&(VOCAB as u32).to_le_bytes());
    payload.extend_from_slice(&vec![0.0f32; l].iter().flat_map(|f| f.to_le_bytes()).collect::<Vec<u8>>());
    for _ in 0..l {
        payload.extend_from_slice(&1.0f32.to_le_bytes()); // scale
        payload.extend_from_slice(&vec![0u8; VOCAB]);     // i8 weights, quantized zero
    }
    let mut out = Vec::with_capacity(payload.len() + 5 + 32);
    out.extend_from_slice(&ARTIFACT_MAGIC);
    out.push(ARTIFACT_VERSION);
    out.extend_from_slice(&payload);
    out.extend_from_slice(blake3::hash(&payload).as_bytes());
    out
}

const KEY: [u8; 32] = [42u8; 32];
const GENESIS: AppliedState = AppliedState::GENESIS;

fn load(buf: &[u8], pins: &PinTable) -> Result<Specialist, VesselLoadError> {
    load_hosted_bytes(buf, pins, &KEY, &GENESIS).map(|lv| lv.specialist)
}

/// The specialist-sized vessel: a REAL ~10 MB hosted vessel (the
/// banking77 winner's class) mints, verifies, decrypts, and loads whole
/// — the class-aware `MAX_HOSTED_PAYLOAD` cap (reflexer-vessel 049a583)
/// is what admits it, and the PUBLIC 1 MiB bound is exactly why the old
/// single-cap world could not ship this artifact. One real large buffer;
/// every other cap test forges the length instead.
#[test]
fn a_specialist_sized_vessel_loads_whole() {
    let mint = Mint::new(1);
    // A banking77-shaped RISP fixture at real scale: 77 classes x 2^17
    // i8 weights + bias + scale rows ≈ 10.09 MB — the winner's exact
    // shape, assembled in memory (a real mint's payload; never forged).
    // Seal law per risp_fixture: the BLAKE3 covers the BODY (everything
    // after magic+version), not the header bytes.
    let mut body = Vec::new();
    body.extend_from_slice(b"banking77");
    body.push(0);
    body.extend_from_slice(&77u32.to_le_bytes());
    for i in 0..77u32 {
        body.extend_from_slice(format!("class_{i:02}").as_bytes());
        body.push(0);
    }
    body.extend_from_slice(&(VOCAB as u32).to_le_bytes());
    body.extend_from_slice(&vec![0.0f32; 77].iter().flat_map(|f| f.to_le_bytes()).collect::<Vec<u8>>());
    for _ in 0..77 {
        body.extend_from_slice(&1.0f32.to_le_bytes()); // scale
        body.extend_from_slice(&vec![0u8; VOCAB]);     // i8 weights
    }
    let mut risp = Vec::with_capacity(body.len() + 5 + 32);
    risp.extend_from_slice(&ARTIFACT_MAGIC);
    risp.push(ARTIFACT_VERSION);
    risp.extend_from_slice(&body);
    risp.extend_from_slice(blake3::hash(&body).as_bytes());
    assert!(risp.len() > reflexer_vessel::MAX_PAYLOAD, "fixture must exceed the public cap");
    assert!(
        risp.len() <= reflexer_vessel::MAX_HOSTED_PAYLOAD,
        "fixture must fit the hosted cap"
    );

    let buf = mint_hosted(&mint, 1, [0; 32], &risp, &KEY);
    assert_eq!(buf.len(), risp.len() + 16 + 68 + 64);
    let m = load(&buf, &mint.pins)
        .expect("a specialist-sized HOSTED-ONLY vessel loads whole under the class-aware cap");
    assert_eq!(m.suite, "banking77");
    assert_eq!(m.labels.len(), 77);
}

#[test]
fn a_good_hosted_vessel_loads_whole() {
    let mint = Mint::new(1);
    let risp = risp_fixture("ag_news", &["world", "sports", "business", "sci"]);
    let buf = mint_hosted(&mint, 7, [0; 32], &risp, &KEY);
    let m = load(&buf, &mint.pins).expect("a well-formed HOSTED-ONLY vessel loads");
    assert_eq!(m.suite, "ag_news");
    assert_eq!(m.labels.len(), 4);
}

#[test]
fn a_tampered_payload_is_refused_by_the_outer_seal() {
    let mint = Mint::new(1);
    let risp = risp_fixture("toy", &["a", "b"]);
    let mut buf = mint_hosted(&mint, 1, [0; 32], &risp, &KEY);
    let last = buf.len() - 1;
    buf[last] ^= 0xff; // flip one ciphertext byte
    let err = load(&buf, &mint.pins).expect_err("tamper must refuse");
    assert!(
        matches!(err, VesselLoadError::Format(_)),
        "the OUTER signature is the first wall: {err}"
    );
}

#[test]
fn a_wrong_key_yields_a_refused_payload_not_garbage_scores() {
    let mint = Mint::new(1);
    let risp = risp_fixture("toy", &["a", "b"]);
    let buf = mint_hosted(&mint, 1, [0; 32], &risp, &KEY);
    let wrong = [43u8; 32];
    let err = load_hosted_bytes(&buf, &mint.pins, &wrong, &GENESIS)
        .expect_err("a wrong key must refuse");
    assert!(
        matches!(err, VesselLoadError::Payload(_)),
        "garbage never becomes scores: {err}"
    );
}

#[test]
fn a_downgraded_vessel_is_refused() {
    let mint = Mint::new(1);
    let risp = risp_fixture("toy", &["a", "b"]);
    let parent = [0u8; 32];
    let v9 = mint_hosted(&mint, 9, parent, &risp, &KEY);
    // The host has applied 9; an attacker replays the genesis-signed v5
    // (a validly signed, OLDER lineage — the replay/downgrade arm).
    let v5 = mint_hosted(&mint, 5, parent, &risp, &KEY);
    assert!(load(&v5, &mint.pins).is_ok(), "v5 alone is well-formed");
    let applied = AppliedState { artifact_version: 9 };
    let err = load_hosted_bytes(&v5, &mint.pins, &KEY, &applied)
        .expect_err("an older vessel must refuse");
    assert_eq!(
        err,
        VesselLoadError::Downgrade { vessel: 5, applied: 9 },
        "replay/downgrade refused with the versions named"
    );
    let _ = v9;
}

#[test]
fn a_public_release_vessel_refuses_in_the_hosted_lane() {
    let mint = Mint::new(1);
    let risp = risp_fixture("toy", &["a", "b"]);
    // A PUBLIC-RELEASE vessel (payload plaintext — no envelope nonce).
    let buf = mint_signed(&mint, Class::PublicRelease, 3, [0; 32], &risp);
    let err = load(&buf, &mint.pins).expect_err("public class must refuse");
    assert_eq!(
        err,
        VesselLoadError::WrongClass(Class::PublicRelease),
        "the hosted lane loads HOSTED-ONLY only"
    );
}

#[test]
fn an_unknown_or_revoked_key_fails_closed() {
    let mint = Mint::new(1);
    let risp = risp_fixture("toy", &["a", "b"]);
    let buf = mint_hosted(&mint, 2, [0; 32], &risp, &KEY);
    // unknown key-id
    let stranger = Mint::new(2);
    let err = load(&buf, &stranger.pins).expect_err("unknown key must refuse");
    assert!(matches!(err, VesselLoadError::Format(_)));
    // revoked key-id — rebuild the table the public way and revoke
    let revoked_pins = PinTable::with_key(1, VerifyingKey::from(&mint.signing)).revoke(1);
    let err = load(&buf, &revoked_pins).expect_err("revoked key must refuse");
    assert!(matches!(err, VesselLoadError::Format(_)), "{err}");
}

#[test]
fn the_on_disk_seam_matches_the_in_memory_seam() {
    let mint = Mint::new(1);
    let risp = risp_fixture("toy", &["a", "b"]);
    let buf = mint_hosted(&mint, 4, [0; 32], &risp, &KEY);
    let dir = std::env::temp_dir().join(format!(
        "instinct_vessel_gates_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("scratch dir");
    let path: PathBuf = dir.join("toy.vessel");
    std::fs::write(&path, &buf).expect("write fixture");
    let a = riir_instinct::vessel::load_hosted(&path, &mint.pins, &KEY, &GENESIS)
        .expect("disk load");
    let b = load_hosted_bytes(&buf, &mint.pins, &KEY, &GENESIS).expect("memory load");
    assert_eq!(a.specialist.suite, b.specialist.suite);
    assert_eq!(a.commitment_hex, b.commitment_hex);
    assert_eq!(a.artifact_version, 4);
    let _ = std::fs::remove_dir_all(&dir);
}
