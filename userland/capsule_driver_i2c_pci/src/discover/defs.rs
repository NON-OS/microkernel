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
pub(super) const MAX_DEVICES: usize = 128;
pub(super) const PCI_CLASS_SERIAL_BUS: u8 = 0x0c;
pub(super) const PCI_CLASS_SIGNAL_PROC: u8 = 0x11;
pub(super) const PCI_SUBCLASS_OTHER: u8 = 0x80;
pub(super) const ACPI_LPSS_FAMILY: &str = "acpi-lpss";
pub(super) const UNKNOWN_LPSS_FAMILY: &str = "Intel LPSS (unlisted id)";
pub(super) const CLASS_I2C_HID: u32 = 0x0041;
/// The broker's class for an ACPI GPIO community record.
pub const CLASS_GPIO_CTRL: u32 = 0x0080;

/// Upper bound on host controllers probed in one bring-up. Gemini Lake exposes
/// eight LPSS I2C functions; other platforms fewer.
pub const MAX_CONTROLLERS: usize = 8;

#[derive(Clone, Copy, Default)]
pub struct Found {
    pub device_id: u64,
    pub irq_line: u8,
    /// Physical base of the MMIO window, which the LPSS remap register is
    /// programmed with and an ACPI touchpad record names its controller by.
    pub bar0_base: u64,
    pub bar0_size: u64,
    pub pci_device: u16,
    pub clock_hz: u32,
    pub family: &'static str,
    pub is_acpi: bool,
}

impl Found {
    /// An Intel LPSS PCI function: it has the LPSS private register block
    /// (reset, remap, capabilities) behind the DesignWare core. Platform
    /// (ACPI) controllers such as AMD's AMDI0010 do not.
    pub fn is_lpss(&self) -> bool {
        !self.is_acpi
    }
}
