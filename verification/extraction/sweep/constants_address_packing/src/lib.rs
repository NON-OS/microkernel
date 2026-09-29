// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/drivers/pci/constants/address_packing.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/drivers/pci/constants/address_packing.rs"]
pub mod address_packing;

pub fn pci_config_address(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    address_packing::pci_config_address(bus, device, function, offset)
}

