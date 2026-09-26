//! The sealed specialist artifact (RISP v1) — reader + serving forward.
//!
//! The format is riir-train's `instinct_specialist::{encode_artifact,
//! decode_artifact}` (Issue 576): one canonical byte layout, BLAKE3-sealed
//! over the payload, i8-quantized weights with per-class f32 scales. The
//! codec lives THERE (the trainer is the producer); this module is the
//! consumer-side reader, pinned to the same bytes by the format fixtures
//! below and by the live loader (`examples/load_winners`) against the real
//! exported winners.
//!
//! Serving forward: text → events ([`riir_reflex::embed::hashed_tokens_into`]
//! — the ONE tokenizer law, `NB_VOCAB` width) → L2-normalized sorted count
//! bag → per-class `sigmoid(b_c + w_c · x)` → argmax, ties to the lowest
//! class index. Zero allocation after warmup on the score path.

use std::path::Path;

/// Artifact magic — "RISP": Riir-train Instinct SPecialist (riir-train's
/// `ARTIFACT_MAGIC`; a mismatch is a format fork, never a decode).
pub const ARTIFACT_MAGIC: [u8; 4] = *b"RISP";
/// Artifact format version (riir-train's `ARTIFACT_VERSION`).
pub const ARTIFACT_VERSION: u8 = 1;
/// Event vocabulary width — the reflex `nb_scope::NB_VOCAB` value; the
/// artifact pins it and the loader refuses any other width (the lexicon
/// fork guard, both directions).
pub const VOCAB: usize = 1 << 17;

/// A decoded specialist: sorted label universe + dequantized weights.
/// Weight order matches the label order (row-major `class * VOCAB`).
#[derive(Debug, Clone)]
pub struct Specialist {
    pub suite: String,
    pub labels: Vec<String>,
    pub w: Vec<f32>,
    pub b: Vec<f32>,
}

impl Specialist {
    /// Per-class sigmoid scores for one sparse L2-normalized bag
    /// (never softmax). `out.len() == labels.len()`.
    pub fn scores_into(&self, bag: &[(u32, f32)], out: &mut [f32]) {
        for (c, o) in out.iter_mut().enumerate() {
            let mut s = self.b[c];
            for &(j, v) in bag {
                s += self.w[c * VOCAB + j as usize] * v;
            }
            *o = katgpt_core::exact_sigmoid(s);
        }
    }

    /// The serving pick: argmax score, ties resolved to the LOWEST class
    /// index (the engine argmax law).
    pub fn pick(&self, bag: &[(u32, f32)]) -> usize {
        let mut scores = vec![0.0f32; self.labels.len()];
        self.scores_into(bag, &mut scores);
        argmax_lowest_tie(&scores)
    }
}

fn argmax_lowest_tie(scores: &[f32]) -> usize {
    let mut best = 0usize;
    for (i, &s) in scores.iter().enumerate().skip(1) {
        if s > scores[best] {
            best = i;
        }
    }
    best
}

/// The L2-normalized sparse count bag over [`VOCAB`] buckets: events from
/// the reflex tokenizer law, sorted by bucket, collisions summed, L2
/// normalized. Deterministic reduction order (sorted, never a HashMap
/// iteration). Cleared first; allocation-free once warm.
///
/// This is riir-train's training-side `bag_into` — the two sides MUST
/// agree or the serving forward reads a different model than the one
/// trained. Pinned live by `examples/load_winners` against real artifacts.
pub fn bag_into(text: &[u8], out: &mut Vec<(u32, f32)>, scratch: &mut Vec<u32>) {
    riir_reflex::embed::hashed_tokens_into(text, VOCAB, scratch);
    out.clear();
    if scratch.is_empty() {
        return;
    }
    scratch.sort_unstable();
    let mut i = 0usize;
    while i < scratch.len() {
        let bucket = scratch[i];
        let start = i;
        while i < scratch.len() && scratch[i] == bucket {
            i += 1;
        }
        out.push((bucket, (i - start) as f32));
    }
    let n2 = out.iter().map(|(_, w)| w * w).sum::<f32>().sqrt();
    if n2 > 0.0 {
        let inv = 1.0 / n2;
        for w in out.iter_mut() {
            w.1 *= inv;
        }
    }
}

/// Load + verify one artifact file (bytes from disk — weights are runtime
/// data, never committed).
pub fn load_artifact(path: &Path) -> Result<Specialist, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    decode_artifact(&bytes).map_err(|e| format!("{}: {e}", path.display()))
}

/// Decode + verify the BLAKE3 seal, then the structure. Returns the
/// dequantized model.
pub fn decode_artifact(bytes: &[u8]) -> Result<Specialist, String> {
    if bytes.len() < 5 + 32 {
        return Err("artifact too short".into());
    }
    if bytes[..4] != ARTIFACT_MAGIC {
        return Err(format!("bad magic (expected {ARTIFACT_MAGIC:?})"));
    }
    if bytes[4] != ARTIFACT_VERSION {
        return Err(format!("unsupported version {}", bytes[4]));
    }
    let payload = &bytes[5..bytes.len() - 32];
    let seal = &bytes[bytes.len() - 32..];
    let expected = blake3::hash(payload);
    if seal != expected.as_bytes() {
        return Err(format!(
            "BLAKE3 seal mismatch — expected {}, got {}",
            expected.to_hex(),
            hex_of(seal)
        ));
    }
    let mut cur = 0usize;
    let str_until = |payload: &[u8], cur: &mut usize| -> Result<String, String> {
        let end = payload[*cur..]
            .iter()
            .position(|&b| b == 0)
            .ok_or("unterminated string")?;
        let s = std::str::from_utf8(&payload[*cur..*cur + end])
            .map_err(|_| "non-utf8 string")?
            .to_string();
        *cur += end + 1;
        Ok(s)
    };
    let suite = str_until(payload, &mut cur)?;
    let l = u32::from_le_bytes(
        payload
            .get(cur..cur + 4)
            .ok_or("truncated before label count")?
            .try_into()
            .map_err(|_| "label count")?,
    ) as usize;
    cur += 4;
    if l == 0 {
        return Err("zero labels".into());
    }
    let mut labels = Vec::with_capacity(l);
    for _ in 0..l {
        labels.push(str_until(payload, &mut cur)?);
    }
    if payload.len() < cur + 4 {
        return Err("truncated before vocab".into());
    }
    let v = u32::from_le_bytes(payload[cur..cur + 4].try_into().map_err(|_| "vocab")?) as usize;
    cur += 4;
    if v != VOCAB {
        return Err(format!("artifact vocab {v} != this build's {VOCAB} — lexicon fork"));
    }
    if payload.len() < cur + l * 4 + l * (4 + v) {
        return Err("truncated weight block".into());
    }
    let mut b = Vec::with_capacity(l);
    for c in 0..l {
        b.push(f32::from_le_bytes(
            payload[cur + c * 4..cur + c * 4 + 4].try_into().map_err(|_| "bias")?,
        ));
    }
    cur += l * 4;
    let mut w = vec![0.0f32; l * v];
    for c in 0..l {
        let scale = f32::from_le_bytes(payload[cur..cur + 4].try_into().map_err(|_| "scale")?);
        cur += 4;
        for (j, q) in payload[cur..cur + v].iter().enumerate() {
            w[c * v + j] = (*q as i8) as f32 * scale;
        }
        cur += v;
    }
    Ok(Specialist { suite, labels, w, b })
}

fn hex_of(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Hand-build one RISP v1 artifact (riir-train's encoder bytes; no
    /// shared encoder to drift — the live cross-check is the example).
    fn encode(suite: &str, labels: &[&str], b: &[f32], w_row0: &[f32], w_rest: &[f32]) -> Vec<u8> {
        assert_eq!(b.len(), labels.len());
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
        payload.extend_from_slice(&b.iter().flat_map(|f| f.to_le_bytes()).collect::<Vec<u8>>());
        for (c, row) in std::iter::once(w_row0).chain(std::iter::repeat(w_rest)).take(l).enumerate() {
            let max_abs = row.iter().fold(0.0f32, |m, &x| m.max(x.abs()));
            let scale = if max_abs == 0.0 { 1.0 } else { max_abs / 127.0 };
            payload.extend_from_slice(&scale.to_le_bytes());
            for &x in row {
                payload.push(((x / scale).round().clamp(-127.0, 127.0)) as i8 as u8);
            }
            let _ = c;
        }
        let mut out = Vec::with_capacity(payload.len() + 5 + 32);
        out.extend_from_slice(&ARTIFACT_MAGIC);
        out.push(ARTIFACT_VERSION);
        out.extend_from_slice(&payload);
        out.extend_from_slice(blake3::hash(&payload).as_bytes());
        out
    }

    #[test]
    fn artifact_round_trips_and_refuses_corruption() {
        // Derive the probe bucket from the bag law itself — the exact hash
        // value does not matter, only that w[bucket] moves the score.
        let mut probe = Vec::new();
        let mut scratch = Vec::new();
        bag_into(b"good", &mut probe, &mut scratch);
        let h = probe[0].0 as usize;
        let mut w0 = vec![0.0f32; VOCAB];
        w0[h] = 3.0;
        let w_zero = vec![0.0f32; VOCAB];
        let bytes = encode("toy", &["neg", "pos"], &[0.25, -0.5], &w0, &w_zero);
        let m = decode_artifact(&bytes).expect("decode");
        assert_eq!(m.suite, "toy");
        assert_eq!(m.labels, vec!["neg".to_string(), "pos".to_string()]);
        assert_eq!(m.b, vec![0.25, -0.5]);
        let scale = 3.0f32 / 127.0;
        assert!((m.w[h] - 3.0).abs() <= scale * 0.51, "quantization bounded");

        // Any body corruption is refused by the seal.
        let mut corrupt = bytes.clone();
        let mid = 5 + (bytes.len() - 5 - 32) / 2;
        corrupt[mid] ^= 0xff;
        assert!(decode_artifact(&corrupt).is_err());
        // Truncation is refused.
        assert!(decode_artifact(&bytes[..bytes.len() - 8]).is_err());
        // Wrong magic / version are refused.
        let mut bad = bytes.clone();
        bad[0] = b'X';
        assert!(decode_artifact(&bad).is_err());
        let mut ver = bytes.clone();
        ver[4] = 9;
        assert!(decode_artifact(&ver).is_err());
    }

    #[test]
    fn pick_uses_the_bag_law_and_ties_to_the_lowest_class() {
        // Two identical rows → identical scores → the LOWEST index wins.
        let mut probe = Vec::new();
        let mut scratch = Vec::new();
        bag_into(b"good", &mut probe, &mut scratch);
        let h = probe[0].0 as usize;
        let mut w0 = vec![0.0f32; VOCAB];
        w0[h] = 3.0;
        let w1 = w0.clone();
        let bytes = encode("toy", &["a", "b"], &[0.0, 0.0], &w0, &w1);
        let m = decode_artifact(&bytes).unwrap();
        let mut bag = Vec::new();
        let mut scratch = Vec::new();
        bag_into(b"a good good", &mut bag, &mut scratch);
        assert!(!bag.is_empty());
        assert!((bag.iter().map(|(_, w)| w * w).sum::<f32>().sqrt() - 1.0).abs() < 1e-5);
        assert_eq!(m.pick(&bag), 0, "identical classes tie to the lowest index");
        // Zero-event text (pure punctuation) → an empty bag, still a pick
        // (bias-only scores).
        bag_into(b"!!! ...", &mut bag, &mut scratch);
        assert!(bag.is_empty());
        let _ = m.pick(&bag);
    }
}
