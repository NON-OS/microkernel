// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/boot_memory/types/region_stats.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/boot_memory/types/region_stats.rs"]
pub mod region_stats;

pub fn regionstats_free_memory(this: region_stats::RegionStats) -> u64 {
    this.free_memory()
}

