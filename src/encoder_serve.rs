//! The serve-side encoder lane (instinct issue 016 T2): the sealed NLEH
//! head over the live laya-english encode, resident from boot, answering
//! through the SAME decide surface the bag lanes serve.
//!
//! ## The laws this module wires (issue 016's ruling, verbatim in effect)
//!
//! - **Resident-after-load; `eager` stays the DEFAULT posture (issue 018
//!   Lane A's L9 revisit)**: the head and the encoder agent load ONCE per
//!   activation — at boot for an eager row, at the first decision for a
//!   `lazy` row (the 503 `loading` window covers the load; the boot
//!   preflight already refused every config error the lazy row can have).
//!   L2's lazy-once-then-resident contract is inherited VERBATIM; L9's
//!   residue survives as G3 (exactly one load per activation — never a
//!   per-decision load) and the sticky-`Failed` bound (a failed load
//!   never retries on its own). The manifest validator accepts
//!   `eager | lazy` for ENC rows; the embedded default stays eager.
//! - **The agent is !Send — the substrate's own serving law**: reflex's
//!   serve binary loads AND serves the laya agent on ONE worker thread
//!   ("the Metal/objc backend"), handing clients a Send + Sync channel
//!   handle, forwards serialized (one model, one forward at a time). This
//!   lane inherits that law verbatim: the agent lives on a WORKER thread
//!   (issue 018 Lane B: ONE worker per checkpoint — the unit of residency
//!   is the worker, not the lane; every ENC lane on the checkpoint shares
//!   it — `serve-encoder-shared` — or keeps a private one, the per-lane
//!   default); the lane holds the channel handle; the device never
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
    encode_features, load_sealed, nleh_forward, nleh_pick, nleh_standardize, NlehHead,
};
use crate::server::{Arm, ServedDecision, ServedQuestion, SuiteMeta, WeightSource};
use crate::specialist::winner_bridge;

/// The backend half of the bytes boot (the hook [`crate::server`]
/// consults for ENC rows): the raw head bytes → the lane. The bytes ARE
/// the sealed head — the lane's own parse law consumes them.
pub fn enc_boot_bytes(
    suite: &'static str,
    seat: Seat,
    artifact_bytes: &[u8],
    arm: Arm,
) -> Result<Box<dyn crate::server::LaneBackend>, String> {
    let lane = EncoderLane::from_parts(suite, seat, artifact_bytes, arm)?;
    Ok(Box::new(lane))
}

/// The backend half of the vessel boot (issue 016 T4): the HOSTED-ONLY
/// head vessel — the same authenticity / class / monotonic walk the bag
/// vessels carry ([`crate::vessel::load_hosted_head_bytes`]), the RAW
/// decrypted NLEH payload handed to the lane's own parse law. The
/// monotonic apply is the arsenal's existing epoch machinery: the facts
/// (commitment, version, parent) flow to the slot install exactly as the
/// bag facts do, so a swap/downgrade is refused by the same gate that
/// refuses the bag vessels' — no new lineage code. (Moved verbatim from
/// server.rs at the Proposal-052 carve seam; this module is the moat's
/// boot half.)
#[cfg(feature = "vessel")]
#[allow(clippy::too_many_arguments)]
pub fn enc_boot_vessel(
    suite: &'static str,
    seat: Seat,
    vessel_bytes: &[u8],
    pins: &reflexer_vessel::PinTable,
    key: &[u8; 32],
    applied: &crate::vessel::AppliedState,
    arm: Arm,
) -> Result<
    (
        Box<dyn crate::server::LaneBackend>,
        crate::server::VesselFacts,
    ),
    String,
> {
    let loaded = crate::vessel::load_hosted_head_bytes(vessel_bytes, pins, key, applied)
        .map_err(|e| format!("suite {suite}: head vessel refused: {e}"))?;
    let lane = EncoderLane::from_parts(suite, seat, &loaded.head_bytes, arm)?;
    let facts = crate::server::VesselFacts {
        commitment_hex: loaded.commitment_hex,
        artifact_version: loaded.artifact_version,
        parent_commitment: loaded.parent_commitment,
    };
    Ok((Box::new(lane), facts))
}

/// The extension-point contract ([`crate::server::LaneBackend`]) — the
/// encoder lane's dispatch is the same four methods the bag variants
/// expose, so the arity-erased server treats the classes uniformly (the
/// fully-qualified calls are the inherent methods — the trait's names
/// shadow them inside this impl).
impl crate::server::LaneBackend for EncoderLane {
    fn decide(
        &mut self,
        state: &str,
        options: Option<&[String]>,
    ) -> Result<ServedDecision, String> {
        EncoderLane::decide(self, state, options)
    }

    fn decide_multi(
        &mut self,
        state: &str,
        questions: &[ServedQuestion<'_>],
    ) -> Result<Vec<ServedDecision>, String> {
        EncoderLane::decide_multi(self, state, questions)
    }

    fn meta(&self) -> &SuiteMeta {
        EncoderLane::meta(self)
    }

    fn centroid(&self) -> [f32; crate::arsenal_ops::DIM] {
        EncoderLane::centroid(self)
    }
}

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

/// The worker's boot report: the device label + the CHECKPOINT DIGEST
/// (issue 018 Lane B: the worker carries the checkpoint digest; the epoch
/// stays per-lane with its head).
#[derive(Clone)]
struct WorkerBoot {
    device: &'static str,
    checkpoint_digest16: String,
}

// ── the shared encode worker (issue 018 Lane B) ────────────────────────
// The unit of residency moves from the LANE to the worker. One worker
// thread + one agent per REGISTRY KEY: `ckpt:<checkpoint>` under
// `serve-encoder-shared` (every ENC lane on the checkpoint shares it —
// N × 848 MB → 1 × 848 MB), or `lane:<suite>:<head-digest>` in the
// default build (byte-for-byte today's per-lane worker, same code path).
// The registry stores WEAK handles: the lanes own the worker's residency,
// so releasing the LAST lane frees the RAM (Lane A's release semantics)
// and the next attach after that pays one fresh load — exactly one load
// per ACTIVATION (G3). Creation (load + boot warmup + ready wait) runs
// UNDER the registry lock, so two lanes waking cold on the same key
// serialize and the load runs exactly once.

/// The per-lane in-flight bound — the old `sync_channel(64)` back-pressure,
/// carried into the shared-worker world so one bursty lane can occupy at
/// most this many slots of the shared queue (the fairness/queue gate).
const LANE_INFLIGHT_CAP: usize = 64;
/// The worker's job queue capacity (per WORKER; the per-lane cap bounds
/// each lane's contribution).
const WORKER_QUEUE_CAP: usize = 256;

/// The registry entry: a weak sender + the boot facts a later attach
/// reads without re-loading (an upgraded handle serves; a dead weak
/// spawns a fresh worker).
struct WorkerEntry {
    jobs: std::sync::Weak<mpsc::SyncSender<EncodeJob>>,
    boot: WorkerBoot,
}

/// What a lane gets from the registry: the shared sender handle (Arc —
/// the LAST drop ends the worker thread and frees the agent) + the boot
/// facts.
#[derive(Clone)]
struct WorkerHandle {
    jobs: std::sync::Arc<mpsc::SyncSender<EncodeJob>>,
    boot: WorkerBoot,
}

/// The worker's boot body: load the agent, pay the pipeline compile, and
/// report the device + checkpoint digest. Generic so the registry gates
/// (`worker_tests`) exercise exactly-once / release semantics WITHOUT the
/// laya weights.
type WorkerBootFn<A> = Box<dyn FnOnce() -> Result<(A, WorkerBoot), String> + Send>;
/// The worker's per-job body: encode one job on the agent into the reused
/// scratch buffer. Runs serialized on the worker thread.
type WorkerEncodeFn<A> =
    Box<dyn Fn(&A, &EncodeJob, &mut Vec<f32>) -> Result<EncodedFeatures, String> + Send + Sync>;

fn worker_registry() -> &'static std::sync::Mutex<std::collections::HashMap<String, WorkerEntry>> {
    static WORKERS: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<String, WorkerEntry>>,
    > = std::sync::OnceLock::new();
    WORKERS.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

/// The promoted ENC topology (issue 018, the serving-soak promotion):
/// under `serve-encoder-shared` the registry keys ONE encode worker per
/// checkpoint — and the documented demote switch, `RIIR_INSTINCT_ENCODER_SHARED=0`
/// (the exact literal), bit-restores the per-lane keys. Serving decisions
/// are byte-identical under both topologies (the 0050 fingerprint law + the
/// 0054 through-HTTP parity); the switch exists for ops and for the A/B
/// instrument's per-lane arm post-promotion, not as a perf knob. Read at
/// lane attach only (a handful of calls per boot) — never on the decide
/// path.
#[cfg(feature = "serve-encoder-shared")]
fn shared_topology_enabled() -> bool {
    std::env::var("RIIR_INSTINCT_ENCODER_SHARED").ok().as_deref() != Some("0")
}

/// The effective worker-topology label for gates and records:
/// "shared-worker" under the shared feature without the demote switch,
/// else "per-lane".
#[cfg(feature = "serve-encoder")]
pub fn encoder_topology_label() -> &'static str {
    #[cfg(feature = "serve-encoder-shared")]
    {
        if shared_topology_enabled() {
            return "shared-worker";
        }
    }
    "per-lane"
}

/// The registry key for one lane's worker. Shared mode keys the
/// CHECKPOINT (every lane on it shares one worker); per-lane keys the
/// lane (a swapped head gets a fresh worker — per-lane semantics
/// preserved exactly). The topology is the feature's, demotable at
/// runtime by [`shared_topology_enabled`]'s switch.
fn worker_key(suite: &'static str, head_digest16: &str) -> String {
    #[cfg(feature = "serve-encoder-shared")]
    {
        if !shared_topology_enabled() {
            let _ = head_digest16;
            return format!("lane:{suite}");
        }
        let _ = (suite, head_digest16);
        format!("ckpt:{ENC_CHECKPOINT_LABEL}")
    }
    #[cfg(not(feature = "serve-encoder-shared"))]
    {
        let _ = head_digest16;
        format!("lane:{suite}")
    }
}

/// Get-or-spawn one worker for `key`. Creation holds the registry lock
/// through the ready wait — co-key loads serialize and run EXACTLY once
/// (G3). `A` is the agent type (`RiirAgent` in production; `()` in the
/// weight-free registry gates).
fn get_or_spawn_worker<A: 'static>(
    key: &str,
    boot: WorkerBootFn<A>,
    encode: WorkerEncodeFn<A>,
) -> Result<WorkerHandle, String> {
    let mut map = worker_registry()
        .lock()
        .map_err(|e| format!("encoder worker registry poisoned: {e}"))?;
    if let Some(entry) = map.get(key) {
        if let Some(jobs) = entry.jobs.upgrade() {
            return Ok(WorkerHandle {
                jobs,
                boot: WorkerBoot {
                    device: entry.boot.device,
                    checkpoint_digest16: entry.boot.checkpoint_digest16.clone(),
                },
            });
        }
        // Dead entry (every lane released): drop it and spawn fresh — the
        // re-activation pays ONE load, by construction.
        map.remove(key);
    }
    let (jobs_tx, jobs_rx) = mpsc::sync_channel::<EncodeJob>(WORKER_QUEUE_CAP);
    let (ready_tx, ready_rx) = mpsc::sync_channel::<Result<WorkerBoot, String>>(1);
    let jobs_arc = std::sync::Arc::new(jobs_tx);
    let label = format!("enc-worker-{key}");
    let thread_key = key.to_string();
    let thread = std::thread::Builder::new()
        .name(label)
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            let key: String = thread_key;
            let (agent, boot_facts) = match boot() {
                Ok(r) => r,
                Err(e) => {
                    let _ = ready_tx.send(Err(e));
                    return;
                }
            };
            let warm = WorkerBoot {
                device: boot_facts.device,
                checkpoint_digest16: boot_facts.checkpoint_digest16.clone(),
            };
            let _ = ready_tx.send(Ok(warm));
            eprintln!(
                "[riir-instinct] encoder worker {key}: warm — {} (checkpoint digest {})",
                boot_facts.device, boot_facts.checkpoint_digest16
            );
            // One model, one forward at a time (the substrate's serving
            // law — also the right posture when a game fires many spot
            // reads concurrently). The worker is TEMPLATE-AGNOSTIC: the
            // template fields ride each job, and the CALLING LANE validates
            // the returned shape against its own head.
            let mut x: Vec<f32> = Vec::new();
            while let Ok(job) = jobs_rx.recv() {
                let out = encode(&agent, &job, &mut x);
                let _ = job.resp.send(out);
            }
            eprintln!("[riir-instinct] encoder worker {key}: stopped (last holder released)");
        })
        .map_err(|e| format!("spawn encoder worker {key}: {e}"))?;
    // The ready wait runs UNDER the registry lock (exactly-once G3). A
    // failed boot removes the entry and joins the dead thread.
    let ready = match ready_rx.recv() {
        Ok(r) => r,
        Err(e) => Err(format!("encoder worker {key} died before ready: {e}")),
    };
    match ready {
        Ok(boot_facts) => {
            let entry = WorkerEntry {
                jobs: std::sync::Arc::downgrade(&jobs_arc),
                boot: WorkerBoot {
                    device: boot_facts.device,
                    checkpoint_digest16: boot_facts.checkpoint_digest16.clone(),
                },
            };
            map.insert(key.to_string(), entry);
            Ok(WorkerHandle {
                jobs: jobs_arc,
                boot: boot_facts,
            })
        }
        Err(e) => {
            map.remove(key);
            drop(map);
            let _ = thread.join();
            Err(e)
        }
    }
}

/// The per-lane in-flight gate: `acquire` blocks while the lane already
/// has `LANE_INFLIGHT_CAP` jobs outstanding (the old bounded-channel
/// back-pressure, one lane cannot starve the shared worker's queue).
struct LaneGate {
    n: std::sync::Mutex<usize>,
    cv: std::sync::Condvar,
}

struct GateGuard<'a> {
    gate: &'a LaneGate,
}

impl LaneGate {
    fn new() -> Self {
        Self {
            n: std::sync::Mutex::new(0),
            cv: std::sync::Condvar::new(),
        }
    }

    fn acquire(&self) -> GateGuard<'_> {
        let mut n = self.n.lock().expect("lane gate lock");
        while *n >= LANE_INFLIGHT_CAP {
            n = self.cv.wait(n).expect("lane gate lock");
        }
        *n += 1;
        GateGuard { gate: self }
    }
}

impl Drop for GateGuard<'_> {
    fn drop(&mut self) {
        let mut n = self.gate.n.lock().expect("lane gate lock");
        *n -= 1;
        self.gate.cv.notify_one();
    }
}

/// The REAL worker boot: load the laya-english agent, hash the resolved
/// weights artifact (the worker-carried checkpoint digest), and pay the
/// pipeline compile on a TEMPLATE-AGNOSTIC warmup encode (the per-lane
/// template validation moved to lane ATTACH — the worker no longer knows
/// a suite's template at boot).
fn real_worker_boot() -> WorkerBootFn<RiirAgent> {
    Box::new(|| {
        let agent = RiirAgent::load(&weights_root(), Checkpoint::English)
            .map_err(|e| format!("laya english load: {e}"))?;
        let device = agent.device();
        let (weights_path, variant) = resolved_weights_facts()?;
        let digest16 = stream_blake3_16(&weights_path)?;
        let t = std::time::Instant::now();
        let warm = ["warmup".to_string(), "warmup".to_string()];
        encode_on(
            &agent,
            &serde_json::json!({ "text": "warmup" }),
            "warmup",
            QKind::Choice,
            "",
            &warm,
        )?;
        eprintln!(
            "[riir-instinct] encoder worker: pipeline warm {} ms ({} posture)",
            t.elapsed().as_millis(),
            variant.unwrap_or("f16")
        );
        Ok((
            agent,
            WorkerBoot {
                device,
                checkpoint_digest16: digest16,
            },
        ))
    })
}

/// The REAL per-job body: the exact encode the lane always ran, now a
/// closure the shared loop owns.
fn real_worker_encode() -> WorkerEncodeFn<RiirAgent> {
    Box::new(|agent: &RiirAgent, job: &EncodeJob, x: &mut Vec<f32>| {
        let enc = encode_on(
            agent,
            &job.state,
            &job.qid,
            job.kind,
            &job.instructions,
            &job.options,
        )?;
        encode_features(&enc.marker_rows, &enc.hidden, enc.d, enc.markers.len(), x)?;
        Ok(EncodedFeatures {
            x: x.clone(),
            n_options: enc.markers.len(),
        })
    })
}

/// The resolved weights artifact under the ACTIVE variant env (the
/// loader's own resolution — `LAYA_WEIGHTS_VARIANT=q8|q4`), with a
/// variant label for the disclosures.
fn resolved_weights_facts() -> Result<(std::path::PathBuf, Option<&'static str>), String> {
    let root = weights_root();
    let sub = Checkpoint::English.subfolder();
    let dir = root.join(sub);
    let (path, posture) = riir_reflex::laya::riir::q8_artifact::resolve_weights_posture(&dir, sub)
        .map_err(|e| format!("resolve weights variant: {e}"))?;
    let label = posture.map(|p| match p {
        riir_reflex::laya::riir::WeightPosture::Q8Artifact => "q8",
        riir_reflex::laya::riir::WeightPosture::Q4Artifact => "q4",
        _ => "f16",
    });
    Ok((path, label))
}

/// Stream-BLAKE3 a file, first 16 hex (the worker-carried checkpoint
/// digest; streaming so the 850 MB class never materializes).
fn stream_blake3_16(path: &std::path::Path) -> Result<String, String> {
    use std::io::Read as _;
    let mut f = std::fs::File::open(path).map_err(|e| format!("open {}: {e}", path.display()))?;
    let mut h = blake3::Hasher::new();
    let mut buf = [0u8; 1 << 20];
    loop {
        let n = f
            .read(&mut buf)
            .map_err(|e| format!("read {}: {e}", path.display()))?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(h.finalize().to_hex()[..16].to_string())
}

/// The ENC checkpoint label — ONE checkpoint serves today (issue 018
/// Lane C's english retrain measured NEGATIVE; the typed head stays on
/// the typed checkpoint in its record-only lane).
pub const ENC_CHECKPOINT_LABEL: &str = "english";

/// The weights half of the lazy-row boot preflight (issue 018 Lane A):
/// every pinned weights file must be PRESENT and digest-matching (the
/// substrate's verify-only half — never downloads), and the ACTIVE
/// variant artifact must resolve with its sidecar. The returned facts
/// feed the memory-budget check (per SHARED worker: distinct checkpoints
/// × artifact bytes, never per lane).
pub struct EncWeightsFacts {
    pub weights_path: std::path::PathBuf,
    pub bytes: u64,
    pub digest16: String,
    pub variant: Option<&'static str>,
}

pub fn preflight_enc_weights() -> Result<EncWeightsFacts, String> {
    let root = weights_root();
    riir_reflex::laya::weights::verify_checkpoint_present(&root, Checkpoint::English)
        .map_err(|e| format!("ENC weights preflight: {e}"))?;
    let (weights_path, variant) = resolved_weights_facts()?;
    let bytes = std::fs::metadata(&weights_path)
        .map_err(|e| format!("stat {}: {e}", weights_path.display()))?
        .len();
    let digest16 = stream_blake3_16(&weights_path)?;
    Ok(EncWeightsFacts {
        weights_path,
        bytes,
        digest16,
        variant,
    })
}

/// The template half of the lazy-row boot preflight (issue 018 Lane A):
/// the head parses, the seat carries ONE canonical question, and the
/// template's presented-option count matches the head's class rows —
/// all WITHOUT a forward pass. Returns the head's class count (the
/// memory-budget receipt's second term). The full seat is rebuilt at the
/// lane's first load (lazy) — disclosed, boot-time cost only.
pub fn preflight_enc_template(
    suite: &'static str,
    head_bytes: &[u8],
    seat: &Seat,
) -> Result<usize, String> {
    let head = crate::encoder_arm::read_nleh_v1(head_bytes)?;
    let facts = head_template_facts(&head, seat, suite)?;
    let _ = facts;
    Ok(head.n_classes)
}

/// The canonical-template facts one lane answers under — shared by the
/// lane boot ([`EncoderLane::from_parts`]) and the lazy preflight
/// ([`preflight_enc_template`]).
fn head_template_facts(
    head: &NlehHead,
    seat: &Seat,
    suite: &'static str,
) -> Result<(String, QKind, String, Vec<String>), String> {
    if seat.suite.cases.iter().any(|c| c.questions.len() != 1) {
        return Err(format!(
            "suite {suite}: posture ENC needs one question per case — a multi-question \
             seat has no canonical template to answer"
        ));
    }
    let q = &seat.suite.cases[0].questions[0];
    // The canonical PRESENTED-option order is the template criteria's
    // own order — NOT the seat's engine label universe (a score suite's
    // engine labels are index strings "0".."4", while the presented
    // space the head scored is the criteria list). The arena encoded the
    // criteria render; this lane answers that same space or refuses.
    let labels = template_options(&q.criteria).ok_or_else(|| {
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
    Ok((q.qid.clone(), q.kind, q.instructions.clone(), labels))
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
    /// serialized). Issue 018 Lane B: the handle is SHARED — every lane
    /// on the same registry key holds an Arc clone of the same sender, and
    /// the LAST drop ends the worker (RAM freed; issue 018's release
    /// semantics).
    jobs: std::sync::Arc<mpsc::SyncSender<EncodeJob>>,
    /// The per-lane in-flight bound (issue 018 Lane B's fairness gate:
    /// one lane occupies at most `LANE_INFLIGHT_CAP` slots of the shared
    /// queue — the old bounded-channel back-pressure, preserved).
    gate: LaneGate,
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
        let (qid, q_kind, q_instructions, labels) = head_template_facts(&head, &seat, suite)?;
        let digest16 = blake3::hash(head_bytes).to_hex()[..16].to_string();

        // The worker OWNS the agent (the reflex serve law: the agent is
        // !Send — the Metal/objc backend — so it is loaded AND served on
        // one thread, forwards serialized; the lane holds the Send+Sync
        // channel handle). Issue 018 Lane B: the registry keys the worker
        // by checkpoint (shared feature) or by lane+head (the default) —
        // creation holds the registry lock through the ready wait, so a
        // load failure or a warmup shape drift fails THIS lane's boot LOUD
        // (or the shared worker's first activation), never a lane that
        // serves from a half-initialized device.
        let worker = get_or_spawn_worker(
            &worker_key(suite, &digest16),
            real_worker_boot(),
            real_worker_encode(),
        )?;

        // Lane ATTACH validation (issue 018 Lane B: the worker no longer
        // knows a suite's template at boot — the template fields ride each
        // job, and the lane validates its own head's shape here). A
        // template drift fails THIS lane's attach loudly, never the shared
        // worker; a failed attach leaves the worker serving its other
        // lanes.
        let (rtx, rrx) = mpsc::sync_channel(1);
        let attach_job = EncodeJob {
            state: serde_json::json!({ "text": "warmup" }),
            qid: qid.clone(),
            kind: q_kind,
            instructions: q_instructions.clone(),
            options: labels.clone(),
            resp: rtx,
        };
        worker
            .jobs
            .send(attach_job)
            .map_err(|_| format!("suite {suite}: the encoder worker is gone at attach"))?;
        let attached = rrx.recv().map_err(|e| {
            format!("suite {suite}: the encoder worker dropped the attach reply: {e}")
        })??;
        if attached.n_options != labels.len() || attached.x.len() != head.feat_dim {
            return Err(format!(
                "suite {suite}: attach validation refused — the encoder rendered {} options / \
                 {} features, the head has {} classes / {} feat_dim (template drift, refuse \
                 loud; the shared worker keeps serving its other lanes)",
                attached.n_options,
                attached.x.len(),
                labels.len(),
                head.feat_dim
            ));
        }
        eprintln!(
            "[riir-instinct] lane {suite} (ENC): attached to encoder worker (device {}, \
             head {digest16}, feat_dim {}) — {} option(s), attach encode ok",
            worker.boot.device,
            head.feat_dim,
            labels.len()
        );

        // The hoarding-gate vector (the bag server's boot-time work,
        // mirrored): the corpus centroid over the train pool under the
        // suite's own bag convention. A lazy ENC row's hoard vector rides
        // the same path — it never gates its own load (the gate's
        // loaded-centroid table reads READY lanes only).
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
            jobs: worker.jobs,
            gate: LaneGate::new(),
            device: worker.boot.device,
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
        validate_presentation_contract(
            q,
            self.q_kind,
            &self.qid,
            &self.q_instructions,
            &self.labels,
        )
        .map_err(|e| {
            format!(
                "suite {}: posture ENC refuses the presentation: {e}",
                self.suite
            )
        })
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
        // thread); the feature vector comes back as plain f32. The lane's
        // in-flight gate bounds its share of the (possibly shared) queue
        // at `LANE_INFLIGHT_CAP` — the fairness gate — and a dead worker
        // refuses LOUD — never a silent empty answer.
        let _flight = self.gate.acquire();
        let (rtx, rrx) = mpsc::sync_channel(1);
        let job = EncodeJob {
            state: state_value,
            qid: q.qid.to_string(),
            kind: q.kind,
            instructions: q.instructions.to_string(),
            options: q.options.to_vec(),
            resp: rtx,
        };
        self.jobs.send(job).map_err(|_| {
            format!(
                "suite {}: the encoder worker is gone — the lane failed",
                self.suite
            )
        })?;
        let feats = rrx.recv().map_err(|_| {
            format!(
                "suite {}: the encoder worker dropped the reply — the lane failed",
                self.suite
            )
        })??;
        if feats.n_options != self.head.n_classes {
            return Err(format!(
                "suite {}: the encoder rendered {} options, the head has {} classes — template \
                 drift, refuse loud",
                self.suite, feats.n_options, self.head.n_classes
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
    /// server's boot-time work, mirrored; the gate's loaded-centroid
    /// table reads READY lanes, so a lazy ENC row's vector never gates
    /// its own load).
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
        return Err(
            "a noul question has no trained head rows (the C1 class space is the \
                    presented-option space)"
                .into(),
        );
    }
    if q.kind != q_kind {
        return Err(format!(
            "kind drift: canonical {q_kind:?}, presented {:?}",
            q.kind
        ));
    }
    if q.instructions != instructions {
        return Err(
            "instructions drift: a custom question text is an encoder input the head \
                    never scored (the C1 certification scope: the utterance→sentiment/intent \
                    consumer, issue 016)"
                .into(),
        );
    }
    if q.qid != qid {
        return Err(format!(
            "qid drift: canonical {qid:?}, presented {:?}",
            q.qid
        ));
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
        [
            "very_negative",
            "negative",
            "neutral",
            "positive",
            "very_positive",
        ]
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

// ── issue 018 Lane B: the worker registry + the lane gate ──────────────
// The registry is generic over the agent type, so this module exercises
// the exactly-once / release / attach / fairness semantics WITHOUT the
// laya weights (a fake boot + a fake encode).
#[cfg(test)]
mod worker_tests {
    use super::*;
    use std::sync::Arc;

    fn fake_boot(loads: std::sync::Arc<std::sync::atomic::AtomicUsize>) -> WorkerBootFn<()> {
        Box::new(move || {
            loads.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            std::thread::sleep(std::time::Duration::from_millis(20));
            Ok((
                (),
                WorkerBoot {
                    device: "fake",
                    checkpoint_digest16: "0123456789abcdef".into(),
                },
            ))
        })
    }

    fn fake_encode() -> WorkerEncodeFn<()> {
        Box::new(|_agent: &(), job: &EncodeJob, _x: &mut Vec<f32>| {
            Ok(EncodedFeatures {
                x: vec![job.options.len() as f32; 8],
                n_options: job.options.len(),
            })
        })
    }

    fn fake_job(n_options: usize) -> (EncodeJob, mpsc::Receiver<Result<EncodedFeatures, String>>) {
        let (rtx, rrx) = mpsc::sync_channel(1);
        let job = EncodeJob {
            state: serde_json::json!({ "text": "t" }),
            qid: "q".into(),
            kind: QKind::Choice,
            instructions: String::new(),
            options: (0..n_options).map(|i| format!("o{i}")).collect(),
            resp: rtx,
        };
        (job, rrx)
    }

    /// G3: two cold attaches on one key cause EXACTLY ONE load, and both
    /// lanes answer through the same worker.
    #[test]
    fn one_load_per_activation_per_shared_worker() {
        let loads = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let key = "wt-one-load";
        let a = get_or_spawn_worker(key, fake_boot(Arc::clone(&loads)), fake_encode())
            .expect("first spawn");
        let b = get_or_spawn_worker(key, fake_boot(Arc::clone(&loads)), fake_encode())
            .expect("second get");
        assert_eq!(
            loads.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "exactly one load"
        );
        // Same worker: identical Arc sender (same address).
        assert_eq!(
            std::sync::Arc::as_ptr(&a.jobs) as usize,
            std::sync::Arc::as_ptr(&b.jobs) as usize,
            "both lanes share one worker sender"
        );
        let (job, rrx) = fake_job(2);
        a.jobs.send(job).expect("send via shared worker");
        let out = rrx.recv().expect("reply").expect("encode ok");
        assert_eq!(out.n_options, 2);
    }

    /// Distinct keys own distinct workers (per-lane mode's guarantee).
    #[test]
    fn distinct_keys_spawn_distinct_workers() {
        let loads = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let a = get_or_spawn_worker(
            "wt-distinct-a",
            fake_boot(Arc::clone(&loads)),
            fake_encode(),
        )
        .expect("a");
        let b = get_or_spawn_worker(
            "wt-distinct-b",
            fake_boot(Arc::clone(&loads)),
            fake_encode(),
        )
        .expect("b");
        assert_ne!(
            std::sync::Arc::as_ptr(&a.jobs) as usize,
            std::sync::Arc::as_ptr(&b.jobs) as usize,
            "distinct keys, distinct workers"
        );
        assert_eq!(loads.load(std::sync::atomic::Ordering::SeqCst), 2);
    }

    /// Release semantics: the LAST holder's drop ends the worker (the
    /// registry's weak handle dies); a re-attach pays ONE fresh load.
    #[test]
    fn last_release_frees_the_worker_and_recall_pays_one_load() {
        let loads = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let key = "wt-release";
        {
            let a = get_or_spawn_worker(key, fake_boot(Arc::clone(&loads)), fake_encode())
                .expect("first activation");
            let b = get_or_spawn_worker(key, fake_boot(Arc::clone(&loads)), fake_encode())
                .expect("second lane, same worker");
            assert_eq!(loads.load(std::sync::atomic::Ordering::SeqCst), 1);
            drop(a);
            // b still holds: the worker lives.
            let again = get_or_spawn_worker(key, fake_boot(Arc::clone(&loads)), fake_encode())
                .expect("upgrade while b holds");
            assert_eq!(
                loads.load(std::sync::atomic::Ordering::SeqCst),
                1,
                "no reload while held"
            );
            drop(again);
            drop(b);
        }
        // Every holder gone: the weak entry is dead; a recall re-loads ONCE.
        let c = get_or_spawn_worker(key, fake_boot(Arc::clone(&loads)), fake_encode())
            .expect("recall after release");
        assert_eq!(
            loads.load(std::sync::atomic::Ordering::SeqCst),
            2,
            "exactly ONE fresh load per activation"
        );
        drop(c);
    }

    /// A failed boot names the error, removes the entry, and the next
    /// attach can retry cleanly (the sticky lane-level Failed state is
    /// the slot's, above this layer — never a retry storm from here).
    #[test]
    fn failed_boot_refuses_and_allows_a_clean_retry() {
        let key = "wt-fail";
        let err = match get_or_spawn_worker(
            key,
            Box::new(|| Err("weights absent".to_string())),
            fake_encode(),
        ) {
            Ok(_) => panic!("a failed boot must refuse"),
            Err(e) => e,
        };
        assert!(err.contains("weights absent"), "{err}");
        let loads = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let ok = get_or_spawn_worker(key, fake_boot(Arc::clone(&loads)), fake_encode())
            .expect("retry ok");
        assert_eq!(loads.load(std::sync::atomic::Ordering::SeqCst), 1);
        drop(ok);
    }

    /// The lane gate: acquire blocks at `LANE_INFLIGHT_CAP` outstanding
    /// jobs and releases on drop (the fairness bound — one lane cannot
    /// occupy more of the shared queue than its cap).
    #[test]
    fn lane_gate_bounds_inflight_at_the_cap() {
        let gate = std::sync::Arc::new(LaneGate::new());
        let mut guards = Vec::new();
        for _ in 0..LANE_INFLIGHT_CAP {
            guards.push(gate.acquire());
        }
        // The cap is full: a further acquire must BLOCK. Probe from a
        // thread with a timeout — it must NOT finish while held.
        let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let g = std::sync::Arc::clone(&done);
        let gate2 = std::sync::Arc::clone(&gate);
        let h = std::thread::spawn(move || {
            let _guard = gate2.acquire();
            g.store(true, std::sync::atomic::Ordering::SeqCst);
        });
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert!(
            !done.load(std::sync::atomic::Ordering::SeqCst),
            "acquire must block while the lane holds {LANE_INFLIGHT_CAP} slots"
        );
        drop(guards.pop());
        h.join()
            .expect("the blocked acquire completes after one release");
        assert!(done.load(std::sync::atomic::Ordering::SeqCst));
    }

    /// Attach-shape honesty mirrors the lane's post-reply guards: the
    /// worker is template-agnostic, the CALLER validates n_options.
    #[test]
    fn worker_answers_the_jobs_own_template_not_a_boot_template() {
        let loads = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let w = get_or_spawn_worker("wt-attach", fake_boot(loads), fake_encode()).expect("w");
        for n in [2usize, 5, 3] {
            let (job, rrx) = fake_job(n);
            w.jobs.send(job).expect("send");
            let out = rrx.recv().expect("reply").expect("ok");
            assert_eq!(out.n_options, n, "the worker renders the JOB's template");
        }
    }
}
