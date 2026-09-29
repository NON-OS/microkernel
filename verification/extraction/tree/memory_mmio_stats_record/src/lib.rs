// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/mmio/stats/record.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn mmiostats_next_id(this: crate::memory::mmio::stats::types::MmioStats) -> u64 {
    this.next_id()
}

pub fn mmiostats_record_mapping(this: crate::memory::mmio::stats::types::MmioStats, size: usize) {
    this.record_mapping(size)
}

pub fn mmiostats_record_unmapping(this: crate::memory::mmio::stats::types::MmioStats, size: usize) {
    this.record_unmapping(size)
}

pub fn mmiostats_record_read(this: crate::memory::mmio::stats::types::MmioStats) {
    this.record_read()
}

pub fn mmiostats_record_write(this: crate::memory::mmio::stats::types::MmioStats) {
    this.record_write()
}

