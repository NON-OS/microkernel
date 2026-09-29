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

use crate::browser::css::ObjectFit;
use crate::browser::image::{blit_rect, Decoded};
use crate::browser::layout::boxmodel::Fragment;

/* A url mask-image: the decoded mask's alpha, stretched over the tile
 * that mask-size and mask-position placed (`dest`), shows the box's
 * background color, which paints nowhere else. The tinted copy is the
 * size of the decoded mask, which the store sized to the box. */
pub(super) fn paint_mask(
    fb: &mut PaintBuffer,
    mask: &Decoded,
    f: &Fragment,
    dest: [i32; 4],
    vis: [i32; 4],
) {
    let (a, rgb) = (f.bg >> 24, f.bg & 0x00ff_ffff);
    if a == 0 {
        return;
    }
    let px = mask.px.iter().map(|p| (((p >> 24) * a / 255) << 24) | rgb).collect();
    let tinted = Decoded { w: mask.w, h: mask.h, px };
    blit_rect(fb, &tinted, dest, ObjectFit::Fill, f.alpha, Some(vis));
}
