// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/buddy_alloc/stats/record.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn allocationstatistics_record_allocation(this: crate::memory::buddy_alloc::stats::types::AllocationStatistics, size: u64) {
    this.record_allocation(size)
}

pub fn allocationstatistics_record_deallocation(this: crate::memory::buddy_alloc::stats::types::AllocationStatistics, size: u64) {
    this.record_deallocation(size)
}

