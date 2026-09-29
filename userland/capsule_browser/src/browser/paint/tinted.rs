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

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use crate::browser::layout::boxmodel::Fragment;
use crate::browser::layout::filter_table::{tint_of, tint_px};
use crate::browser::state::State;

use super::box_fragment::box_fragment;
use super::box_page::TOP;

/* How far past its rectangle a fragment's paint may reach (a shadow, a
 * glyph overhang), so all it drew goes through its filter. */
const SPILL: i32 = 48;

/// Paint `f`, and when a filtered box drew it, put what it changed through
/// that filter's color map. A pixel it left alone keeps the backdrop, which
/// the filter does not touch; an edge pixel it blended is mapped whole.
pub(super) fn paint_one(state: &State, fb: &mut PaintBuffer, f: &Fragment, sy: i32, bottom: i32) {
    let Some(m) = (f.tint != 0).then(|| tint_of(f.tint)).flatten() else {
        return box_fragment(state, fb, f, sy, bottom);
    };
    let (x0, x1) = ((f.x - SPILL).max(0), (f.x + f.w + SPILL).min(fb.width as i32));
    let (y0, y1) = ((sy - SPILL).max(TOP), (sy + f.h + SPILL).min(bottom));
    if x0 >= x1 || y0 >= y1 {
        return;
    }
    let (cols, stride) = ((x1 - x0) as usize, fb.stride_words as usize);
    let at = |y: i32| y as usize * stride + x0 as usize;
    let mut old: Vec<u32> = Vec::with_capacity(cols * (y1 - y0) as usize);
    for y in y0..y1 {
        old.extend_from_slice(&fb.pixels[at(y)..at(y) + cols]);
    }
    box_fragment(state, fb, f, sy, bottom);
    for (i, y) in (y0..y1).enumerate() {
        let prev = &old[i * cols..(i + 1) * cols];
        for (px, &was) in fb.pixels[at(y)..at(y) + cols].iter_mut().zip(prev) {
            if *px != was {
                *px = tint_px(&m, *px);
            }
        }
    }
}
