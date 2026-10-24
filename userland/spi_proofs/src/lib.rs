// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the LPSS SPI host. The shipped modules are
//! mounted at the paths the driver names (`crate::facts`, `crate::core`).

#[path = "../../capsule_driver_spi/src/core/mod.rs"]
pub mod core;
#[path = "../../capsule_driver_spi/src/facts/mod.rs"]
pub mod facts;
#[path = "../../capsule_driver_spi/src/setup/prove.rs"]
pub mod prove;

/// An SSP core and the device behind it, as the fact sheet describes them.
pub mod model;
mod model_status;

#[cfg(test)]
mod exchange_tests;
#[cfg(test)]
mod prove_tests;
#[cfg(test)]
mod settings_tests;
