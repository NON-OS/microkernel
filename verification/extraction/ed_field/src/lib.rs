// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/crypto/asymmetric/ed25519/field/`.
//!
//! The forwarding function exists because a Charon entry point cannot name a
//! `pub(crate)` item from outside the crate that declares it. Mirroring the
//! module makes this crate that crate. It adds no logic.

#[path = "../../../../src/crypto/asymmetric/ed25519/field/mod.rs"]
pub mod field;

pub fn ed25519_ct_eq_32(a: [u8; 32], b: [u8; 32]) -> bool {
    field::ct_eq_32(&a, &b)
}
