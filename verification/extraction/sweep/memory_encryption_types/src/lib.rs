// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/security/hardening/memory_encryption/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/security/hardening/memory_encryption/types.rs"]
pub mod types;

pub fn encryptedregion_new(start: u64, size: usize, key_id: u64) -> types::EncryptedRegion {
    types::EncryptedRegion::new(start, size, key_id)
}

