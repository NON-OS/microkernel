// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/security/crypto/key_management/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/security/crypto/key_management/types.rs"]
pub mod types;

pub fn keytype_key_length(this: types::KeyType) -> usize {
    this.key_length()
}

pub fn keyusage_signing() -> types::KeyUsage {
    types::KeyUsage::signing()
}

pub fn keyusage_verification() -> types::KeyUsage {
    types::KeyUsage::verification()
}

pub fn keyusage_encryption() -> types::KeyUsage {
    types::KeyUsage::encryption()
}

pub fn keyusage_key_exchange() -> types::KeyUsage {
    types::KeyUsage::key_exchange()
}

pub fn keyusage_master() -> types::KeyUsage {
    types::KeyUsage::master()
}

