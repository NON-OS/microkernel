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

//! Discovery against the broker's list: an 8254x is taken only with a BAR0
//! that holds every register the bring-up writes, so no store lands past the
//! mapped window whatever size a spoofed or misprogrammed record reports.

use nonos_libc::{given_back, map_at_most, present, Bar, DeviceRecord};
use nonos_libc::{BAR_KIND_MMIO, BAR_KIND_PIO, BUS_KIND_PCI};

use crate::discover::{find_e1000, Found};
use crate::setup::mmio::map;

fn qemu_e1000(kind: u8, size: u64) -> DeviceRecord {
    let mut r = DeviceRecord { device_id: 7, bus_kind: BUS_KIND_PCI, ..DeviceRecord::empty() };
    (r.vendor, r.device, r.pci_class, r.pci_subclass) = (0x8086, 0x100E, 0x02, 0x00);
    r.bar_count = 1;
    r.bars[0] = Bar { size, kind };
    r
}

#[test]
fn a_full_register_window_is_taken() {
    for size in [0x6000, 0x2_0000] {
        present(&[qemu_e1000(BAR_KIND_MMIO, size)]);
        let found = find_e1000().expect("an 8254x with its whole window");
        assert_eq!((found.device_id, found.bar0_size), (7, size));
    }
}

#[test]
fn a_window_short_of_the_registers_written_is_not() {
    for size in [1, 0x1000, 0x5404, 0x5FFF] {
        present(&[qemu_e1000(BAR_KIND_MMIO, size)]);
        assert!(find_e1000().is_none(), "a {size:#x}-byte BAR0 was taken");
    }
    present(&[qemu_e1000(BAR_KIND_PIO, 0x2_0000)]);
    assert!(find_e1000().is_none(), "a port BAR0 is not the register window");
}

#[test]
fn a_bar_mapped_shorter_than_the_registers_gives_the_claim_back() {
    map_at_most(0x1000);
    let before = given_back().len();
    let err = map(Found { device_id: 7, bar0_size: 0x2_0000 }, 1).err();
    assert_eq!(err, Some("BAR0 mapped short"));
    assert_eq!(given_back()[before..], [("device_release", 7)]);
}

#[test]
fn a_bar_size_at_the_top_of_the_range_is_rounded_without_wrapping() {
    present(&[qemu_e1000(BAR_KIND_MMIO, u64::MAX)]);
    let found = find_e1000().expect("a window past every register");
    let mapped = map(found, 1).expect("asked for whole, not wrapped to nothing");
    assert_eq!(mapped.length, !0xFFF, "the top page, rounded down");
}
