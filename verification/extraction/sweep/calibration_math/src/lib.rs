// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/time/tsc/calibration/math.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/time/tsc/calibration/math.rs"]
pub mod math;

pub fn pm_timer_delta(start: u32, end: u32, is_32bit: bool) -> u32 {
    math::pm_timer_delta(start, end, is_32bit)
}

pub fn reference_timeout_ticks(window_ms: u64, max_tsc_hz: u64) -> u64 {
    math::reference_timeout_ticks(window_ms, max_tsc_hz)
}

