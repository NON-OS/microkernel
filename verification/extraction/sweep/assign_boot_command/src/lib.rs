// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/bus/pci/assign/boot_command.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/bus/pci/assign/boot_command.rs"]
pub mod boot_command;

pub fn boot_command(found: u16, class: u8) -> u16 {
    boot_command::boot_command(found, class)
}

