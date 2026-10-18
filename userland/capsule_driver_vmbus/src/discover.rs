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

//! The VMBus root in the device table: the record the kernel registers
//! once the Hyper-V SynIC is up.

use nonos_libc::{mk_device_list, DeviceRecord, BUS_KIND_VIRT, MK_VMBUS_DEVICE, MK_VMBUS_VENDOR};

/// The device list holds ACPI and fabricated records beside PCI functions.
const MAX_DEVICES: usize = 128;

pub fn find() -> Option<u64> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    buf[..core::cmp::min(n as usize, MAX_DEVICES)].iter().find_map(found)
}

pub fn found(r: &DeviceRecord) -> Option<u64> {
    (r.bus_kind == BUS_KIND_VIRT && r.vendor == MK_VMBUS_VENDOR && r.device == MK_VMBUS_DEVICE)
        .then_some(r.device_id)
}
