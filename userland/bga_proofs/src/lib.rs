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

//! The BGA display capsule, proved against a register window rather than
//! against a display adapter.
//!
//! Each `#[path]` below pulls in the shipping driver source, so these tests
//! run the code that boots, not a copy of it: the DISPI mode set and its
//! read-back, the EDID read at BAR2, the start-up mode choice, and the
//! adapter's half of the display wire (`nonos_gfx_serve::Display`).
//!
//! The interface is worth proving without hardware because Bochs, QEMU and
//! their clones implement the same register set. A driver that programs the
//! wrong bits-per-pixel, or leaves the linear framebuffer undecoded, is wrong
//! against all of them, and a black screen is the same symptom for each.

#[path = "../../capsule_driver_bga/src/constants.rs"]
pub mod constants;
pub mod dispi;
#[path = "../../capsule_driver_bga/src/display/mod.rs"]
pub mod display;
#[path = "../../capsule_driver_bga/src/edid/mod.rs"]
pub mod edid;
#[path = "../../capsule_driver_bga/src/error.rs"]
pub mod error;
#[path = "../../capsule_driver_bga/src/regs.rs"]
pub mod regs;
#[path = "../../capsule_driver_bga/src/setup/mode.rs"]
pub mod start;
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

#[cfg(test)]
mod model;
#[cfg(test)]
mod tests;
