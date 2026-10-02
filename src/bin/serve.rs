//! The hosted instinct decision server (Plan 001 P5 / Issue 002 — the
//! servable binary the cf-container target ships). Answers classification
//! decisions through the hybrid lane over a small std-only HTTP edge:
//!
//! - `GET  /healthz` — liveness + per-suite readiness (lanes load in
//!   background threads; the listener binds FIRST so the container
//!   HEALTHCHECK sees a live process during the seat boot).
//! - `POST /decide` — `{"suite", "state", "options"?}` → the served
//!   decision + the verifiable receipt (Proposal 014 §4: build
//!   fingerprint + feature set + BLAKE3(input) + lane id + decision; a
//!   client running the SAME release can re-derive and check).
//! - `POST /arsenal/release` — `{"suite"}` evicts a loaded LAZY suite
//!   (the L5 curator's release message — a wire-only server has no AOI
//!   to observe; Proposal 001 T5). Loopback only.
//! - `POST /arsenal/swap` — `{"suite", "artifact", "epoch", "force"?}`
//!   atomically hot-swaps the suite's artifact under the monotonic
//!   epoch-tag gate (Proposal 001 T6; interim caller: the operator —
//!   swap POLICY is game-side, 048 L9). Loopback only.
//! - `GET  /` — the endpoint index.
//!
//! The arsenal budget legs (T5): `budget.load = "lazy"` rows are NOT
//! loaded at boot — the first decision triggers the load and the 503
//! window covers it; `eager` rows keep the boot-load posture. The
//! Vendi hoarding gate (katgpt-core `set_admission`) refuses a lazy or
//! swapped load that pushes the loaded set past the certificate
//! (`RIIR_INSTINCT_HOARD_GATE=0` disarms — the exact literal). The admin
//! endpoints' recorded auth posture is LOOPBACK-ONLY: this server has no
//! token surface today, so anything off the loopback interface is
//! refused (403 `remote_forbidden`); the operator/curator calls from the
//! host side.
//!
//! Lane readiness mirrors reflex's own serve postures (`loading` /
//! `ready` / `failed`, plus `unloaded` for lazy rows): a loading, failed
//! or unloaded lane answers 503 with the state named — never a silent
//! modelless fallback, never a fabricated answer. Every refusal carries
//! a machine-readable `code`.
//!
//! The edge borrows reflex's serve SHAPE (borrow-pattern-not-dep) rather
//! than its module, which carries the game-head + laya machinery this
//! lane must not drag in.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use katgpt_core::set_admission::SetAdmissionConfig;
use riir_instinct::arsenal::ArsenalManifest;
use riir_instinct::receipt::{decision_blake3, fingerprint, input_blake3};
use riir_reflex::harness::suites::QKind;
use riir_instinct::arsenal_ops::{
    EpochApply, EpochTag, LaneSlot, LaneState, SwapRefusal, check_epoch_tag, hoard_check,
    hoard_gate_armed,
};
use riir_instinct::server::AnySuiteServer;

include!(concat!(env!("OUT_DIR"), "/build_stamp.rs"));

const MAX_BODY: usize = 1024 * 1024;
/// Error paths that do not consume the announced body drain up to this
/// many bytes before responding — closing on an unread body sends RST
/// and the client loses the refusal (the response it needs most).
const DRAIN_CAP: usize = 4 * 1024 * 1024;
/// The seat boot allocates big stack frames (the arena boots on the main
/// thread; loader threads need real headroom).
const LOADER_STACK: usize = 64 * 1024 * 1024;
const CONN_STACK: usize = 8 * 1024 * 1024;
const READ_TIMEOUT: Duration = Duration::from_secs(10);
const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Plan 426 T6: synth rows enter BEYOND the per-label gold cap — the V5
/// measured posture (extra-cap 128/label, reflex bench 091).
const SYNTH_EXTRA_CAP: usize = 128;

/// The lane's tag digest: the manifest row's pinned artifact digest, or —
/// for an artifact-less A0 row — the suite name's BLAKE3 (the stable lane
/// identity where no artifact exists). The ONE expression both the slot's
/// birth tag and the loader's install tag use, so the epoch gate sees the
/// same digest on both sides (a fork refusal here is a bug, not a gate).
fn lane_tag_digest(row: &riir_instinct::arsenal::VesselRow, suite: &str) -> [u8; 32] {
    row.digest_bytes()
        .unwrap_or_else(|| *blake3::hash(suite.as_bytes()).as_bytes())
}

fn main() {
    let mut datasets_dir = std::env::var("INSTINCT_DATASETS_DIR")
        .unwrap_or_else(|_| "../riir-reflex/.raw/datasets_t20k".into());
    let mut winners_dir = std::env::var("INSTINCT_WINNERS_DIR")
        .unwrap_or_else(|_| "../riir-train/data/instinct_specialists".into());
    let mut bind = std::env::var("INSTINCT_BIND").unwrap_or_else(|_| "0.0.0.0:8080".into());
    // Plan 426 T6's serve posture: unset = the gold corpus everywhere
    // (byte-identical boots); set = suites with a present
    // `<dir>/<suite>_synth.jsonl` seat it.
    let synth_corpus_dir = std::env::var("INSTINCT_SYNTH_CORPUS_DIR").ok();
    let vessel_dir = std::env::var("INSTINCT_VESSEL_DIR").ok();
    let vessel_key_hex = std::env::var("INSTINCT_VESSEL_KEY_HEX").ok();
    #[cfg(feature = "vessel")]
    let vessel_pins_hex = std::env::var("INSTINCT_VESSEL_PINS_HEX").ok();
    #[cfg(feature = "vessel")]
    let state_dir =
        std::env::var("INSTINCT_STATE_DIR").unwrap_or_else(|_| ".instinct_state".into());
    // The arsenal manifest: `INSTINCT_ARSENAL` or `--arsenal <path>`;
    // absent both, the embedded default (Proposal 001, law A5 — one
    // manifest per HOST/deployment).
    let mut arsenal_path: Option<String> = std::env::var("INSTINCT_ARSENAL").ok();
    let mut suite_filter: Option<Vec<String>> = None;
    let mut i = 1;
    let args: Vec<String> = std::env::args().collect();
    while i < args.len() {
        match args[i].as_str() {
            "--datasets-dir" => {
                i += 1;
                datasets_dir = args[i].clone();
            }
            "--winners-dir" => {
                i += 1;
                winners_dir = args[i].clone();
            }
            "--bind" => {
                i += 1;
                bind = args[i].clone();
            }
            "--arsenal" => {
                i += 1;
                arsenal_path = Some(args[i].clone());
            }
            "--suites" => {
                i += 1;
                // Raw names here; resolved against the manifest below
                // (an unknown name refuses loud — the manifest is the
                // only selection surface, there is no table behind it).
                let names: Vec<String> = args[i]
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect();
                if names.is_empty() {
                    die("--suites named no suite");
                }
                suite_filter = Some(names);
            }
            other => die(&format!("unknown arg {other}")),
        }
        i += 1;
    }

    // Vessel mode: INSTINCT_VESSEL_DIR set → the specialists load from
    // HOSTED-ONLY vessels. Fail-closed configuration: the key and at
    // least one operator pin are REQUIRED the moment the dir is set — a
    // half-configured vessel boot must refuse, never fall back to raw
    // winners (that would be the moat leak the class discipline exists
    // to prevent, wearing a fallback's clothes). Runs BEFORE the bind:
    // a misconfigured host never opens a port.
    #[cfg(feature = "vessel")]
    let vessel = match (&vessel_dir, &vessel_key_hex) {
        (Some(dir), Some(key_hex)) => {
            let key_bytes = decode_hex32(key_hex)
                .unwrap_or_else(|| die("INSTINCT_VESSEL_KEY_HEX must be 64 hex chars (32 bytes)"));
            let pins_hex = vessel_pins_hex.as_deref().unwrap_or("");
            if pins_hex.trim().is_empty() {
                die("INSTINCT_VESSEL_PINS_HEX is required in vessel mode (id:pubkey hex, comma-\n                    separated) — no compiled-in pins exist, the operator pins the mint key");
            }
            let mut keys: Vec<(u32, [u8; 32])> = Vec::new();
            for pin in pins_hex.split(',') {
                let pin = pin.trim();
                if pin.is_empty() {
                    continue;
                }
                let Some((id, hex)) = pin.split_once(':') else {
                    die(&format!("INSTINCT_VESSEL_PINS_HEX entry {pin:?} is not id:hex"));
                };
                let id: u32 = id.trim().parse().unwrap_or_else(|_| {
                    die(&format!("INSTINCT_VESSEL_PINS_HEX key-id {id:?} is not a number"))
                });
                let bytes = decode_hex32(hex.trim()).unwrap_or_else(|| {
                    die(&format!("INSTINCT_VESSEL_PINS_HEX entry {id} is not 64 hex chars"))
                });
                keys.push((id, bytes));
            }
            eprintln!(
                "[riir-instinct] vessel mode: dir {dir} · {} pin(s) · state dir {state_dir}",
                keys.len()
            );
            Some(VesselConfig {
                dir: dir.into(),
                key: key_bytes,
                pins: reflexer_vessel::pins_from_bytes(&keys),
                state_dir: state_dir.into(),
            })
        }
        (Some(_), None) => die(
            "INSTINCT_VESSEL_DIR is set without INSTINCT_VESSEL_KEY_HEX — vessel mode is a\n             whole configuration (dir + key + pins), never a partial one",
        ),
        (None, Some(_)) => die(
            "INSTINCT_VESSEL_KEY_HEX is set without INSTINCT_VESSEL_DIR — vessel mode is a\n             whole configuration (dir + key + pins), never a partial one",
        ),
        (None, None) => None,
    };
    #[cfg(not(feature = "vessel"))]
    if vessel_dir.is_some() || vessel_key_hex.is_some() {
        die(
            "vessel env vars are set but this binary was built WITHOUT the `vessel` feature —\n             rebuild with --features vessel (the reader is opt-in by design)",
        );
    }
    #[cfg(not(feature = "vessel"))]
    struct VesselConfig;
    #[cfg(not(feature = "vessel"))]
    let vessel: Option<VesselConfig> = None;
    #[cfg(not(feature = "vessel"))]
    let _ = &vessel;

    // The arsenal manifest (Proposal 001, law A5 — the ONE selection
    // surface): parse, then validate BEFORE lanes resolve AND before the
    // bind — a manifest that refuses never opens a port. Drift (artifact
    // bytes ≠ the pinned digest), an unknown posture arm, a class the
    // reader cannot honor: all loud here, naming the row + field.
    let (manifest, arsenal_digest, arsenal_desc): (ArsenalManifest, String, String) =
        match &arsenal_path {
            Some(p) => {
                let text = std::fs::read_to_string(p)
                    .unwrap_or_else(|e| die(&format!("read arsenal manifest {p}: {e}")));
                let digest = ArsenalManifest::digest_of(&text);
                let m = ArsenalManifest::parse(&text)
                    .unwrap_or_else(|e| die(&format!("arsenal manifest {p}: {e}")));
                (m, digest, p.clone())
            }
            None => (
                ArsenalManifest::embedded_default()
                    .unwrap_or_else(|e| die(&format!("embedded arsenal manifest: {e}"))),
                ArsenalManifest::embedded_manifest_digest(),
                "embedded default".into(),
            ),
        };
    let manifest = std::sync::Arc::new(manifest);
    #[cfg(feature = "vessel")]
    let vctx = match vessel.as_ref() {
        Some(cfg) => riir_instinct::arsenal::ValidateCtx::vessel(
            std::path::Path::new(&winners_dir),
            &cfg.dir,
            &cfg.pins,
        ),
        None => {
            riir_instinct::arsenal::ValidateCtx::raw(std::path::Path::new(&winners_dir))
        }
    };
    #[cfg(not(feature = "vessel"))]
    let vctx = riir_instinct::arsenal::ValidateCtx::raw(std::path::Path::new(&winners_dir));
    manifest.validate(&vctx).unwrap_or_else(|e| {
        die(&format!(
            "arsenal manifest ({arsenal_desc}): validation refused: {e}"
        ))
    });
    eprintln!(
        "[riir-instinct] arsenal: {} row(s) ({arsenal_desc}, digest blake3:{})",
        manifest.rows().len(),
        &arsenal_digest[..16]
    );

    // Suites: the manifest's rows, optionally filtered by --suites (an
    // unknown name refuses loud). The names leak into 'static here — a
    // one-time boot allocation for the registry keys, never the hot path.
    let all_suites: Vec<&'static str> = manifest
        .suites()
        .map(|s| -> &'static str { Box::leak(s.to_string().into_boxed_str()) })
        .collect();
    let suites: Vec<&'static str> = match &suite_filter {
        None => all_suites,
        Some(filter) => filter
            .iter()
            .map(|s| {
                all_suites.iter().copied().find(|r| *r == s).unwrap_or_else(|| {
                    die(&format!(
                        "--suites: {s:?} is not in the arsenal manifest ({arsenal_desc})"
                    ))
                })
            })
            .collect(),
    };
    if suites.is_empty() {
        die("no suites to serve");
    }

    let listener = match TcpListener::bind(&bind) {
        Ok(l) => l,
        Err(e) => die(&format!("bind {bind}: {e}")),
    };
    eprintln!(
        "[riir-instinct] serving on http://{bind} (hosted hybrid lane v{VERSION}, build {})",
        fingerprint()
    );
    eprintln!("datasets: {datasets_dir} · winners: {winners_dir}");
    let allow = allowed_origins();
    if allow.is_empty() {
        eprintln!("[riir-instinct] CORS: closed (no RIIR_INSTINCT_ALLOWED_ORIGIN)");
    } else {
        eprintln!("[riir-instinct] CORS: allowed origins — {}", allow.join(", "));
    }

    // The decstat capture lane (Plan 002 / Issue 004 T1): consent-gated
    // decision-outcome stats — Unset never pushes (the `--stats`
    // semantics). Consent on is a WHOLE configuration: the signing key
    // is required and a missing one refuses at boot (the vessel-mode
    // pattern — a half-configured contributor is worse than none).
    #[cfg(feature = "decstat")]
    {
        let consent = riir_instinct::decstat::consent_enabled(
            std::env::var("RIIR_INSTINCT_STATS").ok().as_deref(),
        );
        if consent {
            let key_path = std::env::var("INSTINCT_ACCOUNT_KEY").unwrap_or_else(|_| {
                die(
                    "RIIR_INSTINCT_STATS=on requires INSTINCT_ACCOUNT_KEY=<64-hex seed file> — \
                     the row is signed with the account key; refusing is the honest posture",
                )
            });
            let key =
                riir_instinct::decstat::load_signing_key(&key_path).unwrap_or_else(|e| die(&e));
            let url = std::env::var("INSTINCT_KAT_SERVICE_URL")
                .unwrap_or_else(|_| riir_instinct::decstat::DEFAULT_SERVICE_URL.into());
            let machine = std::env::var("INSTINCT_MACHINE_LABEL").unwrap_or_default();
            let toolchain = format!("rustc {RUSTC_RELEASE}");
            let sink = Arc::new(riir_instinct::decstat::DecStatSink::new());
            riir_instinct::decstat::install(Some(Arc::clone(&sink)));
            match riir_instinct::decstat::spawn_flusher(
                sink,
                key,
                riir_instinct::decstat::FlushConfig {
                    service_url: url.clone(),
                    machine,
                    toolchain,
                },
            ) {
                Ok(_) => eprintln!(
                    "[riir-instinct] decstat: on — flushing decision stats to {url} every {}s/{} decisions",
                    riir_instinct::decstat::FLUSH_EVERY_SECS,
                    riir_instinct::decstat::FLUSH_THRESHOLD
                ),
                Err(e) => die(&format!("spawn decstat flusher: {e}")),
            }
        } else {
            riir_instinct::decstat::install(None);
            eprintln!(
                "[riir-instinct] decstat: off — decision stats are NOT contributed (set RIIR_INSTINCT_STATS=on + INSTINCT_ACCOUNT_KEY=<seed file> to opt in)"
            );
        }
    }
    #[cfg(not(feature = "decstat"))]
    eprintln!(
        "[riir-instinct] decstat: not compiled (build with --features decstat to enable the contribution lane)"
    );

    // The registry (Proposal 001 T5/T6): one slot per requested suite.
    // Eager rows start Loading and boot their loader now; lazy rows stay
    // Unloaded — the FIRST decision triggers the load and the 503 window
    // covers it. Every slot carries its applied epoch tag from birth:
    // boot initializes epoch 0 from the manifest (the row's pinned
    // artifact digest — Proposal 001 T6).
    #[cfg(feature = "vessel")]
    let vessel: Arc<Option<VesselConfig>> = Arc::new(vessel);
    let ctx = Arc::new(BootCtx {
        datasets_dir,
        winners_dir,
        synth_corpus_dir,
        manifest: Arc::clone(&manifest),
        #[cfg(feature = "vessel")]
        vessel,
    });
    let mut slots: Vec<Arc<LaneSlot<AnySuiteServer>>> = Vec::with_capacity(suites.len());
    let mut eager: Vec<usize> = Vec::new();
    for (idx, s) in suites.iter().enumerate() {
        let row = manifest
            .row(s)
            .unwrap_or_else(|| die(&format!("suite {s}: vanished from the validated manifest")));
        let tag = EpochTag {
            epoch: 0,
            digest: lane_tag_digest(row, s),
        };
        let st = if row.budget.load == "lazy" {
            LaneState::Unloaded { applied: tag }
        } else {
            eager.push(idx);
            LaneState::Loading { applied: tag }
        };
        slots.push(Arc::new(LaneSlot::new(s, st)));
    }
    let state = Arc::new(SrvState { slots, ctx });
    let lazy_n = state.slots.len() - eager.len();
    eprintln!(
        "[riir-instinct] arsenal: {} eager lane(s) loading at boot, {lazy_n} lazy (first decision loads)",
        eager.len()
    );
    for idx in eager {
        spawn_loader(&state, idx);
    }

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let state = Arc::clone(&state);
                let allow = allow.clone();
                let conn = std::thread::Builder::new()
                    .name("conn".into())
                    .stack_size(CONN_STACK)
                    .spawn(move || {
                        if let Err(e) = handle_conn(s, &state, &allow) {
                            eprintln!("[riir-instinct] conn error: {e}");
                        }
                    });
                if let Err(e) = conn {
                    eprintln!("[riir-instinct] spawn conn: {e}");
                }
            }
            Err(e) => eprintln!("[riir-instinct] accept error: {e}"),
        }
    }
}

fn die(msg: &str) -> ! {
    eprintln!("⛔ serve: {msg}");
    std::process::exit(1);
}

/// Everything the loader threads need to (re)build a lane — shared via
/// Arc so the lazy trigger (a connection thread) can spawn a loader.
struct BootCtx {
    datasets_dir: String,
    winners_dir: String,
    /// Plan 426 T6's serve posture: when set and `<dir>/<suite>_synth.jsonl`
    /// (+ its `.blake3` sidecar) exists for a booting suite, the seat loads
    /// it — blake3-verified by the reflex loader, in-universe filtered —
    /// and the lane's engine builds gold-cap-first + synth-beyond (the V5
    /// arm-B construction). Absent file / unset env = the gold corpus,
    /// byte-identical boots.
    synth_corpus_dir: Option<String>,
    manifest: Arc<ArsenalManifest>,
    #[cfg(feature = "vessel")]
    vessel: Arc<Option<VesselConfig>>,
}

/// The serve registry: one slot per requested suite + the boot context
/// the lazy trigger and the swap edge share.
struct SrvState {
    slots: Vec<Arc<LaneSlot<AnySuiteServer>>>,
    ctx: Arc<BootCtx>,
}

/// Bounded single-read (the vessel reader's discipline, mirrored): a
/// regular file only, one `take(cap+1)` read that can never allocate past
/// the ceiling even if the file grows mid-flight. The +1 sees an
/// oversized file and the refusal carries the length.
fn read_bounded(path: &Path, cap: usize) -> Result<Vec<u8>, String> {
    use std::io::Read as _;
    let md = std::fs::metadata(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    if !md.is_file() {
        return Err(format!("{} is not a regular file (artifacts are files)", path.display()));
    }
    if md.len() > cap as u64 + 1 {
        return Err(format!(
            "{} is {} bytes, over the {cap} byte cap",
            path.display(),
            md.len()
        ));
    }
    let mut buf = Vec::new();
    std::fs::File::open(path)
        .map_err(|e| format!("read {}: {e}", path.display()))?
        .take(cap as u64 + 1)
        .read_to_end(&mut buf)
        .map_err(|e| format!("read {}: {e}", path.display()))?;
    if buf.len() > cap {
        return Err(format!(
            "{} is {} bytes, over the {cap} byte cap",
            path.display(),
            buf.len()
        ));
    }
    Ok(buf)
}

/// What a successful load hands the installer: the serving server, the
/// artifact bytes' BLAKE3 (the epoch tag's digest half — the same
/// quantity the manifest row pins), and (vessel mode) the vessel facts
/// the host persists after a full install.
struct LoadedLane {
    server: AnySuiteServer,
    artifact_digest: [u8; 32],
    #[cfg(feature = "vessel")]
    facts: Option<riir_instinct::server::VesselFacts>,
}

/// Load one suite's lane: the seat (datasets) + the artifact (the row's
/// file, or the swap's explicit artifact), read ONCE and built from those
/// bytes. Vessel mode when configured (fail-closed — no raw-winner
/// fallback inside vessel mode), else the raw sealed winners.
fn load_lane(
    ctx: &BootCtx,
    suite: &'static str,
    artifact: Option<&str>,
) -> Result<LoadedLane, String> {
    let Some(row) = ctx.manifest.row(suite) else {
        return Err(format!("suite {suite} is not in the arsenal manifest"));
    };
    let cap = (row.budget.max_payload_mb << 20) as usize;
    // The seated synth corpus (Plan 426 T6): env dir + a present artifact
    // for THIS suite seats it; anything else keeps the gold seat. A present
    // artifact that FAILS verification is fatal (a corrupt corpus must
    // never degrade into a quiet gold seat — the vessel reader's law).
    let seat = match ctx.synth_corpus_dir.as_ref() {
        Some(dir) => {
            let synth = Path::new(dir).join(format!("{suite}_synth.jsonl"));
            if synth.is_file() {
                let s = riir_reflex::harness::runner::seat::prepare_seat_with_synth(
                    suite,
                    Path::new(&ctx.datasets_dir),
                    &synth,
                    SYNTH_EXTRA_CAP,
                )?;
                let meta = s.synth.as_ref().expect("with_synth seated the corpus");
                eprintln!(
                    "  lane {suite}: synth corpus seated — {} rows ({} dropped) · digest {}…",
                    meta.docs.len(),
                    meta.rows_dropped,
                    &meta.digest_hex[..16.min(meta.digest_hex.len())]
                );
                s
            } else {
                riir_reflex::harness::runner::seat::prepare_seat(
                    suite,
                    Path::new(&ctx.datasets_dir),
                )?
            }
        }
        None => riir_reflex::harness::runner::seat::prepare_seat(
            suite,
            Path::new(&ctx.datasets_dir),
        )?,
    };
    #[cfg(feature = "vessel")]
    if let Some(cfg) = ctx.vessel.as_ref() {
        if row.digest.is_none() {
            return Err(format!(
                "suite {suite}: the row carries no artifact digest — the vessel lane loads \
                 artifacts only (an artifact-less A0 row is a raw-posture row)"
            ));
        }
        let name = artifact
            .map(str::to_string)
            .unwrap_or_else(|| row.artifact_file(format!("{suite}_v1.vessel")));
        let path = cfg.dir.join(&name);
        let bytes = read_bounded(&path, reflexer_vessel::PREFIX_LEN.saturating_add(cap))?;
        let digest = *blake3::hash(&bytes).as_bytes();
        let applied = read_applied(&cfg.state_dir, suite);
        let (server, facts) = AnySuiteServer::boot_vessel_bytes(
            suite,
            seat,
            &bytes,
            &ctx.manifest,
            &cfg.pins,
            &cfg.key,
            &applied,
        )?;
        return Ok(LoadedLane {
            server,
            artifact_digest: digest,
            facts: Some(facts),
        });
    }
    // The artifact-less A0 posture (owner 2026-10-02 full-coverage
    // serving): the row pins no digest, so the lane boots the seat engine
    // alone — HybridLane::ReflexOnly, byte-identical to A0. The epoch tag
    // digest is `lane_tag_digest`'s suite-name BLAKE3 — the same value the
    // slot's birth tag carries, so the install is the idempotent no-op.
    if row.digest.is_none() {
        let server = AnySuiteServer::boot_a0_from_seat(suite, seat, &ctx.manifest)?;
        return Ok(LoadedLane {
            server,
            artifact_digest: lane_tag_digest(row, suite),
            #[cfg(feature = "vessel")]
            facts: None,
        });
    }
    let name = artifact
        .map(str::to_string)
        .unwrap_or_else(|| row.artifact_file(format!("{suite}_winner_v1.bin")));
    // The raw-mode convention coupling (Issue 579): a bridged suite loads
    // EXACTLY its bridged file, loud refusal otherwise.
    riir_instinct::specialist::check_winner_file(suite, &name)?;
    let path = Path::new(&ctx.winners_dir).join(&name);
    let bytes = read_bounded(&path, cap)?;
    let digest = *blake3::hash(&bytes).as_bytes();
    let server = AnySuiteServer::boot_bytes(suite, seat, &bytes, &ctx.manifest)?;
    Ok(LoadedLane {
        server,
        artifact_digest: digest,
        #[cfg(feature = "vessel")]
        facts: None,
    })
}

/// The loaded-set view for the hoarding gate: every OTHER ready slot's
/// corpus centroid (the candidate's own slot never votes on itself).
fn loaded_centroids(
    slots: &[Arc<LaneSlot<AnySuiteServer>>],
    except: &str,
) -> Vec<(&'static str, [f32; katgpt_core::set_admission::DIM])> {
    slots
        .iter()
        .filter(|s| s.suite != except)
        .filter_map(|s| {
            let st = s.state.lock().expect("slot lock");
            match &*st {
                LaneState::Ready { centroid, .. } => Some((s.suite, *centroid)),
                _ => None,
            }
        })
        .collect()
}

/// The hoarding gate over one candidate load — the shared body of the
/// lazy loader and the swap edge. Armed by default; the exact literal
/// `RIIR_INSTINCT_HOARD_GATE=0` disarms (and says so, loudly).
fn hoard_gate(
    state: &SrvState,
    suite: &'static str,
    centroid: [f32; katgpt_core::set_admission::DIM],
) -> Result<(), String> {
    if !hoard_gate_armed() {
        eprintln!(
            "[riir-instinct] arsenal: hoard gate DISARMED (RIIR_INSTINCT_HOARD_GATE=0) — \
             {suite} admitted unchecked"
        );
        return Ok(());
    }
    let loaded = loaded_centroids(&state.slots, suite);
    match hoard_check(suite, centroid, &loaded, &SetAdmissionConfig::default()) {
        Ok(rep) => {
            if !rep.judged {
                eprintln!(
                    "[riir-instinct] arsenal: {suite} admitted UNJUDGED (empty corpus — no \
                     centroid direction); set vendi {:.3} at k {}",
                    rep.vendi, rep.k
                );
            } else {
                eprintln!(
                    "[riir-instinct] arsenal: {suite} hoard gate ok (vendi {:.3} / floor {:.3}, \
                     k {}, pr {:.3}{})",
                    rep.vendi,
                    rep.floor,
                    rep.k,
                    rep.participation_ratio,
                    if rep.saturated { ", saturated at d=8" } else { "" }
                );
            }
            Ok(())
        }
        Err(r) => Err(r.to_string()),
    }
}

/// Spawn a lane loader thread (boot for eager rows; the first decision
/// for lazy rows). A spawn failure fails the slot loud — never a slot
/// stuck Loading forever.
fn spawn_loader(state: &Arc<SrvState>, idx: usize) {
    let builder = std::thread::Builder::new()
        .name(format!("boot-{}", state.slots[idx].suite))
        .stack_size(LOADER_STACK);
    let st = Arc::clone(state);
    match builder.spawn(move || run_loader(&st, idx)) {
        Ok(_) => {}
        Err(e) => {
            let msg = format!("spawn loader thread: {e}");
            eprintln!("[riir-instinct] lane {} FAILED: {msg}", state.slots[idx].suite);
            state.slots[idx].fail(msg);
        }
    }
}

/// One lane load: seat + artifact + hoarding gate + the atomic install
/// under the epoch-tag gate. Boot (eager) and the lazy trigger share
/// this body — the manifest artifact loads with the row's tag, so a slot
/// a swap has advanced refuses the stale manifest artifact (never a
/// silent downgrade).
fn run_loader(state: &Arc<SrvState>, idx: usize) {
    let slot = &state.slots[idx];
    let suite = slot.suite;
    let t0 = std::time::Instant::now();
    let lane = match load_lane(&state.ctx, suite, None) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("[riir-instinct] lane {suite} FAILED: {e}");
            slot.fail(e);
            return;
        }
    };
    let meta = lane.server.meta().clone();
    let centroid = lane.server.centroid();
    let tag = EpochTag {
        epoch: 0,
        digest: lane.artifact_digest,
    };
    if let Err(e) = hoard_gate(state, suite, centroid) {
        let msg = format!("hoarding gate refused load: {e}");
        eprintln!("[riir-instinct] lane {suite} FAILED: {msg}");
        slot.fail(msg);
        return;
    }
    match slot.install_ready(lane.server, tag, centroid, false) {
        Ok(outcome) => {
            eprintln!(
                "[riir-instinct] lane {suite} ready in {:.1}s (arm {}, {} labels, cap {}, \
                 source {:?}, epoch {}, install {outcome:?})",
                t0.elapsed().as_secs_f32(),
                meta.arm.name(),
                meta.labels,
                meta.effective_cap,
                meta.source,
                tag.epoch,
            );
            // Persist AFTER the suite is fully up — a boot failure must
            // not advance the vessel lineage gate.
            #[cfg(feature = "vessel")]
            if let (Some(cfg), Some(facts)) = (state.ctx.vessel.as_ref(), lane.facts.as_ref()) {
                if let Err(e) = write_applied(&cfg.state_dir, suite, facts) {
                    eprintln!("[riir-instinct] lane {suite}: persist applied state: {e}");
                }
            }
        }
        Err(e) => {
            let msg = format!("swap gate refused install: {e}");
            eprintln!("[riir-instinct] lane {suite} FAILED: {msg}");
            slot.fail(msg);
        }
    }
}

/// The vessel-boot configuration (vessel feature only) — dir + key +
/// pins, always together.
#[cfg(feature = "vessel")]
struct VesselConfig {
    dir: std::path::PathBuf,
    key: [u8; 32],
    pins: reflexer_vessel::PinTable,
    state_dir: std::path::PathBuf,
}

/// Decode 64 hex chars into 32 bytes (None on any malformed input).
#[cfg(feature = "vessel")]
fn decode_hex32(s: &str) -> Option<[u8; 32]> {
    let s = s.trim();
    if s.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).ok()?;
    }
    Some(out)
}

/// The persisted monotonic-apply state for one suite: `<state_dir>/
/// applied_<suite>.json` — `{"artifact_version": u64, "commitment": hex}`.
/// Absent/corrupt = genesis (0) — a corrupt state file must NOT read as
/// a high version (that would refuse every legitimate successor).
#[cfg(feature = "vessel")]
fn read_applied(state_dir: &std::path::Path, suite: &str) -> riir_instinct::vessel::AppliedState {
    let p = state_dir.join(format!("applied_{suite}.json"));
    #[derive(serde::Deserialize)]
    struct AppliedFile {
        artifact_version: u64,
    }
    std::fs::read_to_string(&p)
        .ok()
        .and_then(|s| serde_json::from_str::<AppliedFile>(&s).ok())
        .map(|f| riir_instinct::vessel::AppliedState {
            artifact_version: f.artifact_version,
        })
        .unwrap_or(riir_instinct::vessel::AppliedState::GENESIS)
}

/// Persist the applied state AFTER a fully successful boot.
#[cfg(feature = "vessel")]
fn write_applied(
    state_dir: &std::path::Path,
    suite: &str,
    facts: &riir_instinct::server::VesselFacts,
) -> Result<(), String> {
    std::fs::create_dir_all(state_dir).map_err(|e| format!("state dir: {e}"))?;
    let doc = serde_json::json!({
        "artifact_version": facts.artifact_version,
        "commitment": facts.commitment_hex,
        "parent_commitment": hex32(&facts.parent_commitment),
    });
    let p = state_dir.join(format!("applied_{suite}.json"));
    std::fs::write(&p, serde_json::to_string_pretty(&doc).unwrap_or_default())
        .map_err(|e| format!("write {}: {e}", p.display()))
}

#[cfg(feature = "vessel")]
fn hex32(b: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for byte in b {
        use std::fmt::Write as _;
        let _ = write!(s, "{byte:02x}");
    }
    s
}

fn allowed_origins() -> Vec<String> {
    std::env::var("RIIR_INSTINCT_ALLOWED_ORIGIN")
        .map(|v| {
            v.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

struct Req {
    method: String,
    path: String,
    content_length: usize,
    origin: Option<String>,
}

fn read_request(reader: &mut BufReader<TcpStream>) -> std::io::Result<Option<Req>> {
    let mut line = String::new();
    if reader.read_line(&mut line)? == 0 {
        return Ok(None); // clean EOF between requests
    }
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("").to_ascii_uppercase();
    let path = parts.next().unwrap_or("/").to_string();
    let mut content_length = 0usize;
    let mut origin = None;
    loop {
        let mut h = String::new();
        if reader.read_line(&mut h)? == 0 {
            break;
        }
        let h = h.trim_end();
        if h.is_empty() {
            break;
        }
        if let Some((name, value)) = h.split_once(':') {
            let (name, value) = (name.trim(), value.trim());
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.parse().unwrap_or(0);
            } else if name.eq_ignore_ascii_case("origin") {
                origin = Some(value.to_string());
            }
        }
    }
    Ok(Some(Req {
        method,
        path,
        content_length,
        origin,
    }))
}

fn cors_echo(origin: Option<&str>, allow: &[String]) -> Option<String> {
    let o = origin?;
    allow
        .iter()
        .any(|a| a.eq_ignore_ascii_case(o))
        .then(|| o.to_string())
}

/// Best-effort body drain for error paths that never read it — closing a
/// connection with unread request bytes sends RST and destroys the
/// response the client needs. Bounded: an absurd body loses its response
/// (the client chose to send garbage).
fn drain_body(reader: &mut BufReader<TcpStream>, n: usize) {
    if n == 0 || n > DRAIN_CAP {
        return;
    }
    let mut remaining = n;
    let mut buf = [0u8; 16 * 1024];
    while remaining > 0 {
        let want = buf.len().min(remaining);
        match reader.read(&mut buf[..want]) {
            Ok(0) | Err(_) => break,
            Ok(got) => remaining -= got,
        }
    }
}

fn respond(
    stream: &mut TcpStream,
    status: &str,
    body: &str,
    cors: Option<&str>,
) {
    let bytes = body.as_bytes();
    let cors_hdr = cors
        .map(|o| format!("Access-Control-Allow-Origin: {o}\r\nVary: Origin\r\n"))
        .unwrap_or_default();
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n{cors_hdr}Connection: close\r\n\r\n",
        bytes.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(bytes);
    let _ = stream.flush();
}

/// The one JSON error shape every refusal shares — a quoted `error` plus a
/// machine-readable `code` (the money-route discipline, additive).
fn json_error(stream: &mut TcpStream, status: &str, code: &str, text: &str, cors: Option<&str>) {
    respond(
        stream,
        status,
        &format!(
            "{{\"error\":{},\"code\":{code:?}}}",
            serde_json::to_string(text).unwrap_or_default()
        ),
        cors,
    );
}

fn handle_conn(
    stream: TcpStream,
    state: &Arc<SrvState>,
    allow: &[String],
) -> std::io::Result<()> {
    stream.set_read_timeout(Some(READ_TIMEOUT))?;
    // The arsenal admin endpoints' recorded auth posture: loopback only
    // (this server has no token surface; anything off the loopback
    // interface is refused before its body is read).
    let local = stream
        .peer_addr()
        .map(|a| a.ip().is_loopback())
        .unwrap_or(false);
    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(stream);
    let Some(req) = read_request(&mut reader)? else {
        return Ok(());
    };
    let cors = cors_echo(req.origin.as_deref(), allow);
    match (req.method.as_str(), req.path.as_str()) {
        ("OPTIONS", _) => match cors {
            Some(o) => {
                let head = format!(
                    "HTTP/1.1 204 No Content\r\nAccess-Control-Allow-Origin: {o}\r\nVary: Origin\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nAccess-Control-Max-Age: 86400\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                );
                let _ = writer.write_all(head.as_bytes());
                let _ = writer.flush();
            }
            None => {
                let head = "HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                let _ = writer.write_all(head.as_bytes());
                let _ = writer.flush();
            }
        },
        ("GET", "/healthz") => {
            respond(&mut writer, "200 OK", &healthz(state), cors.as_deref());
        }
        ("GET", "/") => {
            respond(
                &mut writer,
                "200 OK",
                &format!(
                    "{{\"service\":\"riir-instinct\",\"version\":{VERSION:?},\"build\":{:?},\"endpoints\":[\"GET /healthz\",\"POST /decide\",\"POST /arsenal/release (loopback)\",\"POST /arsenal/swap (loopback)\"]}}",
                    fingerprint()
                ),
                cors.as_deref(),
            );
        }
        ("POST", "/decide") => {
            if req.content_length > MAX_BODY {
                drain_body(&mut reader, req.content_length);
                json_error(&mut writer, "413 Payload Too Large", "too_large", "body too large", cors.as_deref());
                return Ok(());
            }
            let mut body = vec![0u8; req.content_length];
            reader.read_exact(&mut body)?;
            decide_edge(&mut writer, state, &body, cors.as_deref());
        }
        ("POST", "/arsenal/release") | ("POST", "/arsenal/swap") if local => {
            if req.content_length > MAX_BODY {
                drain_body(&mut reader, req.content_length);
                json_error(&mut writer, "413 Payload Too Large", "too_large", "body too large", cors.as_deref());
                return Ok(());
            }
            let mut body = vec![0u8; req.content_length];
            reader.read_exact(&mut body)?;
            if req.path == "/arsenal/release" {
                release_edge(&mut writer, state, &body, cors.as_deref());
            } else {
                swap_edge(&mut writer, state, &body, cors.as_deref());
            }
        }
        ("POST", "/arsenal/release") | ("POST", "/arsenal/swap") => {
            drain_body(&mut reader, req.content_length);
            json_error(
                &mut writer,
                "403 Forbidden",
                "remote_forbidden",
                "the arsenal admin endpoints answer the loopback interface only (the \
                 recorded auth posture — call from the host side)",
                cors.as_deref(),
            );
        }
        ("GET", "/decide") | ("POST", "/healthz") | ("GET", "/arsenal/release") | ("GET", "/arsenal/swap") => {
            drain_body(&mut reader, req.content_length);
            json_error(
                &mut writer,
                "405 Method Not Allowed",
                "method_not_allowed",
                "wrong method for this endpoint",
                cors.as_deref(),
            );
        }
        _ => {
            drain_body(&mut reader, req.content_length);
            json_error(&mut writer, "404 Not Found", "not_found", "not found", cors.as_deref());
        }
    }
    Ok(())
}

fn healthz(state: &SrvState) -> String {
    let mut suites = serde_json::Map::new();
    for slot in &state.slots {
        let st = slot.state.lock().expect("slot lock");
        let mut entry = serde_json::Map::new();
        entry.insert(
            "state".into(),
            serde_json::Value::String(st.as_str().into()),
        );
        // The budget posture + the applied epoch tag (the curator's read;
        // epoch 0 at boot from the manifest, T6).
        if let Some(row) = state.ctx.manifest.row(slot.suite) {
            entry.insert("load".into(), serde_json::Value::String(row.budget.load.clone()));
        }
        if let Some(tag) = st.applied() {
            entry.insert("epoch".into(), serde_json::json!(tag.epoch));
        }
        if let LaneState::Ready { server, .. } = &*st {
            let meta = server.meta();
            entry.insert("arm".into(), serde_json::Value::String(meta.arm.name()));
            entry.insert("source".into(), serde_json::json!(match meta.source {
                riir_instinct::server::WeightSource::RawWinner => "raw_winner",
                riir_instinct::server::WeightSource::Vessel => "vessel",
                riir_instinct::server::WeightSource::ReflexOnly => "reflex_only",
            }));
            entry.insert("labels".into(), serde_json::json!(meta.labels));
            entry.insert("artifact_labels".into(), serde_json::json!(meta.artifact_labels));
            entry.insert("effective_cap".into(), serde_json::json!(meta.effective_cap));
            entry.insert("head_scale".into(), serde_json::json!(meta.head_scale));
            entry.insert("nb_scale".into(), serde_json::json!(meta.nb_scale));
            entry.insert("score_threshold".into(), serde_json::json!(meta.score_threshold));
            entry.insert("distance_threshold".into(), serde_json::json!(meta.distance_threshold));
            entry.insert("winner_blake3".into(), serde_json::json!(meta.winner_blake3));
        }
        if let LaneState::Failed { error, .. } = &*st {
            entry.insert("error".into(), serde_json::Value::String(error.clone()));
        }
        suites.insert(slot.suite.to_string(), serde_json::Value::Object(entry));
    }
    let doc = serde_json::json!({
        "status": "ok",
        "service": "riir-instinct",
        "version": VERSION,
        "build": fingerprint(),
        "features": COMPILED_FEATURES,
        "rustc": {"release": RUSTC_RELEASE, "commit": RUSTC_COMMIT, "host": RUSTC_HOST},
        "suites": serde_json::Value::Object(suites),
    });
    serde_json::to_string(&doc).unwrap_or_else(|_| "{\"status\":\"ok\"}".into())
}

/// The /decide edge: parse → lane lookup → decide → receipt.
fn decide_edge(stream: &mut TcpStream, srv: &Arc<SrvState>, body: &[u8], cors: Option<&str>) {
    #[derive(serde::Deserialize)]
    struct DecideReq {
        suite: String,
        state: String,
        options: Option<Vec<String>>,
        /// The multi-question contract (Issue 011, the `decision_wire`
        /// law: one state, ALL questions answered in one call). Mutually
        /// exclusive with `options` — a body carrying both is ambiguous
        /// and refuses.
        questions: Option<Vec<WireQuestion>>,
    }
    #[derive(serde::Deserialize)]
    struct WireQuestion {
        id: String,
        /// "choice" | "score" | "noul" (the harness `QKind` vocabulary).
        kind: String,
        #[serde(default)]
        instructions: String,
        /// A noul question may carry NO options — the fixed [false, true]
        /// rendering speaks it.
        #[serde(default)]
        options: Vec<String>,
    }
    let parsed: Result<DecideReq, _> = serde_json::from_slice(body);
    let req = match parsed {
        Ok(r) => r,
        Err(e) => {
            json_error(stream, "400 Bad Request", "bad_json", &e.to_string(), cors);
            return;
        }
    };
    if req.options.is_some() && req.questions.is_some() {
        json_error(
            stream,
            "400 Bad Request",
            "bad_field",
            "ambiguous body: `options` and `questions` are two different contracts — send one",
            cors,
        );
        return;
    }
    /// One wire question resolved to its serve form: (id, kind,
    /// instructions, options). A type alias — the tuple rides the
    /// borrow checker, not the reader.
    type WireTuple<'a> = (&'a str, &'a str, &'a str, Vec<String>);
    let wire_questions: Option<Vec<WireTuple<'_>>> = req
        .questions
        .as_ref()
        .map(|qs| {
            qs.iter()
                .map(|q| {
                    (
                        q.id.as_str(),
                        q.kind.as_str(),
                        q.instructions.as_str(),
                        q.options.clone(),
                    )
                })
                .collect()
        });
    if let Some(questions) = &wire_questions {
        for (id, kind, _, _) in questions {
            if !matches!(*kind, "choice" | "score" | "noul") {
                json_error(
                    stream,
                    "400 Bad Request",
                    "bad_field",
                    &format!(
                        "question {id:?}: unknown kind {kind:?} — the wire speaks \"choice\" | \"score\" | \"noul\""
                    ),
                    cors,
                );
                return;
            }
        }
    }
    let Some((idx, slot)) = srv.slots.iter().enumerate().find(|(_, s)| s.suite == req.suite)
    else {
        let registered: Vec<&str> = srv.slots.iter().map(|s| s.suite).collect();
        json_error(
            stream,
            "404 Not Found",
            "unknown_suite",
            &format!(
                "suite {:?} is not served here (served: {registered:?}); the full registry is at GET /healthz",
                req.suite
            ),
            cors,
        );
        return;
    };
    // The lazy trigger (Proposal 001 T5): an Unloaded slot transitions to
    // Loading ATOMICALLY exactly once (a second concurrent decide sees
    // Loading and waits like the boot window); the winner spawns the
    // loader. The 503 covers the load window.
    if slot.begin_lazy_load() {
        eprintln!(
            "[riir-instinct] arsenal: lazy load triggered for {:?} (first decision)",
            req.suite
        );
        spawn_loader(srv, idx);
        json_error(
            stream,
            "503 Service Unavailable",
            "loading",
            &format!(
                "suite {:?} is lazy and not loaded — load triggered by this request; retry shortly",
                req.suite
            ),
            cors,
        );
        return;
    }
    let mut guard = slot.state.lock().expect("slot lock");
    let server = match &mut *guard {
        LaneState::Ready { server, .. } => server,
        LaneState::Loading { .. } => {
            json_error(
                stream,
                "503 Service Unavailable",
                "loading",
                &format!("suite {:?} is still loading (seat boot or lazy load) — retry shortly", req.suite),
                cors,
            );
            return;
        }
        LaneState::Failed { error, .. } => {
            json_error(
                stream,
                "503 Service Unavailable",
                "lane_failed",
                &format!("suite {:?} failed to load: {error}", req.suite),
                cors,
            );
            return;
        }
        LaneState::Unloaded { .. } => unreachable!("the lazy trigger above consumed Unloaded"),
    };
    // The contract dispatch (Issue 011): a `questions` body is the
    // multi-question form; the legacy `options`/bare body stays the
    // single-question form byte-for-byte. The responses share one shape
    // per decision; the multi form wraps them in a top-level `decisions`
    // array (each element carries the same fields + receipt the single
    // form returns).
    let result = match (&wire_questions, req.options.as_deref()) {
        (Some(wire), None) => {
            let served: Vec<riir_instinct::server::ServedQuestion<'_>> = wire
                .iter()
                .map(|(id, kind, instructions, options)| {
                    riir_instinct::server::ServedQuestion {
                        qid: id,
                        kind: match *kind {
                            "choice" => QKind::Choice,
                            "score" => QKind::Score,
                            _ => QKind::Noul,
                        },
                        instructions,
                        options,
                    }
                })
                .collect();
            server
                .decide_multi(&req.state, &served)
                .map(MultiDecision::Many)
        }
        (None, _) => server
            .decide(&req.state, req.options.as_deref())
            .map(MultiDecision::One),
        (Some(_), Some(_)) => unreachable!("the ambiguous body refused above"),
    };
    match result {
        Ok(MultiDecision::One(d)) => {
            let doc = serde_json::json!({
                "suite": d.suite,
                "arm": d.arm,
                "lane": "hybrid",
                "options": d.options,
                "pick": d.pick,
                "pick_index": d.pick_index,
                "probabilities": d.probabilities,
                "specialist_scores": d.specialist_scores,
                "confidence": d.confidence,
                "escalated": d.escalated,
                "abstained": d.abstained,
                "us": d.us,
                "receipt": {
                    "build": fingerprint(),
                    "features": COMPILED_FEATURES,
                    "input": input_blake3(&req.state, &d.options),
                    "decision": decision_blake3(&d),
                    "lane": "hybrid",
                },
            });
            respond(
                stream,
                "200 OK",
                &serde_json::to_string(&doc).unwrap_or_else(|_| "{\"error\":\"serialize\"}".into()),
                cors,
            );
            // The capture is AFTER the response is written — the decide
            // path's latency never sees it, and a capture panic (none
            // expected; the sink is infallible) could not eat a reply.
            #[cfg(feature = "decstat")]
            riir_instinct::decstat::record(d.suite, &d.arm, d.abstained);
        }
        Ok(MultiDecision::Many(decisions)) => {
            let wire = wire_questions.as_deref().unwrap_or_default();
            let docs: Vec<serde_json::Value> = decisions
                .iter()
                .zip(wire)
                .map(|(d, (id, _, _, _))| {
                    serde_json::json!({
                        "suite": d.suite,
                        "arm": d.arm,
                        "lane": "hybrid",
                        "question_id": id,
                        "options": d.options,
                        "pick": d.pick,
                        "pick_index": d.pick_index,
                        "probabilities": d.probabilities,
                        "specialist_scores": d.specialist_scores,
                        "confidence": d.confidence,
                        "escalated": d.escalated,
                        "abstained": d.abstained,
                        "us": d.us,
                        "receipt": {
                            "build": fingerprint(),
                            "features": COMPILED_FEATURES,
                            "input": input_blake3(&req.state, &d.options),
                            "decision": decision_blake3(d),
                            "lane": "hybrid",
                        },
                    })
                })
                .collect();
            let doc = serde_json::json!({
                "suite": req.suite,
                "lane": "hybrid",
                "n_decisions": decisions.len(),
                "decisions": docs,
            });
            respond(
                stream,
                "200 OK",
                &serde_json::to_string(&doc).unwrap_or_else(|_| "{\"error\":\"serialize\"}".into()),
                cors,
            );
            #[cfg(feature = "decstat")]
            for d in &decisions {
                riir_instinct::decstat::record(d.suite, &d.arm, d.abstained);
            }
        }
        Err(e) => {
            // The bridge refusal + empty state + duplicate options — the
            // caller's shape errors (422); everything else is lane state.
            let (status, code) = if e.starts_with("presented options neither")
                || e.starts_with("question ")
                || e.starts_with("suite ") && e.contains("noul")
                || e.contains("multi-question contract")
            {
                ("422 Unprocessable Entity", "bridge_undefined")
            } else if e == "empty state" || e == "empty question set" {
                ("400 Bad Request", "empty_state")
            } else {
                ("422 Unprocessable Entity", "bad_request")
            };
            json_error(stream, status, code, &e, cors);
        }
    }
}

/// The /decide dispatch's two answer shapes — the legacy single decision
/// and the multi-question contract's answer set (same per-decision
/// fields).
enum MultiDecision {
    One(riir_instinct::server::ServedDecision),
    Many(Vec<riir_instinct::server::ServedDecision>),
}

/// The T5 wire release (the L5 curator's message — a wire-only server
/// has no AOI to observe): evict a loaded LAZY suite. The server keeps
/// the applied epoch tag; the next decision re-triggers the load. Eager
/// rows refuse (posture-as-data — the manifest is where the posture
/// changes).
fn release_edge(stream: &mut TcpStream, srv: &SrvState, body: &[u8], cors: Option<&str>) {
    #[derive(serde::Deserialize)]
    struct ReleaseReq {
        suite: String,
    }
    let req: ReleaseReq = match serde_json::from_slice(body) {
        Ok(r) => r,
        Err(e) => {
            json_error(stream, "400 Bad Request", "bad_json", &e.to_string(), cors);
            return;
        }
    };
    let Some(slot) = srv.slots.iter().find(|s| s.suite == req.suite) else {
        let registered: Vec<&str> = srv.slots.iter().map(|s| s.suite).collect();
        json_error(
            stream,
            "404 Not Found",
            "unknown_suite",
            &format!("suite {:?} is not served here (served: {registered:?})", req.suite),
            cors,
        );
        return;
    };
    let posture = srv
        .ctx
        .manifest
        .row(slot.suite)
        .map(|r| r.budget.load.as_str())
        .unwrap_or("eager");
    if posture != "lazy" {
        json_error(
            stream,
            "409 Conflict",
            "not_lazy",
            &format!(
                "suite {:?}'s manifest posture is {posture:?} — release applies to lazy rows \
                 only (edit the manifest to change the posture)",
                req.suite
            ),
            cors,
        );
        return;
    }
    use riir_instinct::arsenal_ops::{ReleaseOutcome, ReleaseRefusal};
    match slot.release() {
        Ok(ReleaseOutcome::Released) => {
            eprintln!(
                "[riir-instinct] arsenal: released {:?} (lazy eviction; epoch kept)",
                req.suite
            );
            respond(
                stream,
                "200 OK",
                &format!(
                    "{{\"status\":\"released\",\"suite\":{},\"epoch\":{}}}",
                    serde_json::to_string(&req.suite).unwrap_or_default(),
                    slot.applied_epoch().unwrap_or(0),
                ),
                cors,
            );
        }
        Ok(ReleaseOutcome::Noop) => {
            respond(
                stream,
                "200 OK",
                &format!(
                    "{{\"status\":\"noop\",\"suite\":{},\"epoch\":{}}}",
                    serde_json::to_string(&req.suite).unwrap_or_default(),
                    slot.applied_epoch().unwrap_or(0),
                ),
                cors,
            );
        }
        Err(ReleaseRefusal::InFlight) => {
            json_error(
                stream,
                "409 Conflict",
                "loading_in_progress",
                &format!("suite {:?} has a load in flight — retry after it settles", req.suite),
                cors,
            );
        }
        Err(ReleaseRefusal::NotReady) => {
            json_error(
                stream,
                "409 Conflict",
                "not_ready",
                &format!(
                    "suite {:?} is in the failed state — nothing loaded to release (the error \
                     stays visible at GET /healthz)",
                    req.suite
                ),
                cors,
            );
        }
    }
}

/// The T6 atomic monotonic swap (interim caller: the operator — swap
/// POLICY is game-side, 048 L9). The caller supplies the epoch (the
/// curator's series); the digest is computed server-side over the
/// artifact bytes (the same quantity the manifest pins), so a fork is a
/// fact about bytes, never about claims. Gates in order: epoch-tag gate
/// (idempotent no-op fast-path, fork/downgrade refusal), the load
/// itself, the Vendi hoarding gate, then the authoritative install gate
/// under the slot lock (a racing swap is caught there). FORCE overrides
/// the epoch gate and logs loudly; the hoarding gate still applies.
fn swap_edge(stream: &mut TcpStream, srv: &Arc<SrvState>, body: &[u8], cors: Option<&str>) {
    #[derive(serde::Deserialize)]
    struct SwapReq {
        suite: String,
        /// Bare artifact filename in the row's directory (the winners dir
        /// in raw mode, the vessels dir in vessel mode).
        artifact: String,
        epoch: u64,
        #[serde(default)]
        force: bool,
    }
    let req: SwapReq = match serde_json::from_slice(body) {
        Ok(r) => r,
        Err(e) => {
            json_error(stream, "400 Bad Request", "bad_json", &e.to_string(), cors);
            return;
        }
    };
    let Some(slot) = srv.slots.iter().find(|s| s.suite == req.suite) else {
        let registered: Vec<&str> = srv.slots.iter().map(|s| s.suite).collect();
        json_error(
            stream,
            "404 Not Found",
            "unknown_suite",
            &format!("suite {:?} is not served here (served: {registered:?})", req.suite),
            cors,
        );
        return;
    };
    let suite = slot.suite;
    // An artifact-less A0 row has nothing to hot-swap (owner 2026-10-02
    // full-coverage serving): the lane IS the modelless tier, and a
    // posture change is a manifest edit — posture-as-data, never a
    // runtime artifact push onto a row that pins no digest.
    if srv.ctx.manifest.row(suite).is_some_and(|r| r.digest.is_none()) {
        json_error(
            stream,
            "400 Bad Request",
            "bad_field",
            &format!(
                "suite {suite:?}: the row carries no artifact digest (artifact-less A0) — \
                 nothing to swap; change the posture in the arsenal manifest",
            ),
            cors,
        );
        return;
    }
    if req.artifact.is_empty()
        || req.artifact.contains('/')
        || req.artifact.contains('\\')
        || req.artifact.contains("..")
    {
        json_error(
            stream,
            "400 Bad Request",
            "bad_field",
            &format!(
                "artifact {:?} must be a bare filename (no path separators)",
                req.artifact
            ),
            cors,
        );
        return;
    }
    // The raw-mode convention coupling (Issue 579): a bridged suite only
    // ever swaps in its bridged winner — any other artifact would serve
    // the suite's bag convention over weights not trained under it.
    // Vessel files carry their own naming convention and skip this (the
    // vessel lane is cfg'd; a banking77 vessel must be re-minted from the
    // v2 artifact by its producer).
    #[cfg(not(feature = "vessel"))]
    if let Err(e) = riir_instinct::specialist::check_winner_file(suite, &req.artifact) {
        json_error(stream, "400 Bad Request", "bad_field", &e, cors);
        return;
    }
    #[cfg(feature = "vessel")]
    if srv.ctx.vessel.is_none() {
        if let Err(e) = riir_instinct::specialist::check_winner_file(suite, &req.artifact) {
            json_error(stream, "400 Bad Request", "bad_field", &e, cors);
            return;
        }
    }
    let Some(row) = srv.ctx.manifest.row(suite) else {
        json_error(stream, "404 Not Found", "unknown_suite", "row vanished", cors);
        return;
    };
    let cap = (row.budget.max_payload_mb << 20) as usize;
    #[cfg(feature = "vessel")]
    let cap = if srv.ctx.vessel.is_some() {
        reflexer_vessel::PREFIX_LEN.saturating_add(cap)
    } else {
        cap
    };
    // Single read: these exact bytes are hashed for the digest half of
    // the tag AND handed to the loader.
    let artifact_path = artifact_path_for(srv, &req.artifact);
    let bytes = match read_bounded(&artifact_path, cap) {
        Ok(b) => b,
        Err(e) => {
            json_error(stream, "404 Not Found", "artifact_not_found", &e, cors);
            return;
        }
    };
    let digest = *blake3::hash(&bytes).as_bytes();

    // Advisory pre-gate (before the seconds-class load): the idempotent
    // no-op fast path answers without any load, and a fork/downgrade
    // against the CURRENT applied tag refuses before it. The
    // authoritative gate runs again at install — a racing swap in between
    // is caught there (applied only ever moves forward, so an advisory
    // refusal can only stay refused or change refusal CLASS at install).
    if !req.force {
        let guard = slot.state.lock().expect("slot lock");
        if let LaneState::Ready { .. } = &*guard {
            if let Some(applied) = guard.applied() {
                match check_epoch_tag(req.epoch, digest, applied) {
                    Ok(EpochApply::Idempotent) => {
                        swap_ok(stream, "noop", &req.suite, req.epoch, &digest, cors);
                        return;
                    }
                    Err(e) => {
                        let code = match e {
                            SwapRefusal::Downgrade { .. } => "downgrade",
                            SwapRefusal::Fork { .. } => "fork",
                        };
                        json_error(stream, "409 Conflict", code, &e.to_string(), cors);
                        return;
                    }
                    Ok(EpochApply::Advance) => {}
                }
            }
        }
    }

    // The load runs OUTSIDE the slot lock — decisions keep serving the
    // old snapshot until the atomic install.
    let lane = match load_lane(&srv.ctx, suite, Some(&req.artifact)) {
        Ok(l) => l,
        Err(e) => {
            json_error(
                stream,
                "400 Bad Request",
                "load_failed",
                &format!("swap artifact refused: {e}"),
                cors,
            );
            return;
        }
    };
    let centroid = lane.server.centroid();
    if let Err(e) = hoard_gate(srv, suite, centroid) {
        json_error(stream, "409 Conflict", "hoard_gate", &e, cors);
        return;
    }
    let tag = EpochTag {
        epoch: req.epoch,
        digest,
    };
    if req.force {
        let applied = slot
            .state
            .lock()
            .expect("slot lock")
            .applied()
            .copied()
            .unwrap_or(EpochTag::GENESIS);
        eprintln!(
            "[riir-instinct] arsenal FORCE swap: {suite} → epoch {} digest blake3:{} over \
             applied (epoch {}, digest blake3:{}) — operator override",
            req.epoch,
            hex_digest(&digest),
            applied.epoch,
            hex_digest(&applied.digest),
        );
    }
    match slot.install_ready(lane.server, tag, centroid, req.force) {
        Ok(outcome) => {
            #[cfg(feature = "vessel")]
            if let (Some(cfg), Some(facts)) = (srv.ctx.vessel.as_ref(), lane.facts.as_ref()) {
                if let Err(e) = write_applied(&cfg.state_dir, suite, facts) {
                    eprintln!("[riir-instinct] arsenal: {suite}: persist applied state: {e}");
                }
            }
            use riir_instinct::arsenal_ops::InstallOutcome;
            let status = match outcome {
                InstallOutcome::Advanced => "advanced",
                InstallOutcome::Idempotent => "noop",
                InstallOutcome::Forced => "forced",
            };
            eprintln!(
                "[riir-instinct] arsenal: swapped {suite} → epoch {} (blake3:{}, {status})",
                req.epoch,
                hex_digest(&digest),
            );
            swap_ok(stream, status, &req.suite, req.epoch, &digest, cors);
        }
        Err(e) => {
            let code = match e {
                SwapRefusal::Downgrade { .. } => "downgrade",
                SwapRefusal::Fork { .. } => "fork",
            };
            json_error(stream, "409 Conflict", code, &e.to_string(), cors);
        }
    }
}

/// The swap artifact's path: the vessels dir in vessel mode, the winners
/// dir in raw mode — the row's own directory either way.
fn artifact_path_for(srv: &SrvState, artifact: &str) -> std::path::PathBuf {
    #[cfg(feature = "vessel")]
    if let Some(cfg) = srv.ctx.vessel.as_ref() {
        return cfg.dir.join(artifact);
    }
    std::path::Path::new(&srv.ctx.winners_dir).join(artifact)
}

/// The swap success body (shared by the noop fast path and the install).
fn swap_ok(
    stream: &mut TcpStream,
    status: &str,
    suite: &str,
    epoch: u64,
    digest: &[u8; 32],
    cors: Option<&str>,
) {
    respond(
        stream,
        "200 OK",
        &format!(
            "{{\"status\":{status:?},\"suite\":{},\"epoch\":{epoch},\"digest\":\"blake3:{}\"}}",
            serde_json::to_string(suite).unwrap_or_default(),
            hex_digest(digest),
        ),
        cors,
    );
}

/// Hex-encode a 32-byte digest (the display form of a blake3 — 64 chars).
fn hex_digest(b: &[u8; 32]) -> String {
    use std::fmt::Write as _;
    let mut s = String::with_capacity(64);
    for byte in b {
        let _ = write!(s, "{byte:02x}");
    }
    s
}

// The receipt halves (`input_blake3` / `decision_blake3`) and the build
// fingerprint (`fingerprint`) are LIB-side now — Plan 043 C0's
// one-definition law: `riir_instinct::receipt` is the ONE home, so the
// submitter, the verifier, and this edge all spell the receipt
// identically. The `use` at the top of this file binds the names; the
// call sites below are unchanged.
