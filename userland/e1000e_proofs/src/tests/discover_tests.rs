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

//! Discovery and the BAR map: an e1000e part is taken only with a BAR0 that
//! holds every register the bring-up writes, and a BAR mapped shorter than
//! that gives the claim back, so no store lands past the window whatever
//! size a spoofed or misprogrammed record reports or the broker maps.

use nonos_libc::{given_back, map_at_most, present, Bar, DeviceRecord};
use nonos_libc::{BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::constants::regs::BAR0_MIN_BYTES;
use crate::constants::Family;
use crate::discover::{find_e1000e, Found};
use crate::setup::mmio::map;

fn i219(size: u64) -> DeviceRecord {
    let mut r = DeviceRecord { device_id: 7, bus_kind: BUS_KIND_PCI, ..DeviceRecord::empty() };
    (r.vendor, r.device, r.pci_class, r.pci_subclass) = (0x8086, 0x15BB, 0x02, 0x00);
    r.bar_count = 1;
    r.bars[0] = Bar { size, kind: BAR_KIND_MMIO };
    r
}

fn found(bar0_size: u64) -> Found {
    Found { device_id: 7, pci_device: 0x15BB, family: Family::PchCnp, bar0_size }
}

#[test]
fn a_full_register_window_is_taken_and_a_short_one_is_not() {
    for size in [BAR0_MIN_BYTES, 0x2_0000] {
        present(&[i219(size)]);
        assert_eq!(find_e1000e().map(|f| (f.device_id, f.bar0_size)), Some((7, size)));
    }
    for size in [1, 0x1000, BAR0_MIN_BYTES - 1] {
        present(&[i219(size)]);
        assert!(find_e1000e().is_none(), "a {size:#x}-byte BAR0 was taken");
    }
}

#[test]
fn a_bar_mapped_shorter_than_the_registers_gives_the_claim_back() {
    map_at_most(BAR0_MIN_BYTES - 0x1000);
    let before = given_back().len();
    assert_eq!(map(found(0x2_0000), 1).err(), Some("BAR0 mapped short"));
    assert_eq!(given_back()[before..], [("device_release", 7)]);
    map_at_most(BAR0_MIN_BYTES);
    assert_eq!(map(found(0x2_0000), 1).map(|m| m.length), Ok(BAR0_MIN_BYTES));
}

#[test]
fn a_bar_size_at_the_top_of_the_range_is_rounded_without_wrapping() {
    let mapped = map(found(u64::MAX), 1).expect("asked for whole, not wrapped to nothing");
    assert_eq!(mapped.length, !0xFFF, "the top page, rounded down");
}
