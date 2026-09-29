// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/paging/stats/query.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn pagingstatistics_total_mappings(this: crate::memory::paging::stats::state::PagingStatistics) -> usize {
    this.total_mappings()
}

pub fn pagingstatistics_page_faults(this: crate::memory::paging::stats::state::PagingStatistics) -> u64 {
    this.page_faults()
}

pub fn pagingstatistics_tlb_flushes(this: crate::memory::paging::stats::state::PagingStatistics) -> u64 {
    this.tlb_flushes()
}

