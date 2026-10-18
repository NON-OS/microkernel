// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the CDC-ECM driver.

extern crate alloc;

#[path = "../../capsule_driver_cdc_ecm/src/ecm/mod.rs"]
pub mod ecm;

#[cfg(test)]
mod address_tests;
#[cfg(test)]
mod bind_tests;
#[cfg(test)]
mod config_fuzz_tests;
#[cfg(test)]
mod device;
#[cfg(test)]
mod frame_tests;
#[cfg(test)]
mod function_tests;
#[cfg(test)]
mod hold_tests;
#[cfg(test)]
mod hostile_tests;
#[cfg(test)]
mod notice_tests;
#[cfg(test)]
mod qemu;
#[cfg(test)]
mod receive_tests;
