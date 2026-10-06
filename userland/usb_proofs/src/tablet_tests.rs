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

//! Absolute pointer reports: a report too short to hold both positions is
//! nothing, only the three defined button bits are buttons, the wheel is a
//! signed byte, and bytes past it are not read.

use crate::hid::tablet_report::{tablet_report, TabletReport};

pub fn at(x: u16, y: u16) -> [u8; 6] {
    let [xl, xh] = x.to_le_bytes();
    let [yl, yh] = y.to_le_bytes();
    [0, xl, xh, yl, yh, 0]
}

#[test]
fn a_report_too_short_to_hold_both_positions_is_nothing() {
    let full = [0x07u8, 0xff, 0x7f, 0xff, 0x7f, 0x01];
    for len in 0..5 {
        assert_eq!(tablet_report(&full[..len]), None, "{len} bytes");
    }
    assert!(tablet_report(&full[..5]).is_some());
}

#[test]
fn positions_are_little_endian_and_the_wheel_is_signed() {
    let got = tablet_report(&[0x01, 0x34, 0x12, 0x78, 0x06, 0xff]);
    assert_eq!(got, Some(TabletReport { buttons: 1, x: 0x1234, y: 0x0678, wheel: -1 }));
    for b in 0..=u8::MAX {
        let mut r = at(0, 0);
        r[5] = b;
        assert_eq!(tablet_report(&r).map(|t| t.wheel), Some(i32::from(b as i8)));
    }
    assert_eq!(tablet_report(&at(5, 6)[..5]).map(|t| t.wheel), Some(0));
}

#[test]
fn only_the_three_button_bits_are_buttons() {
    for b in 0..=u8::MAX {
        let mut r = at(0, 0);
        r[0] = b;
        assert_eq!(tablet_report(&r).map(|t| t.buttons), Some(b & 0x07));
    }
}

#[test]
fn every_position_is_clamped_to_the_logical_range() {
    for v in 0..=u16::MAX {
        let want = i32::from(v.min(0x7fff));
        assert_eq!(tablet_report(&at(v, 0)).map(|t| (t.x, t.y)), Some((want, 0)), "x {v:#x}");
        assert_eq!(tablet_report(&at(0, v)).map(|t| (t.x, t.y)), Some((0, want)), "y {v:#x}");
    }
}

#[test]
fn no_position_scales_past_the_screen_the_input_router_maps_it_onto() {
    // The router places an absolute pointer at x * (width - 1) / 0x7FFF
    // (capsule_input_router state/cursor.rs). Its divisor and this
    // driver's ceiling must stay the same number.
    let router = include_str!("../../capsule_input_router/src/state/cursor.rs");
    assert!(router.contains("const ABS_RANGE_MAX: i64 = 0x7FFF;"));
    for width in [1i64, 2, 640, 1024, 1920, 3840, 7680, 1 << 16] {
        let max = width - 1;
        for v in 0..=u16::MAX {
            let x = i64::from(tablet_report(&at(v, v)).map_or(-1, |t| t.x));
            let scaled = x * max / 0x7FFF;
            assert!((0..=max).contains(&scaled), "{v:#x} on {width} wide is {scaled}");
        }
    }
}

#[test]
fn bytes_past_the_wheel_are_never_read() {
    let base = [0x02u8, 0x00, 0x40, 0x00, 0x20, 0x03];
    let want = tablet_report(&base);
    for fill in [0x00u8, 0x80, 0xff] {
        let mut r = base.to_vec();
        r.resize(8, fill);
        assert_eq!(tablet_report(&r), want);
    }
}
