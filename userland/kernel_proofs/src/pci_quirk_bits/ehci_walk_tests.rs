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
//! The capability walk under a list a device or a fault can make: looped,
//! pointing into the header, or absent, each ending without hanging; and
//! an id byte that is part of another capability's body.

use super::ehci_listed_tests::{allows, space};

#[test]
fn a_looped_capability_list_ends_and_still_closes_its_entries() {
    // 0x50 -> 0x60 -> 0x50 again; legacy support at 0x80.
    let c = space(&[(0x50, 0x01, 0x60), (0x60, 0x0A, 0x50)], &[(0x80, 0x01)]);
    assert_eq!(allows(&c, 0x54), None, "PMCSR on the loop");
    assert_eq!(allows(&c, 0x84), Some(0xE03F));
    let own = space(&[(0x50, 0x01, 0x50)], &[(0x80, 0x01)]);
    assert_eq!(allows(&own, 0x54), None, "an entry naming itself");
    assert_eq!(allows(&own, 0x82), Some(0x0100));
}

#[test]
fn a_pointer_into_the_header_or_a_full_list_ends_the_walk() {
    let c = space(&[(0x50, 0x01, 0x10)], &[(0x80, 0x01)]);
    assert_eq!(allows(&c, 0x54), None);
    assert_eq!(allows(&c, 0x84), Some(0xE03F), "0x10 is no capability");
    // Every dword an entry naming the next: the walk stops at its bound.
    let chain: Vec<(u8, u8, u8)> = (0x40..0xFCu8).step_by(4).map(|o| (o, 0x01, o + 4)).collect();
    let c = space(&chain, &[]);
    for offset in (0x40..0x100u32).step_by(2) {
        assert_eq!(allows(&c, offset), None, "{offset:#x}");
    }
}

#[test]
fn with_no_list_and_after_a_capability_of_unknown_length() {
    let mut c = space(&[], &[(0x50, 0x01)]);
    c[0x06] = 0;
    assert_eq!(allows(&c, 0x54), Some(0xE03F), "no list: nothing to rule out");
    // An unknown kind reaches to the end of the header space; the rule
    // refuses rather than guess where it stops.
    let c = space(&[(0x50, 0x0D, 0x00)], &[(0x80, 0x01)]);
    assert_eq!(allows(&c, 0x84), None);
}

#[test]
fn an_id_byte_inside_another_capability_is_not_legacy_support() {
    // A 64-bit MSI at 0x50 whose address byte at 0x54 reads 01h.
    let c = space(&[(0x50, 0x05, 0x00)], &[(0x52, 0x80), (0x54, 0x01)]);
    for offset in [0x56, 0x58, 0x5A] {
        assert_eq!(allows(&c, offset), None, "{offset:#x}");
    }
    // An id byte past the MSI's 14 bytes is the controller's own.
    let c = space(&[(0x50, 0x05, 0x00)], &[(0x52, 0x80), (0x60, 0x01)]);
    assert_eq!(allows(&c, 0x64), Some(0xE03F));
}
