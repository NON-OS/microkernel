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

//! The Intel graphics function, if the device list has one this driver's
//! id tables cover: display class, vendor 8086, BAR0 (GTTMMADR: registers
//! and the GGTT) and BAR2 (GMADR, the aperture firmware's picture is read
//! through) both memory BARs.

use nonos_intel_display::ids::{family, Family};
use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

/// The broker's display class, and PCI's base class for display.
const CLASS_DISPLAY: u32 = 0x0030;
const PCI_CLASS_DISPLAY: u8 = 0x03;
const MAX_DEVICES: usize = 128;

#[derive(Debug, Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub family: Family,
    pub bar0_size: u64,
    pub bar2_size: u64,
}

pub fn find() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(CLASS_DISPLAY, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    buf[..(n as usize).min(MAX_DEVICES)].iter().find_map(take)
}

fn take(r: &DeviceRecord) -> Option<Found> {
    let pci = r.bus_kind == BUS_KIND_PCI && r.pci_class == PCI_CLASS_DISPLAY;
    let family = family(r.vendor, r.device).filter(|_| pci && r.bar_count > 2)?;
    let (bar0, bar2) = (r.bars[0], r.bars[2]);
    let mmio = bar0.kind == BAR_KIND_MMIO && bar2.kind == BAR_KIND_MMIO;
    (mmio && bar0.size > 0 && bar2.size > 0).then_some(Found {
        device_id: r.device_id,
        family,
        bar0_size: bar0.size,
        bar2_size: bar2.size,
    })
}
