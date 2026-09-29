// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/capabilities/audit/constants.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/capabilities/audit/constants.rs"]
pub mod constants;

pub fn capacity() -> usize {
    constants::capacity()
}

