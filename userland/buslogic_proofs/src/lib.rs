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

//! Host proofs for the BusLogic MultiMaster driver. The driver's files and
//! the shared serve crate's are the shipping source, included by path; only
//! the broker and libc under them are the shim's, with the port I/O answered
//! by a model of the adapter.

extern crate alloc;

#[path = "../../capsule_driver_buslogic/src/adapter.rs"]
pub mod adapter;
#[path = "../../capsule_driver_buslogic/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_buslogic/src/discover.rs"]
pub mod discover;
#[path = "../../capsule_driver_buslogic/src/hba.rs"]
pub mod hba;
#[path = "../../capsule_driver_buslogic/src/port.rs"]
pub mod port;
pub mod setup;
#[path = "../../capsule_driver_buslogic/src/wait.rs"]
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

/// The units, the driver's service name and its reply endpoint, as its
/// main.rs gives them.
pub use units::Units;
pub const NAME: &[u8] = b"driver.buslogic0";
pub const REPLY_ENDPOINT: u64 = 4_294_967_358;

pub mod model;

#[cfg(test)]
mod tests;
