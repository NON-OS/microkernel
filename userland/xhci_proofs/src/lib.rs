// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the xHCI driver's TRB layer.
//! Every command and transfer the driver issues, and every event the
//! controller returns, crosses the ring as a 16-byte TRB. The accessors that
//! read device-written events (completion code, slot id, cycle, type) and
//! the builders that encode control transfers must match the xHCI
//! specification bit for bit: a wrong shift silently addresses the wrong
//! slot or misreads a completion. The proofs run the real TRB source against
//! the spec layouts. The event ring, and every function that reads it, runs
//! over host DMA memory against a producer written from the specification
//! (`event_ring`).

#[path = "../../capsule_driver_xhci/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_xhci/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_xhci/src/trb/mod.rs"]
pub mod trb;

/*
 * The controller, run against a register window and host DMA memory. Every
 * tree below is the shipping one whole, the server and the bring-up among
 * them; the shim's `mk_dma_map` hands the pool page-aligned host memory at a
 * bus address of its own, and its broker serves the window a test offers.
 */
extern crate alloc;

#[path = "../../capsule_driver_xhci/src/contexts/mod.rs"]
pub mod contexts;
#[path = "../../capsule_driver_xhci/src/controller/mod.rs"]
pub mod controller;
pub mod discover;
#[path = "../../capsule_driver_xhci/src/dma/mod.rs"]
pub mod dma;
#[path = "../../capsule_driver_xhci/src/error/mod.rs"]
pub mod error;
#[path = "../../capsule_driver_xhci/src/handles/mod.rs"]
pub mod handles;
#[path = "../../capsule_driver_xhci/src/regs/mod.rs"]
pub mod regs;
#[path = "../../capsule_driver_xhci/src/rings/mod.rs"]
pub mod rings;
#[path = "../../capsule_driver_xhci/src/server/mod.rs"]
pub mod server;
#[path = "../../capsule_driver_xhci/src/setup/mod.rs"]
pub mod setup;
// The capsule is a binary and never exports `SlotTable`, so it has no use
// for a `Default`; the lint only fires because this crate makes it public.
#[allow(clippy::new_without_default)]
#[path = "../../capsule_driver_xhci/src/slots/mod.rs"]
pub mod slots;

#[cfg(test)]
mod conformance;
#[cfg(test)]
mod control_out_tests;
#[cfg(test)]
mod control_short_tests;
#[cfg(test)]
mod event_ring;
#[cfg(test)]
mod hub;
#[cfg(test)]
mod isoch_context_tests;
#[cfg(test)]
mod isoch_in_tests;
#[cfg(test)]
mod isoch_tests;
#[cfg(test)]
mod op_number_tests;
#[cfg(test)]
mod server_tests;
#[cfg(test)]
mod sharing;
#[cfg(test)]
mod xhci_tests;

#[cfg(kani)]
mod kani_proofs;
