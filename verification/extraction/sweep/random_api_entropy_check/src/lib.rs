// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/crypto/random_api/entropy_check.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/crypto/random_api/entropy_check.rs"]
pub mod entropy_check;

pub fn required_entropy_bytes(bits: usize) -> usize {
    entropy_check::required_entropy_bytes(bits)
}

