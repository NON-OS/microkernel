// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for capsule_driver_rtl8153: the shipped rtl module,
//! mounted by path, binds a modelled chip (chip/) on the scripted bus and
//! moves frames through it.

extern crate alloc;

#[path = "../../capsule_driver_rtl8153/src/rtl/mod.rs"]
pub mod rtl;

#[cfg(test)]
mod adapter;
#[cfg(test)]
mod chip;
#[cfg(test)]
mod glue;
#[cfg(test)]
mod helpers;

#[cfg(test)]
mod bind_tests;
#[cfg(test)]
mod bound_tests;
#[cfg(test)]
mod budget_tests;
#[cfg(test)]
mod config_tests;
#[cfg(test)]
mod count_tests;
#[cfg(test)]
mod drain_tests;
#[cfg(test)]
mod gate_tests;
#[cfg(test)]
mod link_tests;
#[cfg(test)]
mod ocp_tests;
#[cfg(test)]
mod order_tests;
#[cfg(test)]
mod phase_tests;
#[cfg(test)]
mod phy_tests;
#[cfg(test)]
mod privacy_tests;
#[cfg(test)]
mod receive_tests;
#[cfg(test)]
mod retry_tests;
#[cfg(test)]
mod rx_driver_fuzz_tests;
#[cfg(test)]
mod rx_fuzz_tests;
#[cfg(test)]
mod rx_tests;
#[cfg(test)]
mod speed_tests;
#[cfg(test)]
mod stall_tests;
#[cfg(test)]
mod state_tests;
#[cfg(test)]
mod table_tests;
#[cfg(test)]
mod tx_tests;
#[cfg(test)]
mod version_tests;
