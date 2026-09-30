//! The serve-side encoder lane (instinct issue 016 T2): the sealed NLEH
//! head over the live laya-english encode, resident from boot, answering
//! through the SAME decide surface the bag lanes serve.
//!
//! ## The laws this module wires (issue 016's ruling, verbatim in effect)
//!
//! - **Resident from boot (Proposal 048 L9, "never loads")**: the head and
//!   the encoder agent load at lane boot, and the first encode (the Metal /
//!   CUDA pipeline compile) is paid at boot as a warmup — never on a
//!   request. A lazy first-route load would be L3 doing a load, which L9
//!   forbids; the manifest validator refuses a `lazy` ENC row outright.
//! - **The agent is !Send — the substrate's own serving law**: reflex's
//!   serve binary loads AND serves the laya agent on ONE worker thread
//!   ("the Metal/objc backend"), handing clients a Send + Sync channel
//!   handle, forwards serialized (one model, one forward at a time). This
//!   lane inherits that law verbatim: the agent lives on the lane's
//!   worker thread; the lane holds the channel handle; the device never
//!   crosses a thread boundary. Forwards serialize — the right posture
//!   when a game fires many spot reads concurrently.
//! - **One serve lane, many consumers; the C1 scope IS the contract**: the
//!   measured +0.0535 LB95 evidence covers the UTTERANCE→SENTIMENT/INTENT
//!   consumer — the suite's CANONICAL question under its CANONICAL option
//!   order. A presentation that reorders, widens, or rewrites the
//!   question is outside the trained distribution and refuses loud — the
//!   head's class rows are option-order-aligned by construction (the
//!   features are per-option marker rows), so a reordered presentation is
//!   not a harder question, it is a DIFFERENT one the head never scored.
//! - **The sync boundary (the latent/raw bridge law)**: only the CLASS /
//!   scalar result crosses — the pick index, the per-class sigmoid scores
//!   (a fixed-size per-presentation vector, the synced analogue of the 5
//!   affect scalars), the confidence, the µs. The encoder's embedding
//!   never leaves the worker (the feature vector crosses ONE process-
//!   internal channel as plain f32, then the head scores it locally).
//! - **The bag server refuses ENC by construction** (`Arm::Enc` never
//!   reaches [`crate::server::SuiteServer`]): the class layers do not mix.
//!
//! The parity gate (`tests/serve_encoder_parity.rs`) replays the frozen
//! Bench-029 read through [`crate::server::AnySuiteServer::boot_bytes`] —
//! the serve path IS the arena path, never a re-derivation.

use std::sync::mpsc;

use riir_reflex::harness::runner::case_questions;
use riir_reflex::harness::runner::seat::Seat;
use riir_reflex::harness::suites::{GoldAnswer, QKind, SuiteCase, SuiteQuestion};
use riir_reflex::laya::config::Checkpoint;
use riir_reflex::laya::riir::{EncodedQuestion, RiirAgent};
use riir_reflex::laya::weights::weights_root;

use crate::encoder_arm::{
    NlehHead, encode_features, load_sealed, nleh_forward, nleh_pick, nleh_standardize,
};
use crate::server::{Arm, ServedDecision, ServedQuestion, SuiteMeta, WeightSource};
use crate::specialist::winner_bridge;

/// One encode job for the worker thread: the state + the template fields
/// (all plain data — the worker re-renders the question bytes itself, the
/// SAME render law, so nothing device-bound ever crosses a thread).
struct EncodeJob {
    state: serde_json::Value,
    qid: String,
    kind: QKind,
    instructions: String,
    options: Vec<String>,
    resp: mpsc::SyncSender<Result<EncodedFeatures, String>>,
}

/// The worker's answer: the C1 feature vector (marker rows ++ pooled mean,
/// plain f32 — Send by construction) + the shape facts the caller's guards
/// check. The embedding NEVER leaves the worker — only this derived,
/// fixed-size vector crosses the process-internal channel.
struct EncodedFeatures {
    x: Vec<f32>,
    n_options: usize,
}

/// The worker's boot report: the device label (the warmup timing prints
/// on the worker itself).
struct WorkerReady {
    device: &'static str,
}

/// One suite's encoder serving lane: the sealed head + the laya-english
/// agent, both resident from boot, plus the suite's canonical question
/// template and presentation.
pub struct EncoderLane {
    suite: &'static str,
    arm: Arm,
    head: NlehHead,
    /// The Send + Sync handle to the worker thread that OWNS the agent
    /// (the reflex serve law: the agent is !Send — the Metal/objc
    /// backend — so it is loaded AND served on one thread, forwards
    /// serialized).
    jobs: mpsc::SyncSender<EncodeJob>,
    /// The device posture label (the agent's own — "metal" / "cuda" /
    /// "cpu" / "ane"), surfaced so a reading can never mistake the box.
    device: &'static str,
    qid: String,
    q_kind: QKind,
    q_instructions: String,
    /// The canonical PRESENTED-option order — the template criteria's own
    /// order (the head's class-row order). NOT the seat's engine label
    /// universe: a score suite's engine labels are index strings ("0"..
    // "4"), while the presented space the head scored is the criteria
    /// list ("very negative"..). The arena encoded the criteria render;
    /// this lane answers that same space or refuses.
    labels: Vec<String>,
    /// The sealed head's digest, first 16 hex (the meta disclosure).
    digest16: String,
    /// The boot summary — stored so the registry's `&SuiteMeta` reads are
    /// uniform with the bag servers (the healthz disclosure shape).
    meta: SuiteMeta,
    centroid: [f32; crate::arsenal_ops::DIM],
    /// Scratch: the head's output, reused per decision (the hot-loop
    /// rule — the feature vector comes back from the worker).
    y: Vec<f32>,
}

impl EncoderLane {
    /// Boot from an ALREADY-READ head artifact (the single-read
    /// discipline: the caller — the serve loader or the swap edge — hashed
    /// these exact bytes for the epoch tag; the manifest validator has
    /// already pinned them against the row digest). The seal law rides the
    /// manifest digest in serve mode; the file-mode seal check
    /// ([`load_sealed`]'s sidecar) is [`Self::from_sealed_file`]'s law.
    pub fn from_parts(
        suite: &'static str,
        seat: Seat,
        head_bytes: &[u8],
        arm: Arm,
    ) -> Result<Self, String> {
        if arm != Arm::Enc {
            return Err(format!(
                "suite {suite}: the encoder lane serves posture ENC only — got {}",
                arm.name()
            ));
        }
        // The serving shape (the bag server's own guard, mirrored): the
        // encoder lane answers the single-question contract. A
        // multi-question suite needs a scoring-shape change first (the
        // typed_decisions note in issue 016's per-suite state).
        if winner_bridge(suite).serves != crate::specialist::ServeContract::SingleQuestion {
            return Err(format!(
                "suite {suite}: posture ENC serves the single-question contract only — a \
                 multi-question suite needs the per-question scoring shape first (issue 016 \
                 scope: typed_decisions NOT screenable as-built)"
            ));
        }
        if seat.suite.cases.iter().any(|c| c.questions.len() != 1) {
            return Err(format!(
                "suite {suite}: posture ENC needs one question per case — a multi-question \
                 seat has no canonical template to answer"
            ));
        }
        let head = crate::encoder_arm::read_nleh_v1(head_bytes)?;
        let q = &seat.suite.cases[0].questions[0];
        let qid = q.qid.clone();
        let q_kind = q.kind;
        let q_instructions = q.instructions.clone();
        // The canonical PRESENTED-option order is the template criteria's
        // own order — NOT the seat's engine label universe (a score
        // suite's engine labels are index strings "0".."4", while the
        // presented space the head scored is the criteria list). The
        // arena encoded the criteria render; this lane answers that same
        // space or refuses.
        let labels = template_options(&q.criteria)
            .ok_or_else(|| {
                format!(
                    "suite {suite}: posture ENC needs a criteria list (object or array) on the \
                     canonical template — a noul template has no presented space for the \
                     encoder class"
                )
            })?;
        if labels.len() != head.n_classes {
            return Err(format!(
                "suite {suite}: the template presents {n} options, the head has {k} classes — \
                 the class space IS the presented space (template drift, refuse loud)",
                n = labels.len(),
                k = head.n_classes
            ));
        }
        let digest16 = blake3::hash(head_bytes).to_hex()[..16].to_string();

        // The worker thread OWNS the agent (the reflex serve law: the
        // agent is !Send — the Metal/objc backend — so it is loaded AND
        // served on one thread, forwards serialized; the lane holds the
        // Send+Sync channel handle). The boot waits for the worker's
        // readiness report: a weights load failure or a warmup shape
        // drift fails the boot LOUD — never a lane that serves from a
        // half-initialized device.
        let (ready_tx, ready_rx) = mpsc::sync_channel::<Result<WorkerReady, String>>(1);
        let (jobs_tx, jobs_rx) = mpsc::sync_channel::<EncodeJob>(64);
        let worker_suite = suite;
        let worker_qid = qid.clone();
        let worker_kind = q_kind;
        let worker_instructions = q_instructions.clone();
        let worker_labels = labels.clone();
        let worker_n_classes = head.n_classes;
        let worker_feat_dim = head.feat_dim;
        let worker = std::thread::Builder::new()
            .name(format!("enc-{worker_suite}"))
            .stack_size(32 * 1024 * 1024)
            .spawn(move || {
                let load = || -> Result<(RiirAgent, &'static str, u128), String> {
                    let agent = RiirAgent::load(&weights_root(), Checkpoint::English)
                        .map_err(|e| format!("laya english load: {e}"))?;
                    let device = agent.device();
                    // The boot warmup: the FIRST encode pays the device
                    // pipeline compile — paid here, never on a request.
                    let t = std::time::Instant::now();
                    let warm_state = serde_json::json!({ "text": "warmup" });
                    let enc = encode_on(&agent, &warm_state, &worker_qid, worker_kind,
                        &worker_instructions, &worker_labels)?;
                    if enc.markers.len() != worker_n_classes {
                        return Err(format!(
                            "the encoder renders {} options, the head has {worker_n_classes} \
                             classes — the canonical template drifted from the trained head; \
                             refuse loud",
                            enc.markers.len()
                        ));
                    }
                    Ok((agent, device, t.elapsed().as_millis()))
                };
                let (agent, device, warm_ms) = match load() {
                    Ok(r) => r,
                    Err(e) => {
                        let _ = ready_tx.send(Err(format!("suite {worker_suite}: {e}")));
                        return;
                    }
                };
                let _ = ready_tx.send(Ok(WorkerReady { device }));
                eprintln!(
                    "[riir-instinct] lane {worker_suite} (ENC): encoder warm — first encode \
                     {warm_ms} ms on {device} (head, {worker_n_classes} classes, feat_dim \
                     {worker_feat_dim})"
                );
                // One model, one forward at a time (the substrate's
                // serving law — also the right posture when a game fires
                // many spot reads concurrently).
                let mut x: Vec<f32> = Vec::new();
                while let Ok(job) = jobs_rx.recv() {
                    let out = encode_on(
                        &agent,
                        &job.state,
                        &job.qid,
                        job.kind,
                        &job.instructions,
                        &job.options,
                    )
                    .and_then(|enc| {
                        if enc.markers.len() != worker_n_classes {
                            return Err(format!(
                                "the encoder renders {} options, the head has \
                                 {worker_n_classes} classes — template drift, refuse loud",
                                enc.markers.len()
                            ));
                        }
                        encode_features(
                            &enc.marker_rows,
                            &enc.hidden,
                            enc.d,
                            enc.markers.len(),
                            &mut x,
                        )?;
                        if x.len() != worker_feat_dim {
                            return Err(format!(
                                "feature dim {} != the head's {worker_feat_dim} — a different \
                                 checkpoint width? refuse loud",
                                x.len()
                            ));
                        }
                        Ok(EncodedFeatures {
                            x: x.clone(),
                            n_options: enc.markers.len(),
                        })
                    })
                    .map_err(|e| format!("encode: {e}"));
                    let _ = job.resp.send(out);
                }
                eprintln!("[riir-instinct] lane {worker_suite} (ENC): worker stopped");
            })
            .map_err(|e| format!("suite {suite}: spawn encoder worker: {e}"))?;
        let ready = ready_rx
            .recv()
            .map_err(|e| format!("suite {suite}: encoder worker died before ready: {e}"))?;
        let ready = match ready {
            Ok(r) => r,
            Err(e) => {
                let _ = worker.join();
                return Err(e);
            }
        };

        // The hoarding-gate vector (the bag server's boot-time work,
        // mirrored): the corpus centroid over the train pool under the
        // suite's own bag convention. The ENC row is eager-only (the
        // validator refuses lazy), so this vector never gates its own
        // load — it keeps the gate's loaded-centroid table honest when a
        // lazy sibling asks.
        let centroid = crate::arsenal_ops::corpus_centroid_with(
            winner_bridge(suite).convention,
            seat.train.iter().map(|d| d.text.as_str()),
        );

        let meta = SuiteMeta {
            suite,
            arm,
            labels: labels.len(),
            artifact_labels: head.n_classes,
            // No bag-cap / scale vocabulary applies to the encoder lane —
            // the zeros are the honest "not a bag lane" reading.
            effective_cap: 0,
            head_scale: 0.0,
            nb_scale: 0.0,
            score_threshold: 0.0,
            distance_threshold: 0.0,
            winner_blake3: digest16.clone(),
            source: WeightSource::RawWinner,
        };

        Ok(Self {
            suite,
            arm,
            head,
            jobs: jobs_tx,
            device: ready.device,
            qid,
            q_kind,
            q_instructions,
            labels,
            digest16,
            meta,
            centroid,
            y: Vec::new(),
        })
    }

    /// The file-mode boot ([`AnySuiteServer::boot`]'s route): the head
    /// from its sealed artifact + the trainer's `.blake3` sidecar — the
    /// seal check the parity gate's law carries. The head parses twice at
    /// this entry (once inside the seal check) — boot-time work, once per
    /// process.
    pub fn from_sealed_file(
        suite: &'static str,
        seat: Seat,
        head_path: &std::path::Path,
        arm: Arm,
    ) -> Result<Self, String> {
        load_sealed(head_path, None)?;
        let bytes =
            std::fs::read(head_path).map_err(|e| format!("read {}: {e}", head_path.display()))?;
        Self::from_parts(suite, seat, &bytes, arm)
    }

    /// The presentation guard — the C1 contract as code: the canonical
    /// kind, the canonical instructions, the canonical qid, and the
    /// canonical option list IN ORDER. Anything else is outside the
    /// measured distribution and refuses loud. (The pure contract lives in
    /// [`validate_presentation_contract`] so the guards test without the
    /// laya weights.)
    fn validate_presentation(&self, q: &ServedQuestion<'_>) -> Result<(), String> {
        validate_presentation_contract(q, self.q_kind, &self.qid, &self.q_instructions, &self.labels)
            .map_err(|e| format!("suite {}: posture ENC refuses the presentation: {e}", self.suite))
    }

    /// One decision under the suite's DEFAULT question template (the bag
    /// server's decide signature — the serve edge calls it identically).
    pub fn decide(
        &mut self,
        state: &str,
        options: Option<&[String]>,
    ) -> Result<ServedDecision, String> {
        let owned: Vec<String> = match options {
            Some(o) => o.to_vec(),
            None => self.labels.clone(),
        };
        let qid = self.qid.clone();
        let instructions = self.q_instructions.clone();
        let kind = self.q_kind;
        let q = [ServedQuestion {
            qid: qid.as_str(),
            kind,
            instructions: instructions.as_str(),
            options: &owned,
        }];
        Ok(self.decide_multi(state, &q)?.remove(0))
    }

    /// The multi-question surface — the C1 scope answers EXACTLY ONE
    /// question per call and refuses wider sets loud (a multi-question
    /// suite needs the scoring-shape change first; issue 016's per-suite
    /// state records the class).
    pub fn decide_multi(
        &mut self,
        state: &str,
        questions: &[ServedQuestion<'_>],
    ) -> Result<Vec<ServedDecision>, String> {
        let t0 = std::time::Instant::now();
        if state.trim().is_empty() {
            return Err("empty state".into());
        }
        if questions.len() != 1 {
            return Err(format!(
                "suite {}: posture ENC answers ONE question per call (the C1 scope, issue 014 \
                 C1) — got {}; a multi-question suite needs the per-question scoring shape \
                 first",
                self.suite,
                questions.len()
            ));
        }
        let q = &questions[0];
        self.validate_presentation(q)?;

        // The wire state is the suite's SERIALIZED-STATE form (the same
        // strings the bag lanes bag and the serve_gates parity callers
        // send — `serialize_state(&case.state)`). The encoder consumes the
        // suite's envelope VALUE: parse the form back and pass the value,
        // so `serialize_state` re-derives the arena's exact prompt bytes
        // on the worker. A string that does not parse is outside the C1
        // convention — refuse loud, never embed prose the head never
        // scored.
        let state_value: serde_json::Value = serde_json::from_str(state).map_err(|_| {
            format!(
                "suite {}: posture ENC consumes the suite's serialized-state form (the \
                 serialize_state string of the suite envelope) — the request's state does not \
                 parse as JSON, so there is no envelope to encode",
                self.suite
            )
        })?;

        // The encode runs on the worker (the agent never crosses a
        // thread); the feature vector comes back as plain f32. A dead
        // worker refuses LOUD — never a silent empty answer.
        let (rtx, rrx) = mpsc::sync_channel(1);
        let job = EncodeJob {
            state: state_value,
            qid: q.qid.to_string(),
            kind: q.kind,
            instructions: q.instructions.to_string(),
            options: q.options.to_vec(),
            resp: rtx,
        };
        self.jobs
            .send(job)
            .map_err(|_| format!("suite {}: the encoder worker is gone — the lane failed", self.suite))?;
        let feats = rrx
            .recv()
            .map_err(|_| {
                format!("suite {}: the encoder worker dropped the reply — the lane failed", self.suite)
            })??;
        if feats.n_options != self.head.n_classes {
            return Err(format!(
                "suite {}: the encoder rendered {} options, the head has {} classes — template \
                 drift, refuse loud",
                self.suite,
                feats.n_options,
                self.head.n_classes
            ));
        }
        if feats.x.len() != self.head.feat_dim {
            return Err(format!(
                "suite {}: feature dim {} != the head's {} — a different checkpoint width? \
                 refuse loud",
                self.suite,
                feats.x.len(),
                self.head.feat_dim
            ));
        }
        let mut x = feats.x;
        nleh_standardize(&self.head, &mut x);
        nleh_forward(&self.head, &x, &mut self.y);
        let pick = nleh_pick(&self.y);
        let options = q.options.to_vec();
        let confidence = f64::from(self.y[pick]);
        let us = u64::try_from(t0.elapsed().as_micros()).unwrap_or(u64::MAX);
        Ok(vec![ServedDecision {
            suite: self.suite,
            arm: self.arm.name(),
            options,
            pick_index: Some(pick),
            pick: Some(q.options[pick].clone()),
            // The sync boundary: the per-class sigmoid scores (fixed-size,
            // one per presented option) and the scalar confidence cross —
            // the embedding never does.
            probabilities: None,
            specialist_scores: Some(self.y.clone()),
            confidence,
            // The deep class answered: the L3 think op is the escalation.
            escalated: true,
            abstained: false,
            us,
        }])
    }

    pub fn suite(&self) -> &'static str {
        self.suite
    }

    pub fn arm(&self) -> &Arm {
        &self.arm
    }

    /// The device posture label ("metal" / "cuda" / "cpu" / "ane").
    pub fn device(&self) -> &'static str {
        self.device
    }

    /// The boot summary — the same &SuiteMeta read the bag servers serve
    /// (the healthz disclosure shape).
    pub fn meta(&self) -> &SuiteMeta {
        &self.meta
    }

    /// The suite's corpus centroid — the hoarding gate's vector (the bag
    /// server's boot-time work, mirrored; the ENC row is eager-only so
    /// this never gates its own load).
    #[must_use]
    pub fn centroid(&self) -> [f32; crate::arsenal_ops::DIM] {
        self.centroid
    }

    /// The canonical presented-option order (the head's class-row order).
    pub fn labels(&self) -> &[String] {
        &self.labels
    }

    /// The sealed head's digest, first 16 hex.
    pub fn head_digest16(&self) -> &str {
        &self.digest16
    }

    /// Borrow the parsed head (the staleness probe's consumer shape).
    pub fn head(&self) -> &NlehHead {
        &self.head
    }
}

/// The synthesized single-question case — the SAME construction the bag
/// server's `decide_multi` uses for the wire's questions (the criteria
/// shapes per kind are the shared render law), so the encoder's question
/// bytes are the arena's by one code path.
fn synth_case_value(
    state: &serde_json::Value,
    qid: &str,
    kind: QKind,
    instructions: &str,
    options: &[String],
) -> SuiteCase {
    let criteria = match kind {
        QKind::Choice => {
            let mut m = serde_json::Map::new();
            for key in options {
                m.insert(key.clone(), serde_json::Value::Null);
            }
            serde_json::Value::Object(m)
        }
        QKind::Score => serde_json::Value::Array(
            options
                .iter()
                .map(|k| serde_json::Value::String(k.clone()))
                .collect(),
        ),
        QKind::Noul => serde_json::Value::Null,
    };
    SuiteCase {
        id: "served".into(),
        state: state.clone(),
        questions: vec![SuiteQuestion {
            qid: qid.to_string(),
            kind,
            instructions: instructions.to_string(),
            criteria,
        }],
        gold: vec![GoldAnswer {
            idx: 0,
            soft: vec![],
            gold_score: None,
        }],
    }
}

/// The canonical presented-option keys of a template criteria value, in
/// the value's own order (the arena's `case_option_keys` law): a Choice
/// object's keys, a Score array's strings. `None` when the template has
/// no presented space (noul / null).
fn template_options(criteria: &serde_json::Value) -> Option<Vec<String>> {
    match criteria {
        serde_json::Value::Object(m) => Some(m.keys().cloned().collect()),
        serde_json::Value::Array(a) => Some(
            a.iter()
                .map(|v| match v {
                    serde_json::Value::String(s) => s.clone(),
                    other => other.to_string(),
                })
                .collect(),
        ),
        _ => None,
    }
}

/// One encode of `state` under a question rendered from the given template
/// fields — the EXACT bytes the arena's frozen read encoded (the seat's own
/// `case_questions` law; the synthesized case carries the template so the
/// render is byte-identical by construction). Runs on the worker thread
/// (the agent never crosses one) and at boot warmup. The state arrives as
/// the suite's own envelope Value (reconstructed from the wire's
/// serialized-state form — see [`EncoderLane::decide_multi`]).
fn encode_on(
    agent: &RiirAgent,
    state: &serde_json::Value,
    qid: &str,
    kind: QKind,
    instructions: &str,
    options: &[String],
) -> Result<EncodedQuestion, String> {
    let case = synth_case_value(state, qid, kind, instructions, options);
    let qs = case_questions(&case);
    if qs.len() != 1 {
        return Err(format!(
            "the question render produced {} questions — the single-question contract broke at \
             the render itself",
            qs.len()
        ));
    }
    agent
        .encode_question(&case.state, &qs[0].1)
        .map_err(|e| format!("encode: {e}"))
}

/// The presentation contract, pure over its inputs — the lane method
/// delegates; the tests run it without the laya weights.
fn validate_presentation_contract(
    q: &ServedQuestion<'_>,
    q_kind: QKind,
    qid: &str,
    instructions: &str,
    labels: &[String],
) -> Result<(), String> {
    if q.kind == QKind::Noul {
        return Err("a noul question has no trained head rows (the C1 class space is the \
                    presented-option space)"
            .into());
    }
    if q.kind != q_kind {
        return Err(format!(
            "kind drift: canonical {q_kind:?}, presented {:?}",
            q.kind
        ));
    }
    if q.instructions != instructions {
        return Err("instructions drift: a custom question text is an encoder input the head \
                    never scored (the C1 certification scope: the utterance→sentiment/intent \
                    consumer, issue 016)"
            .into());
    }
    if q.qid != qid {
        return Err(format!("qid drift: canonical {qid:?}, presented {:?}", q.qid));
    }
    if q.options != labels {
        return Err(format!(
            "presentation drift: the head's class rows are option-order-aligned (per-option \
             marker rows), so a reordered or resized presentation is a DIFFERENT question the \
             head never scored — presented {pret:?} vs canonical {canon:?}",
            pret = q.options,
            canon = labels
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const KIND: QKind = QKind::Score;
    const QID: &str = "sentiment";
    const INSTR: &str = "Classify the review.";

    fn labels() -> Vec<String> {
        ["very_negative", "negative", "neutral", "positive", "very_positive"]
            .iter()
            .map(|s| (*s).to_string())
            .collect()
    }

    fn q<'a>(
        kind: QKind,
        qid: &'a str,
        instructions: &'a str,
        options: &'a [String],
    ) -> ServedQuestion<'a> {
        ServedQuestion {
            qid,
            kind,
            instructions,
            options,
        }
    }

    #[test]
    fn canonical_presentation_passes() {
        let labels = labels();
        let s = q(KIND, QID, INSTR, &labels);
        assert!(validate_presentation_contract(&s, KIND, QID, INSTR, &labels).is_ok());
    }

    #[test]
    fn reordered_presentation_refuses() {
        let mut presented = labels();
        presented.reverse();
        let canonical = labels();
        let s = q(KIND, QID, INSTR, &presented);
        let err = validate_presentation_contract(&s, KIND, QID, INSTR, &canonical).unwrap_err();
        assert!(err.contains("presentation drift"), "{err}");
    }

    #[test]
    fn resized_presentation_refuses() {
        let mut presented = labels();
        presented.pop();
        let canonical = labels();
        let s = q(KIND, QID, INSTR, &presented);
        let err = validate_presentation_contract(&s, KIND, QID, INSTR, &canonical).unwrap_err();
        assert!(err.contains("presentation drift"), "{err}");
    }

    #[test]
    fn noul_presentation_refuses() {
        let labels = labels();
        let s = q(QKind::Noul, QID, INSTR, &[]);
        let err = validate_presentation_contract(&s, KIND, QID, INSTR, &labels).unwrap_err();
        assert!(err.contains("noul"), "{err}");
    }

    #[test]
    fn custom_instructions_refuse() {
        let labels = labels();
        let s = q(KIND, QID, "Rate this product.", &labels);
        let err = validate_presentation_contract(&s, KIND, QID, INSTR, &labels).unwrap_err();
        assert!(err.contains("instructions drift"), "{err}");
    }

    #[test]
    fn foreign_kind_refuses() {
        let labels = labels();
        let s = q(QKind::Choice, QID, INSTR, &labels);
        let err = validate_presentation_contract(&s, KIND, QID, INSTR, &labels).unwrap_err();
        assert!(err.contains("kind drift"), "{err}");
    }

    #[test]
    fn foreign_qid_refuses() {
        let labels = labels();
        let s = q(KIND, "other_qid", INSTR, &labels);
        let err = validate_presentation_contract(&s, KIND, QID, INSTR, &labels).unwrap_err();
        assert!(err.contains("qid drift"), "{err}");
    }
}
