// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/security/image_ceiling/rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/security/image_ceiling/rule.rs"]
pub mod rule;

pub fn over(requested: u64, ceiling: u64) -> u64 {
    rule::over(requested, ceiling)
}

