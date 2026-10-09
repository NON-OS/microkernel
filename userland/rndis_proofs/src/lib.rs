// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the RNDIS driver.

extern crate alloc;

#[path = "../../capsule_driver_rndis/src/rndis/mod.rs"]
pub mod rndis;

#[cfg(test)]
mod address_tests;
#[cfg(test)]
mod batch_tests;
#[cfg(test)]
mod bind_tests;
#[cfg(test)]
mod control_tests;
#[cfg(test)]
mod device;
#[cfg(test)]
mod function_tests;
#[cfg(test)]
mod fuzz_tests;
#[cfg(test)]
mod hostile_tests;
#[cfg(test)]
mod qemu;
#[cfg(test)]
mod transfer_tests;
