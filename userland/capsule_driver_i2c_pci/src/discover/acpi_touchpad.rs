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
//! The kernel's record of one ACPI-declared HID-over-I2C device, and which
//! controller it names.

use super::defs::Found;

/// An ACPI-declared HID-over-I2C device, as the kernel's record carries it.
#[derive(Clone, Copy, Default)]
pub struct AcpiTouchpad {
    /// 7-bit slave address; zero when the firmware fills it in at run time,
    /// in which case the entry only names the controller.
    pub addr: u8,
    /// HID descriptor register (from `_DSM`, or the 0x0001 default).
    pub desc_reg: u16,
    /// The controller index the `_CRS` ResourceSource "I2Cn" names.
    pub controller_idx: Option<u8>,
    /// MMIO base of the platform controller the ResourceSource names, zero
    /// when it is a PCI function or unresolved.
    pub host_base: u64,
    /// The controller's ACPI NameSeg, for the console.
    pub host_name: [u8; 4],
    /// ConnectionSpeed in Hz, zero when unknown.
    pub speed_hz: u32,
    /// HID_INFO_* bits.
    pub info: u8,
    /// GpioInt pin and its community as `_UID`+1 (zero when unknown).
    pub gpio_pin: u16,
    pub gpio_community: u8,
}

impl AcpiTouchpad {
    /// Whether `dev` is the controller this record's ResourceSource names.
    pub fn names(&self, dev: &Found, index_of: impl Fn(u16) -> Option<u8>) -> bool {
        if dev.is_acpi {
            self.host_base != 0 && self.host_base == dev.bar0_base
        } else {
            self.controller_idx.is_some() && index_of(dev.pci_device) == self.controller_idx
        }
    }

    /// The device asks for a bus speed below fast mode.
    pub fn wants_standard_mode(&self) -> bool {
        self.speed_hz != 0 && self.speed_hz < 400_000
    }
}

/// Upper bound on ACPI HID candidates considered. Multi-SKU firmware declares
/// one device per possible pad; only the fitted one answers.
pub const MAX_TARGETS: usize = 8;
