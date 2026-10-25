// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the CCID reader driver. The shipped core is
//! mounted at the path the driver names (`crate::ccid`).

extern crate alloc;

#[path = "../../capsule_driver_ccid/src/ccid/mod.rs"]
pub mod ccid;

/// A reader's configuration, built a descriptor at a time.
pub mod reader;

/// Seeded reader configurations.
#[cfg(test)]
mod desc_sweep;
/// A seeded sweep over one command's exchange.
#[cfg(test)]
mod exchange_sweep;

#[cfg(test)]
mod desc_sweep_tests;
#[cfg(test)]
mod exchange_cut_tests;
#[cfg(test)]
mod exchange_halt_tests;
#[cfg(test)]
mod exchange_sweep_tests;
#[cfg(test)]
mod exchange_tests;
#[cfg(test)]
mod msg_tests;
#[cfg(test)]
mod notice_sweep_tests;
#[cfg(test)]
mod reader_tests;
#[cfg(test)]
mod slots_tests;
