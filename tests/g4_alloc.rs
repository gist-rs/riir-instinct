//! G4 — the hybrid hot path is allocation-free (Issue 005 G4). A dedicated
//! test binary so the counting global allocator sees no concurrent test
//! traffic: lib tests run multi-threaded and would pollute the counter.

use riir_instinct::{
    A0Answer, Cascade, HybridLane, MAX_TOP_K, PriorFusion, Specialist, SpecialistLane,
    prior_fusion_pick,
};
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

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

#[test]
fn hybrid_hot_path_allocates_nothing() {
    // One toy specialist built DIRECTLY (the struct fields are the
    // contract; the artifact path is the loader's business and covered
    // by the format fixtures + the live winner pin).
    let labels = vec!["s0".to_string(), "s1".to_string(), "s2".to_string()];
    let spec = Specialist {
        suite: "toy".into(),
        labels: labels.clone(),
        w: vec![0.0f32; 3 * riir_instinct::specialist::VOCAB],
        b: vec![0.1f32; 3],
    };
    let lane = HybridLane::Specialist(
        SpecialistLane::join(spec, "toy", &labels, Cascade { top_k: 2 }).expect("join"),
    );

    // A deterministic multi-token state bag.
    let mut bag = Vec::new();
    let mut scratch = Vec::new();
    riir_instinct::specialist::bag_into(b"alpha beta gamma", &mut bag, &mut scratch);
    assert!(!bag.is_empty());

    let probs: Vec<f64> = (0..3).map(|i| ((i as f64) * 0.37).cos().abs()).collect();
    let mut survivors = [(0usize, 0.0f64); MAX_TOP_K];
    let mut scores = [0.0f32; MAX_TOP_K];
    let mut label_scores = [0.0f32; 3];
    let fusion = PriorFusion {
        beta: 1.0,
        n_min: 4.0,
        tau_n: 4.0,
    };
    let nb = [1.5f32, -0.5, 0.25];

    // The identity position bridge: the G4 hot path exercises the same
    // per-case class-map argument the arena passes (0 allocations holds
    // through it).
    let class_of_pos: Vec<usize> = (0..probs.len()).collect();

    // Warm any lazy paths OUTSIDE the counting window.
    let a0 = A0Answer {
        probs: &probs,
        pick: 3,
        abstained: true,
    };
    let _ = lane.h1_decide(a0, &bag, &class_of_pos, &mut survivors, &mut scores);
    let _ = prior_fusion_pick(&fusion, &label_scores, &nb, 9, 5);

    ALLOCS.store(0, Ordering::Relaxed);
    COUNTING.store(true, Ordering::Relaxed);
    let mut checksum = 0usize;
    for _ in 0..10_000 {
        let a0 = A0Answer {
            probs: &probs,
            pick: 3,
            abstained: true,
        };
        let d = lane.h1_decide(a0, &bag, &class_of_pos, &mut survivors, &mut scores);
        lane.scores_label_into(&bag, &mut label_scores);
        let f = prior_fusion_pick(&fusion, &label_scores, &nb, 9, 5);
        checksum += d.pick + f.pick + usize::from(d.escalated);
    }
    COUNTING.store(false, Ordering::Relaxed);
    black_box(checksum);
    assert_eq!(
        ALLOCS.load(Ordering::Relaxed),
        0,
        "the hybrid hot path allocated (h1 + prune + fusion + label scores)"
    );
}
