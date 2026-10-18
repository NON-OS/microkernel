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

//! A coordinate's logical range: the minimum counts, and a negative minimum
//! makes the field signed, each bound read at its own item size.

use super::descs::{touch_pad, TIP};
use crate::hid::{decode_touch, parse_report_descriptor};

/// X then Y, 16 bits each, with `range` (Logical Minimum and Maximum items)
/// in force.
fn xy(range: &[u8]) -> Vec<u8> {
    let xy: &[u8] = &[0x75, 0x10, 0x95, 0x01, 0x09, 0x30, 0x81, 0x02, 0x09, 0x31, 0x81, 0x02];
    touch_pad(&[TIP, &[0x05, 0x01], range, xy])
}

/// The report: tip down, then X and Y as given.
fn report(x: u16, y: u16) -> [u8; 6] {
    let (x, y) = (x.to_le_bytes(), y.to_le_bytes());
    [1, 1, x[0], x[1], y[0], y[1]]
}

#[test]
fn a_logical_minimum_is_where_the_coordinate_starts() {
    // Logical Minimum (100), Logical Maximum (1100).
    let l = parse_report_descriptor(&xy(&[0x16, 0x64, 0x00, 0x26, 0x4C, 0x04]));
    let s = decode_touch(&report(600, 100), &l).expect("decoded");
    assert_eq!((s.x, s.x_max), (500, 1000));
    assert_eq!((s.y, s.y_max), (0, 1000));
}

#[test]
fn a_negative_minimum_makes_the_coordinate_signed() {
    // Logical Minimum (-1024) in two bytes, Logical Maximum (1023).
    let l = parse_report_descriptor(&xy(&[0x16, 0x00, 0xFC, 0x26, 0xFF, 0x03]));
    let s = decode_touch(&report(0xFC00, 0), &l).expect("decoded");
    assert_eq!((s.x, s.y, s.x_max), (0, 1024, 2047));
}

#[test]
fn a_one_byte_maximum_of_0xff_over_a_zero_minimum_is_255() {
    let l = parse_report_descriptor(&xy(&[0x15, 0x00, 0x25, 0xFF]));
    assert_eq!((l.x.logical_max, l.y.logical_max), (255, 255));
}
