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

//! Host proofs for the SVGA II display driver. The driver files are the
//! shipping source, included by path; only the port pair under `regs` is
//! replaced, by a model of the adapter.

#[path = "../../capsule_driver_vmware_svga/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_vmware_svga/src/discover.rs"]
pub mod discover;
#[path = "../../capsule_driver_vmware_svga/src/fifo/mod.rs"]
pub mod fifo;
#[path = "../../capsule_driver_vmware_svga/src/init/mod.rs"]
pub mod init;
pub mod model;
#[path = "../../capsule_driver_vmware_svga/src/protocol/mod.rs"]
pub mod protocol;
pub mod regs;
#[path = "../../capsule_driver_vmware_svga/src/screen/mod.rs"]
pub mod screen;
pub mod server;
#[path = "../../capsule_driver_vmware_svga/src/setup/mod.rs"]
pub mod setup;

#[cfg(test)]
mod bench;
#[cfg(test)]
mod tests;
