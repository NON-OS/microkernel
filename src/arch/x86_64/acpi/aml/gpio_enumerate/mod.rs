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

//! Enumerate the platform GPIO community controllers from the ACPI namespace.
//!
//! A touchpad's GpioInt descriptor names a GPIO controller as its
//! ResourceSource; that controller is an ACPI device (never a PCI function), so
//! PCI enumeration cannot find it and the community's interrupt status
//! register stays unreachable until the device is matched here by `_HID`.

mod crs;
mod enumerate;
mod hid_match;
mod sideband;
mod uid;

pub use enumerate::enumerate_gpio_controllers;
pub use sideband::{community_window, sbreg_from_bar0};
