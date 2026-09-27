// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/security/capsule_manifest/verify/caps_bits.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/security/capsule_manifest/verify/caps_bits.rs"]
pub mod caps_bits;

pub fn within_ceiling(required: u64, optional: u64, ceiling: u64) -> bool {
    caps_bits::within_ceiling(required, optional, ceiling)
}

pub fn grant_within_manifest(required: u64, optional: u64, granted: u64) -> bool {
    caps_bits::grant_within_manifest(required, optional, granted)
}

pub fn install_caps(required: u64, optional: u64, granted: u64) -> u64 {
    caps_bits::install_caps(required, optional, granted)
}

