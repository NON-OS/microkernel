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
//! class, a GPIO controller as GPIO_CTRL with its `_UID`. The interrupt
//! carries its `_CRS` trigger and polarity in the record's flags; a GSI
//! past a byte is refused, never cut to fit `irq_line`. Every BAR slot
//! past the windows is empty: a slot is a physical window or nothing, so
//! no walk over the BARs can take another value for an address. The
//! `_HID` and path live beside the record (`platform_paths`). Pure:
//! `kernel_proofs` mounts it.

use super::super::class::ids;
use super::super::device::{Bar, BarKind, BusKind, DeviceRecord, DEVICE_FLAG_DEPS_UNMET};
use super::super::device::{DEVICE_FLAG_IRQ_ACTIVE_LOW, DEVICE_FLAG_IRQ_EDGE};
use super::platform_wire::{Platform, ACTIVE_LOW, DEPS_UNMET, EDGE, GPIO, HAS_IRQ, I2C};

/// An interrupt whose GSI fits `irq_line` (0xFF is "none").
pub fn usable_irq(p: &Platform) -> bool {
    p.irq_flags & HAS_IRQ != 0 && p.gsiv < 0xFF
}

pub fn record(p: &Platform, clock_hz: u32) -> DeviceRecord {
    let mut bars = [Bar::empty(); 6];
    let n = p.windows.len().min(bars.len());
    for (bar, &(base, size)) in bars.iter_mut().zip(&p.windows[..n]) {
        *bar = Bar { base, size, kind: BarKind::Mmio as u8, flags: 0, aux: 0, _pad: [0; 2] };
    }
    let irq = usable_irq(p);
    let mut r = DeviceRecord {
        bus_kind: BusKind::Acpi as u8,
        bar_count: n as u8,
        irq_line: if irq { p.gsiv as u8 } else { 0xFF },
        irq_pin: u8::from(irq),
        irq_source: if irq { p.gsiv } else { 0 },
        bars,
        ..DeviceRecord::empty()
    };
    for (on, bit) in [
        (p.flags & DEPS_UNMET != 0, DEVICE_FLAG_DEPS_UNMET),
        (irq && p.irq_flags & EDGE != 0, DEVICE_FLAG_IRQ_EDGE),
        (irq && p.irq_flags & ACTIVE_LOW != 0, DEVICE_FLAG_IRQ_ACTIVE_LOW),
    ] {
        r.flags |= if on { bit } else { 0 };
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
