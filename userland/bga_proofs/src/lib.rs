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

//! The BGA display capsule, proved against a register window and video memory in memory. Each
//! `#[path]` pulls in the shipping driver source: the DISPI ID check, mode set and its order,
//! read-back, the firmware's state kept or written back, the EDID read at BAR2, the start-up mode,
//! one bring-up attempt end to end against `libc_shim`'s broker and kernel, and the adapter's half
//! of the display wire. Bochs, QEMU and their clones share the register set, so a driver wrong
//! against one is wrong against all.

#[path = "../../capsule_driver_bga/src/constants.rs"]
pub mod constants;
pub mod dispi;
#[path = "../../capsule_driver_bga/src/display/mod.rs"]
pub mod display;
#[path = "../../capsule_driver_bga/src/edid/mod.rs"]
pub mod edid;
#[path = "../../capsule_driver_bga/src/error.rs"]
pub mod error;
/// The shipping register access, each write answered by the adapter model.
pub mod regs;
#[path = "../../capsule_driver_bga/src/turn.rs"]
pub mod turn;

/// The kernel's DGON, the one definition, mounted as libc mounts it: `turn`
/// reaches its parser through the crate root.
#[path = "../../../src/hardware/broker/acpi/dgon_wire.rs"]
pub mod dgon;
use dgon::dgon_parse;

/// The kernel's display list, which the compositor walks to find bga.
#[path = "../../../src/services/registry/displays.rs"]
#[allow(dead_code)]
mod displays;

/// One bring-up attempt end to end (`tests::setup_tests` and the rest),
/// against `libc_shim`'s broker and kernel.
#[cfg(test)]
#[path = "../../capsule_driver_bga/src/announce.rs"]
mod announce;
#[cfg(test)]
#[path = "../../capsule_driver_bga/src/discover.rs"]
mod discover;
#[cfg(test)]
#[path = "../../capsule_driver_bga/src/handles.rs"]
mod handles;
#[cfg(test)]
#[path = "../../capsule_driver_bga/src/setup/mod.rs"]
mod setup;

#[cfg(test)]
mod kernel_ids;
#[cfg(test)]
mod model;
#[cfg(test)]
mod services;
#[cfg(test)]
use kernel_ids::{hardware, process};
#[cfg(test)]
mod syscall;
#[cfg(test)]
mod tests;
