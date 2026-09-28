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
    /// One class's sigmoid score for one sparse L2-normalized bag (never
    /// softmax). The single math body — [`Self::scores_into`] delegates.
    pub fn score_class(&self, bag: &[(u32, f32)], c: usize) -> f32 {
        let mut s = self.b[c];
        for &(j, v) in bag {
            s += self.w[c * VOCAB + j as usize] * v;
        }
        katgpt_core::exact_sigmoid(s)
    }

    /// Per-class sigmoid scores for one sparse L2-normalized bag
    /// (never softmax). `out.len() == labels.len()`.
    pub fn scores_into(&self, bag: &[(u32, f32)], out: &mut [f32]) {
        for (c, o) in out.iter_mut().enumerate() {
            *o = self.score_class(bag, c);
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

/// The L2-normalized sparse PRESENCE bag over [`VOCAB`] buckets: the same
/// reflex tokenizer events, duplicates COLLAPSED (binary presence — the
/// NBSVM feature convention), each distinct bucket valued `1/√nnz` (so the
/// bag is unit length). Sorted, never a HashMap iteration. Cleared first;
/// allocation-free once warm.
///
/// This is riir-train's training-side `instinct_nbsvm::presence_bag_into`
/// at `norm = true` — the two sides MUST agree or the serving forward
/// reads a different model than the one trained (Issue 579 / Bench 612's
/// serving-convention spec). Pinned by the structure test below; the
/// live cross-check is the arena's A1 row landing at the train-side
/// holdout level.
pub fn presence_bag_into(text: &[u8], out: &mut Vec<(u32, f32)>, scratch: &mut Vec<u32>) {
    riir_reflex::embed::hashed_tokens_into(text, VOCAB, scratch);
    out.clear();
    if scratch.is_empty() {
        return;
    }
    scratch.sort_unstable();
    scratch.dedup();
    let v = 1.0 / (scratch.len() as f32).sqrt();
    out.extend(scratch.iter().map(|&b| (b, v)));
}

/// The input-bag convention an artifact was trained under. The RISP
/// format cannot carry it (v1 format fixed; the codec is the producer's),
/// so it is declared HERE, keyed by suite — see [`winner_bridge`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BagConvention {
    /// L2-normalized COUNT bags — the established v1 winners (Issue 576).
    Count,
    /// L2-normalized PRESENCE bags — the Issue 579 nbsvm v2 winner (the
    /// per-class ratio scaling is folded into the exported weights).
    Presence,
}

impl BagConvention {
    /// Build the bag this convention names. The single dispatch — every
    /// bag-building call site goes through here so a convention can never
    /// desync from the artifact it feeds.
    pub fn bag_into(self, text: &[u8], out: &mut Vec<(u32, f32)>, scratch: &mut Vec<u32>) {
        match self {
            Self::Count => bag_into(text, out, scratch),
            Self::Presence => presence_bag_into(text, out, scratch),
        }
    }

    /// The record/manifest spelling.
    pub fn name(self) -> &'static str {
        match self {
            Self::Count => "count",
            Self::Presence => "presence",
        }
    }
}

/// The serve contract a suite answers on the hosted lane (Issue 011):
/// the declaration lives on the bridge so the wire shape is decided at
/// ONE home beside the file + bag coupling, never inferred from case
/// shapes at boot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServeContract {
    /// Every case carries exactly one question; the synthesized request
    /// carries case[0]'s template. The single-question `decide` form.
    SingleQuestion,
    /// The suite's cases carry question SETS (typed_decisions: 5 heads
    /// over one state) — the upstream `decision_wire` law ("one state,
    /// ALL questions answered in one call — no per-question round
    /// trips"): the serve form is `decide_multi`, and the single-question
    /// `decide` refuses loud (a case[0]-template answer for an arbitrary
    /// state would be a fished partial read).
    MultiQuestion,
}

/// The per-suite winner bridge — ONE home for the 578 coupling lesson
/// (winner names + training conventions couple to the consumer; a change
/// is THIS window's act, never a train-side silent swap).
///
/// `file`: `None` = the established `<suite>_winner_v1.bin` convention;
/// `Some(name)` = the suite's winner carries its own name (the manifest's
/// raw-mode `file` override must agree — [`check_winner_file`]).
/// `convention`: the bag the winner's weights were trained under — every
/// bag-building site for the suite MUST dispatch through it.
/// `serves`: the suite's serve contract ([`ServeContract`], Issue 011).
#[derive(Debug, Clone, Copy)]
pub struct WinnerBridge {
    pub file: Option<&'static str>,
    pub convention: BagConvention,
    pub serves: ServeContract,
}

/// Issue 579 / Bench 612: banking77's winner is the nbsvm v2 artifact —
/// deliberately NOT a `winner_v1` name (that name stays the refused v1
/// artifact for reproduction), trained over L2-normalized PRESENCE bags.
/// Every other suite keeps the v1 file + count-bag convention.
/// Issue 011: typed_decisions serves the multi-question contract (the
/// only suite whose cases carry question sets).
pub fn winner_bridge(suite: &str) -> WinnerBridge {
    match suite {
        "banking77" => WinnerBridge {
            file: Some("banking77_nbsvm_v2.bin"),
            convention: BagConvention::Presence,
            serves: ServeContract::SingleQuestion,
        },
        "typed_decisions" => WinnerBridge {
            file: None,
            convention: BagConvention::Count,
            serves: ServeContract::MultiQuestion,
        },
        _ => WinnerBridge {
            file: None,
            convention: BagConvention::Count,
            serves: ServeContract::SingleQuestion,
        },
    }
}

/// The raw-mode coupling check: a suite with a bridged winner file must
/// load EXACTLY that file — a manifest row (or a swap request) naming any
/// other artifact would serve the suite's convention over weights that
/// were not trained under it, silently. RAW MODE ONLY: vessel files carry
/// the `<suite>_v1.vessel` convention and are checked by nothing here.
pub fn check_winner_file(suite: &str, file: &str) -> Result<(), String> {
    match winner_bridge(suite).file {
        Some(expected) if file != expected => Err(format!(
            "suite {suite}: artifact {file:?} is not the bridged winner {expected:?} — the \
             input-convention coupling (Issue 579) forbids serving this suite from any other \
             raw artifact; update winner_bridge in the same change as the swap"
        )),
        _ => Ok(()),
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

    /// The train-side `instinct_nbsvm::presence_bag_into(norm = true)`
    /// mirror test: sorted, unique, each value 1/√nnz, empty text → empty
    /// bag (the training-side test's own arms, verbatim).
    #[test]
    fn presence_bag_is_sorted_unique_and_normed() {
        let mut out = Vec::new();
        let mut s = Vec::new();
        presence_bag_into(b"alpha bravo alpha", &mut out, &mut s);
        assert!(out.windows(2).all(|w| w[0].0 < w[1].0), "sorted");
        assert!(!out.is_empty());
        // "alpha" appears twice but presence collapses it: the bag holds
        // exactly the DISTINCT events (bigrams included by the tokenizer
        // law) — its cardinality equals the count bag's.
        let mut probe = Vec::new();
        bag_into(b"alpha bravo alpha", &mut probe, &mut s);
        assert_eq!(out.len(), probe.len(), "presence == the count bag's distinct buckets");
        let inv = 1.0 / (out.len() as f32).sqrt();
        assert!(out.iter().all(|(_, v)| (*v - inv).abs() < 1e-7));
        let norm: f32 = out.iter().map(|(_, v)| v * v).sum();
        assert!((norm - 1.0).abs() < 1e-5, "unit length");
        presence_bag_into(b"!!! ...", &mut out, &mut s);
        assert!(out.is_empty(), "pure punctuation carries no events");
    }

    /// The bridge table, both directions: banking77 is the v2 presence
    /// lane; every other arena suite stays on the v1 file + count bags;
    /// typed_decisions is the one MultiQuestion serve contract (Issue 011).
    #[test]
    fn winner_bridge_pins_the_convention_coupling() {
        use crate::specialist::ServeContract;
        let b = winner_bridge("banking77");
        assert_eq!(b.file, Some("banking77_nbsvm_v2.bin"));
        assert_eq!(b.convention, BagConvention::Presence);
        assert_eq!(b.serves, ServeContract::SingleQuestion);
        let t = winner_bridge("typed_decisions");
        assert_eq!(t.file, None, "typed keeps the winner_v1 convention");
        assert_eq!(t.convention, BagConvention::Count);
        assert_eq!(t.serves, ServeContract::MultiQuestion);
        for suite in [
            "ag_news",
            "emotion",
            "sst5",
            "massive_intent_en",
            "xnli_en",
            "prompt_injections",
            "a_suite_that_never_exists",
        ] {
            let w = winner_bridge(suite);
            assert_eq!(w.file, None, "{suite}: unexpected bridged file");
            assert_eq!(w.convention, BagConvention::Count, "{suite}: unexpected convention");
            assert_eq!(w.serves, ServeContract::SingleQuestion, "{suite}: unexpected contract");
        }
        // The dispatch routes by convention.
        let mut a = Vec::new();
        let mut b2 = Vec::new();
        let mut s = Vec::new();
        BagConvention::Count.bag_into(b"alpha bravo alpha", &mut a, &mut s);
        bag_into(b"alpha bravo alpha", &mut b2, &mut s);
        assert_eq!(a, b2, "Count dispatches to bag_into");
        BagConvention::Presence.bag_into(b"alpha bravo alpha", &mut a, &mut s);
        presence_bag_into(b"alpha bravo alpha", &mut b2, &mut s);
        assert_eq!(a, b2, "Presence dispatches to presence_bag_into");
        // The coupling check: the bridged file passes, any other refuses.
        assert!(check_winner_file("banking77", "banking77_nbsvm_v2.bin").is_ok());
        let err = check_winner_file("banking77", "banking77_winner_v1.bin")
            .expect_err("the refused v1 artifact must not load under the v2 convention");
        assert!(err.contains("banking77_nbsvm_v2.bin"), "{err}");
        assert!(check_winner_file("ag_news", "ag_news_winner_v1.bin").is_ok());
        assert!(check_winner_file("ag_news", "anything_else.bin").is_ok(), "unbridged suites are unconstrained");
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
