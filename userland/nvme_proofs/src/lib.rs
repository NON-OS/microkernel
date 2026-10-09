// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the NVMe driver's untrusted-input parsers.

pub mod admin;
#[path = "../../capsule_driver_nvme/src/constants/mod.rs"]
pub mod constants;
pub mod controller;
#[path = "../../capsule_driver_nvme/src/error/mod.rs"]
pub mod error;
pub mod nvm;
#[path = "../../capsule_driver_nvme/src/protocol/mod.rs"]
pub mod protocol;
pub mod server;

#[cfg(test)]
mod nvme_tests;
#[cfg(test)]
mod reason_tests;

#[cfg(kani)]
mod kani_proofs;
#[path = "../../capsule_driver_nvme/src/server/medium_rule.rs"]
pub mod medium_rule;
#[cfg(test)]
mod medium_tests;
#[cfg(test)]
mod doorbell_tests;
#[cfg(test)]
mod ready_tests;
#[cfg(test)]
mod completion_tests;
#[cfg(test)]
mod foreign_tests;
#[cfg(test)]
mod geometry_tests;
#[cfg(test)]
mod hostile_tests;
#[cfg(test)]
mod hmb_layout_tests;
#[cfg(test)]
mod hmb_plan_tests;

// The real pure pieces of bring-up: the clock budget every wait is timed on,
// the order controllers are tried in and the one served, the console line,
// and the 64-bit register split.
#[path = "../../capsule_driver_nvme/src/clock/budget.rs"]
pub mod budget;
#[path = "../../capsule_driver_nvme/src/discover/choice.rs"]
pub mod choice;
#[path = "../../capsule_driver_nvme/src/log/line/mod.rs"]
pub mod line;
#[path = "../../capsule_driver_nvme/src/regs/lo_hi.rs"]
pub mod lo_hi;
#[path = "../../capsule_driver_nvme/src/discover/rank.rs"]
pub mod rank;
#[cfg(test)]
mod bringup_tests;

// The kernel client's map from 512-byte sectors onto the namespace's LBAs.
// Its items are pub(crate) for the kernel's sake; the tests use them all.
#[path = "../../../src/hardware/nvme_capsule/client/lba_map.rs"]
#[allow(dead_code)]
pub mod lba_map;
#[cfg(test)]
mod lba_map_tests;
