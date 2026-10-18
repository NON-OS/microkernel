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

//! QEMU's tablet positions: every one held to its logical range, 0 to
//! 0x7FFF, which the input router scales onto the screen, so none lands
//! past the screen's edge; and its edges by name.

use super::qemu::{at, tablet};

#[test]
fn every_position_is_clamped_to_the_logical_range() {
    for v in 0..=u16::MAX {
        let want = i32::from(v.min(0x7fff));
        assert_eq!(tablet(&at(v, 0)).map(|t| (t.x, t.y)), Some((want, 0)), "x {v:#x}");
        assert_eq!(tablet(&at(0, v)).map(|t| (t.x, t.y)), Some((0, want)), "y {v:#x}");
    }
}

#[test]
fn no_position_scales_past_the_screen_the_input_router_maps_it_onto() {
    // The router places an absolute pointer at x * (width - 1) / 0x7FFF
    // (capsule_input_router state/cursor.rs). Its divisor and this
    // driver's ceiling must stay the same number.
    let router = include_str!("../../../capsule_input_router/src/state/cursor.rs");
    assert!(router.contains("const ABS_RANGE_MAX: i64 = 0x7FFF;"));
    for width in [1i64, 2, 640, 1024, 1920, 3840, 7680, 1 << 16] {
        let max = width - 1;
        for v in 0..=u16::MAX {
            let x = i64::from(tablet(&at(v, v)).map_or(-1, |t| t.x));
            let scaled = x * max / 0x7FFF;
            assert!((0..=max).contains(&scaled), "{v:#x} on {width} wide is {scaled}");
        }
    }
}

#[test]
fn tablet_report_edges() {
    type Got = Option<(i32, i32, i32, u8)>;
    let cases: Vec<(&str, Vec<u8>, Got)> = vec![
        ("five bytes", vec![0, 0, 0, 0, 0], None),
        ("origin", vec![0, 0, 0, 0, 0, 0], Some((0, 0, 0, 0))),
        ("top of the range", vec![0, 0xff, 0x7f, 0xff, 0x7f, 0], Some((0x7fff, 0x7fff, 0, 0))),
        ("one past the top", vec![0, 0x00, 0x80, 0x00, 0x80, 0], Some((0x7fff, 0x7fff, 0, 0))),
        ("0xffff", vec![0, 0xff, 0xff, 0xff, 0xff, 0], Some((0x7fff, 0x7fff, 0, 0))),
        ("wheel most negative", vec![0, 0, 0, 0, 0, 0x80], Some((0, 0, -128, 0))),
        ("every button bit", vec![0xff, 0, 0, 0, 0, 0], Some((0, 0, 0, 0x07))),
        ("eight bytes", vec![1, 1, 0, 2, 0, 3, 0xff, 0xff], Some((1, 2, 3, 1))),
    ];
    for (name, raw, want) in cases {
        let got = tablet(&raw).map(|t| (t.x, t.y, t.wheel, t.buttons));
        assert_eq!(got, want, "{name}");
    }
}
