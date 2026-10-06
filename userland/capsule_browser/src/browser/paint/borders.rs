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

use super::corners::corners;
use super::fill_page::fill_page;

/* A fragment's border edges on screen row `sy`, each shortened by the
 * corner radius at either end, then the rounded corners between them. */
pub(super) fn paint_borders(fb: &mut PaintBuffer, f: &Fragment, sy: i32, clip: Option<[i32; 4]>) {
    let edge = super::fade::fade(f.border_color, f.alpha);
    let [tl, tr, br, bl] = corners(f.radius, f.w, f.h);
    let [bt, brw, bb, blw] = f.border.map(|b| b as i32);
    if bt > 0 {
        fill_page(fb, f.x + tl, sy, f.w - tl - tr, bt, edge, clip);
    }
    if bb > 0 {
        fill_page(fb, f.x + bl, sy + f.h - bb, f.w - bl - br, bb, edge, clip);
    }
    if blw > 0 {
        fill_page(fb, f.x, sy + tl, blw, f.h - tl - bl, edge, clip);
    }
    if brw > 0 {
        fill_page(fb, f.x + f.w - brw, sy + tr, brw, f.h - tr - br, edge, clip);
    }
    super::border_corners::paint_corner_arcs(fb, f, sy, edge, clip);
}
