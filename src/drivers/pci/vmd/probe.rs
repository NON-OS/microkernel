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

//! One function behind a VMD read into the record the broker lists.

use super::super::capabilities::{collect_all_capabilities, get_power_management_info};
use super::super::config::ConfigSpace;
use super::super::types::{ClassCode, DeviceId, HeaderType, PciAddress, PciDevice};
use super::bar_size::bars;

/// `None` for an absent function or a bridge: the broker has nothing to hand
/// a capsule for one.
pub(super) fn probe(address: PciAddress) -> Option<PciDevice> {
    let config = ConfigSpace::new(address);
    let id = config.read32(0x00).ok()?;
    let vendor_id = (id & 0xFFFF) as u16;
    if vendor_id == 0xFFFF || vendor_id == 0 {
        return None;
    }
    let header = config.read8(0x0E).ok()?;
    if header & 0x7F != 0 {
        return None;
    }
    let class_reg = config.read32(0x08).ok()?;
    let subsys = config.read32(0x2C).ok()?;

    let mut dev = PciDevice::new(address);
    dev.device_id_info = DeviceId {
        vendor_id,
        device_id: (id >> 16) as u16,
        subsystem_vendor_id: (subsys & 0xFFFF) as u16,
        subsystem_id: (subsys >> 16) as u16,
        revision: (class_reg & 0xFF) as u8,
    };
    dev.class_code =
        ClassCode::new((class_reg >> 24) as u8, (class_reg >> 16) as u8, (class_reg >> 8) as u8);
    dev.header_type = HeaderType::from(header);
    dev.multifunction = header & 0x80 != 0;
    dev.bars = bars(&config);
    dev.capabilities = collect_all_capabilities(&config).unwrap_or_default();
    dev.power_management = get_power_management_info(&config).ok().flatten();
    // No INTx and no MSI-X through a VMD; see the module note.
    dev.interrupt_line = 0xFF;
    dev.interrupt_pin = 0;
    dev.sync_compat_fields();
    Some(dev)
}
