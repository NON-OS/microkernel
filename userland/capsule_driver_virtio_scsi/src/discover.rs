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

//! The adapter in the device table: the first virtio-scsi function, modern
//! or transitional; the transport decides later whether it has the modern
//! capabilities.

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI};
use nonos_virtio::{BarInfo, Bars};

use crate::constants::pci::{VIRTIO_SCSI_MODERN_ID, VIRTIO_SCSI_TRANSITIONAL_ID, VIRTIO_VENDOR_ID};

/// The device list holds ACPI and fabricated records beside PCI functions.
const MAX_DEVICES: usize = 128;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub bars: Bars,
}

pub fn find() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    buf[..core::cmp::min(n as usize, MAX_DEVICES)]
        .iter()
        .find(|r| is_match(r))
        .map(|r| Found { device_id: r.device_id, bars: bars(r) })
}

pub fn is_match(r: &DeviceRecord) -> bool {
    r.vendor == VIRTIO_VENDOR_ID
        && (r.device == VIRTIO_SCSI_MODERN_ID || r.device == VIRTIO_SCSI_TRANSITIONAL_ID)
        && r.bus_kind == BUS_KIND_PCI
}

/// The broker's BAR list in the shared transport's terms.
fn bars(r: &DeviceRecord) -> Bars {
    let mut out = [BarInfo::ABSENT; 6];
    for (slot, bar) in out.iter_mut().zip(r.bars.iter()) {
        *slot = match bar.kind {
            BAR_KIND_MMIO => BarInfo::mmio(bar.size),
            BAR_KIND_PIO => BarInfo::io(bar.size),
            _ => BarInfo::ABSENT,
        };
    }
    out
}
