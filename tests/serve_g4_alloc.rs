//! G4 — the bag serve path's allocation pin (instinct issue 021, the
//! ESC cheap-path alloc lead). A dedicated test binary (the house
//! counting-allocator pattern, `g4_alloc.rs` + Rethink's
//! `esc_g4_alloc.rs`) so the global allocator sees no concurrent test
//! traffic.
//!
//! SCOPE: the CHEAP bag server's own serve path — `decide` /
//! `decide_multi` on a booted [`AnySuiteServer`]. This is the open
//! lane's own surface (the serve bin answers through it directly) AND
//! the ESC cheap tier's inner half; Rethink's `esc_g4_alloc` pins the
//! COMPOSED armed path (190, a ceiling) and deliberately includes this
//! surface inside its number — that pin is NOT restated here: this
//! gate measures the cheap server WITHOUT the wrapper prelude, the
//! gate leg, and the receipt `format!`, so a regression local to the
//! bag server reds here even when the composed number happens to hold.
//!
//! The measured classes (issue 021's leads, inventoried by Rethink's
//! G4 doc): the synth case construction (a `serde_json::Value` build
//! per request), the `eval_seat` result Vecs (per-question probs —
//! GONE since reflex issue 070 lead 2: the eval refills the per-server
//! `eval_frame` instead), the bridge prelude (per-question pos Vecs +
//! the rendered options), and the receipt tier (pick String, options
//! clone, arm-name String) — the single-question `decide` prelude's
//! template clones left with issue 022 lead 2 (the take/replace window
//! moves the three fields out of `self` for the call; the None-options
//! arm presents the TAKEN labels, zero allocations). None of these are
//! waste (every allocation is exact-size) — the pin exists so the
//! surface can only move DOWN deliberately (an ownership/scratch
//! refactor) and never UP silently.
//!
//! Skip-loud posture: `INSTINCT_ENCODER_PARITY=1` (the pay-the-boot
//! opt-in — the seat preparation pays the minutes-class selection
//! ladders) + the t20k datasets + the winners dir. A skip is a
//! deferral, never a green.

use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use riir_instinct::arsenal::ArsenalManifest;
use riir_instinct::server::{
    AnySuiteServer, ServedQuestion, synth_served_case, synth_served_case_into,
};
use riir_reflex::harness::runner::seat::prepare_seat;

/// The suite: sst5 — the same population Rethink's esc_g4 measures (the
/// ESC cheap tier's measured suite), so the two pins decompose against
/// one shared fixture front.
const SUITE: &str = "sst5";
const WARMUP: usize = 5;
const MEASURED: usize = 20;

/// The per-decision allocation ceiling for `decide` (single-question
/// wire entry). MEASURED-THEN-PINNED: **28** (2026-10-06, issue 022
/// lead 2 — the decide prelude's template clones left the hot path: the
/// take/replace window moves `labels`/`qid`/`q_instructions` out of
/// `self` for the call and `decide_multi` reads the `labels_len` boot
/// mirror; the sst5 None-options prelude was 1 Vec + 5 label Strings +
/// 2 Strings = 8, was **36**, issue 022 lead 1's pin, itself down from
/// **42**/83; `_into` steady/cold still read **0**). Deterministic
/// across runs (28 reproduced ×3). A red means a new allocation class
/// joined the serve path — or a legitimate code change moved it:
/// re-measure, inventory, re-pin with the delta named in the commit.
const PINNED_MAX_DECIDE_ALLOCS: usize = 28;

static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static COUNTING: AtomicBool = AtomicBool::new(false);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.realloc(ptr, layout, new_size) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if COUNTING.load(Ordering::Relaxed) {
            ALLOCS.fetch_add(1, Ordering::Relaxed);
        }
        unsafe { System.alloc_zeroed(layout) }
    }
}

#[global_allocator]
static A: Counting = Counting;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn datasets_dir() -> PathBuf {
    std::env::var("INSTINCT_DATASETS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root().join("../riir-reflex/.raw/datasets_t20k"))
}

fn winners_dir() -> PathBuf {
    std::env::var("INSTINCT_WINNERS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root().join("../riir-train/data/instinct_specialists"))
}

fn skip_loud(why: &str) {
    eprintln!("SKIP loud: {why} — a skip is a deferral, never a green");
}

#[test]
fn cheap_serve_path_allocation_pin() {
    if std::env::var("INSTINCT_ENCODER_PARITY").as_deref() != Ok("1") {
        skip_loud(
            "INSTINCT_ENCODER_PARITY=1 not set — the boot pays the minutes-class \
             selection ladders",
        );
        return;
    }
    let datasets = datasets_dir();
    let winners = winners_dir();
    if !datasets.join(SUITE).is_dir() || !winners_dir().is_dir() {
        skip_loud("the sst5 datasets or the winners dir are absent");
        return;
    }

    // The sst5 RESEARCH posture, carried verbatim from Rethink's
    // esc_g4_alloc (the row left the production manifest at the licence
    // demotions, issue 023 T3 — test-local, the record's own digest; no
    // escalate table: the harness/gates posture, legal by design).
    let manifest_text = "[[vessel]]\nsuite   = \"sst5\"\ndigest  = \"blake3:430558d6210737a267249500e0c3df4a0534d344752a1b4dae9a0e6952d2c001\"\nclass   = \"hosted_only\"\nposture = { arm = \"A1\" }\npin_keys = []\nbudget  = { load = \"eager\", max_payload_mb = 16 }\n";
    let manifest =
        ArsenalManifest::parse(manifest_text).expect("the sst5 research posture parses");

    // Boot the CHEAP server only (no encoder lane — this pin is the bag
    // surface) on a big-stack thread: the seat boot's stack frames exceed
    // a test thread's 2 MiB default (the serve_gates pattern).
    let datasets_for_boot = datasets.clone();
    let winners_for_boot = winners.clone();
    let mut server = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let seat = prepare_seat(SUITE, &datasets_for_boot)?;
            AnySuiteServer::boot_cheap_from_seat(SUITE, seat, &winners_for_boot, &manifest)
        })
        .expect("spawn boot thread")
        .join()
        .expect("boot thread panicked")
        .expect("the cheap server boots");

    // The cal cases + state strings (the replay population — the same
    // front Rethink's esc_g4 measures).
    let seat = prepare_seat(SUITE, &datasets).expect("prepare_seat");
    let states: Vec<String> = seat.cal_state_strs.clone();
    assert!(
        states.len() >= WARMUP + MEASURED,
        "the cal front must cover {WARMUP} warmup + {MEASURED} measured cases"
    );

    // Warm any lazy paths OUTSIDE the counting window (the specialist
    // loader, the nb/oc gathers, the engine's first eval).
    let mut warm_checksum = 0usize;
    for state in states.iter().take(WARMUP) {
        let d = server.decide(state, None).expect("warmup decide");
        warm_checksum += d.pick_index.unwrap_or(0);
    }
    black_box(warm_checksum);

    // Decomposition: the synth case construction ALONE (pure fn — its
    // own counting window), so the case share of a decision is quotable
    // beside the total.
    let state0 = states[WARMUP].as_str();
    let template_q = [ServedQuestion {
        qid: "q",
        kind: riir_reflex::harness::suites::QKind::Choice,
        instructions: "instructions",
        options: seat.labels.as_slice(),
    }];
    let before_case = ALLOCS.load(Ordering::Relaxed);
    COUNTING.store(true, Ordering::Relaxed);
    let case = synth_served_case(state0, &template_q);
    COUNTING.store(false, Ordering::Relaxed);
    let case_allocs = ALLOCS.load(Ordering::Relaxed) - before_case;
    black_box(&case);
    drop(case);

    // Issue 022 intake — the REUSE form's own split: the decide path
    // calls `synth_served_case_into` on the per-server scratch, where
    // keep-when-equal should hold in steady state (same template, same
    // option set). Cold = the first `_into` into a fresh case (builds
    // everything); steady = the second `_into` with IDENTICAL inputs
    // (the keep path). The fresh-form window above is a STAND-IN for the
    // in-decide cost only until this window exists — the issue's 21-of-42
    // attribution was read off the fresh form.
    let mut reuse_case = synth_served_case(state0, &template_q);
    let before_cold_into = ALLOCS.load(Ordering::Relaxed);
    COUNTING.store(true, Ordering::Relaxed);
    synth_served_case_into(state0, &template_q, &mut reuse_case);
    COUNTING.store(false, Ordering::Relaxed);
    let cold_into_allocs = ALLOCS.load(Ordering::Relaxed) - before_cold_into;
    let before_steady_into = ALLOCS.load(Ordering::Relaxed);
    COUNTING.store(true, Ordering::Relaxed);
    synth_served_case_into(state0, &template_q, &mut reuse_case);
    COUNTING.store(false, Ordering::Relaxed);
    let steady_into_allocs = ALLOCS.load(Ordering::Relaxed) - before_steady_into;
    black_box(&reuse_case);
    drop(reuse_case);

    // Measure: one decide per case.
    let mut max_decide = 0usize;
    for state in states.iter().skip(WARMUP).take(MEASURED) {
        let before = ALLOCS.load(Ordering::Relaxed);
        COUNTING.store(true, Ordering::Relaxed);
        let d = server.decide(state, None).expect("measured decide");
        COUNTING.store(false, Ordering::Relaxed);
        let delta = ALLOCS.load(Ordering::Relaxed) - before;
        black_box(&d);
        max_decide = max_decide.max(delta);
    }

    eprintln!(
        "serve g4: {MEASURED} decides measured — max per-decision allocs {max_decide} \
         (synth case fresh: {case_allocs}; into cold: {cold_into_allocs}; into steady: \
         {steady_into_allocs}; PINNED_MAX_DECIDE_ALLOCS \
         {PINNED_MAX_DECIDE_ALLOCS}) — a red means a new allocation class joined the \
         serve path, or this is the owed re-pin: measure, inventory, pin"
    );
    assert!(
        max_decide <= PINNED_MAX_DECIDE_ALLOCS,
        "the serve path allocated {max_decide} for one decision (pin \
         {PINNED_MAX_DECIDE_ALLOCS}) — a new allocation class joined the hot path, or \
         this is the owed re-pin: measure, inventory, and pin the measured value"
    );
}
