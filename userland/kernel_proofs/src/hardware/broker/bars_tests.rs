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

//! A platform device's record holds its windows and nothing else in a BAR
//! slot, and no slot past them can be mapped.

use super::acpi::platform_record::record;
use super::acpi::platform_wire::{GPIO, I2C, SD};
use super::class::ids;
use super::device::{Bar, BarKind};
use super::mmio::bar_select::{mappable, BarRefusal};
use super::round_trip_tests::platform;

#[test]
fn unused_slots_are_empty_and_refuse_a_mapping() {
    let w = [(0xFED0_C000, 0x1000), (0xFED0_D000, 0x1000)];
    for class in [I2C, SD, GPIO] {
        let r = record(&platform(class, b"INT33FC", &w, b"\\_SB.GPO0"), 100_000_000);
        assert_eq!(r.bar_count, 2);
        for (i, bar) in r.bars.iter().enumerate().skip(2) {
            assert_eq!((bar.base, bar.size, bar.kind), (0, 0, BarKind::None as u8), "slot {i}");
        }
        for i in 0..8u8 {
            let got = mappable(&r.bars, r.bar_count, i).map(|b| (b.base, b.size));
            let want = w.get(usize::from(i)).copied().ok_or(BarRefusal::BadIndex);
            assert_eq!(got, want, "slot {i}");
        }
    }
}

#[test]
fn each_class_in_the_shape_its_drivers_read() {
    let w = [(0xFE10_4000, 0x1000)];
    let i2c = record(&platform(I2C, b"INT3432", &w, b"\\_SB.I2C1"), 100_000_000);
    assert_eq!((i2c.class, i2c.pci_class, i2c.bars[0].aux), (ids::SERIAL, 0x0C, 100_000_000));
    let sd = record(&platform(SD, b"80860F14", &w, b"\\_SB.SDHA"), 0);
    assert_eq!((sd.class, sd.pci_class, sd.pci_subclass), (ids::BLOCK, 0x08, 0x05));
    let gpio = record(&platform(GPIO, b"INT33FC", &w, b"\\_SB.GPO0"), 0);
    assert_eq!((gpio.class, gpio.device, gpio.irq_source), (ids::GPIO_CTRL, 0xFFFF, 0xFFFF_FFFF));
}

#[test]
fn a_slot_inside_the_count_must_still_be_a_memory_window() {
    let mmio = Bar { base: 0x1000, size: 0x1000, kind: BarKind::Mmio as u8, ..Bar::empty() };
    let pio = Bar { kind: BarKind::Pio as u8, ..mmio };
    let empty = Bar { size: 0, ..mmio };
    let bars = [mmio, pio, empty, Bar::empty(), Bar::empty(), Bar::empty()];
    assert!(mappable(&bars, 6, 0).is_ok());
    assert_eq!(mappable(&bars, 6, 1).map(|_| ()), Err(BarRefusal::NotMmio));
    assert_eq!(mappable(&bars, 6, 2).map(|_| ()), Err(BarRefusal::NotMmio), "no bytes");
    assert_eq!(mappable(&bars, 6, 3).map(|_| ()), Err(BarRefusal::NotMmio));
    assert_eq!(mappable(&bars, 0, 0).map(|_| ()), Err(BarRefusal::BadIndex));
}
