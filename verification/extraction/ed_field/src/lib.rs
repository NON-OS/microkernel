// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/crypto/asymmetric/ed25519/field/`.
//!
//! This crate existed to compare ed25519's own `ct_eq_32` against the shared one
//! in `crypto/util/constant_time`, which were the same function except that only
//! the shared one carried a compiler fence. The duplicate is gone: ed25519
//! delegates now, and the theorem that the two loops were one function is what
//! made that substitution safe.
//!
//! It still mirrors both modules, because the property worth holding has changed
//! rather than disappeared. What matters now is that ed25519 really does reach
//! the fenced implementation and has not quietly grown a second one again.
//!
//! The forwarding functions exist because a Charon entry point cannot name a
//! `pub(crate)` item from outside the crate that declares it. Mirroring the
//! module makes this crate that crate. They add no logic.

#[path = "../../../../src/crypto/asymmetric/ed25519/field/mod.rs"]
pub mod field;

/*
 * The ed25519 source reaches the shared comparison by its absolute path, so the
 * mirror has to carry that path too. These are the real kernel modules, included
 * the same way everything else here is.
 */
pub mod crypto;

pub fn ed25519_ct_eq_32(a: [u8; 32], b: [u8; 32]) -> bool {
    field::ct_eq_32(&a, &b)
}

pub fn shared_ct_eq_32(a: [u8; 32], b: [u8; 32]) -> bool {
    crypto::util::constant_time::ct_eq_32(&a, &b)
}
