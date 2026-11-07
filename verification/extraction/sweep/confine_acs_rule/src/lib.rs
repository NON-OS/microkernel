// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/confine/acs_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/confine/acs_rule.rs"]
pub mod acs_rule;

pub fn enabling(cap: u16, ctrl: u16, needed: u16) -> u16 {
    acs_rule::enabling(cap, ctrl, needed)
}

