// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/security/pac/key.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/security/pac/key.rs"]
pub mod key;

pub fn packey_new(lo: u64, hi: u64) -> key::PacKey {
    key::PacKey::new(lo, hi)
}

