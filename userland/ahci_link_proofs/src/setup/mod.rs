// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's open path, claim to port scan, and a port's hand-back, run
//! against a whole model HBA over the stand-in nonos_libc. The broker handles
//! and console lines it takes are mounted here too, and named at the crate
//! root as in the driver.

#[path = "../../../capsule_driver_ahci/src/setup/claim.rs"]
mod claim;
mod dma32_tests;
#[path = "../../../capsule_driver_ahci/src/setup/driver.rs"]
mod driver;
mod driver_drop_tests;
mod give_back_tests;
#[path = "../../../capsule_driver_ahci/src/handles/mod.rs"]
pub mod handles;
#[path = "../../../capsule_driver_ahci/src/setup/irq.rs"]
mod irq;
#[path = "../../../capsule_driver_ahci/src/log.rs"]
pub mod log;
#[path = "../../../capsule_driver_ahci/src/setup/master.rs"]
mod master;
#[path = "../../../capsule_driver_ahci/src/setup/mmio.rs"]
mod mmio;
#[path = "../../../capsule_driver_ahci/src/setup/open.rs"]
mod open;
mod open_tests;
#[path = "../../../capsule_driver_ahci/src/setup/pci.rs"]
mod pci;
#[cfg(test)]
mod port_map_tests;
mod refused_write_tests;
#[path = "../../../capsule_driver_ahci/src/setup/remap.rs"]
mod remap;
#[path = "../../../capsule_driver_ahci/src/setup/say_hba.rs"]
mod say_hba;
#[path = "../../../capsule_driver_ahci/src/setup/say_port_map.rs"]
mod say_port_map;
#[path = "../../../capsule_driver_ahci/src/setup/serve.rs"]
mod serve;
mod serve_tests;
mod spared_tests;
#[path = "../../../capsule_driver_ahci/src/setup/walked.rs"]
mod walked;

pub use driver::Driver;
