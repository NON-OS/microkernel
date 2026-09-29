// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/heap/types/stats.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/heap/types/stats.rs"]
pub mod stats;

pub fn heapstats_free_memory(this: stats::HeapStats) -> usize {
    this.free_memory()
}

