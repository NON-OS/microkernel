// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Host proofs for the virtio-scsi driver. The driver's files and the
//! shared serve crate's are the shipping source, included by path; only the
//! libc under them is the shim's, and the device is a model.

extern crate alloc;

#[path = "../../capsule_driver_virtio_scsi/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_virtio_scsi/src/discover.rs"]
pub mod discover;
#[path = "../../capsule_driver_virtio_scsi/src/hba.rs"]
pub mod hba;
#[path = "../../capsule_driver_virtio_scsi/src/queue/mod.rs"]
pub mod queue;
pub mod setup;
#[path = "../../capsule_driver_virtio_scsi/src/wait.rs"]
pub mod wait;

// nonos_scsi_serve, at the crate root where its files name each other.
#[path = "../../nonos_scsi_serve/src/say/mod.rs"]
pub mod say;
#[path = "../../nonos_scsi_serve/src/server/mod.rs"]
pub mod server;
#[path = "../../nonos_scsi_serve/src/units.rs"]
pub mod units;
#[allow(dead_code, unused_imports)]
#[path = "../../nonos_scsi_serve/src/wire/mod.rs"]
pub mod wire;

/// The transport's common configuration as the transport proofs' model
/// device names it.
pub mod common {
    pub use nonos_virtio::common::*;
}
pub mod features {
    pub use nonos_virtio::features::*;
}

pub mod model;

#[cfg(test)]
mod tests;
