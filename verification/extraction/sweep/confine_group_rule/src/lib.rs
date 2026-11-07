// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/confine/group_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/confine/group_rule.rs"]
pub mod group_rule;

pub fn may_share(built: bool, hardened: bool, vendor: bool, reaches: bool) -> bool {
    group_rule::may_share(built, hardened, vendor, reaches)
}

