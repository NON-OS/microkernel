// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/bus/pci/assign/command_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/bus/pci/assign/command_rule.rs"]
pub mod command_rule;

pub fn while_sizing(found: u16) -> u16 {
    command_rule::while_sizing(found)
}

pub fn after_assign(found: u16) -> u16 {
    command_rule::after_assign(found)
}

