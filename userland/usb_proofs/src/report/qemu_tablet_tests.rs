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

//! QEMU's usb-tablet read through its own report descriptor, as the driver
//! now reads every report protocol interface: a report shorter than the
//! descriptor says is nothing, only the three buttons it declares are
//! buttons, the wheel is a signed byte, and bytes past the report are not
//! read. Before, these bytes were read in a fixed layout from any device.

use super::qemu::{at, tablet};
use crate::hid::tablet::TabletReport;

#[test]
fn a_report_shorter_than_its_descriptor_says_is_nothing() {
    let full = [0x07u8, 0xff, 0x7f, 0xff, 0x7f, 0x01];
    for len in 0..6 {
        assert_eq!(tablet(&full[..len]), None, "{len} bytes");
    }
    assert!(tablet(&full).is_some());
}

#[test]
fn positions_are_little_endian_and_the_wheel_is_signed() {
    let got = tablet(&[0x01, 0x34, 0x12, 0x78, 0x06, 0xff]);
    let want = TabletReport { buttons: 1, x: 0x1234, y: 0x0678, wheel: -1, at: true };
    assert_eq!(got, Some(want));
    for b in 0..=u8::MAX {
        let mut r = at(0, 0);
        r[5] = b;
        assert_eq!(tablet(&r).map(|t| t.wheel), Some(i32::from(b as i8)), "byte {b:#x}");
    }
}

#[test]
fn only_the_three_button_bits_it_declares_are_buttons() {
    for b in 0..=u8::MAX {
        let mut r = at(0, 0);
        r[0] = b;
        assert_eq!(tablet(&r).map(|t| t.buttons), Some(b & 0x07));
    }
}

#[test]
fn bytes_past_the_report_are_never_read() {
    let base = [0x02u8, 0x00, 0x40, 0x00, 0x20, 0x03];
    let want = tablet(&base);
    for fill in [0x00u8, 0x80, 0xff] {
        let mut r = base.to_vec();
        r.resize(8, fill);
        assert_eq!(tablet(&r), want);
    }
}
