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

//! The indicator's reserved band painted over whatever a frame composed
//! beneath it, a window covering the whole canvas included: black, a green
//! mark for the camera and an amber one for the microphone when on, dim
//! marks when off; nothing outside the band or the damaged rectangle.

use crate::damage::Rect;
use crate::frame_pacer::indicator_band::*;
use crate::state::indicator::{Indicators, KIND_CAMERA};
use crate::sw_blitter::Surface;

const W: u32 = 120;
const H: u32 = 40;
const WINDOW: u32 = 0xFFE0_E0E0;

fn canvas() -> Vec<u32> {
    vec![WINDOW; (W * H) as usize]
}

fn draw_on(px: &mut [u32], clip: Rect, ind: &Indicators) {
    let byte_len = px.len() as u64 * 4;
    let s =
        Surface { base_va: px.as_mut_ptr() as u64, stride: W * 4, width: W, height: H, byte_len };
    draw(s, clip, ind, false, false, false);
}

#[test]
fn the_band_covers_a_window_and_shows_what_captures() {
    let (mut px, mut ind) = (canvas(), Indicators::default());
    ind.apply(KIND_CAMERA, true, 40).unwrap();
    draw_on(&mut px, Rect { x: 0, y: 0, width: W, height: H }, &ind);
    let at = |x: u32, y: u32| px[(y * W + x) as usize];
    let b = band(W, H);
    assert_eq!((b.x, b.width, b.height), (W - BAND_W, BAND_W, BAND_H));
    assert_eq!(at(b.x, 0), BAND_ARGB, "the window does not show through");
    assert_eq!(at(b.x + 8, 5), CAMERA_ON);
    assert_eq!(at(b.x + 28, 5), MARK_OFF, "the microphone is off");
    assert_eq!((at(b.x - 1, 0), at(b.x, BAND_H)), (WINDOW, WINDOW), "nothing outside it");
}

#[test]
fn only_the_damaged_part_of_the_band_is_drawn() {
    let mut px = canvas();
    let b = band(W, H);
    draw_on(&mut px, Rect { x: 0, y: 0, width: b.x + 2, height: 1 }, &Indicators::default());
    let at = |x: u32, y: u32| px[(y * W + x) as usize];
    assert_eq!((at(b.x, 0), at(b.x + 1, 0), at(b.x + 2, 0)), (BAND_ARGB, BAND_ARGB, WINDOW));
    assert_eq!(at(b.x, 1), WINDOW);
}
