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

use crate::constants::pci::{AMD_VENDOR_ID, MMIO_BAR, PCNET_DEVICE_ID, WINDOW_BYTES};

/// The device list holds ACPI and fabricated records beside PCI functions.
const MAX_DEVICES: usize = 128;
const PCI_CLASS_NETWORK: u8 = 0x02;
const PCI_SUBCLASS_ETHERNET: u8 = 0x00;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
}

pub fn find_pcnet() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    let count = core::cmp::min(n as usize, MAX_DEVICES);
    buf[..count].iter().find(|r| is_match(r)).map(|r| Found { device_id: r.device_id })
}

// The interrupt line is not asked about: the driver polls, and UEFI firmware
// often leaves Interrupt Line at 0xFF on a card that works.
fn is_match(r: &DeviceRecord) -> bool {
    let bar = MMIO_BAR as usize;
    r.vendor == AMD_VENDOR_ID
        && r.device == PCNET_DEVICE_ID
        && r.bus_kind == BUS_KIND_PCI
        && r.pci_class == PCI_CLASS_NETWORK
        && r.pci_subclass == PCI_SUBCLASS_ETHERNET
        && (r.bar_count as usize) > bar
        && r.bars[bar].kind == BAR_KIND_MMIO
        && r.bars[bar].size >= WINDOW_BYTES
}
