// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the AHCI driver: its decisions, and its open, command and
//! hand-back paths on a model HBA. The tree mirrors the driver's paths.

extern crate alloc;

pub mod choose;
/// The driver's clock, on the host's.
pub mod clock;
pub mod constants;
#[cfg(test)]
pub mod controller;
/// Which PCI functions the driver takes for an AHCI HBA.
pub mod discover;
#[cfg(test)]
pub mod engine;
#[path = "../../capsule_driver_ahci/src/error/mod.rs"]
pub mod error;
/// What the driver takes from a drive's IDENTIFY DEVICE block.
#[path = "../../capsule_driver_ahci/src/identity/mod.rs"]
pub mod identity;
/// A model HBA port and drive the command path runs against.
#[cfg(test)]
mod model;
/// The request wire format, and the one parser of a request body: what a
/// client asks to read or write, checked against the disk before any DMA.
#[path = "../../capsule_driver_ahci/src/protocol/mod.rs"]
pub mod protocol;
/// The MMIO accessor: host memory, or the model port a test plugs in.
#[cfg(test)]
mod regs;
// Its `parse` is visible to its parent only: built at the root, by its tests.
#[cfg(test)]
#[path = "../../capsule_driver_ahci/src/server/handlers/rw_parse.rs"]
mod rw_parse;
pub mod server;
pub use server::{medium_rule, power_off_rule};
/// The driver's open path and what it takes, over the stand-in nonos_libc.
#[cfg(test)]
mod setup;
#[cfg(test)]
use setup::{handles, log};

#[cfg(test)]
mod bring_up_tests;
#[cfg(test)]
mod choose_tests;
#[cfg(test)]
mod discover_tests;
#[cfg(test)]
mod identify_reply_tests;
#[cfg(test)]
mod identity_fuzz_tests;
#[cfg(test)]
mod identity_tests;
#[cfg(test)]
mod medium_tests;
#[cfg(test)]
mod names_tests;
#[cfg(test)]
mod ports_tests;
#[cfg(test)]
mod reason_tests;
#[cfg(test)]
mod remap_tests;
#[cfg(test)]
mod request_tests;
#[cfg(test)]
mod sig_tests;
#[cfg(test)]
mod span_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod window_tests;
