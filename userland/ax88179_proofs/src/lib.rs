// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the AX88179 driver.

extern crate alloc;

#[path = "../../capsule_driver_ax88179/src/ax/mod.rs"]
pub mod ax;

#[cfg(test)]
mod aggregate;
#[cfg(test)]
mod bind_tests;
#[cfg(test)]
mod chip;
#[cfg(test)]
mod fuzz_rx_tests;
#[cfg(test)]
mod harness;
#[cfg(test)]
mod hostile_rx_tests;
#[cfg(test)]
mod ids_tests;
#[cfg(test)]
mod init_tests;
#[cfg(test)]
mod link_tests;
#[cfg(test)]
mod model;
#[cfg(test)]
mod rx_model;
#[cfg(test)]
mod rx_tests;
#[cfg(test)]
mod seen;
#[cfg(test)]
mod status_tests;
#[cfg(test)]
mod tx_tests;
