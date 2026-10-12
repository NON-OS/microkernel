// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the virtio-net driver.
//!
//! The queues are plain data over DMA regions, so the proofs build the real
//! `RxQueue` and `TxQueue` over host memory, write hostile used-ring entries
//! the way a malicious device would, and run the real receive, transmit,
//! dispatch and handler code. What stands in: the legacy register file
//! (`regs`), the clock (`clock`), the reply call (`server::error`) and the
//! driver struct (`setup`), whose bring-up and teardown call the broker.

pub mod clock;
#[path = "../../capsule_driver_virtio_net/src/constants/mod.rs"]
pub mod constants;
#[cfg(test)]
mod discover;
pub mod modern;
// The legacy queue programming as it ships, over the model register file.
#[path = "../../capsule_driver_virtio_net/src/init/program_queue.rs"]
pub mod program_queue;
#[path = "../../capsule_driver_virtio_net/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_virtio_net/src/queue/mod.rs"]
pub mod queue;
pub mod regs;
#[path = "../../capsule_driver_virtio_net/src/rx.rs"]
pub mod rx;
#[path = "../../capsule_driver_virtio_net/src/scrub.rs"]
pub mod scrub;
pub mod server;
pub mod setup;
pub mod transport;
#[path = "../../capsule_driver_virtio_net/src/tx.rs"]
pub mod tx;

#[cfg(test)]
mod mac_tests;
#[cfg(test)]
mod station_tests;
#[cfg(test)]
mod tests;

#[cfg(kani)]
mod kani_proofs;
