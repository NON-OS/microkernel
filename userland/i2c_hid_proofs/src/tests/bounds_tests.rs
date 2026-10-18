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

//! Each bound of the walk, at it and one past it: the touch pad is laid out
//! at the bound and the whole descriptor refused past it.

use super::descs::{touch_pad, TIP};
use crate::hid::parse_report_descriptor;

/// X and Y of 16 bits on the Generic Desktop page.
pub(super) const XY: &[u8] = &[
    0x05, 0x01, 0x75, 0x10, 0x95, 0x01, 0x26, 0xB0, 0x04, 0x09, 0x30, 0x81, 0x02, 0x09, 0x31, 0x81,
    0x02,
];

pub(super) fn lays_out(extra: &[u8]) -> bool {
    parse_report_descriptor(&touch_pad(&[TIP, XY, extra])).is_absolute_touch()
}

#[test]
fn an_item_cut_short_by_the_end_refuses_the_descriptor() {
    let mut d = touch_pad(&[TIP, XY]);
    assert!(parse_report_descriptor(&d).is_absolute_touch());
    d.extend([0x26, 0xFF]);
    assert!(!parse_report_descriptor(&d).is_absolute_touch(), "a two-byte item with one byte");
    let mut long = touch_pad(&[TIP, XY]);
    long.extend([0xFE, 0x05, 0x00, 1, 2]);
    assert!(!parse_report_descriptor(&long).is_absolute_touch(), "a cut long item");
}

#[test]
fn a_whole_long_item_is_stepped_over() {
    assert!(lays_out(&[0xFE, 0x02, 0x00, 1, 2]));
}

#[test]
fn push_and_pop_are_held_to_four_saved_states() {
    assert!(lays_out(&[[0xA4; 4], [0xB4; 4]].concat()));
    assert!(!lays_out(&[[0xA4; 5], [0xB4; 5]].concat()), "a fifth Push");
    assert!(!lays_out(&[0xB4]), "a Pop with nothing pushed");
}

#[test]
fn collections_are_held_to_eight_deep_and_must_close() {
    let nest = |k: usize| [[0xA1, 0x02].repeat(k), vec![0xC0; k]].concat();
    assert!(lays_out(&nest(7)), "eight deep with the application");
    assert!(!lays_out(&nest(8)), "nine deep");
    assert!(!lays_out(&[0xA1, 0x02]), "a collection left open");
    assert!(!lays_out(&[0xC0]), "an End Collection with none open");
}
