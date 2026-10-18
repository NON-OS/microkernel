// NONOS Operating System (AGPL-3.0-or-later)
// The real I/O queue and its namespace check, the test as its controller.
#[path = "../../../capsule_driver_nvme/src/nvm/alloc.rs"]
mod alloc;
#[path = "../../../capsule_driver_nvme/src/nvm/answer_by.rs"]
mod answer_by;
#[path = "../../../capsule_driver_nvme/src/nvm/bound.rs"]
mod bound;
#[cfg(test)]
mod bound_tests;
#[cfg(test)]
mod clock_tests;
#[path = "../../../capsule_driver_nvme/src/nvm/constants.rs"]
mod constants;
#[path = "../../../capsule_driver_nvme/src/nvm/copy.rs"]
mod copy;
#[cfg(test)]
mod copy_tests;
#[path = "../../../capsule_driver_nvme/src/nvm/doorbell.rs"]
mod doorbell;
#[path = "../../../capsule_driver_nvme/src/nvm/failure.rs"]
mod failure;
#[cfg(test)]
mod fill_tests;
#[path = "../../../capsule_driver_nvme/src/nvm/flush.rs"]
mod flush;
#[path = "../../../capsule_driver_nvme/src/nvm/geometry/mod.rs"]
mod geometry;
#[cfg(test)]
mod late_tests;
#[cfg(test)]
mod model;
#[cfg(test)]
mod no_clock_tests;
#[path = "../../../capsule_driver_nvme/src/nvm/park.rs"]
mod park;
#[cfg(test)]
mod park_tests;
#[path = "../../../capsule_driver_nvme/src/nvm/prp.rs"]
mod prp;
#[cfg(test)]
mod prp_tests;
#[path = "../../../capsule_driver_nvme/src/nvm/queue.rs"]
mod queue;
#[path = "../../../capsule_driver_nvme/src/nvm/restart.rs"]
mod restart;
#[path = "../../../capsule_driver_nvme/src/nvm/retry.rs"]
mod retry;
#[cfg(test)]
mod retry_tests;
#[path = "../../../capsule_driver_nvme/src/nvm/ring_wait.rs"]
mod ring_wait;
#[path = "../../../capsule_driver_nvme/src/nvm/served.rs"]
mod served;
#[path = "../../../capsule_driver_nvme/src/nvm/settle_rule.rs"]
mod settle_rule;
#[cfg(test)]
mod settle_tests;
#[path = "../../../capsule_driver_nvme/src/nvm/setup.rs"]
mod setup;
#[path = "../../../capsule_driver_nvme/src/nvm/submit.rs"]
mod submit;
#[path = "../../../capsule_driver_nvme/src/nvm/transfer.rs"]
mod transfer;
#[path = "../../../capsule_driver_nvme/src/nvm/wait.rs"]
mod wait;

pub(crate) use constants::IO_QID;
pub use constants::{MAX_SECTORS, SECTOR_SIZE};
pub(crate) use doorbell::cq_head_doorbell;
#[cfg(test)]
pub(crate) use doorbell::sq_tail_doorbell;
pub use geometry::{max_transfer_bytes, NamespaceGeometry, Refusal};
pub use queue::IoQueue;
pub use setup::bring_up;
