// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for nonos_uas, against a USB 2 and a SuperSpeed
//! UAS disk (disks.rs) and a SCSI command of the proofs' own.

/// The disks' configurations, built a descriptor at a time.
pub mod disks;

#[cfg(test)]
mod dropped_tests;
#[cfg(test)]
mod exchange_tests;
#[cfg(test)]
mod iu_tests;
#[cfg(test)]
mod pipes_tests;
#[cfg(test)]
mod status_sweep;
#[cfg(test)]
mod status_sweep_tests;
