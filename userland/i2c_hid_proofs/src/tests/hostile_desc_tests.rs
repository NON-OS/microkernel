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

//! Descriptors past a bound, at each kind of item: none may lay out a field
//! whose end does not fit a report, and none may make a decode overflow.

use super::descs::{touch_pad, TIP};
use crate::hid::{decode_touch, parse_report_descriptor, TouchLayout};

/// X and Y, 16 bits each.
const XY: &[u8] = &[
    0x05, 0x01, 0x75, 0x10, 0x95, 0x01, 0x26, 0xB0, 0x04, 0x09, 0x30, 0x81, 0x02, 0x09, 0x31, 0x81,
    0x02,
];

fn decodes_nothing_wild(l: &TouchLayout) {
    for n in 0..300 {
        let _ = decode_touch(&vec![1u8; n], l);
    }
}

#[test]
fn a_report_size_of_four_billion_bits_lays_out_nothing() {
    // Report Size (0xFFFFFFFF) in a four-byte item, ahead of the tip.
    let huge: &[u8] = &[0x77, 0xFF, 0xFF, 0xFF, 0xFF, 0x95, 0x01, 0x09, 0x30, 0x81, 0x02];
    let l = parse_report_descriptor(&touch_pad(&[huge, TIP, XY]));
    assert!(!l.is_absolute_touch(), "laid out a touch report past 2^32 bits");
    decodes_nothing_wild(&l);
}

#[test]
fn a_report_count_of_four_billion_lays_out_nothing() {
    let huge: &[u8] = &[0x75, 0x01, 0x97, 0xFF, 0xFF, 0xFF, 0xFF, 0x81, 0x03];
    let l = parse_report_descriptor(&touch_pad(&[huge, TIP, XY]));
    assert!(!l.is_absolute_touch(), "laid out fields past four billion constant bits");
    decodes_nothing_wild(&l);
}
