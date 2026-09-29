// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/fs/utils/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/fs/utils/types.rs"]
pub mod types;

pub fn scanconfig_new() -> types::ScanConfig {
    types::ScanConfig::new()
}

pub fn scanconfig_with_max_depth(this: types::ScanConfig, depth: usize) -> types::ScanConfig {
    this.with_max_depth(depth)
}

pub fn scanconfig_hidden_only(this: types::ScanConfig) -> types::ScanConfig {
    this.hidden_only()
}


pub fn scanconfig_admits_hidden(this: types::ScanConfig, hidden: bool) -> bool {
    this.admits_hidden(hidden)
}
