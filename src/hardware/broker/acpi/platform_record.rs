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

//! The device-table record for an admitted platform device, in the shape
//! the drivers already read for these controllers: an I2C host as SERIAL
//! with its source clock, an SD/eMMC host as BLOCK with the SD host PCI
//! class, a GPIO controller as GPIO_CTRL with its `_UID`. Every BAR slot
//! past the windows is empty: a slot is a physical window or nothing, so
//! no walk over the BARs can take another value for an address. The
//! `_HID` and path live beside the record (`platform_paths`). Pure:
//! `kernel_proofs` mounts it.

use super::super::class::ids;
use super::super::device::{Bar, BarKind, BusKind, DeviceRecord, DEVICE_FLAG_DEPS_UNMET};
use super::platform_wire::{Platform, DEPS_UNMET, GPIO, HAS_IRQ, I2C};

pub fn record(p: &Platform, clock_hz: u32) -> DeviceRecord {
    let mut bars = [Bar::empty(); 6];
    let n = p.windows.len().min(bars.len());
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
    if p.flags & DEPS_UNMET != 0 {
        r.flags |= DEVICE_FLAG_DEPS_UNMET;
    }
    match p.class {
        I2C => {
            (r.class, r.pci_class) = (ids::SERIAL, 0x0C);
            r.bars[0].aux = clock_hz;
        }
        GPIO => (r.class, r.device) = (ids::GPIO_CTRL, (p.uid & 0xFFFF) as u16),
        _ => (r.class, r.pci_class, r.pci_subclass) = (ids::BLOCK, 0x08, 0x05),
    }
    r
}
