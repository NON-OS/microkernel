// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the RTL8139 driver's RX ring primitives.
//!
//! The RTL8139 receives into a byte ring the device fills with per-packet
//! headers: a status word and a raw length, both device-chosen, and the
//! driver walks the ring by offset arithmetic. The ring readers and the
//! wrapping copy are the memory-safety core of that walk: every access they
//! make must land inside the ring no matter what offsets and lengths the
//! device induces. The proofs run the real primitives over a host ring.
//! The gate above them (`read_frame`) rejects bad status, short raw
//! lengths, and frames larger than Ethernet or the caller's buffer before
//! any copy; it reaches the part through PIO syscalls, which the shim
//! answers from a modelled part, so it runs here too (`recv_tests`), and
//! every access it performs goes through the primitives proven here.
//!
//! The setup sequence and the bring-up are run against the same part too:
//! an attempt either comes up or gives back every grant and the claim.

// The server loop allocates its buffers through `alloc`, as in the capsule.
extern crate alloc;

#[path = "../../capsule_driver_rtl8139/src/constants/mod.rs"]
pub mod constants;
// The driver's src directory, so discover.rs finds its modules beside it.
#[path = "../../capsule_driver_rtl8139/src"]
mod discover_dir {
    pub mod discover;
}
pub use discover_dir::discover;
pub mod init;
#[path = "../../capsule_driver_rtl8139/src/pio.rs"]
pub mod pio;
#[path = "../../capsule_driver_rtl8139/src/protocol/mod.rs"]
pub mod protocol;
pub mod ring;
/// The receive gate above the ring readers, over a ring in host memory.
#[path = "../../capsule_driver_rtl8139/src/rx/mod.rs"]
pub mod rx;
pub mod server;
pub mod setup;
/// The send path: four transmit slots behind the port grant.
#[path = "../../capsule_driver_rtl8139/src/tx/mod.rs"]
pub mod tx;

#[cfg(test)]
mod bus_master_tests;
#[cfg(test)]
mod dma_tests;
#[cfg(test)]
mod finish_tests;
#[cfg(test)]
mod ipc_tests;
#[cfg(test)]
mod order;
#[cfg(test)]
mod part;
#[cfg(test)]
mod predecessor_tests;
#[cfg(test)]
mod probe;
#[cfg(test)]
mod recv_tests;
#[cfg(test)]
mod reset_clock_tests;
#[cfg(test)]
mod reset_tests;
#[cfg(test)]
mod rtl_tests;
#[cfg(test)]
mod sequence_tests;
#[cfg(test)]
mod tx_watch_tests;

#[cfg(kani)]
mod kani_proofs;
