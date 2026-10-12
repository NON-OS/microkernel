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
//! Finding a UHCI controller: class 0Ch, subclass 03h, programming
//! interface 00h, with its registers in an I/O BAR4. The first one found is
//! served; a chipset with several serves the two ports of the first.

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_PIO};

use crate::facts::io::IO_SPAN;
use crate::facts::pci::{CLASS_SERIAL, IO_BAR, PROGIF_UHCI, SUBCLASS_USB};

/// The list holds ACPI and fabricated records beside PCI functions.
const MAX_DEVICES: usize = 128;

#[derive(Clone, Copy, Debug)]
pub struct Found {
    pub device_id: u64,
    pub vendor: u16,
    pub device: u16,
}

pub fn find() -> Option<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let n = usize::try_from(n).ok()?.min(MAX_DEVICES);
    buf[..n].iter().find(|r| is_uhci(r)).map(|r| Found {
        device_id: r.device_id,
        vendor: r.vendor,
        device: r.device,
    })
}

fn is_uhci(r: &DeviceRecord) -> bool {
    let bar = r.bars[IO_BAR as usize];
    r.pci_class == CLASS_SERIAL
        && r.pci_subclass == SUBCLASS_USB
        && r.pci_progif == PROGIF_UHCI
        && r.bar_count as u32 > IO_BAR
        && bar.kind == BAR_KIND_PIO
        && bar.size >= IO_SPAN
}
