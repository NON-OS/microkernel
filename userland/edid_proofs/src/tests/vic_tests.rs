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

//! The VIC table as the data file lays it out.

use crate::cta::vic::vic;

#[test]
fn the_common_formats_are_where_the_standard_puts_them() {
    let v = |c| vic(c).map(|v| (v.width, v.height, v.hz, v.interlaced, v.pixel_khz));
    assert_eq!(v(1), Some((640, 480, 60, false, 25_175)));
    assert_eq!(v(4), Some((1280, 720, 60, false, 74_250)));
    assert_eq!(v(5), Some((1920, 1080, 60, true, 74_250)));
    assert_eq!(v(16), Some((1920, 1080, 60, false, 148_500)));
    assert_eq!(v(31), Some((1920, 1080, 50, false, 148_500)));
    assert_eq!(v(97), Some((3840, 2160, 60, false, 594_000)));
    assert_eq!(v(118), Some((3840, 2160, 120, false, 1_188_000)));
    assert_eq!(v(199), Some((7680, 4320, 60, false, 2_376_000)));
    assert_eq!(v(219), Some((4096, 2160, 120, false, 1_188_000)));
}

#[test]
fn exactly_the_listed_codes_answer() {
    let listed: Vec<u8> = (0..=255).filter(|&c| vic(c).is_some()).collect();
    let want: Vec<u8> = (1..=127).chain(193..=219).collect();
    assert_eq!(listed, want);
}

#[test]
fn every_entry_is_a_plausible_timing() {
    for v in (0..=255).filter_map(vic) {
        assert!(v.width >= 640 && v.height >= 240 && (24..=240).contains(&v.hz));
        // The clock covers at least the active pixels at the rate.
        let active = v.width as u64 * v.height as u64 * v.hz as u64;
        let per = if v.interlaced { 2 } else { 1 };
        assert!(v.pixel_khz as u64 * 1000 * per >= active, "{v:?}");
    }
}
