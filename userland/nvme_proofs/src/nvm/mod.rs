// NONOS Operating System (AGPL-3.0-or-later)
// The real NVMe transfer constants. The upstream module also carries
// queue-setup constants that only the device bring-up path uses.
#[path = "../../../capsule_driver_nvme/src/nvm/constants.rs"]
#[allow(dead_code)]
mod constants;
// The real doorbell offsets the I/O queue rings.
#[path = "../../../capsule_driver_nvme/src/nvm/doorbell.rs"]
#[allow(dead_code)]
mod doorbell;
// The real decision whether a namespace gets an I/O queue.
#[path = "../../../capsule_driver_nvme/src/nvm/geometry/mod.rs"]
mod geometry;

pub(crate) use constants::IO_QID;
pub use constants::{MAX_SECTORS, SECTOR_SIZE};
pub(crate) use doorbell::cq_head_doorbell;
#[cfg(test)]
pub(crate) use doorbell::sq_tail_doorbell;
pub use geometry::{max_transfer_bytes, NamespaceGeometry, Refusal};
