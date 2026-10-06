// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the AX88179 / AX88178A driver.

extern crate alloc;

#[path = "../../capsule_driver_ax88179/src/ax/mod.rs"]
pub mod ax;

#[cfg(test)]
mod bind_tests;
#[cfg(test)]
mod bound;
#[cfg(test)]
mod calls;
#[cfg(test)]
mod chip;
#[cfg(test)]
mod failure_tests;
#[cfg(test)]
mod layout;
#[cfg(test)]
mod link_tests;
#[cfg(test)]
mod match_tests;
#[cfg(test)]
mod medium_tests;
#[cfg(test)]
mod queue_tests;
#[cfg(test)]
mod rx_tests;
#[cfg(test)]
mod tx_tests;
