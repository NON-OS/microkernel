// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the EHCI host. The shipped modules are mounted
//! at the paths the driver names them by (`crate::facts`, ...); only the
//! capsule's platform, setup and loop, which call the kernel, are left out.

#[path = "../../capsule_driver_ehci/src/devices/mod.rs"]
pub mod devices;
#[path = "../../capsule_driver_ehci/src/env/mod.rs"]
pub mod env;
#[path = "../../capsule_driver_ehci/src/facts/mod.rs"]
pub mod facts;
#[path = "../../capsule_driver_ehci/src/hc/mod.rs"]
pub mod hc;
#[path = "../../capsule_driver_ehci/src/sched/mod.rs"]
pub mod sched;
#[path = "../../capsule_driver_ehci/src/server/mod.rs"]
pub mod server;
#[path = "../../capsule_driver_ehci/src/wire/mod.rs"]
pub mod wire;
#[path = "../../capsule_driver_ehci/src/xfer/mod.rs"]
pub mod xfer;

/// A model controller and the devices on its ports.
pub mod model;

#[cfg(test)]
mod bulk_tests;
#[cfg(test)]
mod control_tests;
#[cfg(test)]
mod ep0_fault_tests;
#[cfg(test)]
mod interrupt_tests;
#[cfg(test)]
mod poll_share_tests;
#[cfg(test)]
mod poll_tests;
#[cfg(test)]
mod port_tests;
#[cfg(test)]
mod recover_tests;
#[cfg(test)]
mod start_tests;
#[cfg(test)]
mod wire_tests;
