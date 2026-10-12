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

//! Hostile report descriptors, by name: items cut short, long items, and
//! too many items. Each is refused whole, never read past, never panics.
//! The bounds on sizes and structure are `bound_tests` and `nesting_tests`.

use super::vectors::QEMU_TABLET;
use super::{layout, vectors, vectors_more};
use crate::descriptors::report::parse;

/// The ends of the items of `desc`, walked here apart from the driver.
fn item_ends(desc: &[u8]) -> Vec<usize> {
    let (mut at, mut ends) = (0, Vec::new());
    while at < desc.len() {
        at += match desc[at] {
            0xfe => 3 + usize::from(desc[at + 1]),
            p => 1 + [0, 1, 2, 4][usize::from(p & 3)],
        };
        ends.push(at);
    }
    ends
}

#[test]
fn a_descriptor_cut_inside_an_item_is_refused() {
    let all = [vectors::QEMU_TABLET, vectors::GAMEPAD, vectors::MEDIA_KEYS, vectors::KEYBOARD];
    let more = [vectors_more::ID_MOUSE, vectors_more::TOUCHSCREEN, vectors_more::LONG_MOUSE];
    for desc in all.into_iter().chain(more) {
        let ends = item_ends(desc);
        assert_eq!(ends.last(), Some(&desc.len()));
        for cut in 0..desc.len() {
            let got = parse(&desc[..cut]);
            if !ends.contains(&cut) && cut != 0 {
                assert!(got.is_err(), "cut at {cut} inside an item");
            }
        }
    }
}

#[test]
fn a_long_item_is_stepped_over_and_one_cut_short_is_refused() {
    let mut long = vec![0xfe, 3, 0x10, 1, 2, 3];
    long.extend_from_slice(QEMU_TABLET);
    assert_eq!(layout(&long), layout(QEMU_TABLET));
    let mut empty = vec![0xfe, 0, 0x10];
    empty.extend_from_slice(QEMU_TABLET);
    assert_eq!(layout(&empty), layout(QEMU_TABLET));
    for tail in [&[0xfe][..], &[0xfe, 3], &[0xfe, 3, 0x10, 1, 2], &[0xfe, 255, 0]] {
        let d = [QEMU_TABLET, tail].concat();
        assert!(parse(&d).is_err(), "{tail:02x?}");
    }
}

#[test]
fn an_empty_or_all_padding_descriptor_has_nothing_to_bind() {
    for d in [vec![], vec![0x00; 512]] {
        let l = layout(&d);
        assert!(!l.bindable(8) && l.longest == 0, "{} bytes", d.len());
    }
    assert!(parse(&[0x00; 513]).is_err(), "513 items is past the bound");
}
