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

//! The firmware's touchpad declaration as setup reads it, and the record of
//! the controller one names. Discovery itself lists devices through the
//! broker and stays out.

#[path = "../../capsule_driver_i2c_pci/src/discover/acpi_touchpad.rs"]
mod acpi_touchpad;
#[path = "../../capsule_driver_i2c_pci/src/discover/found.rs"]
mod found;
// The capsule's own controller match, and the kernel's rule for starting it.
#[cfg(test)]
#[path = "../../capsule_driver_i2c_pci/src/discover/classify.rs"]
mod classify;
#[cfg(test)]
#[allow(dead_code)] // classify reads only some of the capsule's constants
#[path = "../../capsule_driver_i2c_pci/src/discover/defs.rs"]
mod defs;
#[cfg(test)]
#[path = "../../../src/hardware/inventory/i2c_host.rs"]
mod kernel_i2c_host;
#[cfg(test)]
mod host_gate_tests;
#[cfg(test)]
mod spawn_glue_tests;

pub use acpi_touchpad::{AcpiTouchpad, MAX_TARGETS};
pub use found::{Found, MAX_CONTROLLERS};
