// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/crypto/asymmetric/ed25519/point/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod crypto;

pub fn gep3_identity() -> crate::crypto::asymmetric::ed25519::point::types::GeP3 {
    crate::crypto::asymmetric::ed25519::point::types::GeP3::identity()
}

