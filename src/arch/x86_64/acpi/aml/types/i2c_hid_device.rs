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

/// A touchpad (or other I2C-HID peripheral) enumerated from the ACPI AML
/// namespace. The fields carry the firmware-declared parameters a driver needs
/// to bind the device without blind probing.
#[derive(Debug, Clone, Copy)]
pub struct I2cHidDevice {
    /// 7-bit I2C slave address from the I2cSerialBus descriptor. 10-bit
    /// addressing is out of scope; the raw 16-bit field is masked to 7 bits.
    pub slave_addr: u16,
    /// The GPIO pin number carrying the device interrupt, taken from the first
    /// entry of the GpioInt descriptor's pin table.
    pub gpio_pin: u16,
    /// HID descriptor register offset. ACPI declares this via the device's
    /// _DSM method (not parsed by this bounded extractor), so it defaults to
    /// 0x0001, the value used by the vast majority of I2C-HID touchpads.
    pub hid_desc_reg: u16,
    /// The seven-character EISAID (or the first eight bytes of a string _HID)
    /// that matched, padded with zero bytes. Retained for diagnostics.
    pub hid: [u8; 8],
    /// True when a GpioInt descriptor was found and `gpio_pin` is meaningful.
    pub has_gpio: bool,
    /// The final NameSeg of the I2cSerialBus ResourceSource, i.e. the ACPI name
    /// of the host controller the device hangs off (for example `I2C4`). Zero
    /// when the descriptor carried no ResourceSource. This is what selects the
    /// correct controller when several I2C hosts are present.
    pub controller: [u8; 4],
    /// The final NameSeg of the GpioInt ResourceSource, i.e. the ACPI name of
    /// the GPIO community controller carrying `gpio_pin` (for example `GPO0`).
    /// Zero when the descriptor carried no usable ResourceSource. This is what
    /// selects the correct community when several are present.
    pub gpio_controller: [u8; 4],
}

impl I2cHidDevice {
    /// Default HID descriptor register when _DSM is not evaluated.
    pub const DEFAULT_HID_DESC_REG: u16 = 0x0001;

    pub fn new(hid: [u8; 8]) -> Self {
        Self {
            slave_addr: 0,
            gpio_pin: 0,
            hid_desc_reg: Self::DEFAULT_HID_DESC_REG,
            hid,
            has_gpio: false,
            controller: [0; 4],
            gpio_controller: [0; 4],
        }
    }
}
