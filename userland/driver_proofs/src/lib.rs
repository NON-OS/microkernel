// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for driver request parsers over untrusted input.

#[path = "../../capsule_driver_ahci/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_ahci/src/constants/mod.rs"]
pub mod constants;
pub mod server;
/// The bounded bring-up schedule every driver shares.
#[path = "../../libc/src/bringup/policy.rs"]
pub mod bringup_policy;

#[cfg(test)]
mod ahci_tests;
#[cfg(test)]
mod bringup_decide_tests;
#[cfg(test)]
mod bringup_tests;
#[cfg(test)]
mod recv_turn_tests;

#[cfg(kani)]
mod kani_proofs;
