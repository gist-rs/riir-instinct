# Issue 024 — serve_edge slow-drip lane-hold: per-read timeout is not a request deadline

**Status:** OPEN — robustness defect, verified 2026-10-30 (distill session zed-m3-impeccable-distill / riir-refine Research 356; independently confirmed by the verdict reviewer)

## Finding

`src/serve_edge.rs:82,943` sets ONLY a per-read timeout before reading the request:

```rust
const READ_TIMEOUT: Duration = Duration::from_secs(10);
...
stream.set_read_timeout(Some(READ_TIMEOUT))?;
```

`SO_RCVTIMEO` bounds a **single blocked `read`**, and **any byte received resets it**. A slow-drip peer (1 byte per 9 s) keeps each read returning `Ok(1)` forever — `read_request` never returns, the connection's accept slot is pinned indefinitely. There is no total request deadline, no watchdog, no `shutdown(Both)` backstop.

Severity is bounded (the serve edge is loopback by default; CORS closed), but the lane-hold is real: one hung or malicious local client holds a connection slot forever, and repeated slow-drip connections exhaust the accept loop.

## Reference fix shape (external, measured)

`pbakaus/impeccable` @ `02523aec` `crates/common/src/http.rs:461–531` fixed exactly this class: a **total wall-clock deadline across the whole request read** enforced by a 50 ms-polling watchdog thread that calls `shutdown(Both)` when it elapses, then restores the prior per-read timeout (kept as a coarse backstop).

Platform caveat carried in their fix (measured, triage D3 BLOCKER): **on Windows, a `recv` already parked in the kernel is NOT unblocked by another thread's `shutdown`** — the per-read timeout must stay as the second enforcement axis; the watchdog alone cannot unblock a parked read there. The deadline must be enforced on BOTH axes per-platform.

## Fix sketch

1. Wrap the request read in `read_request_deadline(stream, max_body, deadline)`-style: total deadline (10 s is far more than any legitimate request needs), watchdog thread + `done` AtomicBool, `shutdown(Both)` on expiry, restore prior timeout after.
2. The per-read `READ_TIMEOUT` stays as the Windows backstop + coarse limit.

## Refs

- riir-refine `.research/356_impeccable_design_detector_distill.md` (the distill verdict; the corpus rule candidate `per-read-timeout-vs-request-deadline` will encode the general law)
- riir-reflex issue 103 (the same defect in the reflex serve edge)
