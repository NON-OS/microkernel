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

//! Host proofs for the PIIX IDE disk driver. Every driver file below is the
//! shipping source, included by path; only the broker under them is the
//! shim's, with the port I/O answered by a model of the PIIX bus master
//! function and the drives on its two channels.

extern crate alloc;

#[path = "../../capsule_driver_piix_ide/src/channel/mod.rs"]
pub mod channel;
#[path = "../../capsule_driver_piix_ide/src/constants/mod.rs"]
pub mod constants;
#[path = "../../capsule_driver_piix_ide/src/discover.rs"]
pub mod discover;
#[path = "../../capsule_driver_piix_ide/src/disk/mod.rs"]
pub mod disk;
#[path = "../../capsule_driver_piix_ide/src/io/mod.rs"]
pub mod io;
#[path = "../../capsule_driver_piix_ide/src/regs/mod.rs"]
pub mod regs;
#[path = "../../capsule_driver_piix_ide/src/say/mod.rs"]
pub mod say;
pub mod server;
#[path = "../../capsule_driver_piix_ide/src/setup/mod.rs"]
pub mod setup;
#[path = "../../capsule_driver_piix_ide/src/wait.rs"]
pub mod wait;
#[allow(dead_code, unused_imports)]
#[path = "../../capsule_driver_piix_ide/src/wire/mod.rs"]
pub mod wire;

/// The kernel's listing of compatibility-mode channels, with the broker
/// types it names.
pub mod hardware;
pub mod model;

#[cfg(test)]
mod tests;
