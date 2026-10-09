// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the AHCI driver's pure decisions: the link-up predicates,
//! the served-port choice, the rules a drive's IDENTIFY block is held to, the
//! span and PRD count a command may carry, the wait on a command against a
//! hostile port, the recovery after a failed one, and the ports the mapped
//! ABAR window reaches. A directory tree mirroring the driver's module path
//! lets the included files' `crate::constants::regs` and `super::candidate`
//! paths resolve unchanged.

pub mod choose;
/// The driver's clock, on the host's.
pub mod clock;
pub mod constants;
pub mod controller;
/// Which PCI functions the driver takes for an AHCI HBA.
pub mod discover;
pub mod engine;
#[path = "../../capsule_driver_ahci/src/error/mod.rs"]
pub mod error;
/// What the driver takes from a drive's IDENTIFY DEVICE block.
#[path = "../../capsule_driver_ahci/src/identity/mod.rs"]
pub mod identity;
/// The request wire format, and the one parser of a request body: what a
/// client asks to read or write, checked against the disk before any DMA.
#[path = "../../capsule_driver_ahci/src/protocol/mod.rs"]
pub mod protocol;
/// The MMIO accessor, pointed by the recovery proofs at host memory.
#[cfg(test)]
#[path = "../../capsule_driver_ahci/src/regs/mod.rs"]
mod regs;
// Its `parse` is visible only to its parent, here the crate root, so it is
// built where the tests that call it are.
#[cfg(test)]
#[path = "../../capsule_driver_ahci/src/server/handlers/rw_parse.rs"]
mod rw_parse;

/// The kernel's Intel VMD list, which the driver's rule must agree with.
#[cfg(test)]
#[path = "../../../src/hardware/inventory/vmd.rs"]
mod kernel_vmd;

#[cfg(test)]
mod bring_up_tests;
#[cfg(test)]
mod choose_tests;
#[cfg(test)]
mod completion_hostile_tests;
#[cfg(test)]
mod completion_tests;
#[cfg(test)]
mod discover_tests;
#[cfg(test)]
mod identify_reply_tests;
#[cfg(test)]
mod identity_fuzz_tests;
#[cfg(test)]
mod identity_tests;
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
#[path = "../../capsule_driver_ahci/src/server/medium_rule.rs"]
pub mod medium_rule;
#[cfg(test)]
mod medium_tests;
#[cfg(test)]
mod names_tests;
#[cfg(test)]
mod ports_tests;
