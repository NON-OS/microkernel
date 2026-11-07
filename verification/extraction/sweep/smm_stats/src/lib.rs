// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/smm/stats.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/smm/stats.rs"]
pub mod stats;

pub fn smmstats_new() -> stats::SmmStats {
    stats::SmmStats::new()
}

pub fn smmstats_reset(this: stats::SmmStats) {
    this.reset()
}

