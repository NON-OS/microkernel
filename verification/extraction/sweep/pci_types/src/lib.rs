// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/devices/pci/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/devices/pci/types.rs"]
pub mod types;

pub fn pcidevice_bdf(this: types::PciDevice) -> u16 {
    this.bdf()
}

pub fn pcidevice_is_bridge(this: types::PciDevice) -> bool {
    this.is_bridge()
}

pub fn pcidevice_is_storage(this: types::PciDevice) -> bool {
    this.is_storage()
}

pub fn pcidevice_is_network(this: types::PciDevice) -> bool {
    this.is_network()
}

pub fn pcidevice_is_display(this: types::PciDevice) -> bool {
    this.is_display()
}

