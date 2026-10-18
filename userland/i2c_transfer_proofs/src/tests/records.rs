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

//! Device records as src/hardware/broker builds them: an LPSS PCI function
//! as table/pci_record.rs does, a platform controller as acpi_i2c/record.rs
//! does, a pad as acpi_i2c/hid_record.rs does. PCI ids are enumeration
//! indices from 0; a platform record takes the highest id so far plus one.
//! A pad's bars[0] is kind None: its base is the host's device_id, u64::MAX
//! for none, its size the host's NameSeg; pci_progif holds "I2Cn" as n+1.

use nonos_libc::{Bar, DeviceRecord, BAR_FLAG_MEM64, BAR_KIND_MMIO, BUS_KIND_ACPI, BUS_KIND_PCI};

use crate::bench::GEMINI_LAKE_HZ;

/// Where an LPSS function's window sits, both halves set; a platform one's.
pub const WINDOW: u64 = 0x0000_0002_9180_1000;
pub const PLATFORM_WINDOW: u64 = 0xFEDC_3000;
/// The bench controller's "I2C0" as pci_progif carries it.
pub const I2C0: u8 = 1;
/// The kernel's I2C-HID and platform serial-bus classes.
const CLASS_I2C_HID: u32 = 0x0041;
const CLASS_SERIAL: u32 = 0x0060;
const EMPTY: DeviceRecord = DeviceRecord::empty();

fn bars(bar0: Bar) -> [Bar; 6] {
    let mut bars = [Bar::empty(); 6];
    bars[0] = bar0;
    bars
}

fn window(base: u64, aux: u32) -> [Bar; 6] {
    let flags = BAR_FLAG_MEM64;
    bars(Bar { base, size: 0x1000, aux, kind: BAR_KIND_MMIO, flags, _pad: [0; 2] })
}

/// Gemini Lake's I2C0 (8086:31AC) at PCI id `id`.
pub fn lpss(id: u64) -> DeviceRecord {
    let (vendor, device, bars) = (0x8086, 0x31AC, window(WINDOW, 0));
    let r = DeviceRecord { device_id: id, bus_kind: BUS_KIND_PCI, vendor, device, bars, ..EMPTY };
    DeviceRecord { pci_class: 0x11, pci_subclass: 0x80, bar_count: 1, ..r }
}

/// A platform I2C controller (AMD's AMDI0010, say), its clock in aux.
pub fn platform(id: u64) -> DeviceRecord {
    let (class, bars) = (CLASS_SERIAL, window(PLATFORM_WINDOW, GEMINI_LAKE_HZ));
    let r = DeviceRecord { device_id: id, bus_kind: BUS_KIND_ACPI, class, bars, ..EMPTY };
    DeviceRecord { pci_class: 0x0c, bar_count: 1, ..r }
}

/// A pad at `addr` (descriptor 0x0001) on `host`, named "I2Cn" by `progif`.
pub fn hid(host: u64, progif: u8, addr: u8) -> DeviceRecord {
    let name = u64::from(u32::from_le_bytes(*b"I2C0"));
    let bars = bars(Bar { base: host, size: name, ..Bar::empty() });
    let r = DeviceRecord { bus_kind: BUS_KIND_ACPI, class: CLASS_I2C_HID, bars, ..EMPTY };
    DeviceRecord { vendor: addr.into(), device: 0x0001, pci_progif: progif, ..r }
}

/// The same pad declaring a ConnectionSpeed of 100 kHz.
pub fn slow(mut r: DeviceRecord) -> DeviceRecord {
    r.bars[0].aux = 100_000;
    r
}
