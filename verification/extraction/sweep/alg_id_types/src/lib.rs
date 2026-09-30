// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/crypto/asymmetric/alg_id/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/crypto/asymmetric/alg_id/types.rs"]
pub mod types;

pub fn algid_as_u8(this: types::AlgId) -> u8 {
    this.as_u8()
}

