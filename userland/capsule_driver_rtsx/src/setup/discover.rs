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

//! Every Realtek card reader in the broker's list is named in the log; the
//! first one this driver has a bring-up for is the one it takes.

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::chip::{family, is_linux_reader, register_bar, Family};
use crate::log::{emit, Line};

/// Every function the broker lists (class 0 asks for all of them); a
/// card reader is class FFh, which the broker files under "other".
const MAX_DEVICES: usize = 128;
/// The host registers end at BIER (0x18 + 4).
const MIN_BAR_BYTES: u64 = 0x1C;

#[derive(Clone, Copy, Debug)]
pub struct Found {
    pub device_id: u64,
    pub device: u16,
    pub family: Family,
    pub bar: u32,
    pub bar_size: u64,
}

pub fn find() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    let mut chosen = None;
    for r in &buf[..(n as usize).min(MAX_DEVICES)] {
        if r.bus_kind != BUS_KIND_PCI || !is_linux_reader(r.vendor, r.device, r.pci_class) {
            continue;
        }
        let bar = register_bar(r.device);
        let regs = r.bars[bar as usize];
        let usable = regs.kind == BAR_KIND_MMIO && regs.size >= MIN_BAR_BYTES;
        let fam = family(r.vendor, r.device, r.pci_class);
        let mut line = Line::start();
        line.text(b"reader 10ec:").hex(r.device as u64, 4);
        match (fam, usable, chosen.is_some()) {
            (None, _, _) => line.text(b" has no bring-up here yet; left alone"),
            (Some(_), false, _) => line.text(b" has no usable register BAR; left alone"),
            (Some(_), true, true) => line.text(b" is a second reader; left alone"),
            (Some(f), true, false) => {
                let (device_id, device, bar_size) = (r.device_id, r.device, regs.size);
                chosen = Some(Found { device_id, device, family: f, bar, bar_size });
                line.text(b" taken")
            }
        };
        emit(&mut line);
    }
    chosen
}
