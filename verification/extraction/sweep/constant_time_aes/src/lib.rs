// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/security/crypto/constant_time/aes.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/security/crypto/constant_time/aes.rs"]
pub mod aes;

pub fn sbox_ct(input: u8) -> u8 {
    aes::sbox_ct(input)
}

