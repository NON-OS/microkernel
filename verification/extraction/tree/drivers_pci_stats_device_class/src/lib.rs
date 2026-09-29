// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/drivers/pci/stats/device_class.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod drivers;

pub fn deviceclassstats_new() -> crate::drivers::pci::stats::device_class::DeviceClassStats {
    crate::drivers::pci::stats::device_class::DeviceClassStats::new()
}

pub fn deviceclassstats_total(this: crate::drivers::pci::stats::device_class::DeviceClassStats) -> u64 {
    this.total()
}

