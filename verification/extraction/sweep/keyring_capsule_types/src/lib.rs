// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/security/keyring_capsule/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/security/keyring_capsule/types.rs"]
pub mod types;

pub fn keytype_to_u8(this: types::KeyType) -> u8 {
    this.to_u8()
}

