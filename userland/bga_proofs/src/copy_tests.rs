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

//! The present copy: the rect's rows from the shadow, at the request's
//! offset, into VRAM at the rect, and nothing else touched.

use crate::gfx::{copy, Rect};

const W: usize = 16;
const H: usize = 8;
const STRIDE: u32 = (W * 4) as u32;

#[test]
fn the_rect_and_only_the_rect_reaches_vram() {
    let shadow: Vec<u32> = (0..(W * H) as u32).map(|i| 0x1000_0000 | i).collect();
    let mut vram = vec![0u32; W * H];
    let r = Rect { x: 3, y: 2, w: 5, h: 4 };
    let offset = (2 * STRIDE + 3 * 4) as u64;
    // SAFETY: both buffers span the mode; the rect and offset are inside it.
    unsafe { copy(shadow.as_ptr() as u64, vram.as_mut_ptr() as u64, STRIDE, r, offset) };
    for y in 0..H {
        for x in 0..W {
            let inside = (3..8).contains(&x) && (2..6).contains(&y);
            let want = if inside { shadow[y * W + x] } else { 0 };
            assert_eq!(vram[y * W + x], want, "pixel {x},{y}");
        }
    }
}

#[test]
fn the_rows_are_read_from_the_offset_given() {
    /*
     * The compositor names where the rect starts in the shadow. A copy that
     * recomputed it from x and y would show the right pixels only while the
     * two agree; one that ignored it would show the wrong ones when they do
     * not.
     */
    let shadow: Vec<u32> = (0..(W * H) as u32).collect();
    let mut vram = vec![0u32; W * H];
    let r = Rect { x: 0, y: 0, w: 2, h: 2 };
    let offset = (5 * STRIDE + 4) as u64;
    // SAFETY: as above; the source starts at row 5, column 1.
    unsafe { copy(shadow.as_ptr() as u64, vram.as_mut_ptr() as u64, STRIDE, r, offset) };
    assert_eq!([vram[0], vram[1], vram[W], vram[W + 1]], [81, 82, 97, 98]);
}
