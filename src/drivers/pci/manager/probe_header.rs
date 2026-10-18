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
//! What a function's header says, before its capabilities are walked.

use super::super::config::read32_unchecked;
use super::super::constants::*;
use super::super::types::{ClassCode, DeviceId, HeaderType, PciAddress, PciDevice};

/// The function's ids, class, header type and interrupt line and pin; None
/// where no function answers.
pub(super) fn read_header(bus: u8, device_num: u8, function: u8) -> Option<PciDevice> {
    let id = read32_unchecked(bus, device_num, function, CFG_VENDOR_ID as u8);
    let vendor_id = (id & 0xFFFF) as u16;

    if vendor_id == 0xFFFF || vendor_id == 0x0000 {
        return None;
    }

    let device_id = ((id >> 16) & 0xFFFF) as u16;

    let class_reg = read32_unchecked(bus, device_num, function, CFG_REVISION_ID as u8);
    let revision = (class_reg & 0xFF) as u8;
    let prog_if = ((class_reg >> 8) & 0xFF) as u8;
    let subclass = ((class_reg >> 16) & 0xFF) as u8;
    let class = ((class_reg >> 24) & 0xFF) as u8;

    let header_reg = read32_unchecked(bus, device_num, function, CFG_CACHE_LINE_SIZE as u8);
    let header_type_raw = ((header_reg >> 16) & 0xFF) as u8;
    let multifunction = (header_type_raw & HDR_TYPE_MULTIFUNCTION) != 0;
    let header_type = HeaderType::from(header_type_raw);

    let subsys_reg = read32_unchecked(bus, device_num, function, CFG_SUBSYSTEM_VENDOR_ID as u8);
    let subsystem_vendor_id = (subsys_reg & 0xFFFF) as u16;
    let subsystem_id = ((subsys_reg >> 16) & 0xFFFF) as u16;

    let int_reg = read32_unchecked(bus, device_num, function, CFG_INTERRUPT_LINE as u8);
    let mut device = PciDevice::new(PciAddress::new(bus, device_num, function));
    device.device_id_info =
        DeviceId { vendor_id, device_id, subsystem_vendor_id, subsystem_id, revision };
    device.class_code = ClassCode::new(class, subclass, prog_if);
    device.header_type = header_type;
    device.multifunction = multifunction;
    device.interrupt_line = (int_reg & 0xFF) as u8;
    device.interrupt_pin = ((int_reg >> 8) & 0xFF) as u8;
    Some(device)
}
