// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/fs/cache/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/fs/cache/types.rs"]
pub mod types;

pub fn cachestatistics_new() -> types::CacheStatistics {
    types::CacheStatistics::new()
}

pub fn cachestatistics_reset(this: types::CacheStatistics) {
    this.reset()
}

