// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/buddy_alloc/types/stats.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/buddy_alloc/types/stats.rs"]
pub mod stats;

pub fn allocstats_new() -> stats::AllocStats {
    stats::AllocStats::new()
}

pub fn allocstats_free_memory(this: stats::AllocStats, total: u64) -> u64 {
    this.free_memory(total)
}

