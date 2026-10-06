// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the CDC-NCM driver.

extern crate alloc;

#[path = "../../capsule_driver_cdc_ncm/src/ncm/mod.rs"]
pub mod ncm;

#[cfg(test)]
mod align_tests;
#[cfg(test)]
mod bind_tests;
#[cfg(test)]
mod bind_variant_tests;
#[cfg(test)]
mod device;
#[cfg(test)]
mod function_tests;
#[cfg(test)]
mod hostile_chain_tests;
#[cfg(test)]
mod hostile_tests;
#[cfg(test)]
mod limits_tests;
#[cfg(test)]
mod ntb;
#[cfg(test)]
mod rx_tests;
#[cfg(test)]
mod spec;
#[cfg(test)]
mod tx_tests;
