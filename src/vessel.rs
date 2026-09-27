//! The HOSTED-ONLY vessel reader (Plan 001 P4 / Issue 001).
//!
//! riir-reflexer ships the public vessel FORMAT — parse/verify for
//! PUBLIC-RELEASE, refusing the HOSTED-ONLY class fail-closed on
//! uncontrolled hardware. The specialist weights ARE the moat, so the
//! HOSTED-ONLY reader lives HERE (private, forever) and minting stays in
//! riir-train. This module is that reader:
//!
//! 1. **Verify before anything** — `reflexer_vessel::open` (the format
//!    crate's law: ed25519-STRICT over the header ‖ payload, caps
//!    checked before signature work, unknown key-id fails closed). One
//!    read, bounded; hash/verify/decrypt see the SAME bytes.
//! 2. **Class discipline** — the hosted serving lane loads HOSTED-ONLY
//!    vessels ONLY. A PUBLIC-RELEASE vessel in the hosted path is a
//!    configuration error (an extractable artifact where the moat
//!    should be) — refused loud, never loaded with a warning.
//! 3. **Decrypt second** — the payload is a BLAKE3-XOF keystream XOR
//!    under the host's 32-byte key + a fresh per-mint 16-byte nonce
//!    (the riir-neuron-db `local_kv::crypto` pattern — zero new deps;
//!    integrity is the OUTER vessel's signature over the ciphertext,
//!    so a flipped byte breaks the seal before decryption runs). The
//!    key arrives as a parameter — where the host keeps it (secret
//!    store / env / KMS) is the host's business, never a repo file.
//! 4. **Whole snapshot, never a blend** — the payload decodes to one
//!    [`Specialist`] (the RISP v1 reader, seal-verified) or the load
//!    fails; there is no merge path.
//! 5. **Monotonic apply** — the vessel's `artifact_version` must be
//!    strictly newer than the applied state (replay/downgrade refused;
//!    the force path is the caller's operator action and must log).
//!    `parent_commitment` chains lineages; the reader discloses it.
//!
//! Keys rotate by key-id: the caller supplies the
//! [`reflexer_vessel::PinTable`] (build it with `pins_from_bytes`,
//! revoke with `PinTable::revoke`). The compiled-in DEFAULT pins stay
//! EMPTY until the first real artifact ships — the first minting key is
//! pinned in the same change that ships it (the reflexer P3 law).

use std::path::Path;

use reflexer_vessel::{self, Class, PinTable};

use crate::specialist::Specialist;

/// Where a load refused. One variant per failure CLASS; every arm of
/// the gate taxonomy (`tests/vessel_gates.rs`) maps to exactly one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VesselLoadError {
    /// The format crate refused (tampered bytes, unknown/revoked key,
    /// bad caps, unreadable file — the outer seal is the first wall).
    Format(String),
    /// A PUBLIC-RELEASE vessel reached the hosted lane (a configuration
    /// error — the hosted path loads HOSTED-ONLY only).
    WrongClass(Class),
    /// The decrypted payload is not a valid RISP v1 specialist (wrong
    /// key producing garbage, truncated payload, or a broken inner
    /// seal).
    Payload(String),
    /// The vessel's artifact_version is not strictly newer than the
    /// applied state (replay or downgrade).
    Downgrade { vessel: u64, applied: u64 },
}

impl std::fmt::Display for VesselLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Format(e) => write!(f, "vessel refused by the format reader: {e}"),
            Self::WrongClass(c) => write!(
                f,
                "vessel class {} refused — the hosted lane loads HOSTED-ONLY only",
                c.as_str()
            ),
            Self::Payload(e) => write!(f, "vessel payload refused: {e}"),
            Self::Downgrade { vessel, applied } => write!(
                f,
                "vessel artifact_version {vessel} refused — applied state is at {applied} \
                 (replay/downgrade; a forced downgrade is an operator action and must log)"
            ),
        }
    }
}

impl std::error::Error for VesselLoadError {}

/// The applied-state half of the monotonic gate: what the host has
/// loaded now (the caller persists it — across restarts it comes back
/// from the host's state store; a fresh host is genesis at 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppliedState {
    pub artifact_version: u64,
}

impl AppliedState {
    /// Genesis — nothing applied yet.
    pub const GENESIS: Self = Self { artifact_version: 0 };
}

/// A loaded specialist plus the facts the host needs to persist.
#[derive(Debug)]
pub struct LoadedVessel {
    pub specialist: Specialist,
    /// The vessel's commitment (blake3 over the signed region) — the
    /// lineage-chain handle and the audit identity of what was applied.
    pub commitment_hex: String,
    pub artifact_version: u64,
    pub parent_commitment: [u8; 32],
}

/// The confidentiality envelope: 16-byte fresh nonce ‖ keystream-XOR
/// ciphertext. Domain-separated from every other BLAKE3 keystream user
/// (neuron-db's local-kv contexts, the specialist's own seal).
const HOSTED_CONTEXT: &[u8] = b"riir-instinct/hosted-vessel/v1";
const NONCE_LEN: usize = 16;

/// Decrypt/encrypt in place: XOR `buf` with the BLAKE3 XOF keystream
/// bound to `(key, context ‖ nonce)`. Symmetric — the minting side
/// ([`encrypt_payload`]) is the same op.
fn xor_keystream(key: &[u8; 32], nonce: &[u8; 16], buf: &mut [u8]) {
    let mut hasher = blake3::Hasher::new_keyed(key);
    hasher.update(HOSTED_CONTEXT);
    hasher.update(nonce);
    let mut counter: u64 = 0;
    for chunk in buf.chunks_mut(1024) {
        // `OutputReader::fill` OVERWRITES the destination with the raw
        // keystream — the XOR against the body happens here, per byte.
        let mut ks = hasher.finalize_xof();
        ks.set_position(counter);
        let mut stream = [0u8; 1024];
        let take = chunk.len();
        let (stream, _) = stream.split_at_mut(take);
        ks.fill(stream);
        for (b, k) in chunk.iter_mut().zip(stream.iter()) {
            *b ^= k;
        }
        counter += take as u64;
    }
}

/// Encrypt a RISP artifact's bytes for minting (riir-train's side of
/// the contract — mirrored here so the reader's own gate fixtures mint
/// real envelopes, and so the minter and reader share ONE spelling of
/// the shape). `nonce` must be fresh per mint.
#[must_use]
pub fn encrypt_payload(key: &[u8; 32], nonce: &[u8; 16], risp_bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(NONCE_LEN + risp_bytes.len());
    out.extend_from_slice(nonce);
    out.extend_from_slice(risp_bytes);
    let (_, body) = out.split_at_mut(NONCE_LEN);
    xor_keystream(key, nonce, body);
    out
}

/// Load a specialist vessel from `path` (Issue 001 T1–T4).
///
/// `pins` is the host's key-id pin table (rotation + revocation live
/// there); `key` is the hosted decryption key from the host's secret
/// store; `applied` is the monotonic gate's applied state.
///
/// Errors are the taxonomy of [`VesselLoadError`] — every gate arm in
/// `tests/vessel_gates.rs` pins one.
pub fn load_hosted(
    path: &Path,
    pins: &PinTable,
    key: &[u8; 32],
    applied: &AppliedState,
) -> Result<LoadedVessel, VesselLoadError> {
    // Single-read discipline, mirrored from `reflexer_vessel::open`:
    // regular-files only (a FIFO on the vessel path would hang boot),
    // one bounded take, then the whole verification walk over THOSE
    // bytes. The format crate's `open` refuses HOSTED-ONLY by its own
    // law, so the hosted reader assembles the same guarantees here and
    // takes the verified payload directly.
    if !std::fs::metadata(path)
        .map_err(|e| VesselLoadError::Format(e.to_string()))?
        .is_file()
    {
        return Err(VesselLoadError::Format(format!(
            "{} is not a regular file (vessels are files, not streams)",
            path.display()
        )));
    }
    let buf = std::fs::read(path).map_err(|e| VesselLoadError::Format(e.to_string()))?;
    load_hosted_bytes(&buf, pins, key, applied)
}

/// The in-memory seam over [`load_hosted`] (the gates decode fixtures
/// they minted into buffers; a hosted service may equally hold bytes).
pub fn load_hosted_bytes(
    buf: &[u8],
    pins: &PinTable,
    key: &[u8; 32],
    applied: &AppliedState,
) -> Result<LoadedVessel, VesselLoadError> {
    // Authenticity FIRST, via the format crate's own primitives: peek
    // (structure + caps), pin resolution (unknown/revoked fail closed),
    // ed25519-STRICT over header ‖ payload. `decode`'s class refusal is
    // only ever spoken about an authentic vessel — this reader IS the
    // hosted lane, so it consumes that authenticated vessel instead of
    // refusing it. Every cryptographic primitive here is the format
    // crate's own; nothing about the seal is re-implemented.
    let (header, sig_bytes) =
        reflexer_vessel::peek(buf).map_err(|e| VesselLoadError::Format(e.to_string()))?;
    let verify_key = pins
        .resolve_key(header.key_id)
        .map_err(|e| VesselLoadError::Format(e.to_string()))?;
    let signature = dalek_signature(&sig_bytes);
    verify_key
        .verify_strict(&signed_message(buf), &signature)
        .map_err(|_| VesselLoadError::Format("signature failed strict verification".into()))?;
    if header.payload_len != (buf.len() - reflexer_vessel::PREFIX_LEN) as u64 {
        return Err(VesselLoadError::Format(format!(
            "payload_len mismatch: header says {}, file carries {}",
            header.payload_len,
            buf.len() - reflexer_vessel::PREFIX_LEN
        )));
    }
    let payload = buf[reflexer_vessel::PREFIX_LEN..].to_vec();
    load_parts(&header, payload, key, applied)
}

fn load_parts(
    header: &reflexer_vessel::Header,
    payload: Vec<u8>,
    key: &[u8; 32],
    applied: &AppliedState,
) -> Result<LoadedVessel, VesselLoadError> {
    // Class discipline: the hosted lane loads HOSTED-ONLY only. A
    // PUBLIC-RELEASE artifact here is a moat leak wearing a config
    // mistake's clothes — refuse, never load-with-warning.
    if header.class != Class::HostedOnly {
        return Err(VesselLoadError::WrongClass(header.class));
    }
    // Monotonic gate before ANY payload work: a replayed or downgraded
    // vessel is refused without decrypting a byte.
    if header.artifact_version <= applied.artifact_version {
        return Err(VesselLoadError::Downgrade {
            vessel: header.artifact_version,
            applied: applied.artifact_version,
        });
    }
    if payload.len() < NONCE_LEN + 1 {
        return Err(VesselLoadError::Payload(format!(
            "payload {} B is shorter than the nonce envelope ({NONCE_LEN} B nonce + body)",
            payload.len()
        )));
    }
    let (nonce, body) = payload.split_at(NONCE_LEN);
    let nonce: [u8; NONCE_LEN] = nonce.try_into().expect("split at NONCE_LEN");
    let mut plain = body.to_vec();
    xor_keystream(key, &nonce, &mut plain);
    let specialist = crate::specialist::decode_artifact(&plain)
        .map_err(VesselLoadError::Payload)?;
    // The commitment is blake3 over the signed region — computed with
    // the format crate's own function over the verified parts.
    let commitment = reflexer_vessel::commitment_of(header, &payload);
    Ok(LoadedVessel {
        specialist,
        commitment_hex: hex32(&commitment),
        artifact_version: header.artifact_version,
        parent_commitment: header.parent_commitment,
    })
}

/// hex-encode 32 bytes (the format crate's helper is private; this is
/// the display form of a blake3 digest — [0-9a-f] only, no allocation
/// beyond the string).
fn hex32(b: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for byte in b {
        use std::fmt::Write as _;
        let _ = write!(s, "{byte:02x}");
    }
    s
}

/// The signature's message: header ‖ payload (the format crate's own
/// `signed_message` is private — mirrored verbatim, attributed; the
/// wire layout is the contract those tests already pin).
fn signed_message(buf: &[u8]) -> Vec<u8> {
    let mut msg = Vec::with_capacity(reflexer_vessel::HEADER_LEN + (buf.len() - reflexer_vessel::PREFIX_LEN));
    msg.extend_from_slice(&buf[..reflexer_vessel::HEADER_LEN]);
    msg.extend_from_slice(&buf[reflexer_vessel::PREFIX_LEN..]);
    msg
}

/// Decode the 64-byte ed25519 signature from the wire.
fn dalek_signature(bytes: &[u8; 64]) -> ed25519_dalek::Signature {
    ed25519_dalek::Signature::from_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The envelope round-trips (encrypt → decrypt = identity), and a
    /// wrong key does not.
    #[test]
    fn envelope_round_trips_and_a_wrong_key_does_not() {
        let key = [7u8; 32];
        let nonce = [9u8; 16];
        let plain: Vec<u8> = (0..300u32).map(|i| (i % 251) as u8).collect();
        let sealed = encrypt_payload(&key, &nonce, &plain);
        assert_eq!(sealed.len(), plain.len() + NONCE_LEN);

        let (n, body) = sealed.split_at(NONCE_LEN);
        let mut rt = body.to_vec();
        xor_keystream(&key, n.try_into().unwrap(), &mut rt);
        assert_eq!(rt, plain, "the envelope is symmetric");

        let wrong = [8u8; 32];
        let mut bad = body.to_vec();
        xor_keystream(&wrong, n.try_into().unwrap(), &mut bad);
        assert_ne!(bad, plain, "a wrong key must not decrypt");
    }

    /// Fresh nonces change the ciphertext under the same key (the
    /// envelope is not deterministic, so two mints of one artifact are
    /// two distinct vessel payloads).
    #[test]
    fn fresh_nonce_changes_the_ciphertext() {
        let key = [3u8; 32];
        let plain = b"the same risp bytes";
        let a = encrypt_payload(&key, &[1u8; 16], plain);
        let b = encrypt_payload(&key, &[2u8; 16], plain);
        assert_ne!(a[NONCE_LEN..], b[NONCE_LEN..]);
    }
}
