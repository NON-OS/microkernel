// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/capabilities/multisig/constants.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/capabilities/multisig/constants.rs"]
pub mod constants;

pub fn max_signers() -> usize {
    constants::max_signers()
}

pub fn max_threshold() -> usize {
    constants::max_threshold()
}

