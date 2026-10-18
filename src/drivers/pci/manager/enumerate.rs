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

//! Every function on every bus of segment 0, then the domains behind VMDs.

extern crate alloc;

use alloc::vec::Vec;

use super::super::config::read32_unchecked;
use super::super::constants::*;
use super::super::types::PciDevice;
use super::probe::probe_device;

pub(super) fn enumerate_bus(bus: u8, devices: &mut Vec<PciDevice>) {
    for device_num in 0..32u8 {
        let id = read32_unchecked(bus, device_num, 0, CFG_VENDOR_ID as u8);
        let vendor_id = (id & 0xFFFF) as u16;

        if vendor_id == 0xFFFF || vendor_id == 0x0000 {
            continue;
        }

        let header_reg = read32_unchecked(bus, device_num, 0, CFG_CACHE_LINE_SIZE as u8);
        let header_type = ((header_reg >> 16) & 0xFF) as u8;
        let multifunction = (header_type & HDR_TYPE_MULTIFUNCTION) != 0;

        if let Some(dev) = probe_device(bus, device_num, 0) {
            devices.push(dev);
        }

        if multifunction {
            for function in 1..8u8 {
                if let Some(dev) = probe_device(bus, device_num, function) {
                    devices.push(dev);
                }
            }
        }
    }
}

pub(super) fn enumerate_all_buses() -> Vec<PciDevice> {
    let mut devices = Vec::new();

    for bus in 0..=255u8 {
        enumerate_bus(bus, &mut devices);
    }
    // After segment 0, which holds the VMDs whose domains these are.
    let behind_vmd = super::super::vmd::children(&devices);
    devices.extend(behind_vmd);

    devices
}
