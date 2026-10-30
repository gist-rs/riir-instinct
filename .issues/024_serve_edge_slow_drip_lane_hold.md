# Issue 024 — serve_edge slow-drip lane-hold: per-read timeout is not a request deadline

**Status:** FIXED 2026-10-30 (zed-m3-batch316; the fix rides the corpus rule landing — riir-refine batch 316 `4d1fdb2e` encodes the general law this issue verified at home)

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

## Resolution (2026-10-30)

`src/serve_edge.rs` gained `RequestDeadlineGuard` (`arm` / `arm_with` / `end` + Drop-disarm):
the request read runs under a TOTAL wall-clock deadline (`REQUEST_DEADLINE` = 10 s) enforced on
BOTH axes — the socket read timeout set to the full deadline (the always-works axis; the Windows
parked-recv quirk) + a 50 ms-poll watchdog thread that calls `shutdown(Both)` at expiry (the
trickle backstop). `handle_conn` arms before the head read and `end`s after the head parses
(restore the coarse per-read `READ_TIMEOUT` for the bounded body reads); Drop disarms on the
error paths. The per-read `READ_TIMEOUT` stays as the coarse backstop exactly as the source kept
it. Regression gates (`request_deadline_tests`, 2 tests): a real loopback slow-drip peer (1 byte
per 60 ms then silence) is released AT the deadline (the watchdog's shutdown unblocks the parked
read; the partial receipt proves the drip was being served), and a fast legitimate request
completes undisturbed under the armed deadline. Gates: clippy -D 0, lib 105/105. The corpus rule
encoding this law: riir-refine `per-read-timeout-vs-request-deadline` (batch 316).
