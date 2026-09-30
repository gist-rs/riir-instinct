//! The serve lane's receipt primitives (Plan 043 Phase C0 — the
//! one-definition law): the build fingerprint, the input/decision
//! BLAKE3 halves, and the manifest fingerprint, lifted VERBATIM out of
//! the serve binary so the submitter, the verifier, and the HTTP edge
//! all spell the receipt from ONE definition.
//!
//! Byte-identity is the contract: the [`fingerprint`], [`input_blake3`]
//! and [`decision_blake3`] bodies are the exact code the serve bin ran
//! before the lift (`src/bin/serve.rs` delegates now), and the receipt
//! shape gates in `tests/serve_gates.rs` pin the rendered sizes. The
//! build stamp (`COMPILED_FEATURES` / `RUSTC_*`) is the generated
//! `OUT_DIR/build_stamp.rs` — the same file the serve bin includes, so
//! the lib and the bin fingerprint byte-identically by construction.
//!
//! Pure, ungated: the only consumers today are the serve binary (the
//! `/decide` receipt) and the decstat-gated verify lane — the items
//! unused at default features carry a targeted `allow(dead_code)`
//! naming that fact, never a blanket module allow.

include!(concat!(env!("OUT_DIR"), "/build_stamp.rs"));

use std::sync::OnceLock;

/// The build fingerprint — BLAKE3 over the toolchain + feature-set
/// stamp (the receipt's build half; Proposal 014 §4). 16 hex chars.
#[cfg_attr(not(feature = "decstat"), allow(dead_code))] // bin-side consumer at default features
#[must_use]
pub fn fingerprint() -> String {
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
        h.update(env!("CARGO_PKG_VERSION").as_bytes());
        h.finalize().to_hex().to_string()[..16].to_string()
    })
    .clone()
}

/// BLAKE3 over the request's decision inputs — the receipt's input half
/// (64 hex chars). The NUL separators are load-bearing: they keep
/// `(state="ab", options=["c"])` distinct from `(state="a",
/// options=["bc"])`.
#[cfg_attr(not(feature = "decstat"), allow(dead_code))] // bin-side consumer at default features
#[must_use]
pub fn input_blake3(state: &str, options: &[String]) -> String {
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
/// decision half (64 hex chars). The canonical JSON object's field set
/// is the wire contract: adding a field changes every decision hash.
#[cfg_attr(not(feature = "decstat"), allow(dead_code))] // bin-side consumer at default features
#[must_use]
pub fn decision_blake3(d: &crate::server::ServedDecision) -> String {
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

/// The effective manifest fingerprint — 8 raw bytes (16 hex on the
/// wire). ONE helper so submitter and verifier spell it identically
/// (Plan 043 C1): `Some(text)` digests the LOADED manifest file's
/// exact text (the `INSTINCT_ARSENAL` / `--arsenal` override), `None`
/// digests the embedded default — the digest the serve bin prints at
/// boot (`ArsenalManifest::embedded_manifest_digest`).
#[cfg_attr(not(feature = "decstat"), allow(dead_code))] // decstat-gated consumer at default features
#[must_use]
pub fn manifest_fingerprint(loaded: Option<&str>) -> [u8; 8] {
    fp8_from_hex(&manifest_fingerprint_hex(loaded)).expect("digest hex is well-formed")
}

/// The same fingerprint as hex — the form the wire carries
/// (`manifest_fp_hex`, 16 chars).
#[cfg_attr(not(feature = "decstat"), allow(dead_code))] // decstat-gated consumer at default features
#[must_use]
pub fn manifest_fingerprint_hex(loaded: Option<&str>) -> String {
    let full = match loaded {
        Some(text) => crate::arsenal::ArsenalManifest::digest_of(text),
        None => crate::arsenal::ArsenalManifest::embedded_manifest_digest(),
    };
    full[..16].to_string()
}

/// Decode 16 hex chars into the 8 raw bytes (a `build_fp` /
/// `manifest_fp` on the wire). `None` on any malformed input.
#[cfg_attr(not(feature = "decstat"), allow(dead_code))] // decstat-gated consumer at default features
#[must_use]
pub fn fp8_from_hex(s: &str) -> Option<[u8; 8]> {
    if s.len() != 16 || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let mut out = [0u8; 8];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}

/// Decode 64 hex chars into 32 raw bytes (an input/decision hash on
/// the wire). `None` on any malformed input.
#[cfg_attr(not(feature = "decstat"), allow(dead_code))] // decstat-gated consumer at default features
#[must_use]
pub fn hash32_from_hex(s: &str) -> Option<[u8; 32]> {
    if s.len() != 64 || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let mut out = [0u8; 32];
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The build fingerprint is 16 lowercase hex chars and stable
    /// across calls (the OnceLock).
    #[test]
    fn fingerprint_is_16_hex_and_stable() {
        let a = fingerprint();
        let b = fingerprint();
        assert_eq!(a.len(), 16);
        assert_eq!(a, b);
        assert!(a.bytes().all(|c| c.is_ascii_hexdigit()));
    }

    /// The input hash composes exactly `state ‖ NUL ‖ opt ‖ NUL …` —
    /// pinned against a direct blake3 over that byte layout, and the
    /// NUL separators keep (state, options) splittings distinct.
    #[test]
    fn input_blake3_is_the_nul_joined_composition() {
        let direct = {
            let mut h = blake3::Hasher::new();
            h.update(b"ab");
            h.update(b"\0");
            h.update(b"c");
            h.update(b"\0");
            h.finalize().to_hex().to_string()
        };
        assert_eq!(
            input_blake3("ab", &["c".to_string()]),
            direct,
            "the composition drifted from state‖NUL‖opt‖NUL"
        );
        assert_ne!(
            input_blake3("ab", &["c".to_string()]),
            input_blake3("a", &["bc".to_string()]),
            "missing NUL separators would collide these"
        );
    }

    /// The decision hash composes exactly the canonical JSON object —
    /// pinned against a hand-built twin of the field set.
    #[test]
    fn decision_blake3_is_the_canonical_object_hash() {
        let d = crate::server::ServedDecision {
            suite: "ag_news",
            arm: "A0".to_string(),
            options: vec!["w".into(), "b".into(), "s".into(), "p".into()],
            pick_index: Some(2),
            pick: Some("s".to_string()),
            probabilities: Some(vec![0.1, 0.2, 0.6, 0.1]),
            specialist_scores: None,
            confidence: 0.6,
            escalated: false,
            abstained: false,
            us: 42,
        };
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
        let expect = blake3::hash(serde_json::to_string(&canonical).unwrap().as_bytes())
            .to_hex()
            .to_string();
        assert_eq!(decision_blake3(&d), expect);
        assert_eq!(decision_blake3(&d).len(), 64);
        // `us` (wall time) must NOT be in the hash — a re-run on a
        // slower machine reproduces the same decision hash.
        let mut slower = d.clone();
        slower.us = 99_999;
        assert_eq!(decision_blake3(&d), decision_blake3(&slower));
    }

    /// The manifest fingerprint: `Some(text)` = the first 8 bytes of
    /// blake3(text); `None` = the embedded default's prefix (the digest
    /// the serve boot line prints); hex round-trips through
    /// [`fp8_from_hex`].
    #[test]
    fn manifest_fingerprint_tracks_the_loaded_or_embedded_digest() {
        let loaded = manifest_fingerprint_hex(Some("some manifest text"));
        let direct = blake3::hash(b"some manifest text").to_hex();
        assert_eq!(loaded, direct[..16]);
        let embedded = manifest_fingerprint_hex(None);
        assert_eq!(
            embedded,
            crate::arsenal::ArsenalManifest::embedded_manifest_digest()[..16],
        );
        assert_ne!(loaded, embedded);
        let bytes = manifest_fingerprint(Some("some manifest text"));
        assert_eq!(fp8_from_hex(&loaded), Some(bytes));
        assert_eq!(fp8_from_hex("short"), None);
        assert_eq!(fp8_from_hex("zzzzzzzzzzzzzzzz"), None);
    }
}
