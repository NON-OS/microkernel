// NONOS Operating System (AGPL-3.0-or-later)
//! Host proof for the boot wall-clock source picker. The real picker is included
//! via #[path]; a thin public wrapper keeps it referenced in the library build
//! while the assertions live in the tests.

#[path = "../../../src/sys/clock/resolve.rs"]
mod resolve;

/// Non-test reference so the included picker is not flagged unused in the
/// library build.
pub fn pick(handoff: u64, calibrated: u64, fresh: u64) -> u64 {
    resolve::pick_nonzero(handoff, || calibrated, || fresh)
}

#[cfg(test)]
mod tests;

// The TSC frequency arithmetic: CPUID leaf 0x15/0x16 per Linux's
// native_calibrate_tsc, and the PM timer reference used when the PIT is gated.
#[path = "../../../src/arch/x86_64/time/tsc/calibration/math.rs"]
pub mod tsc_math;

#[cfg(test)]
mod tsc_math_tests;

// The order every CPU picks its TSC rate in: the boot CPU's settled rate,
// then CPUID, and the PIT poll only when neither exists.
#[path = "../../../src/arch/x86_64/cpu/frequency_pick.rs"]
mod frequency_pick;

/// Keeps the included picker referenced in the library build.
pub fn pick_tsc(settled: u64, enumerated: Option<u64>, measured: u64) -> u64 {
    frequency_pick::pick_tsc_hz(settled, || enumerated, || measured)
}

#[cfg(test)]
mod frequency_pick_tests;
