// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the NVMe driver: parsers, queues, recovery, teardown.

pub mod admin;
#[path = "../../capsule_driver_nvme/src/constants/mod.rs"]
pub mod constants;
pub mod controller;
pub mod discover;
#[path = "../../capsule_driver_nvme/src/error/mod.rs"]
pub mod error;
pub mod nvm;
#[path = "../../capsule_driver_nvme/src/protocol/mod.rs"]
pub mod protocol;
pub mod recover;
pub mod server;
pub mod setup;

// The driver's clock, DMA, registers, console and handles over the libc
// shim, and the controller the tests play on host memory.
#[cfg(test)]
mod fake;
mod host;
pub use host::{clock, dma, handles, log, regs};

#[cfg(test)]
mod cc_tests;
#[cfg(test)]
mod completion_tests;
#[cfg(test)]
mod doorbell_tests;
#[cfg(test)]
mod foreign_tests;
#[cfg(test)]
mod geometry_tests;
#[cfg(test)]
mod hmb_layout_tests;
#[cfg(test)]
mod hmb_plan_tests;
#[cfg(test)]
mod hostile_tests;
#[cfg(kani)]
mod kani_proofs;
#[path = "../../capsule_driver_nvme/src/server/medium_rule.rs"]
pub mod medium_rule;
#[cfg(test)]
mod medium_tests;
#[cfg(test)]
mod nvme_tests;
#[cfg(test)]
mod power_off_tests;
#[cfg(test)]
mod ready_tests;
#[cfg(test)]
mod reason_tests;

// The real pure pieces of bring-up: the clock budget every wait is timed on,
// the order controllers are tried in and the one served, the console line,
// and the 64-bit register split.
#[cfg(test)]
mod bringup_tests;
pub use host::{clock::budget, log::line, regs::lo_hi};
#[path = "../../capsule_driver_nvme/src/discover/choice.rs"]
pub mod choice;
#[path = "../../capsule_driver_nvme/src/discover/rank.rs"]
pub mod rank;

// The kernel client's map from 512-byte sectors onto the namespace's LBAs.
// Its items are pub(crate) for the kernel's sake; the tests use them all.
#[path = "../../../src/hardware/nvme_capsule/client/lba_map.rs"]
#[allow(dead_code)]
pub mod lba_map;
#[cfg(test)]
mod lba_map_tests;
