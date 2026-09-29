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

use crate::browser::layout::boxmodel::{Content, Fragment};
use crate::browser::state::State;

use super::fill_page::fill_page;

const IMG_BG: u32 = 0xFF20_2A30;
const IMG_EDGE: u32 = 0xFF46_A6B2;

/* An image fragment on screen row `sy`: the decoded raster fitted into the
 * box, or while it has not arrived a framed placeholder with its alt text. */
pub(super) fn paint_image(
    state: &State,
    fb: &mut PaintBuffer,
    f: &Fragment,
    sy: i32,
    clip: Option<[i32; 4]>,
) {
    let Content::Image { src, alt, fit } = &f.content else { return };
    /* The store resolves the fragment's src against the page base the way
     * the fetch did, remembering the join, so a frame costs a lookup. */
    if let Some(img) = state.images.ready_for(state.base.as_ref(), src) {
        crate::browser::image::blit_into(
            fb,
            img,
            [f.x.max(0) as u32, sy.max(0) as u32, f.w.max(0) as u32, f.h.max(0) as u32],
            *fit,
            f.alpha,
            clip,
        );
    } else {
        fill_page(fb, f.x, sy, f.w, f.h, IMG_BG, clip);
        fill_page(fb, f.x, sy, f.w, 1, IMG_EDGE, clip);
        fill_page(fb, f.x, sy + f.h - 1, f.w, 1, IMG_EDGE, clip);
        fill_page(fb, f.x, sy, 1, f.h, IMG_EDGE, clip);
        fill_page(fb, f.x + f.w - 1, sy, 1, f.h, IMG_EDGE, clip);
        let label = if alt.is_empty() { "image" } else { alt.as_str() };
        if f.h >= 24 && f.w >= 48 {
            fb.text_ttf(f.x + 8, sy + 6, label, IMG_EDGE, 14.0);
        }
    }
}
