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

//! Range limits (E-EDID 1.4 Tables 3.26 and 3.27).

use crate::base::range::RangeLimits;
use crate::build_dtd::display;

fn range(flags: u8, v: (u8, u8), h: (u8, u8), clock: u8, cvt: Option<(u8, u8)>) -> [u8; 18] {
    let mut d = display(0xFD, &[]);
    d[4] = flags;
    d[5..10].copy_from_slice(&[v.0, v.1, h.0, h.1, clock]);
    if let Some((b12, b13)) = cvt {
        d[10] = 0x04;
        d[11] = 0x11;
        d[12] = b12;
        d[13] = b13;
    }
    d
}

#[test]
fn the_plain_limits_read_as_written() {
    let r = RangeLimits::decode(&range(0, (48, 75), (30, 83), 17, None)).unwrap();
    assert_eq!((r.v_min_hz, r.v_max_hz, r.h_min_khz, r.h_max_khz), (48, 75, 30, 83));
    assert_eq!((r.max_pixel_khz, r.max_h_active), (170_000, 0));
}

#[test]
fn the_offset_flags_add_255_where_they_say() {
    let r = RangeLimits::decode(&range(0x02 | 0x0C, (48, 5), (10, 30), 60, None)).unwrap();
    assert_eq!((r.v_min_hz, r.v_max_hz), (48, 260), "10: the vertical maximum only");
    assert_eq!((r.h_min_khz, r.h_max_khz), (265, 285), "11: both horizontal");
}

#[test]
fn the_offsets_never_leave_a_minimum_above_its_maximum() {
    assert!(RangeLimits::decode(&range(0x0C, (48, 75), (30, 10), 60, None)).is_none());
    assert!(RangeLimits::decode(&range(0x08, (48, 75), (200, 10), 60, None)).is_some());
    assert!(RangeLimits::decode(&range(0, (76, 75), (30, 83), 17, None)).is_none());
}

#[test]
fn cvt_support_trims_the_clock_and_caps_the_line() {
    // 3 quarter-MHz steps off 600 MHz; 2560 = 8 * 320 = 8 * (0x40 | 1 << 8).
    let r = RangeLimits::decode(&range(0, (48, 75), (30, 160), 60, Some((3 << 2 | 1, 0x40))));
    let r = r.unwrap();
    assert_eq!((r.max_pixel_khz, r.max_h_active), (599_250, 2560));
}
