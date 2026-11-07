// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/fs/procfs/loadavg_math.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/fs/procfs/loadavg_math.rs"]
pub mod loadavg_math;

pub fn weight(window_s: u64) -> u64 {
    loadavg_math::weight(window_s)
}

pub fn fold(avg: u64, w: u64, now: u64) -> u64 {
    loadavg_math::fold(avg, w, now)
}

