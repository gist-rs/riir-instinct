//! The serve-side encoder lane's parity gate (instinct issue 016 T2):
//! the SERVE surface — an ENC manifest row through
//! [`AnySuiteServer::boot_bytes`] — replays the frozen Bench-029 read
//! (`.benchmarks/029_sst5_encoder_c1/`), and the serve lane's correct
//! count must equal the record's `accuracy × n` EXACTLY (cell identity
//! with the frozen read; the record is the ONE expected-count source,
//! never a hand-typed constant here).
//!
//! Two measured facts this gate carries from the issue:
//!
//! - The tier claim is the **L3 think slot** (issue 016's layer-fit
//!   table: 500–1000 ms per slot), not the modelless µs tier the bag
//!   gates assert — so the latency bound is the 1 s slot ceiling, and the
//!   p50/p99 print beside it with the device label (a reading can never
//!   mistake the posture).
//! - **Determinism is compute-once-and-record**: the same request
//!   re-answered must give the same pick (the authority records the
//!   decision; deterministic replay reads the record — a re-answer that
//!   diverges would be a device nondeterminism finding).
//!
//! SKIPs loud (a deferral, never a green) unless
//! `INSTINCT_ENCODER_PARITY=1` and the material exists: the sealed head
//! (`INSTINCT_ENCODER_ART`, default the Bench-029 artifact in
//! riir-train), the t20k datasets (`INSTINCT_DATASETS_DIR`), and the laya
//! weights (`LAYA_WEIGHTS_DIR` / `LAYA_HOME`, the lane's own resolution).
//! The device posture is `LAYA_DEVICE` (metal / cuda / cpu) — the frozen
//! read ran Metal on the M3; the argmax contract is what must transfer.

use std::path::PathBuf;

use riir_instinct::arsenal::{ArsenalManifest, ValidateCtx};
use riir_instinct::server::AnySuiteServer;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn datasets_dir() -> PathBuf {
    std::env::var("INSTINCT_DATASETS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root().join("../riir-reflex/.raw/datasets_t20k"))
}

fn head_path() -> PathBuf {
    std::env::var("INSTINCT_ENCODER_ART")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root().join("../riir-train/.raw/t599/t6_s0.bin"))
}

/// The gate is the sst5 C1 record's — another suite has no frozen read to
/// replay and refuses loud rather than silently skipping the count pin.
const SUITE: &str = "sst5";

#[test]
fn serve_encoder_lane_replays_the_frozen_bench029_read() {
    if std::env::var("INSTINCT_ENCODER_PARITY").as_deref() != Ok("1") {
        eprintln!(
            "SKIP loud: INSTINCT_ENCODER_PARITY=1 not set — the gate needs the sealed head, \
             the t20k datasets and the laya weights (a laya run, minutes-class); a skip is a \
             deferral, never a green"
        );
        return;
    }
    let head = head_path();
    let datasets = datasets_dir();
    if !head.is_file() || !datasets.join(SUITE).is_dir() {
        eprintln!(
            "SKIP loud: the head artifact {} or the datasets dir {} is absent",
            head.display(),
            datasets.display()
        );
        return;
    }
    let bytes = std::fs::read(&head).expect("read the sealed head");
    let digest = blake3::hash(&bytes).to_hex().to_string();
    let file_name = head
        .file_name()
        .unwrap_or_else(|| panic!("head path has a filename"))
        .to_string_lossy()
        .into_owned();
    // The ENC row the deployment shape carries: explicit `file` (the
    // winner convention does not apply), eager budget (L9 resident from
    // boot), the head's own BLAKE3. The manifest↔file drift gate runs for
    // real here — the winners dir IS the head's parent.
    let manifest_text = format!(
        "[[vessel]]\nsuite   = {SUITE:?}\ndigest  = \"blake3:{digest}\"\nclass   = \
         \"hosted_only\"\nfile    = {file_name:?}\nposture = {{ arm = \"ENC\" }}\npin_keys = \
         []\nbudget  = {{ load = \"eager\", max_payload_mb = 16 }}\n"
    );
    let manifest = ArsenalManifest::parse(&manifest_text).expect("the ENC manifest parses");
    let winners = head
        .parent()
        .unwrap_or_else(|| panic!("head path has a parent"))
        .to_path_buf();
    manifest
        .validate(&ValidateCtx::raw(&winners))
        .unwrap_or_else(|e| panic!("the ENC manifest must validate against the head file: {e}"));

    // The frozen record: the ONE expected-count source (the 029 read,
    // cell-identical across the trainer's read, the M3-Metal dump and the
    // live arena encode — three postures already agreeing).
    let record = repo_root().join(".benchmarks/029_sst5_encoder_c1/predictions.json");
    let doc: serde_json::Value = serde_json::from_reader(
        std::fs::File::open(&record)
            .unwrap_or_else(|e| panic!("open the frozen 029 record {}: {e}", record.display())),
    )
    .expect("parse the frozen 029 record");
    let entry = doc["frozen_test_predictions"]
        .as_array()
        .and_then(|a| {
            a.iter()
                .find(|s| s["suite"].as_str() == Some(SUITE))
        })
        .unwrap_or_else(|| panic!("the 029 record lacks the {SUITE} entry"));
    let ea = &entry["encoder_arm"];
    let n_frozen = ea["n"].as_u64().expect("the record's n") as usize;
    let acc = ea["accuracy"].as_f64().expect("the record's accuracy");
    let expected_correct = (acc * n_frozen as f64).round() as usize;

    // The seat: the test-split population must BE the frozen read's, and
    // the states/golds extract before the boot consumes the seat.
    let suite_static: &'static str = SUITE;
    let seat = {
        let datasets = datasets.clone();
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                riir_reflex::harness::runner::seat::prepare_seat(suite_static, &datasets)
            })
            .expect("spawn seat thread")
            .join()
            .expect("seat thread panicked")
            .expect("prepare the sst5 seat")
    };
    assert_eq!(
        seat.suite.cases.len(),
        n_frozen,
        "the seat's test split ({}) must be the frozen read's population ({n_frozen}) — \
         a datasets re-baseline reds here, never a silent re-derivation",
        seat.suite.cases.len()
    );
    let states: Vec<String> = seat.state_strs.clone();
    let golds: Vec<usize> = seat.suite.cases.iter().map(|c| c.gold[0].idx).collect();
    drop(seat);

    // Boot through the SERVE surface (the loader's entry — the manifest
    // grammar, the ENC route and the lane all in one call), on the
    // big-stack thread the seat frames need.
    let bytes_for_boot = bytes.clone();
    let datasets_for_boot = datasets;
    let mut server = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let seat = riir_reflex::harness::runner::seat::prepare_seat(suite_static, &datasets_for_boot)
                .expect("prepare the boot seat");
            AnySuiteServer::boot_bytes(suite_static, seat, &bytes_for_boot, &manifest)
        })
        .expect("spawn boot thread")
        .join()
        .expect("boot thread panicked")
        .unwrap_or_else(|e| panic!("boot the ENC lane: {e}"));
    assert_eq!(
        server.meta().arm.name(),
        "ENC",
        "the boot arm must be the ENC posture"
    );
    let device = server_meta_device(&server);

    // The replay: the full frozen population, one decide per case, the
    // canonical presentation (options = None → the seat's label universe).
    let mut durs_us: Vec<u64> = Vec::with_capacity(states.len());
    let mut picks: Vec<Option<usize>> = Vec::with_capacity(states.len());
    let mut correct = 0usize;
    for (i, state) in states.iter().enumerate() {
        let d = server
            .decide(state, None)
            .unwrap_or_else(|e| panic!("case {i}: decide: {e}"));
        if d.pick_index == Some(golds[i]) {
            correct += 1;
        }
        picks.push(d.pick_index);
        durs_us.push(d.us);
    }

    // Leg 2 — the ARENA leg (the decisive diff): the frozen arena runner
    // (`eval_encoder_arm`, the code that PRODUCED the Bench-029 read) over
    // the first cases of the SAME seat, in THIS process, on THIS box, with
    // THESE weights. If the serve lane's picks drift from the arena's on
    // identical inputs, the serve render diverged (my bug); if they agree
    // and the count still misses the frozen record, the frozen read is not
    // reproducible at this box/posture (a device drift finding, loudly).
    {
        let seat = {
            let datasets = repo_root().join("../riir-reflex/.raw/datasets_t20k");
            std::thread::Builder::new()
                .stack_size(64 * 1024 * 1024)
                .spawn(move || {
                    riir_reflex::harness::runner::seat::prepare_seat(suite_static, &datasets)
                })
                .expect("spawn seat thread")
                .join()
                .expect("seat thread panicked")
                .expect("prepare the arena-leg seat")
        };
        let n_leg = 32.min(seat.suite.cases.len());
        let art_path = head.clone();
        let cases = &seat.suite.cases[..n_leg];
        let arena = riir_instinct::encoder_arm::eval_encoder_arm(
            cases,
            &art_path,
            riir_reflex::laya::config::Checkpoint::English,
        )
        .unwrap_or_else(|e| panic!("the arena leg failed: {e}"));
        let mut agree = 0usize;
        for i in 0..n_leg {
            if picks[i] == Some(arena.picks[i]) {
                agree += 1;
            } else {
                eprintln!(
                    "DIFF case {i}: serve {:?} vs arena {} (state head: {:.40})",
                    picks[i],
                    arena.picks[i],
                    states[i].replace('\n', " ")
                );
            }
        }
        eprintln!("ENC DIFF LEG: serve-lane vs arena-runner agreement {agree}/{n_leg}");
        assert_eq!(
            agree, n_leg,
            "the serve lane's picks drifted from the arena runner's on identical inputs — \
             the serve render diverged (a lane bug), not a posture question"
        );
    }
    assert_eq!(
        correct, expected_correct,
        "the serve lane's replay drifted from the frozen Bench-029 read: {correct} correct \
         vs {expected_correct} expected over {n_frozen} — a device-posture or template drift \
         reds HERE, never a silent re-derivation"
    );

    // Compute-once-and-record: the same requests re-answered must give
    // the same picks (the sync-boundary law's replay half).
    for (i, state) in states.iter().take(32).enumerate() {
        let d = server
            .decide(state, None)
            .unwrap_or_else(|e| panic!("determinism case {i}: {e}"));
        assert_eq!(
            d.pick_index, picks[i],
            "case {i}: the replay is not deterministic — compute-once-and-record is broken"
        );
    }

    // The tier claim is the L3 think slot (issue 016's layer-fit table:
    // 500–1000 ms per slot) — the 1 s slot ceiling, NOT the modelless µs
    // tier the bag gates assert.
    let mut sorted = durs_us.clone();
    sorted.sort_unstable();
    let n = sorted.len();
    let p50 = sorted[n / 2];
    let p99_idx = (n * 99 / 100).min(n - 1);
    let p99 = sorted[p99_idx];
    let tail_support = n - p99_idx;
    assert!(
        p99 < 1_000_000,
        "p99 {p99} µs breaches the L3 slot's 1 s ceiling — the encoder lane does not fit the \
         think-depth tier on this box"
    );

    eprintln!(
        "ENC PARITY: suite {SUITE} · device {device} · correct {correct}/{n_frozen} (frozen \
         {expected_correct}) · p50 {p50} µs · p99 {p99} µs (tail support {tail_support}/{n}) · \
         head blake3:{}",
        &digest[..16]
    );
    eprintln!(
        "BOX: os {} · arch {} · parity {} (the frozen read ran Metal on the M3 — the argmax \
         contract is what transfers)",
        std::env::consts::OS,
        std::env::consts::ARCH,
        std::env::var("LAYA_DEVICE").as_deref().unwrap_or("(lane default)")
    );
}

/// The lane's device label, through the arity-erased server (the ENC
/// variant is feature-gated; this gate only compiles with the feature).
fn server_meta_device(server: &AnySuiteServer) -> &'static str {
    match server {
        AnySuiteServer::Enc(lane) => lane.device(),
        _ => panic!("the ENC boot must produce the encoder lane"),
    }
}

// ── T4: the HOSTED-ONLY head vessel (issue 016 T4) ─────────────────────

/// Assemble one HOSTED-ONLY v1 vessel over `payload` — the minter's exact
/// wire layout (riir-train `vessel_mint.rs`, pinned equal by the cross-repo
/// gate): 68-byte header ‖ strict-ed25519(header ‖ payload) ‖
/// nonce ‖ ciphertext. TEST-ONLY assembly with a TEST key; a production
/// mint stays the riir-train bin's (the owner's key material never enters
/// any repo).
#[cfg(all(feature = "vessel", feature = "serve-encoder"))]
fn assemble_head_vessel(
    head_bytes: &[u8],
    key: &ed25519_dalek::SigningKey,
    key_id: u32,
    version: u64,
    nonce: [u8; 16],
) -> Vec<u8> {
    use ed25519_dalek::Signer;
    let payload = riir_instinct::vessel::encrypt_payload(
        &key.to_bytes(),
        &nonce,
        head_bytes,
    );
    let mut header = [0u8; 68];
    header[0..8].copy_from_slice(b"RFLEXVSL");
    header[8..12].copy_from_slice(&1u32.to_le_bytes()); // format_version
    header[12..16].copy_from_slice(&1u32.to_le_bytes()); // class HOSTED-ONLY
    header[16..20].copy_from_slice(&key_id.to_le_bytes());
    header[20..28].copy_from_slice(&version.to_le_bytes());
    header[28..60].copy_from_slice(&[0u8; 32]); // parent: genesis
    header[60..68].copy_from_slice(&(payload.len() as u64).to_le_bytes());
    let mut signed = Vec::with_capacity(68 + payload.len());
    signed.extend_from_slice(&header);
    signed.extend_from_slice(&payload);
    let sig = key.sign(&signed);
    let mut vessel = Vec::with_capacity(68 + 64 + payload.len());
    vessel.extend_from_slice(&header);
    vessel.extend_from_slice(&sig.to_bytes());
    vessel.extend_from_slice(&payload);
    vessel
}

/// The T4 round trip: the same head rides the HOSTED-ONLY vessel — minted
/// here with a TEST key + deterministic nonce — and boots through
/// [`AnySuiteServer::boot_vessel_bytes`]; the vessel lane's decisions must
/// equal the raw lane's EXACTLY (the envelope is confidentiality, never
/// semantics), the lineage facts must carry the minted version, and the
/// monotonic gate must refuse a downgrade (v1 over an applied v1) without
/// decrypting a byte. Gated on BOTH features (the reader needs the vessel
/// crate, the lane needs the encoder) — a build without either refuses the
/// ENC vessel route loud at boot instead.
#[cfg(all(feature = "vessel", feature = "serve-encoder"))]
#[test]
fn head_vessel_boots_the_same_lane_and_refuses_a_downgrade() {
    if std::env::var("INSTINCT_ENCODER_PARITY").as_deref() != Ok("1") {
        eprintln!(
            "SKIP loud: INSTINCT_ENCODER_PARITY=1 not set — the vessel arm needs the head \
             artifact, the datasets and the laya weights"
        );
        return;
    }
    let head = head_path();
    let datasets = datasets_dir();
    if !head.is_file() || !datasets.join(SUITE).is_dir() {
        eprintln!("SKIP loud: the head artifact / the datasets dir is absent");
        return;
    }
    let head_bytes = std::fs::read(&head).expect("read the sealed head");

    // The TEST minting key + pin table (operator key material never enters
    // any repo; the --nonce seam's determinism role).
    let mint = ed25519_dalek::SigningKey::from_bytes(&[42u8; 32]);
    // The envelope key IS the minting seed in the minter's contract (one
    // `--key` is both the ed25519 seed and the blake3-XOF keyed hash key —
    // `INSTINCT_VESSEL_KEY_HEX` == the minter's `--key`).
    let vessel_key = mint.to_bytes();

    let vessel = assemble_head_vessel(&head_bytes, &mint, 7, 1, [3u8; 16]);
    let manifest_text = format!(
        "[[vessel]]\nsuite   = {SUITE:?}\ndigest  = \"blake3:{}\"\nclass   = \
         \"hosted_only\"\nfile    = \"{SUITE}_v1.vessel\"\nposture = {{ arm = \"ENC\" }}\n\
         pin_keys = [7]\nbudget  = {{ load = \"eager\", max_payload_mb = 16 }}\n",
        blake3::hash(&vessel).to_hex()
    );
    // Parse-shaped once here (the grammar the deployment row carries); the
    // boots re-parse per closure below.
    ArsenalManifest::parse(&manifest_text).expect("the ENC vessel manifest parses");

    // The states/golds extract before the boot consumes the seat.
    let suite_static: &'static str = SUITE;
    let seat = {
        let datasets = datasets.clone();
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                riir_reflex::harness::runner::seat::prepare_seat(suite_static, &datasets)
            })
            .expect("spawn seat thread")
            .join()
            .expect("seat thread panicked")
            .expect("prepare the sst5 seat")
    };
    let states: Vec<String> = seat.state_strs.clone();
    let n = states.len().min(32);
    drop(seat);

    // The vessel boot (the swap/lazy loader's vessel entry).
    let applied = riir_instinct::vessel::AppliedState::GENESIS;
    let vessel_for_boot = vessel.clone();
    let manifest_for_boot = riir_instinct::arsenal::ArsenalManifest::parse(&manifest_text)
        .expect("parse for boot");
    let pins_for_boot = reflexer_vessel::pins_from_bytes(&[(7u32, mint.verifying_key().to_bytes())]);
    let datasets_for_boot = datasets.clone();
    let mut server = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let seat = riir_reflex::harness::runner::seat::prepare_seat(suite_static, &datasets_for_boot)
                .expect("prepare the boot seat");
            AnySuiteServer::boot_vessel_bytes(
                suite_static,
                seat,
                &vessel_for_boot,
                &manifest_for_boot,
                &pins_for_boot,
                &vessel_key,
                &applied,
            )
        })
        .expect("spawn boot thread")
        .join()
        .expect("boot thread panicked")
        .unwrap_or_else(|e| panic!("boot the ENC head vessel: {e}"));
    assert_eq!(server.0.meta().arm.name(), "ENC");
    assert_eq!(
        server.1.artifact_version, 1,
        "the lineage facts carry the minted version (the monotonic apply's input)"
    );

    // Decision parity: the vessel lane's first n answers equal the raw
    // lane's (the envelope is confidentiality, never semantics). The raw
    // lane boots from the SAME head bytes for the comparison.
    let vessel_picks: Vec<Option<usize>> = states[..n]
        .iter()
        .map(|s| {
            server
                .0
                .decide(s, None)
                .unwrap_or_else(|e| panic!("vessel case: {e}"))
                .pick_index
        })
        .collect();
    let raw_bytes = head_bytes.clone();
    let datasets_for_raw = datasets.clone();
    let manifest_for_raw = riir_instinct::arsenal::ArsenalManifest::parse(&manifest_text)
        .expect("parse for raw");
    let mut raw_server = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let seat = riir_reflex::harness::runner::seat::prepare_seat(suite_static, &datasets_for_raw)
                .expect("prepare the raw boot seat");
            AnySuiteServer::boot_bytes(suite_static, seat, &raw_bytes, &manifest_for_raw)
        })
        .expect("spawn raw boot thread")
        .join()
        .expect("raw boot thread panicked")
        .unwrap_or_else(|e| panic!("boot the raw ENC lane: {e}"));
    for (i, s) in states[..n].iter().enumerate() {
        let raw = raw_server
            .decide(s, None)
            .unwrap_or_else(|e| panic!("raw case {i}: {e}"));
        assert_eq!(
            vessel_picks[i], raw.pick_index,
            "case {i}: the vessel lane drifted from the raw lane — the envelope changed \
             semantics, which it must never do"
        );
    }

    // The monotonic gate: v1 re-applied over an applied v1 is a DOWNGRADE
    // — refused without decrypting a byte (the bag vessels' own law). The
    // wrong-key arm is the confidentiality wall (garbage never parses).
    let applied1 = riir_instinct::vessel::AppliedState { artifact_version: 1 };
    let manifest_for_boot2 = riir_instinct::arsenal::ArsenalManifest::parse(&manifest_text)
        .expect("parse for downgrade");
    let pins_for_boot2 = reflexer_vessel::pins_from_bytes(&[(7u32, mint.verifying_key().to_bytes())]);
    let err = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let seat = riir_reflex::harness::runner::seat::prepare_seat(suite_static, &datasets)
                .expect("prepare the downgrade seat");
            AnySuiteServer::boot_vessel_bytes(
                suite_static,
                seat,
                &vessel,
                &manifest_for_boot2,
                &pins_for_boot2,
                &vessel_key,
                &applied1,
            )
        })
        .expect("spawn downgrade thread")
        .join()
        .expect("downgrade thread panicked");
    let err = match err {
        Err(e) => e,
        Ok(_) => panic!("a downgrade must refuse, never boot"),
    };
    assert!(
        err.contains("refused"),
        "the downgrade refusal must come from the vessel walk: {err}"
    );
}
