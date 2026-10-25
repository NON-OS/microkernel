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
//! The band's marks are glyphs, each set pixel in its colour (on or dim), the
//! rest of the band the bar's colour over any window.

use crate::damage::Rect;
use crate::frame_pacer::band_glyphs::{set, Glyph, CAMERA, LOCK, MICROPHONE, SCREEN, SIDE};
use crate::frame_pacer::indicator_band::*;
use crate::state::indicator::{Indicators, KIND_CAMERA, KIND_MICROPHONE};
use crate::sw_blitter::Surface;

const W: u32 = 200;
const H: u32 = 30;
const WINDOW: u32 = 0xFF12_3456;

fn drawn(on: [bool; 4]) -> Vec<u32> {
    let mut px = vec![WINDOW; (W * H) as usize];
    let mut ind = Indicators::default();
    ind.apply(KIND_CAMERA, on[0], 40).unwrap();
    ind.apply(KIND_MICROPHONE, on[1], 41).unwrap();
    let byte_len = px.len() as u64 * 4;
    let s =
        Surface { base_va: px.as_mut_ptr() as u64, stride: W * 4, width: W, height: H, byte_len };
    draw(s, Rect { x: 0, y: 0, width: W, height: H }, &ind, on[2], on[3], false);
    px
}

fn shows(px: &[u32], g: &Glyph, slot: u32, colour: u32) {
    let x0 = band(W, H).x + 7 + 20 * slot;
    for (x, y) in (0..SIDE).flat_map(|y| (0..SIDE).map(move |x| (x, y))) {
        let want = if set(g, x, y) { colour } else { BAND_ARGB };
        assert_eq!(px[((3 + y) * W + x0 + x) as usize], want, "mark {slot} ({x}, {y})");
    }
}

#[test]
fn every_mark_on_and_off() {
    let marks =
        [(CAMERA, CAMERA_ON), (MICROPHONE, MICROPHONE_ON), (SCREEN, SCREEN_ON), (LOCK, TRUSTED_ON)];
    for lit in 0..=4 {
        let px = drawn(core::array::from_fn(|i| i == lit));
        for (slot, (g, on)) in marks.iter().enumerate() {
            shows(&px, g, slot as u32, if slot == lit { *on } else { MARK_OFF });
        }
    }
    assert_eq!((BAND_ARGB, MARK_OFF), (0xFF04_080C, 0xFF4A_5560));
}

#[test]
fn no_window_shows_in_the_band() {
    let b = band(W, H);
    for on in [[false; 4], [true; 4]] {
        let px = drawn(on);
        let mut inside = (0..b.height).flat_map(|y| (b.x..W).map(move |x| (y * W + x) as usize));
        assert!(inside.all(|i| px[i] != WINDOW));
        assert_eq!(px[(b.height * W + b.x) as usize], WINDOW, "the window below the band");
    }
}
