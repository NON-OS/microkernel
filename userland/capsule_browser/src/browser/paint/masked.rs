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

use alloc::vec;
use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use crate::browser::layout::boxmodel::Fragment;
use crate::browser::layout::fade_table::fade_value;
use crate::browser::layout::hit_screen::frag_screen_y;
use crate::browser::state::State;

use super::box_fragment::box_fragment;
use super::box_page::TOP;
use super::grad::mask_layers;
use super::mask_weights::{mix, weights};

/* How far past its rectangle a fragment's paint may reach (a shadow, a
 * glyph overhang); the blend covers it so nothing drawn escapes the mask. */
const SPILL: i32 = 48;

/// Paint `f`, drawn inside the gradient-masked box `owner`, through that
/// mask: the pixels it can touch are kept, it paints, and each then shows
/// the old and new mixed by the mask's coverage there. A mask this painter
/// cannot draw leaves the fragment painted as if unmasked.
pub(super) fn paint_masked(state: &State, fb: &mut PaintBuffer, f: &Fragment, owner: &Fragment) {
    let (scroll, bottom) = (state.scroll as i32, fb.height as i32);
    let sy = TOP + frag_screen_y(f.y, f.fixed, f.sticky, scroll);
    let layers = fade_value(owner.fade).and_then(|v| mask_layers(&v, owner.w, owner.h));
    let Some(layers) = layers.filter(|_| owner.w > 0 && owner.h > 0) else {
        return box_fragment(state, fb, f, sy, bottom);
    };
    let oy = TOP + frag_screen_y(owner.y, owner.fixed, owner.sticky, scroll);
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
    let (mut m, mut tmp) = (vec![0u32; cols], vec![0u32; cols]);
    let o = [owner.x, oy, owner.w, owner.h];
    for (i, y) in (y0..y1).enumerate() {
        weights(&layers, o, owner.fade_isect, (y, x0), &mut m, &mut tmp);
        let prev = &old[i * cols..(i + 1) * cols];
        for ((px, &was), &k) in fb.pixels[at(y)..at(y) + cols].iter_mut().zip(prev).zip(&m) {
            *px = mix(was, *px, k);
        }
    }
}
