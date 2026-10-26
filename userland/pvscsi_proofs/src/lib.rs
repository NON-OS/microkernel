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

//! Host proofs for the PVSCSI driver. The driver's files and the shared
//! serve crate's are the shipping source, included by path; only the libc
//! under them is the shim's, and the adapter is a model.

extern crate alloc;

#[path = "../../capsule_driver_pvscsi/src/adapter.rs"]
pub mod adapter;
#[path = "../../capsule_driver_pvscsi/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_pvscsi/src/discover.rs"]
pub mod discover;
#[path = "../../capsule_driver_pvscsi/src/hba.rs"]
pub mod hba;
#[path = "../../capsule_driver_pvscsi/src/moved.rs"]
pub mod moved;
#[path = "../../capsule_driver_pvscsi/src/regs.rs"]
pub mod regs;
#[path = "../../capsule_driver_pvscsi/src/wait.rs"]
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

pub mod model;

#[cfg(test)]
mod tests;
