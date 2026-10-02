//! The decstat receipt VERIFIER (Plan 043 Phase C1): the judge half of
//! the receipt reward class. A lease item carries only `{suite,
//! input_hash, decision_hash, build_fp, manifest_fp}` — the verifier
//! resolves `input_hash` against its own blake3 input map over its own
//! frozen pool copy (D1, corpus-referenced replay: no text crosses the
//! wire), re-runs the case through the SAME decide surface `/decide`
//! serves, and compares decision hashes.
//!
//! Outcome vocabulary (the wire's three + the two silent skips):
//!
//! - **toolchain skew** — the item's `build_fp` ≠ this binary's
//!   [`crate::receipt::fingerprint`]: skip LOUD, no verdict (a verdict
//!   from a different release proves nothing).
//! - **manifest skew** — the item's `manifest_fp` ≠ this process's
//!   effective manifest fingerprint: skip LOUD, no verdict (a different
//!   posture is a different answer space).
//! - **corpus skew** — the input hash resolves to nothing this
//!   verifier's pool holds: `corpus_skew` verdict with the zero hash
//!   (counted by the floor, never an error — D1).
//! - **mismatch / verified** — the case resolved and re-ran; the hash
//!   did / did not reproduce.
//!
//! The seat engine boots ONCE per [`Verifier::boot`] on a 64 MiB-stack
//! thread (the seat boot allocates big frames — the serve binary gives
//! its loaders the same headroom), one server per manifest suite.
//! Boot env mirrors the serve binary exactly: `INSTINCT_DATASETS_DIR` /
//! `INSTINCT_WINNERS_DIR` / `INSTINCT_SYNTH_CORPUS_DIR` / the arsenal
//! override — resolved by the RUNNER and handed in as a
//! [`BootConfig`], so this module stays pure over its inputs.

use std::collections::HashMap;
use std::path::Path;

use riir_kat::kat_protocol_decstat_receipt::DecStatLeaseItemWire;
use riir_kat::kat_protocol_decstat_verdict::{
    DECSTAT_VERDICT_CORPUS_SKEW, DECSTAT_VERDICT_MISMATCH, DECSTAT_VERDICT_VERIFIED,
};
use riir_reflex::harness::suites::{QKind, SuiteCase};

use crate::receipt;
use crate::server::{AnySuiteServer, ServedDecision};

/// The seat boot's stack headroom (the serve loader's `LOADER_STACK` —
/// the seat boot allocates big frames on a test thread's 2 MiB default).
const BOOT_STACK: usize = 64 * 1024 * 1024;

/// Plan 426 T6's serve posture: synth rows enter BEYOND the per-label
/// gold cap (the V5 measured posture, reflex bench 091) — the same cap
/// the serve binary's `SYNTH_EXTRA_CAP` carries.
const SYNTH_EXTRA_CAP: usize = 128;

/// One judged lease item. `Verdict::ToolchainSkew` /
/// `Verdict::ManifestSkew` are SILENT outcomes — they send no verdict
/// to the service (a skew verdict would be a claim about the wrong
/// release/posture, and the lane's vocabulary has no byte for it).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// The case resolved and re-ran; the hash reproduced.
    Verified([u8; 32]),
    /// The case resolved and re-ran; the hash did NOT reproduce.
    Mismatch { claimed: [u8; 32], computed: [u8; 32] },
    /// The input hash resolves in no frozen corpus this verifier holds.
    CorpusSkew,
    /// `build_fp` ≠ this binary's — skip loud, no verdict.
    ToolchainSkew { item: [u8; 8], ours: [u8; 8] },
    /// `manifest_fp` ≠ this process's effective manifest — skip loud,
    /// no verdict.
    ManifestSkew { item_hex: String, ours_hex: String },
}

impl Verdict {
    /// The wire outcome byte — `None` for the silent skips.
    #[must_use]
    pub fn wire_outcome(&self) -> Option<u8> {
        match self {
            Verdict::Verified(_) => Some(DECSTAT_VERDICT_VERIFIED),
            Verdict::Mismatch { .. } => Some(DECSTAT_VERDICT_MISMATCH),
            Verdict::CorpusSkew => Some(DECSTAT_VERDICT_CORPUS_SKEW),
            Verdict::ToolchainSkew { .. } | Verdict::ManifestSkew { .. } => None,
        }
    }

    /// The computed decision hash (all-zeros for `corpus_skew`, per
    /// the verdict wire's contract).
    #[must_use]
    pub fn computed_hash(&self) -> [u8; 32] {
        match self {
            Verdict::Verified(h) | Verdict::Mismatch { computed: h, .. } => *h,
            _ => [0u8; 32],
        }
    }

    /// True for the two silent skips (the runner counts them but sends
    /// nothing).
    #[must_use]
    pub fn is_skipped(&self) -> bool {
        self.wire_outcome().is_none()
    }
}

/// The judge breakdown the floor reports (Plan 043 C2) — the skew
/// enemies of D5, each its own counter, never pooled.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Breakdown {
    pub toolchain_skipped: u64,
    pub manifest_skipped: u64,
    pub corpus_skew: u64,
    pub mismatch: u64,
    pub verified: u64,
}

impl Breakdown {
    /// Fold one verdict into the counters.
    pub fn record(&mut self, v: &Verdict) {
        match v {
            Verdict::ToolchainSkew { .. } => self.toolchain_skipped += 1,
            Verdict::ManifestSkew { .. } => self.manifest_skipped += 1,
            Verdict::CorpusSkew => self.corpus_skew += 1,
            Verdict::Mismatch { .. } => self.mismatch += 1,
            Verdict::Verified(_) => self.verified += 1,
        }
    }

    /// The same-release reproduction rate — D5's floor quantity:
    /// `verified / (verified + mismatch)`. 0.0 when nothing re-ran.
    #[must_use]
    pub fn reproduction_rate(&self) -> f64 {
        let denom = self.verified + self.mismatch;
        if denom == 0 {
            0.0
        } else {
            self.verified as f64 / denom as f64
        }
    }
}

/// Everything the boot needs — the runner resolves the env contract
/// (mirroring the serve binary) and hands it in.
#[derive(Debug, Clone)]
pub struct BootConfig {
    pub datasets_dir: String,
    pub winners_dir: String,
    /// The serve posture: when set and `<dir>/<suite>_synth.jsonl` is
    /// present for a booting suite, the synth corpus seats it (Plan 426
    /// T6); anything else keeps the gold seat.
    pub synth_corpus_dir: Option<String>,
    pub manifest: crate::arsenal::ArsenalManifest,
    /// Optional suite filter (the `--suites` surface); an unknown name
    /// is a boot error (the manifest is the one selection surface).
    pub suite_filter: Option<Vec<String>>,
}

/// One booted suite's replay surface: the live server plus the case
/// snapshot the map was built from (the submitter role samples these).
pub struct SuiteHandle {
    pub name: &'static str,
    pub n_cases: usize,
    cases: Vec<SuiteCase>,
    state_strs: Vec<String>,
    server: AnySuiteServer,
}

/// Where one input hash resolves: the suite handle + which case, and
/// which question of that case (a multi-question case yields one
/// receipt per question).
#[derive(Debug, Clone, Copy)]
struct MapKey {
    suite: usize,
    case: usize,
    question: usize,
}

/// The booted verifier: the input map + the live engines + the two
/// fingerprints every judgement is screened against.
pub struct Verifier {
    suites: Vec<SuiteHandle>,
    map: HashMap<String, MapKey>,
    build_fp: [u8; 8],
    manifest_fp: [u8; 8],
    /// The LOADED manifest text (`INSTINCT_ARSENAL`), when the runner
    /// overrode the embedded default — what the manifest fingerprint
    /// digests. `None` = embedded default.
    loaded_manifest: Option<String>,
    /// Suites that refused to boot, with the reason (loud-skip record;
    /// the runner surfaces it and refuses when NOTHING booted).
    pub skipped_suites: Vec<(String, String)>,
}

impl SuiteHandle {
    /// The suite's serving arm name (the boot posture).
    #[must_use]
    pub fn arm_name(&self) -> String {
        self.server.meta().arm.name().to_string()
    }

    /// One case's state string (the map was keyed from these).
    #[must_use]
    pub fn state_str_of(&self, case_idx: usize) -> Option<&str> {
        self.state_strs.get(case_idx).map(String::as_str)
    }
}

impl Verifier {
    /// This binary's build fingerprint (16 hex) — what `build_fp`
    /// screens against.
    #[must_use]
    pub fn build_fp_hex(&self) -> String {
        receipt::fingerprint()
    }

    /// This process's effective manifest fingerprint (16 hex) — what
    /// `manifest_fp` screens against.
    #[must_use]
    pub fn manifest_fp_hex(&self) -> String {
        receipt::manifest_fingerprint_hex(self.loaded_manifest.as_deref())
    }

    /// The booted suites, in manifest order (skips excluded).
    #[must_use]
    pub fn suites(&self) -> &[SuiteHandle] {
        &self.suites
    }

    /// One case's state string — the map's keys were computed from
    /// these exact bytes.
    #[must_use]
    pub fn state_str(&self, suite_idx: usize, case_idx: usize) -> &str {
        self.suites
            .get(suite_idx)
            .and_then(|h| h.state_str_of(case_idx))
            .unwrap_or("")
    }

    /// Boot every (filtered) manifest suite on one big-stack thread:
    /// prepare the seat, snapshot the cases, boot the server, then
    /// build the input map by DRIVING the cases (the map keys are the
    /// exact serve-receipt input hashes — `input_blake3(state,
    /// d.options)` over the same decide surface, never a re-derivation).
    ///
    /// # Errors
    ///
    /// A string naming the failure — a manifest that refuses, an
    /// unknown filtered suite name, or (the runner's refusal case) zero
    /// suites booted.
    pub fn boot(cfg: BootConfig, loaded_manifest: Option<String>) -> Result<Self, String> {
        std::thread::Builder::new()
            .name("decstat-verify-boot".into())
            .stack_size(BOOT_STACK)
            .spawn(move || Self::boot_inner(cfg, loaded_manifest))
            .map_err(|e| format!("spawn verify boot thread: {e}"))?
            .join()
            .map_err(|_| "verify boot thread panicked".to_string())?
    }

    fn boot_inner(
        cfg: BootConfig,
        loaded_manifest: Option<String>,
    ) -> Result<Self, String> {
        let winners = Path::new(&cfg.winners_dir);
        cfg.manifest
            .validate(&crate::arsenal::ValidateCtx::raw(winners))
            .map_err(|e| format!("arsenal manifest validation refused: {e}"))?;
        let names: Vec<&'static str> = match &cfg.suite_filter {
            None => cfg
                .manifest
                .suites()
                .map(|s| -> &'static str { Box::leak(s.to_string().into_boxed_str()) })
                .collect(),
            Some(filter) => {
                for f in filter {
                    if cfg.manifest.row(f).is_none() {
                        return Err(format!("suite {f:?} is not in the arsenal manifest"));
                    }
                }
                filter
                    .iter()
                    .map(|f| -> &'static str { Box::leak(f.clone().into_boxed_str()) })
                    .collect()
            }
        };
        if names.is_empty() {
            return Err("no suites to verify (the manifest named none)".into());
        }
        let datasets = Path::new(&cfg.datasets_dir);
        let mut suites: Vec<SuiteHandle> = Vec::with_capacity(names.len());
        let mut skipped: Vec<(String, String)> = Vec::new();
        let mut map: HashMap<String, MapKey> = HashMap::new();
        for name in names {
            let boot = (|| -> Result<SuiteHandle, String> {
                // The seat: synth-seated when the env dir carries the
                // suite's artifact (the serve posture), else gold.
                let synth = cfg
                    .synth_corpus_dir
                    .as_ref()
                    .map(|dir| Path::new(dir).join(format!("{name}_synth.jsonl")))
                    .filter(|p| p.is_file());
                let seat = match synth {
                    Some(path) => riir_reflex::harness::runner::seat::prepare_seat_with_synth(
                        name,
                        datasets,
                        &path,
                        SYNTH_EXTRA_CAP,
                    )?,
                    None => riir_reflex::harness::runner::seat::prepare_seat(name, datasets)?,
                };
                let cases = seat.suite.cases.clone();
                let state_strs = seat.state_strs.clone();
                let server =
                    AnySuiteServer::boot_from_seat(name, seat, winners, &cfg.manifest)?;
                Ok(SuiteHandle {
                    name,
                    n_cases: cases.len(),
                    cases,
                    state_strs,
                    server,
                })
            })();
            match boot {
                Ok(mut handle) => {
                    let suite_idx = suites.len();
                    for (case_idx, case) in handle.cases.iter().enumerate() {
                        let state = &handle.state_strs[case_idx];
                        // Drive ONCE per case at map-build time; every
                        // question's decision contributes its input
                        // hash (the same drive `judge` replays).
                        match drive_case(&mut handle.server, case, state) {
                            Ok(decisions) => {
                                for (question, d) in decisions.iter().enumerate() {
                                    let key = receipt::input_blake3(state, &d.options);
                                    map.insert(
                                        key,
                                        MapKey { suite: suite_idx, case: case_idx, question },
                                    );
                                }
                            }
                            Err(e) => eprintln!(
                                "[decstat-verify] {} case {}: map drive refused ({e}) — case skipped",
                                name, case_idx
                            ),
                        }
                    }
                    eprintln!(
                        "[decstat-verify] booted {} ({} case(s), arm {})",
                        name,
                        handle.n_cases,
                        handle.server.meta().arm.name()
                    );
                    suites.push(handle);
                }
                Err(e) => {
                    eprintln!(
                        "[decstat-verify] SKIP loud: suite {name} refused to boot — {e}"
                    );
                    skipped.push((name.to_string(), e));
                }
            }
        }
        if suites.is_empty() {
            return Err(format!(
                "zero suites booted ({} skipped: {:?}) — the verify lane has nothing to judge",
                skipped.len(),
                skipped.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>()
            ));
        }
        Ok(Self {
            suites,
            map,
            build_fp: receipt::fp8_from_hex(&receipt::fingerprint())
                .expect("fingerprint is 16 hex"),
            manifest_fp: receipt::manifest_fingerprint(loaded_manifest.as_deref()),
            loaded_manifest,
            skipped_suites: skipped,
        })
    }

    /// Drive one case of one suite through the SAME decide surface the
    /// serve binary serves (single-question cases through
    /// [`AnySuiteServer::decide`] — noul with the canonical universe
    /// (`None`), else the criteria keys — and multi-question cases
    /// through [`AnySuiteServer::decide_multi`], noul questions with
    /// the empty presentation the fixed rendering speaks). The gates in
    /// `tests/serve_gates.rs` drive the exact same shapes.
    ///
    /// # Errors
    ///
    /// The decide surface's refusal strings (bridge-undefined, empty
    /// state, lane state).
    pub fn drive_case(
        &mut self,
        suite_idx: usize,
        case_idx: usize,
    ) -> Result<Vec<ServedDecision>, String> {
        // Bound checks first (immutable), then the single &mut borrow —
        // the case/state reads split off the server's mutable borrow at
        // the field level.
        {
            let handle = self
                .suites
                .get(suite_idx)
                .ok_or_else(|| format!("suite index {suite_idx} out of range"))?;
            if handle
                .cases
                .get(case_idx)
                .ok_or_else(|| format!("case index {case_idx} out of range for {}", handle.name))?
                .questions
                .is_empty()
            {
                return Err(format!("case {case_idx} of {} carries no questions", handle.name));
            }
            if handle.state_strs.get(case_idx).is_none() {
                return Err(format!("case index {case_idx} has no state string"));
            }
        }
        let handle = &mut self.suites[suite_idx];
        let case = &handle.cases[case_idx];
        let state = handle.state_strs[case_idx].as_str();
        drive_case(&mut handle.server, case, state)
    }

    /// Judge one leased item per the C1 loop: toolchain screen →
    /// manifest screen → corpus resolution → re-run → hash compare.
    /// Every skip is LOUD (stderr) and counted by [`Breakdown::record`];
    /// a skip sends no verdict.
    pub fn judge(&mut self, item: &DecStatLeaseItemWire) -> Verdict {
        // 1. Toolchain screen: a verdict from a different release
        //    proves nothing — and a MALFORMED fingerprint cannot be
        //    proven ours, so it takes the conservative skip.
        let Some(item_build) = receipt::fp8_from_hex(&item.build_fp_hex) else {
            eprintln!(
                "[decstat-verify] skip loud (toolchain): malformed build_fp {:?} — no verdict",
                item.build_fp_hex
            );
            return Verdict::ToolchainSkew {
                item: [0u8; 8],
                ours: self.build_fp,
            };
        };
        if item_build != self.build_fp {
            eprintln!(
                "[decstat-verify] skip loud (toolchain): item build_fp {} ≠ ours {} — \
                 no verdict (a verdict from a different release proves nothing)",
                item.build_fp_hex,
                receipt::fingerprint()
            );
            return Verdict::ToolchainSkew {
                item: item_build,
                ours: self.build_fp,
            };
        }
        // 2. Manifest screen: a different posture is a different answer
        //    space (the same conservative rule for malformed hex).
        let Some(item_manifest) = receipt::fp8_from_hex(&item.manifest_fp_hex) else {
            eprintln!(
                "[decstat-verify] skip loud (manifest): malformed manifest_fp {:?} — no verdict",
                item.manifest_fp_hex
            );
            return Verdict::ManifestSkew {
                item_hex: item.manifest_fp_hex.clone(),
                ours_hex: receipt::manifest_fingerprint_hex(None),
            };
        };
        if item_manifest != self.manifest_fp {
            eprintln!(
                "[decstat-verify] skip loud (manifest): item manifest_fp {} ≠ ours {} — \
                 no verdict (a different posture is a different answer space)",
                item.manifest_fp_hex,
                hex8(&self.manifest_fp)
            );
            return Verdict::ManifestSkew {
                item_hex: item.manifest_fp_hex.clone(),
                ours_hex: hex8(&self.manifest_fp),
            };
        }
        // 3. Corpus resolution: the D1 law — a miss is a legitimate
        //    outcome, counted by the floor, never a crash. The zero
        //    hash is the wire contract for the corpus-skew outcome.
        let Some(&key) = self.map.get(&item.input_hash_hex.to_ascii_lowercase()) else {
            eprintln!(
                "[decstat-verify] corpus_skew: input {} resolves in no frozen suite this \
                 verifier holds (suite claim {:?})",
                prefix16(&item.input_hash_hex),
                item.suite
            );
            return Verdict::CorpusSkew;
        };
        // 4. Replay: run the case, take the judged question's decision,
        //    hash it canonically, compare to the claim. A malformed
        //    CLAIM hash cannot equal any computed hash — an honest
        //    mismatch, not a crash.
        let computed_hex = match self.drive_case(key.suite, key.case) {
            Ok(mut decisions) => {
                if key.question >= decisions.len() {
                    eprintln!(
                        "[decstat-verify] internal: resolved question {} of a {}-decision \
                         case — treating as corpus skew (the map drifted)",
                        key.question,
                        decisions.len()
                    );
                    return Verdict::CorpusSkew;
                }
                receipt::decision_blake3(&decisions.swap_remove(key.question))
            }
            Err(e) => {
                eprintln!(
                    "[decstat-verify] replay refused for a resolved input ({e}) — \
                     treating as mismatch (the case no longer decides)"
                );
                String::new()
            }
        };
        let computed = receipt::hash32_from_hex(&computed_hex).unwrap_or([0u8; 32]);
        let Some(claimed) = receipt::hash32_from_hex(&item.decision_hash_hex) else {
            eprintln!(
                "[decstat-verify] mismatch: malformed claimed decision hash {:?}",
                item.decision_hash_hex
            );
            return Verdict::Mismatch {
                claimed: [0u8; 32],
                computed,
            };
        };
        if claimed == computed {
            Verdict::Verified(computed)
        } else {
            eprintln!(
                "[decstat-verify] mismatch: claimed {}… vs computed {}…",
                prefix16(&item.decision_hash_hex),
                prefix16(&computed_hex)
            );
            Verdict::Mismatch {
                claimed,
                computed,
            }
        }
    }
}

/// The per-case drive (free fn so the map build and the replay share
/// ONE body — the single-question/multi-question dispatch mirrors the
/// serve gates exactly).
fn drive_case(
    server: &mut AnySuiteServer,
    case: &SuiteCase,
    state: &str,
) -> Result<Vec<ServedDecision>, String> {
    if case.questions.len() == 1 {
        let q = &case.questions[0];
        if q.kind == QKind::Noul {
            // The canonical universe (`decide(None)`) — the prompt_
            // injections gate's drive.
            server.decide(state, None).map(|d| vec![d])
        } else {
            let keys = criteria_keys(q);
            server.decide(state, Some(&keys)).map(|d| vec![d])
        }
    } else {
        let opts: Vec<Vec<String>> = case
            .questions
            .iter()
            .map(|q| {
                if q.kind == QKind::Noul {
                    Vec::new() // the fixed rendering speaks it
                } else {
                    criteria_keys(q)
                }
            })
            .collect();
        let served: Vec<crate::server::ServedQuestion<'_>> = case
            .questions
            .iter()
            .zip(opts.iter())
            .map(|(q, options)| crate::server::ServedQuestion {
                qid: q.qid.as_str(),
                kind: q.kind,
                instructions: q.instructions.as_str(),
                options: options.as_slice(),
            })
            .collect();
        server.decide_multi(state, &served)
    }
}

/// The presented option keys of one question — criteria OBJECT keys in
/// insertion order (the preserve-order law), or the ARRAY elements
/// rendered (score levels). Empty for the Null criteria (noul).
fn criteria_keys(q: &riir_reflex::harness::suites::SuiteQuestion) -> Vec<String> {
    match &q.criteria {
        serde_json::Value::Object(m) => m.keys().cloned().collect(),
        serde_json::Value::Array(a) => a
            .iter()
            .map(|v| match v {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// 8 raw bytes → 16 lowercase hex chars (the wire's fingerprint form).
fn hex8(b: &[u8; 8]) -> String {
    b.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// A display prefix of a wire hash — char-boundary safe (a malformed
/// item hex may carry multibyte bytes; slicing would panic).
fn prefix16(s: &str) -> String {
    s.chars().take(16).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use riir_kat::kat_protocol_decstat_receipt::DecStatLeaseItemWire;

    /// A judge against an EMPTY verifier (no suites booted): the two
    /// skew screens and the corpus-skew miss all resolve without any
    /// data — the loop's screens are pure over the fingerprints.
    fn skew_verifier() -> Verifier {
        Verifier {
            suites: Vec::new(),
            map: HashMap::new(),
            build_fp: [0xAA; 8],
            manifest_fp: [0xBB; 8],
            loaded_manifest: None,
            skipped_suites: Vec::new(),
        }
    }

    fn item(build: &str, manifest: &str, input: &str, decision: &str) -> DecStatLeaseItemWire {
        DecStatLeaseItemWire {
            suite: "ag_news".into(),
            input_hash_hex: input.into(),
            decision_hash_hex: decision.into(),
            build_fp_hex: build.into(),
            manifest_fp_hex: manifest.into(),
        }
    }

    #[test]
    fn toolchain_skew_skips_silent_and_loud() {
        let mut v = skew_verifier();
        // A different release: silent skip, no wire byte.
        let v_skew = v.judge(&item(
            "0000000000000000",
            "1111111111111111",
            &"a".repeat(64),
            &"b".repeat(64),
        ));
        assert_eq!(
            v_skew,
            Verdict::ToolchainSkew {
                item: [0; 8],
                ours: [0xAA; 8]
            }
        );
        assert!(v_skew.is_skipped());
        assert!(v_skew.wire_outcome().is_none());
        // Malformed hex cannot be proven ours — the same conservative skip.
        let v_bad = v.judge(&item(
            "nothex",
            "1111111111111111",
            &"a".repeat(64),
            &"b".repeat(64),
        ));
        assert!(v_bad.is_skipped());
    }

    #[test]
    fn toolchain_match_then_manifest_skew_skips_silent() {
        let mut v = skew_verifier();
        // "Matching" means the verifier's OWN fps — the hand-set
        // [0xAA; 8] / [0xBB; 8], hex-encoded for the wire.
        let build_hex = hex8(&v.build_fp);
        let v_skew = v.judge(&item(
            &build_hex,
            "1111111111111111",
            &"a".repeat(64),
            &"b".repeat(64),
        ));
        assert!(matches!(v_skew, Verdict::ManifestSkew { .. }));
        assert!(v_skew.is_skipped());
        // Matching manifest, unknown input → corpus skew (the D1 miss).
        let manifest_hex = hex8(&v.manifest_fp);
        let v_miss = v.judge(&item(
            &build_hex,
            &manifest_hex,
            &"a".repeat(64),
            &"b".repeat(64),
        ));
        assert_eq!(v_miss, Verdict::CorpusSkew);
        assert!(!v_miss.is_skipped());
        assert_eq!(v_miss.wire_outcome(), Some(DECSTAT_VERDICT_CORPUS_SKEW));
        assert_eq!(v_miss.computed_hash(), [0u8; 32]);
    }

    #[test]
    fn breakdown_records_each_bucket_and_rates_the_reproduction() {
        let mut b = Breakdown::default();
        b.record(&Verdict::Verified([1; 32]));
        b.record(&Verdict::Verified([2; 32]));
        b.record(&Verdict::Mismatch {
            claimed: [3; 32],
            computed: [4; 32],
        });
        b.record(&Verdict::CorpusSkew);
        b.record(&Verdict::ToolchainSkew { item: [0; 8], ours: [1; 8] });
        b.record(&Verdict::ManifestSkew {
            item_hex: "x".into(),
            ours_hex: "y".into(),
        });
        assert_eq!(
            b,
            Breakdown {
                toolchain_skipped: 1,
                manifest_skipped: 1,
                corpus_skew: 1,
                mismatch: 1,
                verified: 2,
            }
        );
        assert!((b.reproduction_rate() - 2.0 / 3.0).abs() < 1e-12);
        assert_eq!(Breakdown::default().reproduction_rate(), 0.0);
    }

    #[test]
    fn the_wire_outcome_vocabulary_is_pinned() {
        assert_eq!(DECSTAT_VERDICT_VERIFIED, 0);
        assert_eq!(DECSTAT_VERDICT_MISMATCH, 1);
        assert_eq!(DECSTAT_VERDICT_CORPUS_SKEW, 2);
        assert_eq!(
            Verdict::Verified([0; 32]).wire_outcome(),
            Some(DECSTAT_VERDICT_VERIFIED)
        );
        assert_eq!(
            Verdict::Mismatch {
                claimed: [0; 32],
                computed: [1; 32]
            }
            .wire_outcome(),
            Some(DECSTAT_VERDICT_MISMATCH)
        );
    }
}
