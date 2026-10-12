// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the UHCI host. The shipped modules are mounted
//! at the paths the driver names them by (`crate::facts`, ...); only the
//! capsule's platform, setup and loop, which call the kernel, are left out.

#[path = "../../capsule_driver_uhci/src/devices/mod.rs"]
pub mod devices;
#[path = "../../capsule_driver_uhci/src/env/mod.rs"]
pub mod env;
#[path = "../../capsule_driver_uhci/src/facts/mod.rs"]
pub mod facts;
#[path = "../../capsule_driver_uhci/src/hc/mod.rs"]
pub mod hc;
#[path = "../../capsule_driver_uhci/src/sched/mod.rs"]
pub mod sched;
#[path = "../../capsule_driver_uhci/src/server/mod.rs"]
pub mod server;
#[path = "../../capsule_driver_uhci/src/wire/mod.rs"]
pub mod wire;
#[path = "../../capsule_driver_uhci/src/xfer/mod.rs"]
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
mod start_tests;
#[cfg(test)]
mod td_tests;
#[cfg(test)]
mod wire_tests;
