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

//! The adapter in the device table: the first PVSCSI function, and the
//! memory BAR its registers are in.

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::constants::pci::{MEM_SPACE, PVSCSI_DEVICE_ID, VMWARE_VENDOR_ID};

/// The device list holds ACPI and fabricated records beside PCI functions.
const MAX_DEVICES: usize = 128;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub bar: u8,
}

pub fn find() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    buf[..core::cmp::min(n as usize, MAX_DEVICES)].iter().find_map(found)
}

/// The record as this driver takes it: PVSCSI with a memory BAR of at
/// least 32 KiB, the first such.
pub fn found(r: &DeviceRecord) -> Option<Found> {
    if r.vendor != VMWARE_VENDOR_ID || r.device != PVSCSI_DEVICE_ID || r.bus_kind != BUS_KIND_PCI {
        return None;
    }
    let bar = r.bars[..(r.bar_count as usize).min(6)]
        .iter()
        .position(|b| b.kind == BAR_KIND_MMIO && b.size >= MEM_SPACE)?;
    Some(Found { device_id: r.device_id, bar: bar as u8 })
}
