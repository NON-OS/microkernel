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

//! Finding an EHCI controller: class 0Ch, subclass 03h, programming
//! interface 20h, with its registers in memory BAR0. Every one found is
//! served, as one host (server/many): an Intel ICH or PCH has two.

use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO};

use crate::facts::pci::{CLASS_SERIAL, MMIO_BAR, MMIO_MIN, PROGIF_EHCI, SUBCLASS_USB};
use crate::server::many::MAX_HC;

/// The list holds ACPI and fabricated records beside PCI functions.
const MAX_DEVICES: usize = 128;

#[derive(Clone, Copy, Debug)]
pub struct Found {
    pub device_id: u64,
    pub vendor: u16,
    pub device: u16,
    /// BAR0's size, for the grant.
    pub size: u64,
}

/// Every EHCI function, in the order the list gives them, MAX_HC at most.
pub fn find_all() -> [Option<Found>; MAX_HC] {
    let mut out = [None; MAX_HC];
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    let n = usize::try_from(n).unwrap_or(0).min(MAX_DEVICES);
    for (slot, r) in out.iter_mut().zip(buf[..n].iter().filter(|r| is_ehci(r))) {
        let size = r.bars[MMIO_BAR as usize].size;
        *slot = Some(Found { device_id: r.device_id, vendor: r.vendor, device: r.device, size });
    }
    out
}

pub fn find() -> Option<Found> {
    find_all()[0]
}

fn is_ehci(r: &DeviceRecord) -> bool {
    let bar = r.bars[MMIO_BAR as usize];
    r.pci_class == CLASS_SERIAL
        && r.pci_subclass == SUBCLASS_USB
        && r.pci_progif == PROGIF_EHCI
        && r.bar_count as u32 > MMIO_BAR
        && bar.kind == BAR_KIND_MMIO
        && bar.size >= MMIO_MIN
}
