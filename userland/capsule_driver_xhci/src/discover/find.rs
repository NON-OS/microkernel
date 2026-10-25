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
//! Finding the xHCI controllers. A laptop often has more than one: the
//! chipset's (Intel PCH, AMD FCH), a Thunderbolt or USB4 controller beside
//! it, and AMD Ryzen APUs carry two of their own. Every one found is
//! returned, the chipset's first, so it is the primary and keeps root port
//! numbers from 1: the ports a boot stick, keyboard and mouse sit on.

use crate::constants::CLASS_USB_HOST_XHCI;
use alloc::vec::Vec;
use nonos_libc::{mk_device_list, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_FDT, BUS_KIND_PCI};

use super::found::Found;
use super::thunderbolt::is_thunderbolt;

const MAX_DEVICES: usize = 64;
const PCI_CLASS_SERIAL_BUS: u8 = 0x0c;
const PCI_SUBCLASS_USB: u8 = 0x03;
const PCI_PROGIF_XHCI: u8 = 0x30;

/// Every usable xHCI controller, the chipset's ahead of any Thunderbolt one,
/// otherwise in the order the bus lists them.
pub fn find_all_xhci() -> Vec<Found> {
    let mut buf = [DeviceRecord::empty(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return Vec::new();
    }
    let count = core::cmp::min(n as usize, MAX_DEVICES);
    let mut out: Vec<Found> = buf[..count].iter().filter_map(usable).collect();
    // A stable sort: equal keys keep the bus order.
    out.sort_by_key(|f| is_thunderbolt(f.vendor, f.device));
    out
}
fn usable(r: &DeviceRecord) -> Option<Found> {
    if r.class != CLASS_USB_HOST_XHCI || !raw_xhci(r) || r.bar_count == 0 {
        return None;
    }
    let bar0 = r.bars[0];
    if bar0.kind != BAR_KIND_MMIO || bar0.size == 0 {
        return None;
    }
    let on_pci = r.bus_kind == BUS_KIND_PCI;
    let spi = if on_pci { 0 } else { r.irq_source };
    Some(Found {
        device_id: r.device_id,
        bar0_size: bar0.size,
        vendor: r.vendor,
        device: r.device,
        on_pci,
        spi,
    })
}
/// A PCI function, or a device-tree controller, which the broker lists with
/// the same class code and no config space.
fn raw_xhci(r: &DeviceRecord) -> bool {
    r.pci_class == PCI_CLASS_SERIAL_BUS
        && (r.bus_kind == BUS_KIND_PCI || r.bus_kind == BUS_KIND_FDT)
        && r.pci_subclass == PCI_SUBCLASS_USB
        && r.pci_progif == PCI_PROGIF_XHCI
}
