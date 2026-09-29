// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/drivers/pci/stats/atomics.rs"]
pub mod atomics;

#[path = "../../../../../../../../src/drivers/pci/stats/pci_stats.rs"]
pub mod pci_stats;

pub use pci_stats::PciStats;
