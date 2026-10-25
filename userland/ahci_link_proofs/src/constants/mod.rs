// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../capsule_driver_ahci/src/constants/ata.rs"]
pub mod ata;
#[path = "../../../capsule_driver_ahci/src/constants/budget.rs"]
pub mod budget;
#[path = "../../../capsule_driver_ahci/src/constants/identify.rs"]
pub mod identify;
#[path = "../../../capsule_driver_ahci/src/constants/pci.rs"]
pub mod pci;
#[path = "../../../capsule_driver_ahci/src/constants/port.rs"]
pub mod port;
#[path = "../../../capsule_driver_ahci/src/constants/power_off.rs"]
pub mod power_off;
#[path = "../../../capsule_driver_ahci/src/constants/regs/mod.rs"]
pub mod regs;
#[path = "../../../capsule_driver_ahci/src/constants/timing.rs"]
pub mod timing;

pub use pci::AHCI_ABAR_BAR;
pub use port::{MAX_PORTS, PORT_KIND_NONE, PORT_KIND_SATA};
pub use regs::{HBA_CAP, HBA_CAP2, HBA_GHC, HBA_PI, HBA_VS, SIG_ATAPI, SIG_PM, SIG_SATA, SIG_SEMB};
