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

//! The lines a function arriving or leaving after the boot scan is logged
//! with: where it is, its id, and what a driver may do with it now.

use crate::drivers::pci::types::PciDevice;
use crate::sys::serial::Line;

pub(super) fn say(at: (u8, u8, u8), found: &PciDevice, id: u64, disabled: bool, fenced: Option<u32>) {
    let (bus, dev, func) = at;
    let iommu: &[u8] = if crate::memory::iommu::capabilities().enforcing {
        b"; IOMMU denies its DMA until it is claimed"
    } else {
        b"; no IOMMU in service, unconfined like every device"
    };
    let mut l = Line::new();
    l.str(b"[ACPI] pci ").hex(u64::from(bus)).str(b":").hex(u64::from(dev)).str(b".");
    l.dec(u64::from(func)).str(b" answered after _PS0: ").hex(u64::from(found.vendor_id));
    l.str(b":").hex(u64::from(found.device_id)).str(b" device=").hex(id);
    match fenced {
        Some(pid) => l.str(b"; registers in a window pid ").dec(u64::from(pid)).str(b" may map"),
        None if disabled => l.str(b"; BARs unassigned, marked unusable"),
        None => l.str(iommu),
    };
    l.end();
}

/// `revoked`: the holder and how many irq, dma, pio and mmio grants it lost.
pub(super) fn left(at: (u8, u8, u8), id: u64, revoked: Option<(u32, [usize; 4])>) {
    let (bus, dev, func) = at;
    let mut l = Line::new();
    l.str(b"[ACPI] pci ").hex(u64::from(bus)).str(b":").hex(u64::from(dev)).str(b".");
    l.dec(u64::from(func)).str(b" left: device=").hex(id);
    let Some((pid, [i, d, p, m])) = revoked else {
        l.str(b" unheld").end();
        return;
    };
    l.str(b" holder ").dec(u64::from(pid)).str(b" revoked irq ").dec(i as u64);
    l.str(b" dma ").dec(d as u64).str(b" pio ").dec(p as u64).str(b" mmio ").dec(m as u64);
    l.end();
}
