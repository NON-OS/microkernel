// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/drivers/security/pci.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod drivers;

pub fn is_config_write_allowed(offset: u8) -> bool {
    crate::drivers::security::pci::is_config_write_allowed(offset)
}

pub fn is_sensitive_config_read(offset: u8) -> bool {
    crate::drivers::security::pci::is_sensitive_config_read(offset)
}

pub fn build_config_address(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    crate::drivers::security::pci::build_config_address(bus, device, function, offset)
}

