// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/drivers/pci/types/address.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod drivers;

pub fn pciaddress_new(bus: u8, device: u8, function: u8) -> crate::drivers::pci::types::address::PciAddress {
    crate::drivers::pci::types::address::PciAddress::new(bus, device, function)
}

pub fn pciaddress_from_bdf(bdf: u16) -> crate::drivers::pci::types::address::PciAddress {
    crate::drivers::pci::types::address::PciAddress::from_bdf(bdf)
}

pub fn pciaddress_to_bdf(this: crate::drivers::pci::types::address::PciAddress) -> u16 {
    this.to_bdf()
}

pub fn pciaddress_config_address(this: crate::drivers::pci::types::address::PciAddress, offset: u8) -> u32 {
    this.config_address(offset)
}

