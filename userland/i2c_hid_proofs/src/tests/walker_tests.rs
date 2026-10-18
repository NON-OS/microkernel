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

//! Legal descriptors the walk has to lay out as HID 1.11 says: a report
//! continued after another report's items, globals saved and restored by
//! Push and Pop, X and Y given as one usage range, and a usage that names
//! its own page in four bytes.

use super::descs::{touch_pad, TIP};
use crate::hid::{parse_report_descriptor, TouchLayout};

/// X at 8, then the report's Y; Report Size 16, Logical Maximum 1200.
const XY: &[u8] = &[0x05, 0x01, 0x26, 0xB0, 0x04, 0x75, 0x10, 0x95, 0x01, 0x09, 0x30, 0x81, 0x02];

fn at(l: &TouchLayout) -> [(u32, u32); 3] {
    [
        (l.tip.bit_offset, l.tip.bit_size),
        (l.x.bit_offset, l.x.bit_size),
        (l.y.bit_offset, l.y.bit_size),
    ]
}

#[test]
fn a_report_continued_after_another_reports_items_keeps_its_offsets() {
    let other: &[u8] = &[0x85, 0x02, 0x06, 0x00, 0xFF, 0x09, 0x01, 0x75, 0x08, 0x81, 0x02];
    let back: &[u8] = &[0x85, 0x01, 0x05, 0x01, 0x75, 0x10, 0x09, 0x31, 0x81, 0x02];
    let l = parse_report_descriptor(&touch_pad(&[TIP, XY, other, back]));
    assert_eq!(at(&l), [(0, 1), (8, 16), (24, 16)], "Y restarted the report");
}

#[test]
fn pop_restores_the_globals_push_saved() {
    let push_x: &[u8] =
        &[0x05, 0x01, 0x26, 0xB0, 0x04, 0x75, 0x10, 0x95, 0x01, 0xA4, 0x09, 0x30, 0x81, 0x02];
    let constant: &[u8] = &[0x05, 0x09, 0x75, 0x04, 0x81, 0x03];
    let pop_y: &[u8] = &[0xB4, 0x09, 0x31, 0x81, 0x02];
    let l = parse_report_descriptor(&touch_pad(&[TIP, push_x, constant, pop_y]));
    assert_eq!(at(&l), [(0, 1), (8, 16), (28, 16)]);
    assert_eq!(l.y.logical_max, 1200);
}

#[test]
fn x_and_y_given_as_one_usage_range_are_both_found() {
    let range: &[u8] =
        &[0x05, 0x01, 0x19, 0x30, 0x29, 0x31, 0x26, 0xB0, 0x04, 0x75, 0x10, 0x95, 0x02, 0x81, 0x02];
    let l = parse_report_descriptor(&touch_pad(&[TIP, range]));
    assert_eq!(at(&l), [(0, 1), (8, 16), (24, 16)]);
}

#[test]
fn a_four_byte_usage_keeps_its_own_page() {
    // On the Simulation Controls page (0x02, no bit of it in 0x0D), Usage
    // (0x000D0042) is still the tip switch: its page replaces the page in
    // force, it is not merged with it.
    let tip: &[u8] =
        &[0x05, 0x02, 0x0B, 0x42, 0x00, 0x0D, 0x00, 0x25, 0x01, 0x75, 0x01, 0x95, 0x01];
    let tip_pad: &[u8] = &[0x81, 0x02, 0x95, 0x07, 0x81, 0x03];
    let l = parse_report_descriptor(&touch_pad(&[tip, tip_pad, XY, &[0x09, 0x31, 0x81, 0x02]]));
    assert_eq!(at(&l), [(0, 1), (8, 16), (24, 16)]);
    assert!(l.is_absolute_touch());
}
