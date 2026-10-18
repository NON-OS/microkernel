// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for driver request parsers over untrusted input.

#[path = "../../capsule_driver_ahci/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_ahci/src/constants/mod.rs"]
pub mod constants;
pub mod server;
/// The bounded bring-up every driver shares: its exit codes, its schedule,
/// and its serve loop's turn.
#[path = "../../libc/src/bringup/deadline.rs"]
pub mod bringup_deadline;
#[path = "../../libc/src/bringup/policy.rs"]
pub mod bringup_policy;
#[path = "../../libc/src/bringup/recv_turn.rs"]
pub mod bringup_recv;
#[path = "../../libc/src/bringup/schedule.rs"]
pub mod bringup_schedule;

#[cfg(test)]
mod ahci_tests;
#[cfg(test)]
mod bringup_decide_tests;
#[cfg(test)]
mod bringup_tests;
#[cfg(test)]
mod deadline_tests;
#[cfg(test)]
mod recv_turn_tests;

#[cfg(kani)]
mod kani_proofs;
