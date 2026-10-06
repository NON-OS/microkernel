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

//! HID-over-I2C devices from the ACPI namespace: matched by `_HID` or
//! `_CID` (PNP0C50/ACPI0C50), with the slave address, bus speed, interrupt
//! (GpioInt or APIC), HID descriptor register (`_DSM` function 1) and host
//! controller name the firmware declares statically.

mod configs;
mod dsm;
mod enumerate;
pub mod hids;
mod names;
mod parse;
mod resources;
mod types;
mod walk;

pub use enumerate::{
    enumerate_i2c_hid_devices, enumerate_platform_i2c_hosts, find_touchpads, find_touchscreens,
};
pub use hids::{classify_hid_device, TOUCHPAD_HIDS, TOUCHSCREEN_HIDS};
pub use parse::{parse_hid_devices, parse_platform_controllers, I2cHostName};
pub use types::{HidInterrupt, I2cHidDevice, I2cHidDeviceType};
