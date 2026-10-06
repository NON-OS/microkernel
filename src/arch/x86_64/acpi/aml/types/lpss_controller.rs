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

/// An LPSS-style I2C host controller enumerated from the ACPI namespace by its
/// `_HID`, with the MMIO window and interrupt recovered from its `_CRS`. On a
/// modern laptop the controller is an ACPI device (for example INT33C3 or
/// AMDI0010), not a PCI function, so PCI enumeration never finds it and the
/// touchpad hanging off it stays unreachable until the controller is found
/// this way.
#[derive(Debug, Clone, Copy)]
pub struct LpssController {
    /// Base of the controller's MMIO register window, from the Memory32Fixed
    /// descriptor in `_CRS`.
    pub mmio_base: u64,
    /// Length of that MMIO window in bytes.
    pub mmio_size: u32,
    /// The controller interrupt number, from the Extended Interrupt descriptor.
    pub irq: u32,
    /// True when an Extended Interrupt descriptor was found and `irq` is
    /// meaningful.
    pub has_irq: bool,
    /// The eight-byte `_HID` that matched, retained for diagnostics.
    pub hid: [u8; 8],
}

impl LpssController {
    pub fn new(hid: [u8; 8]) -> Self {
        Self { mmio_base: 0, mmio_size: 0, irq: 0, has_irq: false, hid }
    }

    /// A controller record is usable only once its MMIO window is known; the
    /// interrupt is optional because the transfer engine can poll.
    pub fn is_valid(&self) -> bool {
        self.mmio_base != 0 && self.mmio_size != 0
    }
}
