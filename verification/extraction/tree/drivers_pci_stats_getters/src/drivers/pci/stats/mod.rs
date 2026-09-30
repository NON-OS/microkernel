// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/drivers/pci/stats/atomics.rs"]
pub mod atomics;

#[path = "../../../../../../../../src/drivers/pci/stats/getters.rs"]
pub mod getters;

pub use getters::{get_msi_capable_devices, get_msix_capable_devices, get_pcie_devices, get_total_devices};
