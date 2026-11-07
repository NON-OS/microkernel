// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/multiboot/stats.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/multiboot/stats.rs"]
pub mod stats;

pub fn multibootstats_new() -> stats::MultibootStats {
    stats::MultibootStats::new()
}

pub fn multibootstats_reset(this: stats::MultibootStats) {
    this.reset()
}

