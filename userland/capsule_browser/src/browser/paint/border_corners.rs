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

use nonos_app_skeleton::PaintBuffer;

use crate::browser::layout::boxmodel::Fragment;

use super::corners::{corners, inset};
use super::fill_page::fill_page;

/* The rounded corners of a fragment's border on screen row `sy`: in each
 * corner of radius r, row by row, the ring between the outer curve and the
 * inner one of radius r - w, w the wider of the two borders meeting there.
 * The straight edges stop where the corners start (paint_borders). */
pub(super) fn paint_corner_arcs(
    fb: &mut PaintBuffer,
    f: &Fragment,
    sy: i32,
    edge: u32,
    clip: Option<[i32; 4]>,
) {
    let [bt, brw, bb, blw] = f.border.map(|b| b as i32);
    let [tl, tr, br, bl] = corners(f.radius, f.w, f.h);
    /* (radius, border width, on the right, on the bottom) per corner. */
    let spec = [
        (tl, bt.max(blw), false, false),
        (tr, bt.max(brw), true, false),
        (br, bb.max(brw), true, true),
        (bl, bb.max(blw), false, true),
    ];
    for (r, w, right, bottom) in spec.into_iter().filter(|s| s.0 > 0 && s.1 > 0) {
        let ri = r - w;
        for i in 0..r {
            let outer = inset(r, i);
            let inner = if i < w || ri <= 0 { r } else { w + inset(ri, i - w) };
            if inner > outer {
                let x = if right { f.x + f.w - inner } else { f.x + outer };
                let y = if bottom { sy + f.h - 1 - i } else { sy + i };
                fill_page(fb, x, y, inner - outer, 1, edge, clip);
            }
        }
    }
}
