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

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI};

use crate::constants::pci::{layout_of, Layout};

/// The device list holds ACPI and fabricated records beside PCI functions.
const MAX_DEVICES: usize = 128;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub layout: Layout,
    pub fb_bytes: u64,
    pub fifo_bytes: u64,
}

pub fn find_svga() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    buf[..core::cmp::min(n as usize, MAX_DEVICES)].iter().find_map(found)
}

/// An SVGA II function with its ports in I/O space and its framebuffer and
/// FIFO in memory, in the order its ids call for.
pub fn found(r: &DeviceRecord) -> Option<Found> {
    let l = layout_of(r.vendor, r.device)?;
    let kind = |i: usize| r.bars[i].kind;
    let whole = r.bus_kind == BUS_KIND_PCI
        && r.bar_count > 2
        && kind(l.port as usize) == BAR_KIND_PIO
        && kind(l.fb as usize) == BAR_KIND_MMIO
        && kind(l.fifo as usize) == BAR_KIND_MMIO;
    let (fb, fifo) = (r.bars[l.fb as usize].size, r.bars[l.fifo as usize].size);
    whole.then_some(Found { device_id: r.device_id, layout: l, fb_bytes: fb, fifo_bytes: fifo })
}
