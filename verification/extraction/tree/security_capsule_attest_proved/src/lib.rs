// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/security/capsule_attest/proved.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod security;

pub fn proved_is_vendor(this: crate::security::capsule_attest::proved::Proved) -> bool {
    this.is_vendor()
}

