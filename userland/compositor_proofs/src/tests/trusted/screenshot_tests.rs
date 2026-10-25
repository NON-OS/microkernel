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

//! Screenshots: only a holder of ScreenCapture may take one, and the band's
//! blue notice shows while the capture is shown. What a capture holds is in
//! trusted_capture_tests.

use crate::damage::Rect;
use crate::display::screenshot_rule::{may_capture, SCREEN_CAPTURE};
use crate::frame_pacer::indicator_band::{band, draw, SCREEN_ON};
use crate::state::indicator::Indicators;
use crate::sw_blitter::Surface;

fn surface(px: &mut [u32], w: u32, h: u32, stride_px: u32) -> Surface {
    let byte_len = px.len() as u64 * 4;
    Surface {
        base_va: px.as_mut_ptr() as u64,
        stride: stride_px * 4,
        width: w,
        height: h,
        byte_len,
    }
}

#[test]
fn only_a_screen_capture_holder_may_capture() {
    assert_eq!(SCREEN_CAPTURE, Some(1 << 39));
    assert!(may_capture(40, |pid, mask| pid == 40 && mask == 1 << 39));
    assert!(!may_capture(41, |pid, mask| pid == 40 && mask == 1 << 39), "another process");
    assert!(!may_capture(40, |_, mask| mask == 1 << 37), "the camera's bit is not it");
    assert!(!may_capture(0, |_, _| true));
}

#[test]
fn the_band_shows_the_capture() {
    let mut px = vec![0u32; 120 * 40];
    let s = surface(&mut px, 120, 40, 120);
    draw(s, Rect { x: 0, y: 0, width: 120, height: 40 }, &Indicators::default(), true, false, false);
    assert_eq!(px[(5 * 120 + band(120, 40).x + 48) as usize], SCREEN_ON);
}
