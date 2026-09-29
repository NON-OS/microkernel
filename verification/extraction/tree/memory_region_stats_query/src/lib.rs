// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/region/stats/query.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn regionstatistics_total_regions(this: crate::memory::region::stats::state::RegionStatistics) -> usize {
    this.total_regions()
}

pub fn regionstatistics_allocated_bytes(this: crate::memory::region::stats::state::RegionStatistics) -> u64 {
    this.allocated_bytes()
}

pub fn regionstatistics_free_bytes(this: crate::memory::region::stats::state::RegionStatistics) -> u64 {
    this.free_bytes()
}

pub fn regionstatistics_allocation_count(this: crate::memory::region::stats::state::RegionStatistics) -> u64 {
    this.allocation_count()
}

pub fn regionstatistics_deallocation_count(this: crate::memory::region::stats::state::RegionStatistics) -> u64 {
    this.deallocation_count()
}

pub fn regionstatistics_merge_count(this: crate::memory::region::stats::state::RegionStatistics) -> u64 {
    this.merge_count()
}

pub fn regionstatistics_split_count(this: crate::memory::region::stats::state::RegionStatistics) -> u64 {
    this.split_count()
}

pub fn regionstatistics_fragmentation_count(this: crate::memory::region::stats::state::RegionStatistics) -> usize {
    this.fragmentation_count()
}

