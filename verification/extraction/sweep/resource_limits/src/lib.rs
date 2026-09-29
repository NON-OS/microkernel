// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/capabilities/resource/limits.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/capabilities/resource/limits.rs"]
pub mod limits;

pub fn has_at_least(remaining: u64, amount: u64) -> bool {
    limits::has_at_least(remaining, amount)
}

