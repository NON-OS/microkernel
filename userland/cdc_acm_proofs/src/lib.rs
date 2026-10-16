// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the CDC ACM driver. The shipped modules are
//! mounted at the paths the driver names (`crate::acm`, `crate::wire`).

extern crate alloc;

#[path = "../../capsule_driver_cdc_acm/src/acm/mod.rs"]
pub mod acm;
#[path = "../../capsule_driver_cdc_acm/src/frame.rs"]
pub mod frame;
#[path = "../../capsule_driver_cdc_acm/src/wire.rs"]
pub mod wire;

/// A modem's configuration, built a descriptor at a time.
pub mod modem;

#[cfg(test)]
mod function_tests;
#[cfg(test)]
mod hostile_tests;
#[cfg(test)]
mod line_tests;
#[cfg(test)]
mod notice_tests;
#[cfg(test)]
mod owner_tests;
