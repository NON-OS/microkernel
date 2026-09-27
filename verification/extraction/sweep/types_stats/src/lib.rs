// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/region/types/stats.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/region/types/stats.rs"]
pub mod stats;

pub fn regionstats_new() -> stats::RegionStats {
    stats::RegionStats::new()
}

pub fn regionstats_total_memory(this: stats::RegionStats) -> u64 {
    this.total_memory()
}

