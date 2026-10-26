// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the FIDO2 key driver. The shipped CTAPHID core
//! is mounted at the path the driver names (`crate::ctaphid`).

extern crate alloc;

#[path = "../../capsule_driver_fido/src/ctaphid/mod.rs"]
pub mod ctaphid;

/// A key's side of the wire: its configuration and its reports, written
/// here from the specification as recalled, not from the driver.
pub mod keyside;

#[cfg(test)]
mod frame_sweep;
#[cfg(test)]
mod frame_sweep_tests;
#[cfg(test)]
mod hostile_tests;
#[cfg(test)]
mod job_tests;
#[cfg(test)]
mod key_tests;
#[cfg(test)]
mod talk_tests;
