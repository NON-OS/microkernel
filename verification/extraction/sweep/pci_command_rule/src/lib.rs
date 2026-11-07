// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/pci/command_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/pci/command_rule.rs"]
pub mod command_rule;

pub fn at_claim(current: u16, port_only: bool) -> u16 {
    command_rule::at_claim(current, port_only)
}

