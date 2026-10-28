// NONOS Operating System (AGPL-3.0-or-later)

//! Which claim step the kernel refused and with what errno, kept for the
//! status op: a laptop has no serial port, so the panel is where a refused
//! claim is read. The steps are the RTL8821CE's, so the panel's reading of
//! them (`nonos_wifi_client::StopWhy`) holds for this driver too.

use core::sync::atomic::{AtomicU32, Ordering};

/// The broker refused the claim itself.
pub const STEP_CLAIM: u32 = 1;
/// The claim held; the Memory Space write to the command register did not.
pub const STEP_MEMORY: u32 = 2;
/// The register window could not be mapped.
pub const STEP_MAP: u32 = 3;

static STEP: AtomicU32 = AtomicU32::new(0);
static ERRNO: AtomicU32 = AtomicU32::new(0);

/// Keep `step` and the call's return `rc`; a return that is not negative
/// keeps errno 0 (a claim's epoch 0 is a refusal with no errno).
pub fn note(step: u32, rc: i64) {
    let errno = if rc < 0 { rc.unsigned_abs().min(u32::MAX as u64) as u32 } else { 0 };
    STEP.store(step, Ordering::Relaxed);
    ERRNO.store(errno, Ordering::Relaxed);
}

/// Forget a refusal once a later try held.
pub fn clear() {
    note(0, 0);
}

/// The step that refused last (0: none) and its errno, positive.
pub fn read() -> (u32, u32) {
    (STEP.load(Ordering::Relaxed), ERRNO.load(Ordering::Relaxed))
}
