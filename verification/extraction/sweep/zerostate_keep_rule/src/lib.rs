// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/security/zerostate/keep_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/security/zerostate/keep_rule.rs"]
pub mod keep_rule;

pub fn after(armed: bool, granted: bool) -> bool {
    keep_rule::after(armed, granted)
}

