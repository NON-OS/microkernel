// NONOS Operating System (AGPL-3.0-or-later)

//! Which step of the claim refused and with what errno, kept for the status
//! op. A laptop has no serial port, so the panel is the only place a refused
//! claim can be read; "could not be claimed" alone does not say whether the
//! broker refused the claim, the command register or the register window.

use core::sync::atomic::{AtomicU32, Ordering};

/// The broker refused the claim itself.
pub const STEP_CLAIM: u32 = 1;
/// The claim held, but the Memory Space write to the command register was
/// refused.
pub const STEP_MEMORY: u32 = 2;
/// The register window could not be mapped.
pub const STEP_MAP: u32 = 3;

static STEP: AtomicU32 = AtomicU32::new(0);
static ERRNO: AtomicU32 = AtomicU32::new(0);

/// Keep `step` and the syscall's return `rc` (an errno, negative), so the
/// last refused try is the one the panel shows. A return that is not negative
/// keeps errno 0: the claim's epoch 0 is a refusal with no errno.
pub fn note(step: u32, rc: i64) {
    let errno = if rc < 0 { rc.unsigned_abs().min(u32::MAX as u64) as u32 } else { 0 };
    STEP.store(step, Ordering::Relaxed);
    ERRNO.store(errno, Ordering::Relaxed);
}

/// Forget a refusal once a later try held, so the panel does not name it.
pub fn clear() {
    note(0, 0);
}

/// The step that refused last (0: none) and its errno, as a positive number.
pub fn read() -> (u32, u32) {
    (STEP.load(Ordering::Relaxed), ERRNO.load(Ordering::Relaxed))
}
