// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/gic/state.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/gic/state.rs"]
pub mod state;

pub fn set_bases(dist: u64, redist: u64) {
    state::set_bases(dist, redist)
}

pub fn dist_base() -> u64 {
    state::dist_base()
}

pub fn redist_base() -> u64 {
    state::redist_base()
}

