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

use crate::hardware::broker::device::{Bar, BarKind, BusKind, DeviceRecord};

use super::register::Described;

/// The record a capsule lists: no vendor, the interface in `device`, its
/// windows as memory BARs in tree order, and the SPI as `irq_source`.
pub(super) fn record(d: &Described) -> DeviceRecord {
    let (class, pci_class, pci_subclass, pci_progif) = d.interface.class();
    let mut bars = [Bar::empty(); 6];
    for (bar, (base, size)) in bars.iter_mut().zip(&d.windows[..d.window_count]) {
        *bar = Bar { base: *base, size: *size, kind: BarKind::Mmio as u8, ..Bar::empty() };
    }
    DeviceRecord {
        bus_kind: BusKind::Fdt as u8,
        pci_class,
        pci_subclass,
        pci_progif,
        class,
        device: d.interface as u16,
        bar_count: d.window_count.min(6) as u8,
        irq_source: d.irq.map_or(0, |(intid, _)| intid),
        bars,
        ..DeviceRecord::empty()
    }
}
