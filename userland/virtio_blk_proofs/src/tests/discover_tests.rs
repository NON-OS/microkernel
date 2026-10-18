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

//! Discovery finds every virtio-blk function with a register BAR, whatever
//! its interrupt. One with no INTx pin, or a line the firmware left at
//! 0xFF, was skipped, so the MSI-X path never ran for the functions that
//! need it.

use nonos_libc::{present, Bar, DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::discover::find_virtio_blk;

fn record(device: u16, pin: u8, line: u8) -> DeviceRecord {
    let mut r = DeviceRecord::empty();
    (r.device_id, r.vendor, r.device, r.bus_kind) = (9, 0x1AF4, device, BUS_KIND_PCI);
    (r.irq_pin, r.irq_line) = (pin, line);
    r.bars[1] = Bar { kind: BAR_KIND_MMIO, size: 0x1000, ..r.bars[1] };
    r.bars[4] = Bar { kind: BAR_KIND_MMIO, size: 0x4000, ..r.bars[4] };
    r
}

#[test]
fn a_function_with_no_routed_line_is_found_with_no_line() {
    for (device, pin, line) in [(0x1042u16, 0u8, 0u8), (0x1001, 1, 0xFF), (0x1042, 0, 11)] {
        present(&[record(device, pin, line)]);
        let found = find_virtio_blk().expect("an MSI-X-only virtio-blk was skipped");
        assert_eq!(found.intx, None, "{device:#x} pin {pin} line {line}");
        assert_eq!((found.register_bar, found.register_size), (1, 0x1000));
    }
    present(&[record(0x1001, 1, 11)]);
    assert_eq!(find_virtio_blk().map(|f| f.intx), Some(Some(11)));
}

#[test]
fn nothing_else_is_taken_for_a_virtio_blk() {
    let mut other = record(0x1000, 1, 11);
    let mut acpi = record(0x1001, 1, 11);
    acpi.bus_kind = BUS_KIND_PCI + 1;
    let mut no_bar = record(0x1042, 1, 11);
    no_bar.bars = DeviceRecord::empty().bars;
    let mut vendor = record(0x1001, 1, 11);
    vendor.vendor = 0x8086;
    other.device_id = 1;
    for r in [other, acpi, no_bar, vendor] {
        present(&[r]);
        assert!(find_virtio_blk().is_none(), "{:#x}:{:#x} taken", r.vendor, r.device);
    }
    present(&[]);
    assert!(find_virtio_blk().is_none());
}
