// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/mmio/stats/query.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn mmiostats_total_regions(this: crate::memory::mmio::stats::types::MmioStats) -> usize {
    this.total_regions()
}

pub fn mmiostats_total_mapped_size(this: crate::memory::mmio::stats::types::MmioStats) -> u64 {
    this.total_mapped_size()
}

pub fn mmiostats_read_operations(this: crate::memory::mmio::stats::types::MmioStats) -> u64 {
    this.read_operations()
}

pub fn mmiostats_write_operations(this: crate::memory::mmio::stats::types::MmioStats) -> u64 {
    this.write_operations()
}

