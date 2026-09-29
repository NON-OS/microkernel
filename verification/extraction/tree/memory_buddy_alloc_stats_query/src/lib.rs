// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/buddy_alloc/stats/query.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn allocationstatistics_total_allocated(this: crate::memory::buddy_alloc::stats::types::AllocationStatistics) -> u64 {
    this.total_allocated()
}

pub fn allocationstatistics_peak_allocated(this: crate::memory::buddy_alloc::stats::types::AllocationStatistics) -> u64 {
    this.peak_allocated()
}

pub fn allocationstatistics_allocation_count(this: crate::memory::buddy_alloc::stats::types::AllocationStatistics) -> usize {
    this.allocation_count()
}

pub fn allocationstatistics_free_count(this: crate::memory::buddy_alloc::stats::types::AllocationStatistics) -> usize {
    this.free_count()
}

