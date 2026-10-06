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

use alloc::vec::Vec;

use crate::arch::x86_64::acpi::aml::{enumerate_gpio_controllers, enumerate_i2c_controllers};
use crate::arch::x86_64::acpi::devices::i2c::{
    enumerate_i2c_hid_devices, enumerate_platform_i2c_hosts, I2cHidDeviceType,
};

use super::super::table::register_platform_device;
use super::hid_record::hid_record;
use super::record::device_record;
use super::report::report;

/// Register the ACPI-enumerated platform I2C controllers and the firmware's
/// HID-over-I2C devices with the broker table, and say on the boot console
/// what was found. Touchpads (and devices whose class the `_HID` does not
/// tell) are registered, touchpads first so the host driver tries them
/// first; touchscreens are left out, since the input driver drives a pad.
/// A machine whose firmware declares no such device registers nothing and
/// says nothing.
pub fn register_acpi_i2c() {
    for ctl in enumerate_i2c_controllers() {
        register_platform_device(device_record(&ctl));
    }
    let gpio = enumerate_gpio_controllers();
    let hosts = enumerate_platform_i2c_hosts();
    let mut devices: Vec<_> = enumerate_i2c_hid_devices()
        .into_iter()
        .filter(|d| d.device_type != I2cHidDeviceType::Touchscreen)
        .collect();
    devices.sort_by_key(|d| d.device_type != I2cHidDeviceType::Touchpad);
    for dev in &devices {
        report(dev, &hosts);
        register_platform_device(hid_record(dev, &gpio, &hosts));
    }
}
