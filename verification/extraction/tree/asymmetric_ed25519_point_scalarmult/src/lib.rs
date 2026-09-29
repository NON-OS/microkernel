// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/crypto/asymmetric/ed25519/point/scalarmult.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod crypto;

pub fn ct_byte_mask(bit: u8) -> u8 {
    crate::crypto::asymmetric::ed25519::point::scalarmult::ct_byte_mask(bit)
}

