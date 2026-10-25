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

//! The first presents' lines: the box around a frame's rectangles, and a
//! grid of samples that tells an unset (black) buffer from a drawn one,
//! never reading outside the surface.

use crate::damage::Rect;
use crate::frame_pacer::present_note::{bbox, lit, note, GRID};
use crate::sw_blitter::Surface;

fn r(x: u32, y: u32, width: u32, height: u32) -> Rect {
    Rect { x, y, width, height }
}

fn surface(px: &mut [u32], w: u32, h: u32) -> Surface {
    let byte_len = (w * h * 4) as u64;
    Surface { base_va: px.as_mut_ptr() as u64, stride: w * 4, width: w, height: h, byte_len }
}

#[test]
fn the_box_holds_every_rectangle() {
    assert!(bbox(&[]).is_empty());
    let b = bbox(&[r(10, 20, 5, 5), r(0, 0, 0, 0), r(40, 2, 10, 3)]);
    assert_eq!((b.x, b.y, b.width, b.height), (10, 2, 40, 23));
}

#[test]
fn black_and_alpha_only_read_unlit_and_drawn_reads_lit() {
    let (w, h) = (64, 48);
    let mut px = vec![0xFF00_0000u32; (w * h) as usize];
    assert_eq!(lit(surface(&mut px, w, h), r(0, 0, w, h)), (0, GRID * GRID));
    px.iter_mut().for_each(|p| *p = 0xFF20_4060);
    assert_eq!(lit(surface(&mut px, w, h), r(0, 0, w, h)), (GRID * GRID, GRID * GRID));
}

#[test]
fn a_small_box_samples_each_pixel_and_outside_reads_nothing() {
    let (w, h) = (16, 16);
    let mut px = vec![0u32; (w * h) as usize];
    px[(5 * w + 3) as usize] = 0x0000_0001;
    assert_eq!(lit(surface(&mut px, w, h), r(2, 4, 3, 2)), (1, 6));
    assert_eq!(lit(surface(&mut px, w, h), r(20, 20, 8, 8)), (0, 0));
    assert_eq!(lit(surface(&mut px, w, h), r(12, 12, 8, 8)).1, 16, "only the part inside");
}

#[test]
fn the_line_says_box_serial_samples_and_answer() {
    let l = note(1, &[r(0, 0, 1920, 1080)], 3, (0, 64), false);
    assert_eq!(
        l,
        "present 1: 1 rects, box 1920x1080 at 0,0, serial 3, lit 0 of 64 sampled, refused"
    );
}
