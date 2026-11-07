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

#[test]
fn a_window_asked_full_screen_before_its_first_buffer_fits_the_screen() {
    // 1920x1080 is 5 quarters: 150. A Qt client has its viewport before its
    // first commit and draws at the fractional scale it was told.
    let d = display(5);
    let qt = Scale { buffer: 1, dest: None, viewport: 40 };
    let fill = (1920, 1040);
    let e = expected_ratio(qt, d);
    assert_eq!(e, d);
    let told = (to_logical(fill.0, e), to_logical(fill.1, e));
    assert_eq!(told, (1536, 832));
    // It draws told times 1.25 and names told as its destination.
    let drawn = Scale { dest: Some((told.0 as u32, told.1 as u32)), ..qt };
    assert_eq!(shown(drawn, (1920, 1040), window_ratio(drawn, d)), (1920, 1040));
    // At the default 1:1 it was told the screen and shown a quarter past it.
    let wrong = Scale { dest: Some((1920, 1040)), ..qt };
    assert_eq!(shown(wrong, (2400, 1300), window_ratio(wrong, d)), (2400, 1300));
}

#[test]
fn a_client_with_no_viewport_is_expected_at_its_buffer_scale() {
    let d = display(5);
    assert_eq!(expected_ratio(scale(1, None), d), 120, "a GTK3 window at scale 1 is 1:1");
    assert_eq!(expected_ratio(scale(0, None), d), 120);
    assert_eq!(expected_ratio(scale(2, None), display(8)), 240);
    assert_eq!(expected_ratio(scale(2, None), d), d, "never past the display");
    assert_eq!(expected_ratio(scale(1, Some((800, 600))), d), d);
    // Each is the ratio its first buffer is then shown at.
    for s in [scale(1, None), scale(2, None), scale(1, Some((800, 600)))] {
        assert_eq!(expected_ratio(s, d), window_ratio(s, d));
    }
}

#[test]
fn a_mode_asked_is_told_at_the_ratio_the_window_will_be_shown_at() {
    let ask = include_str!("../../../../capsule_linux/src/linux/wayland/toplevel/ask.rs");
    assert!(ask.contains("let told = logical(told, ratio(guest, toplevel));"));
    let at = ask.find("fn ratio(").expect("ratio");
    let body = &ask[at..at + ask[at..].find("\n}\n").expect("end")];
    let shown = body.find("w.toplevel == Some(toplevel)").expect("its own window shown");
    assert!(body[shown..].contains("return guest.scene.win.frame.ratio;"));
    assert!(body.contains("expected_ratio("), "else the ratio it will be shown at");
}
