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

use super::parse::{parse_hid_devices, parse_platform_controllers, I2cHostName};
use super::types::I2cHidDevice;
use crate::arch::x86_64::acpi::aml::tables::aml_blocks;

/// Every HID-over-I2C device the DSDT and SSDTs declare. Empty when the
/// firmware declares none (a desktop, or a laptop whose pad is on PS/2 or
/// USB); never panics.
pub fn enumerate_i2c_hid_devices() -> Vec<I2cHidDevice> {
    let mut out = Vec::new();
    for aml in aml_blocks() {
        out.extend(parse_hid_devices(&aml));
    }
    out
}

/// Every platform (non-PCI) I2C host controller with its ACPI name.
pub fn enumerate_platform_i2c_hosts() -> Vec<I2cHostName> {
    let mut out = Vec::new();
    for aml in aml_blocks() {
        out.extend(parse_platform_controllers(&aml));
    }
    out
}

pub fn find_touchpads() -> Vec<I2cHidDevice> {
    enumerate_i2c_hid_devices().into_iter().filter(|d| d.is_touchpad()).collect()
}

pub fn find_touchscreens() -> Vec<I2cHidDevice> {
    enumerate_i2c_hid_devices().into_iter().filter(|d| d.is_touchscreen()).collect()
}
