// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the CDC-ECM driver.

extern crate alloc;

#[path = "../../capsule_driver_cdc_ecm/src/ecm/mod.rs"]
pub mod ecm;

#[cfg(test)]
mod bind_tests;
#[cfg(test)]
mod frame_tests;
#[cfg(test)]
mod qemu;
