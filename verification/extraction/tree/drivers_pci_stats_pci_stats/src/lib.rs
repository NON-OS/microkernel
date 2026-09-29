// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/drivers/pci/stats/pci_stats.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod drivers;

pub fn pcistats_new() -> crate::drivers::pci::stats::pci_stats::PciStats {
    crate::drivers::pci::stats::pci_stats::PciStats::new()
}

pub fn pcistats_snapshot() -> crate::drivers::pci::stats::pci_stats::PciStats {
    crate::drivers::pci::stats::pci_stats::PciStats::snapshot()
}

pub fn pcistats_average_enumeration_time_us(this: crate::drivers::pci::stats::pci_stats::PciStats) -> u64 {
    this.average_enumeration_time_us()
}

