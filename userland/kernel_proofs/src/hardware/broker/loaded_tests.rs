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

//! A table AML loads at run time (F-93): only whole tables pass, a record
//! of one is marked LOADED, the same bytes again add nothing, a full list
//! refuses and keeps what it had, and a load moves the digest.

use super::acpi::loaded_rule::{admit, checked, Admit, LOADED};
use super::acpi::table_measure::{digest, Measured};

/// A table with `body`, its length and checksum set as firmware sets them.
fn table(sig: &[u8; 4], body: &[u8]) -> Vec<u8> {
    let mut t = vec![0u8; 36];
    t[..4].copy_from_slice(sig);
    t.extend_from_slice(body);
    let len = t.len() as u32;
    t[4..8].copy_from_slice(&len.to_le_bytes());
    let sum = t.iter().fold(0u8, |s, b| s.wrapping_add(*b));
    t[9] = 0u8.wrapping_sub(sum);
    t
}

#[test]
fn only_a_whole_table_passes() {
    let t = table(b"SSDT", b"\x10\x05\x5c\x2e");
    assert_eq!(checked(&t), Some(*b"SSDT"));
    assert_eq!(checked(&t[..35]), None, "shorter than a header");
    assert_eq!(checked(&t[..t.len() - 1]), None, "shorter than its length");
    let mut longer = t.clone();
    longer.push(0);
    assert_eq!(checked(&longer), None, "longer than its length");
    for at in 0..t.len() {
        let mut bad = t.clone();
        bad[at] ^= 0x01;
        assert_eq!(checked(&bad), None, "a byte changed at {at}");
    }
}

#[test]
fn the_list_grows_by_new_loads_and_refuses_when_full() {
    let (a, b) = (table(b"SSDT", b"a"), table(b"SSDT", b"b"));
    let (ma, mb) = (Measured::of(*b"SSDT", LOADED, &a), Measured::of(*b"SSDT", LOADED, &b));
    let firmware = Measured::of(*b"SSDT", 0, &a);
    assert_ne!(ma, firmware, "a run-time load never reads as the root's table");
    let mut list = vec![firmware];
    let before = digest(&list);
    assert_eq!(admit(&mut list, ma, 3), Admit::Added);
    assert_ne!(digest(&list), before, "a load moves the digest");
    assert_eq!(admit(&mut list, ma, 3), Admit::Known, "loaded again: nothing added");
    assert_eq!(admit(&mut list, mb, 3), Admit::Added);
    let full = list.clone();
    let mc = Measured::of(*b"SSDT", LOADED, &table(b"SSDT", b"c"));
    assert_eq!(admit(&mut list, mc, 3), Admit::Full);
    assert_eq!(list, full, "a full list keeps what it had");
    assert_eq!(admit(&mut list, mb, 3), Admit::Known, "a known table is no refusal");
}
