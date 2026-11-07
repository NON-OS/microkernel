// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/paging/stats/state.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/paging/stats/state.rs"]
pub mod state;

pub fn pagingstatistics_new() -> state::PagingStatistics {
    state::PagingStatistics::new()
}

