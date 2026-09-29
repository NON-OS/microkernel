// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/crypto/symmetric/aes/core.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod crypto;

pub fn gf_mul(a: u8, b: u8) -> u8 {
    crate::crypto::symmetric::aes::core::gf_mul(a, b)
}

