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

//! A platform device the rule admitted, added to the device table in the
//! shape the drivers already read for these controllers: an I2C host as
//! SERIAL with its source clock, an SD/eMMC host as BLOCK with the SD host
//! PCI class, a GPIO controller with its windows and its `_HID` in bar 5.
//! No DMAR ANDD or IVRS ACPI-HID entry is parsed yet, so its DMA is not
//! confined by an IOMMU domain, and the line says so.

use super::super::acpi_i2c::clock::source_clock_hz;
use super::super::class::ids;
use super::super::device::{Bar, BarKind, BusKind, DeviceRecord};
use super::super::table;
use super::platform_paths::{keep, Named};
use super::platform_wire::{Platform, GPIO, HAS_IRQ, I2C};
use crate::sys::serial::Line;

const HID_BAR: usize = 5;

fn record(p: &Platform) -> DeviceRecord {
    let mut bars = [Bar::empty(); 6];
    let n = p.windows.len().min(HID_BAR);
    for (bar, &(base, size)) in bars.iter_mut().zip(&p.windows[..n]) {
        *bar = Bar { base, size, kind: BarKind::Mmio as u8, flags: 0, aux: 0, _pad: [0; 2] };
    }
    let irq = p.irq_flags & HAS_IRQ != 0;
    let mut r = DeviceRecord {
        bus_kind: BusKind::Acpi as u8,
        bar_count: n as u8,
        irq_line: if irq && p.gsiv < 0xFF { p.gsiv as u8 } else { 0xFF },
        irq_pin: u8::from(irq),
        irq_source: if irq { p.gsiv } else { 0 },
        bars,
        ..DeviceRecord::empty()
    };
    match p.class {
        I2C => {
            (r.class, r.pci_class) = (ids::SERIAL, 0x0C);
            r.bars[0].aux = source_clock_hz(&p.hid);
        }
        GPIO => {
            (r.class, r.device) = (ids::GPIO_CTRL, (p.uid & 0xFFFF) as u16);
            r.bars[HID_BAR].base = u64::from_le_bytes(p.hid);
        }
        _ => (r.class, r.pci_class, r.pci_subclass) = (ids::BLOCK, 0x08, 0x05),
    }
    r
}

pub fn add(p: Platform) -> u64 {
    let id = table::register_platform_device(record(&p));
    let name = p.hid.iter().copied().take_while(|c| *c != 0).collect::<alloc::vec::Vec<u8>>();
    let mut l = Line::new();
    l.str(b"[ACPI] platform ").str(&p.path).str(b" ").str(&name).str(b" device=").hex(id);
    l.str(b" windows=")
        .dec(p.windows.len() as u64)
        .str(b"; no ANDD/ACPI-HID entry parsed: unconfined");
    l.end();
    keep(Named { device_id: id, hid: p.hid, uid: p.uid, path: p.path });
    id
}
