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

//! Per-output scale in quarters: the shell's steps or the panel's whole
//! one, the canvas each step gives, and the fractional stretch: every
//! screen pixel the canvas pixel its centre falls in, a canvas rectangle
//! mapped to the screen rectangle it covers, nothing outside it written.

use crate::damage::Rect;
use crate::display::scale_rule::{canvas_for, chosen, whole, STEPS};
use crate::sw_blitter::stretch::stretch;
use crate::sw_blitter::Surface;

fn surface(px: &mut [u32], w: u32, h: u32) -> Surface {
    let byte_len = px.len() as u64 * 4;
    Surface { base_va: px.as_mut_ptr() as u64, stride: w * 4, width: w, height: h, byte_len }
}

#[test]
fn the_shells_step_wins_and_the_panel_rule_stands_in() {
    assert_eq!(STEPS, [4, 5, 6, 8, 12, 16]);
    assert_eq!((chosen(Some(6), 2), chosen(None, 2), chosen(Some(7), 1)), (6, 8, 4));
    assert_eq!(canvas_for(2560, 1600, 6), (1706, 1066));
    assert_eq!(canvas_for(3840, 2160, 5), (3072, 1728));
    assert_eq!((whole(5), whole(6), whole(8)), (1, 2, 2));
}

#[test]
fn one_and_a_half_times_doubles_every_other_canvas_pixel() {
    let mut canvas: Vec<u32> = (0..4 * 2).collect();
    let mut screen = vec![0xEEu32; 6 * 3];
    let r = Rect { x: 0, y: 0, width: 4, height: 2 };
    let out = stretch(surface(&mut canvas, 4, 2), surface(&mut screen, 6, 3), r, 6).unwrap();
    assert_eq!((out.x, out.y, out.width, out.height), (0, 0, 6, 3));
    assert_eq!(&screen[..6], &[0, 1, 1, 2, 3, 3], "row 0: canvas 1 and 3 each twice");
    assert_eq!(&screen[6..12], &[4, 5, 5, 6, 7, 7], "canvas row 1 takes screen rows 1 and 2");
    assert_eq!(&screen[12..], &[4, 5, 5, 6, 7, 7]);
}

#[test]
fn a_damaged_rectangle_maps_to_the_screen_pixels_it_covers() {
    let mut canvas: Vec<u32> = (0..8 * 8).collect();
    let mut screen = vec![0xEEu32; 10 * 10];
    let r = Rect { x: 4, y: 4, width: 2, height: 1 };
    let out = stretch(surface(&mut canvas, 8, 8), surface(&mut screen, 10, 10), r, 5).unwrap();
    assert_eq!((out.x, out.y, out.width, out.height), (5, 5, 2, 1), "1.25 times");
    assert_eq!(screen[5 * 10 + 5..5 * 10 + 7], [36, 37]);
    let written = screen.iter().filter(|&&p| p != 0xEE).count();
    assert_eq!(written, 2, "nothing outside it");
}
