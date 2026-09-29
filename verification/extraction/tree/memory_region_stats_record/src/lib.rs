// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/region/stats/record.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn regionstatistics_record_allocation(this: crate::memory::region::stats::state::RegionStatistics, size: u64) {
    this.record_allocation(size)
}

pub fn regionstatistics_record_deallocation(this: crate::memory::region::stats::state::RegionStatistics, size: u64) {
    this.record_deallocation(size)
}

pub fn regionstatistics_record_merge(this: crate::memory::region::stats::state::RegionStatistics) {
    this.record_merge()
}

pub fn regionstatistics_record_split(this: crate::memory::region::stats::state::RegionStatistics) {
    this.record_split()
}

pub fn regionstatistics_add_region(this: crate::memory::region::stats::state::RegionStatistics, size: u64, is_free: bool) {
    this.add_region(size, is_free)
}

pub fn regionstatistics_remove_region(this: crate::memory::region::stats::state::RegionStatistics, size: u64, is_free: bool) {
    this.remove_region(size, is_free)
}

pub fn regionstatistics_record_fragmentation(this: crate::memory::region::stats::state::RegionStatistics) {
    this.record_fragmentation()
}

pub fn regionstatistics_reduce_fragmentation(this: crate::memory::region::stats::state::RegionStatistics) {
    this.reduce_fragmentation()
}

