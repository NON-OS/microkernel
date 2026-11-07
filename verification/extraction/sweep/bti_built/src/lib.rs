// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/security/bti/built.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/security/bti/built.rs"]
pub mod built;

pub fn built_with_bti() -> bool {
    built::built_with_bti()
}

pub fn built_with_pac_ret() -> bool {
    built::built_with_pac_ret()
}

