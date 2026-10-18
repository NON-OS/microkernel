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

//! The controller on the bench, as discovery would record it.

use crate::discover::Found;

/// Gemini Lake's I2C input clock, the part this capsule was brought up on.
pub const GEMINI_LAKE_HZ: u32 = 133_000_000;
/// Where the controller's window sits physically, for the LPSS remap.
pub const LPSS_PHYS: u64 = 0x0000_0001_0000_0000;

/// Gemini Lake's I2C0.
pub fn controller() -> Found {
    Found {
        device_id: 1,
        bar0_base: LPSS_PHYS,
        bar0_size: 0x1000,
        pci_device: 0x31AC,
        clock_hz: GEMINI_LAKE_HZ,
        family: "Gemini Lake",
        is_acpi: false,
    }
}
