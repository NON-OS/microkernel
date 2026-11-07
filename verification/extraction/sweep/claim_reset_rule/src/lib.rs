// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/claim/reset_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/claim/reset_rule.rs"]
pub mod reset_rule;

pub fn pcie_flr(devcap: u32) -> bool {
    reset_rule::pcie_flr(devcap)
}

pub fn af_flr(afcap: u8) -> bool {
    reset_rule::af_flr(afcap)
}

