// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/drivers/pci/stats/getters.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod drivers;

pub fn get_total_devices() -> u64 {
    crate::drivers::pci::stats::getters::get_total_devices()
}

pub fn get_pcie_devices() -> u64 {
    crate::drivers::pci::stats::getters::get_pcie_devices()
}

pub fn get_msi_capable_devices() -> u64 {
    crate::drivers::pci::stats::getters::get_msi_capable_devices()
}

pub fn get_msix_capable_devices() -> u64 {
    crate::drivers::pci::stats::getters::get_msix_capable_devices()
}

