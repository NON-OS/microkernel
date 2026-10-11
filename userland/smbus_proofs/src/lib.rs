// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the i801 SMBus host. The shipped modules are
//! mounted at the paths the driver names them by (`crate::regs`, ...).

#[path = "../../capsule_driver_smbus/src/errno.rs"]
pub mod errno;
#[path = "../../capsule_driver_smbus/src/hostc.rs"]
pub mod hostc;
#[path = "../../capsule_driver_smbus/src/ports.rs"]
pub mod ports;
#[path = "../../capsule_driver_smbus/src/regs.rs"]
pub mod regs;
/// The answers; the loop around them calls the kernel.
pub mod server;
#[path = "../../capsule_driver_smbus/src/wire/mod.rs"]
pub mod wire;
#[path = "../../capsule_driver_smbus/src/xfer/mod.rs"]
pub mod xfer;

/// A model of the host and the devices on its bus.
pub mod model;

#[cfg(test)]
mod answer_tests;
#[cfg(test)]
mod block_tests;
#[cfg(test)]
mod error_tests;
#[cfg(test)]
mod hostc_tests;
#[cfg(test)]
mod parse_tests;
#[cfg(test)]
mod transfer_tests;
