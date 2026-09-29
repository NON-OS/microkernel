// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/capabilities/chain/constants.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/capabilities/chain/constants.rs"]
pub mod constants;

pub fn max_chain_depth() -> usize {
    constants::max_chain_depth()
}

