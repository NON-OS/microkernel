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

/// How the device signals a pending input report, from its `_CRS`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidInterrupt {
    /// No interrupt resource was found; the host has to poll.
    None,
    /// A GpioInt resource: `pin` on the GPIO controller the device names.
    Gpio { pin: u16, level: bool, active_high: bool },
    /// An Interrupt (Extended IRQ) resource: an APIC GSI.
    Apic { gsi: u32, level: bool, active_high: bool },
}

/// An HID-over-I2C device found in the ACPI namespace, with everything the
/// firmware declares statically about it. Values the firmware computes at run
/// time (an `_INI` that patches the address, a `_DSM` that returns a variable)
/// cannot be read without an AML interpreter and stay at their "unknown"
/// value, which the drivers answer by probing.
#[derive(Debug, Clone)]
pub struct I2cHidDevice {
    /// `_HID` as decoded (EISAID expanded, or the first eight string bytes).
    pub hid: [u8; 8],
    /// The `_CID` that made it an HID-over-I2C device (PNP0C50 or ACPI0C50),
    /// zero when it was recognised by its vendor `_HID` alone.
    pub cid: [u8; 8],
    /// 7-bit slave address from the I2cSerialBus resource, zero when unknown.
    pub slave_addr: u8,
    /// The I2cSerialBus resource asks for 10-bit addressing, which this
    /// driver stack does not implement.
    pub ten_bit: bool,
    /// ConnectionSpeed in Hz, zero when unknown.
    pub speed_hz: u32,
    /// HID descriptor register: the `_DSM` function 1 answer when it is a
    /// constant, otherwise the 0x0001 most devices use.
    pub hid_desc_reg: u16,
    /// True when `hid_desc_reg` came from the `_DSM`, false for the default.
    pub desc_reg_from_dsm: bool,
    /// Trailing NameSeg of the host controller (I2cSerialBus ResourceSource,
    /// or the enclosing scope when the resource names none), e.g. `I2C4`.
    pub controller: [u8; 4],
    /// Trailing NameSeg of the GPIO controller of a GpioInt resource.
    pub gpio_controller: [u8; 4],
    pub interrupt: HidInterrupt,
    pub device_type: I2cHidDeviceType,
}

impl I2cHidDevice {
    pub const DEFAULT_HID_DESC_REG: u16 = 0x0001;

    pub fn is_touchpad(&self) -> bool {
        self.device_type == I2cHidDeviceType::Touchpad
    }

    pub fn is_touchscreen(&self) -> bool {
        self.device_type == I2cHidDeviceType::Touchscreen
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2cHidDeviceType {
    Unknown,
    Touchpad,
    Touchscreen,
    Keyboard,
    Mouse,
    Stylus,
    Sensor,
}
