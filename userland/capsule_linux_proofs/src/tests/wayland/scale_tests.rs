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

//! The scale a guest's window is shown at, and its pixels brought to it.

use crate::wayland_frame::resample::resample;
use crate::wayland_frame::scale_math::*;

fn scale(buffer: u32, dest: Option<(u32, u32)>) -> Scale {
    Scale { buffer, dest, viewport: 0 }
}

#[test]
fn the_display_is_told_in_120ths_and_in_whole_steps() {
    assert_eq!((display(4), display(5), display(6), display(8)), (120, 150, 180, 240));
    assert_eq!((whole(4), whole(5), whole(6), whole(8)), (1, 1, 1, 2));
}

#[test]
fn a_buffer_scale_is_shown_at_its_logical_size_or_its_own_pixels() {
    let s = scale(2, None);
    assert_eq!(window_ratio(s, display(8)), 240);
    assert_eq!(shown(s, (1600, 1200), 240), (1600, 1200));
    assert_eq!(window_ratio(s, display(4)), 120);
    assert_eq!(shown(s, (1600, 1200), 120), (800, 600));
    let plain = scale(0, None);
    assert_eq!(window_ratio(plain, display(8)), 120, "a client that knows no scale is not grown");
    assert_eq!(shown(plain, (800, 600), 120), (800, 600));
}

#[test]
fn a_viewport_is_its_destination_at_the_display_scale_as_foot_draws_it() {
    let s = scale(1, Some((800, 600)));
    let e = window_ratio(s, display(5));
    assert_eq!(e, 150);
    assert_eq!(shown(s, (1000, 750), e), (1000, 750));
    assert_eq!(shown(s, (1, 1), e), (1000, 750), "a single pixel stretched");
    assert_eq!(shown(scale(1, Some((u32::MAX, 1))), (1, 1), 240).0, u32::MAX);
}

#[test]
fn points_go_between_shown_and_logical_pixels() {
    assert_eq!(point_px((10, -10), 150), (12, -13));
    assert_eq!((to_logical(1000, 150), to_logical(-1, 150), to_logical(7, 0)), (800, -1, 840));
    assert_eq!(point_px((i32::MAX, i32::MIN), 240), (i32::MAX, i32::MIN));
}

#[test]
fn nearest_neighbour_halves_and_doubles() {
    let src: Vec<u8> = (0..16u8).flat_map(|i| [i, i, i, 255]).collect();
    let mut half = vec![0u8; 2 * 2 * 4];
    assert!(resample(&src, (4, 4, 16), &mut half, (2, 2)));
    let firsts: Vec<u8> = half.chunks(4).map(|p| p[0]).collect();
    assert_eq!(firsts, [5, 7, 13, 15]);
    let mut double = vec![0u8; 8 * 8 * 4];
    assert!(resample(&src, (4, 4, 16), &mut double, (8, 8)));
    assert_eq!((double[0], double[4], double[8], double[8 * 8 * 4 - 4]), (0, 0, 1, 15));
    assert!(!resample(&src[..60], (4, 4, 16), &mut half, (2, 2)), "a short source");
    assert!(!resample(&src, (4, 4, 16), &mut half[..15], (2, 2)), "a short destination");
    assert!(!resample(&src, (0, 4, 16), &mut half, (2, 2)), "an empty source");
    assert!(!resample(&src, (4, 4, 16), &mut half, (0, 2)), "an empty destination");
}
