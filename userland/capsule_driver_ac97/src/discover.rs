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

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_PIO, BUS_KIND_PCI};

use crate::constants::pci::{
    ICH_AC97_DEVICE_ID, INTEL_VENDOR_ID, NABM_BAR, NABM_MIN_PORTS, NAM_BAR, NAM_MIN_PORTS,
};

/// The device list holds ACPI and fabricated records beside PCI functions.
const MAX_DEVICES: usize = 128;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
}

pub fn find_ac97() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    let r = buf[..core::cmp::min(n as usize, MAX_DEVICES)].iter().find(|r| is_match(r))?;
    Some(Found { device_id: r.device_id })
}

/// The controller with both windows in I/O space and large enough for every
/// register read.
pub fn is_match(r: &DeviceRecord) -> bool {
    let io = |bar: u8, min: u16| {
        let b = r.bars[bar as usize];
        b.kind == BAR_KIND_PIO && b.size >= min as u64
    };
    r.vendor == INTEL_VENDOR_ID
        && r.device == ICH_AC97_DEVICE_ID
        && r.bus_kind == BUS_KIND_PCI
        && r.bar_count > NABM_BAR
        && io(NAM_BAR, NAM_MIN_PORTS)
        && io(NABM_BAR, NABM_MIN_PORTS)
}
