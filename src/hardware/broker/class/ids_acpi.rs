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

//! The class ids only the ACPI enumerator records.

/// Subset of INPUT: an ACPI-declared I2C-HID touchpad, carrying its I2C
/// slave address and HID descriptor register for the HID driver to bind
/// without probing.
pub(in crate::hardware::broker) const I2C_HID: u32 = 0x0041;
