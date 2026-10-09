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

//! One AML block in, its HID-over-I2C devices and I2C host controllers out.
//! Pure over the bytes, so the host proofs run it on firmware-shaped input.

use alloc::vec::Vec;

use super::configs::{has_touchpad_vendor_prefix, ids_of, is_hid_over_i2c_id};
use super::dsm::hid_descriptor_register;
use super::hids::classify_hid_device;
use super::names::{buffer_or_method, named_integer};
use super::resources::{apic_level_and_high, extras, gpio_level_and_high};
use super::types::{HidInterrupt, I2cHidDevice, I2cHidDeviceType};
use super::walk::devices;
use crate::arch::x86_64::acpi::aml::crs::parse_crs;
use crate::arch::x86_64::acpi::aml::types::I2cHidDevice as CrsFields;

/// Every HID-over-I2C device declared in `aml`: matched by a PNP0C50 or
/// ACPI0C50 `_HID` or `_CID`, or by a touchpad vendor `_HID`. A device whose
/// `_STA` is the constant zero is absent and left out.
pub fn parse_hid_devices(aml: &[u8]) -> Vec<I2cHidDevice> {
    let mut out = Vec::new();
    for node in devices(aml) {
        let Some(hid) = ids_of(node.body, b"_HID").first().copied() else { continue };
        let cid = if is_hid_over_i2c_id(&hid) {
            hid
        } else {
            ids_of(node.body, b"_CID").into_iter().find(is_hid_over_i2c_id).unwrap_or([0; 8])
        };
        if cid == [0; 8] && !has_touchpad_vendor_prefix(&hid) {
            continue;
        }
        if named_integer(node.body, b"_STA") == Some(0) {
            continue;
        }
        let crs = buffer_or_method(node.body, b"_CRS");
        let mut f = CrsFields::new(hid);
        if let Some(c) = crs {
            parse_crs(c, &mut f);
        }
        // Templates a _CRS method concatenates live in the device body.
        if f.slave_addr == 0 || !f.has_gpio {
            parse_crs(node.body, &mut f);
        }
        let slave_addr = (f.slave_addr & 0x7F) as u8;
        let regions: Vec<&[u8]> = crs.into_iter().chain(core::iter::once(node.body)).collect();
        let ex = extras(&regions, slave_addr, f.has_gpio.then_some(f.gpio_pin));
        let interrupt = if f.has_gpio {
            let (level, active_high) = ex.gpio_flags.map_or((true, false), gpio_level_and_high);
            HidInterrupt::Gpio { pin: f.gpio_pin, level, active_high }
        } else if let Some((gsi, flags)) = ex.apic {
            let (level, active_high) = apic_level_and_high(flags);
            HidInterrupt::Apic { gsi, level, active_high }
        } else {
            HidInterrupt::None
        };
        let dsm_reg = hid_descriptor_register(node.body);
        let controller = if f.controller != [0; 4] { f.controller } else { node.parent };
        let mut device_type = classify_hid_device(&hid);
        if device_type == I2cHidDeviceType::Unknown && has_touchpad_vendor_prefix(&hid) {
            device_type = I2cHidDeviceType::Touchpad;
        }
        out.push(I2cHidDevice {
            hid,
            cid,
            slave_addr,
            ten_bit: ex.ten_bit,
            speed_hz: ex.speed_hz,
            hid_desc_reg: dsm_reg.unwrap_or(I2cHidDevice::DEFAULT_HID_DESC_REG),
            desc_reg_from_dsm: dsm_reg.is_some(),
            controller,
            gpio_controller: f.gpio_controller,
            interrupt,
            device_type,
        });
    }
    out
}

/// An I2C host controller the firmware declares as a platform device (AMD
/// AMDI0010, Intel Haswell/Broadwell INT33C2..), not as a PCI function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I2cHostName {
    pub name: [u8; 4],
    pub hid: [u8; 8],
    pub mmio_base: u64,
}

const PLATFORM_I2C_HIDS: [&[u8]; 15] = [
    b"INT33C2", b"INT33C3", b"INT3432", b"INT3433", b"INT3442", b"INT3443", b"INT3444",
    b"INT3445", b"INT3446", b"INT3447", b"80860F41", b"808622C1", b"AMDI0010", b"AMDI0510",
    b"AMD0010",
];

fn is_platform_i2c(hid: &[u8; 8]) -> bool {
    PLATFORM_I2C_HIDS.iter().any(|id| {
        if id.len() == 7 {
            &hid[..7] == *id && hid[7] == 0
        } else {
            &hid[..] == *id
        }
    })
}

/// The ACPI name and MMIO base of every platform I2C controller in `aml`, so
/// a touchpad's ResourceSource can be matched to the controller record the
/// broker registers for the same window.
pub fn parse_platform_controllers(aml: &[u8]) -> Vec<I2cHostName> {
    let mut out = Vec::new();
    for node in devices(aml) {
        let Some(hid) = ids_of(node.body, b"_HID").first().copied() else { continue };
        if !is_platform_i2c(&hid) {
            continue;
        }
        let Some(crs) = buffer_or_method(node.body, b"_CRS") else { continue };
        if let Some(base) = memory32_base(crs) {
            out.push(I2cHostName { name: node.name, hid, mmio_base: base });
        }
    }
    out
}

/// The base of the first Memory32Fixed (0x86) descriptor in `region`.
fn memory32_base(region: &[u8]) -> Option<u64> {
    let at = region.windows(3).position(|w| w == [0x86, 0x09, 0x00])?;
    let d = region.get(at + 3..at + 12)?;
    let base = u32::from_le_bytes([d[1], d[2], d[3], d[4]]);
    (base != 0).then_some(u64::from(base))
}
