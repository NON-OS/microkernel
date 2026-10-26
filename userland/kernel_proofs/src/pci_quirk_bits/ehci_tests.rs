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
//! EHCI's legacy support, at the offset its id says, and nowhere else.

use super::quirk_bits::{only, writable, Ident, Route};

/// An EHCI with USBLEGSUP at 0x68: the bytes below an offset are 0x01 only
/// where 0x68 lies 2, 4 or 6 below it.
fn at(offset: u32) -> Ident {
    let byte = |d: u32| if offset.checked_sub(d) == Some(0x68) { 0x01 } else { 0x00 };
    Ident {
        vendor: 0x8086,
        class: 0x0C,
        subclass: 0x03,
        prog_if: 0x20,
        pcie_cap: None,
        before: [byte(2), byte(4), byte(6)],
        device: 0,
        route: Route::NONE,
    }
}

#[test]
fn the_os_owned_semaphore_and_the_smi_bits_are_writable_where_the_capability_is() {
    assert_eq!(writable(&at(0x6A), 0x6A), Some(0x0100), "OS owned, bit 24");
    assert_eq!(writable(&at(0x6C), 0x6C), Some(0xE03F), "SMI enables");
    assert_eq!(writable(&at(0x6E), 0x6E), Some(0xE000), "SMI status, cleared by a one");
    // BIOS still owning (bit 16 = bit 0 here): the OS sets its bit only.
    assert!(only(0x0100, 0x0101, 0x0001));
    assert!(!only(0x0100, 0x0100, 0x0001), "the BIOS-owned bit is not the driver's");
    assert!(only(0xE03F, 0x0000, 0xA03F), "every SMI off");
}

#[test]
fn the_capability_id_other_offsets_and_other_hosts_stay_closed() {
    assert_eq!(writable(&at(0x68), 0x68), None, "the id and next pointer");
    for offset in [0x04, 0x10, 0x60, 0x6B, 0x70, 0x72, 0xC0] {
        assert_eq!(writable(&at(offset), offset), None, "{offset:#x}");
    }
    let near_but_low = Ident { before: [0x01; 3], ..at(0x30) };
    assert_eq!(writable(&near_but_low, 0x30), None, "below 0x40 is the header");
    for prog_if in [0x00, 0x10, 0x30] {
        let other = Ident { prog_if, ..at(0x6A) };
        assert_eq!(writable(&other, 0x6A), None, "{prog_if:#x}");
    }
}
