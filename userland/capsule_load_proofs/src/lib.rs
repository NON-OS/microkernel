// NONOS Operating System (AGPL-3.0-or-later)
//! Host proof for the runtime capsule-load certificate clock gate. The real
//! gate source is included via #[path] so the test pins production behavior.

#[path = "../../../src/kernel_core/process_spawn/capsule_spawn/from_vfs/validity_clock.rs"]
pub mod validity_clock;

/// The kernel's choice of the clock a certificate window is checked against,
/// for the tests: a plausible wall clock passes, an unset one gives `None`.
pub fn validity_now_ms(now: u64) -> Option<u64> {
    validity_clock::validity_now_ms(now)
}

#[cfg(test)]
mod tests;
