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

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::constants::pci::{
    PT_BAR, PT_BYTES, VD_BAR, VD_BYTES, VMWARE_VENDOR_ID, VMXNET3_DEVICE_ID,
};

/// The device list holds ACPI and fabricated records beside PCI functions.
const MAX_DEVICES: usize = 128;
const PCI_CLASS_NETWORK: u8 = 0x02;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
}

pub fn find_vmxnet3() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    let count = core::cmp::min(n as usize, MAX_DEVICES);
    buf[..count].iter().find(|r| is_match(r)).map(|r| Found { device_id: r.device_id })
}

fn is_match(r: &DeviceRecord) -> bool {
    let bar = |i: u32, bytes: u64| {
        let b = r.bars[i as usize];
        (r.bar_count as u32) > i && b.kind == BAR_KIND_MMIO && b.size >= bytes
    };
    r.vendor == VMWARE_VENDOR_ID
        && r.device == VMXNET3_DEVICE_ID
        && r.bus_kind == BUS_KIND_PCI
        && r.pci_class == PCI_CLASS_NETWORK
        && bar(PT_BAR, PT_BYTES)
        && bar(VD_BAR, VD_BYTES)
}
