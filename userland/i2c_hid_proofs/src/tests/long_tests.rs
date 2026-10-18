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

//! An input report too long for one read of the input register is never
//! read whole: it costs its own fields and nothing else, a touch report
//! included. Past Linux's 16 KiB report buffer the descriptor is refused,
//! as Linux refuses it.

use super::bounds_tests::{lays_out, XY};
use super::descs::{touch_pad, MOUSE, TIP};
use crate::hid::{parse_mouse_layout, parse_report_descriptor};

/// Input report 2 of `bytes` constant bytes, its ID byte besides.
fn vendor_report(bytes: u16) -> Vec<u8> {
    let b = bytes.to_le_bytes();
    vec![0x85, 0x02, 0x75, 0x08, 0x96, b[0], b[1], 0x81, 0x03]
}

#[test]
fn a_vendor_report_too_long_for_one_read_leaves_the_touch_report_laid_out() {
    assert!(lays_out(&vendor_report(253)), "254 bytes with its ID");
    assert!(lays_out(&vendor_report(254)), "255 bytes with its ID");
    assert!(lays_out(&vendor_report(16_383)), "16 KiB with its ID");
}

#[test]
fn a_touch_report_too_long_for_one_read_is_not_taken() {
    // The tip, X and Y take five bytes; then constant bytes to the bound.
    let tail = |bytes: u8| vec![0x75, 0x08, 0x95, bytes, 0x81, 0x03];
    let fits = parse_report_descriptor(&touch_pad(&[TIP, XY, &tail(248)]));
    assert!(fits.is_absolute_touch(), "253 bytes of body, 254 with its ID");
    let long = touch_pad(&[TIP, XY, &tail(249)]);
    assert!(!parse_report_descriptor(&long).is_absolute_touch(), "cannot be read whole");
    // Report IDs from 64 on are kept in the set's other words.
    let high = [&long[..7], &[0xC1], &long[8..]].concat();
    assert!(!parse_report_descriptor(&high).is_absolute_touch(), "report ID 0xC1");
}

#[test]
fn a_mouse_report_too_long_for_one_read_is_not_taken() {
    // Buttons and X and Y take three bytes; then constant bytes to the bound.
    let (head, close) = MOUSE.split_at(MOUSE.len() - 2);
    let mouse = |bytes: u8| [head, &[0x75, 0x08, 0x95, bytes, 0x81, 0x03], close].concat();
    assert!(parse_mouse_layout(&mouse(250)).is_relative_mouse(), "253 bytes, 254 with its ID");
    assert!(!parse_mouse_layout(&mouse(251)).is_relative_mouse(), "cannot be read whole");
}

#[test]
fn a_report_past_linuxs_16_kib_buffer_refuses_the_descriptor() {
    assert!(!lays_out(&vendor_report(16_384)), "16 KiB and its ID byte");
    let count = [0x75, 0x01, 0x97, 0x01, 0x00, 0x02, 0x00, 0x81, 0x03];
    assert!(!lays_out(&count), "a Report Count of 131073");
}
