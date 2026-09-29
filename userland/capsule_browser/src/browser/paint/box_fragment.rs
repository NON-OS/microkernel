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

use super::box_page::TOP;
use super::fill_rounded::fill_rounded;

/* One display-list rectangle: background, border edges, then content. The
 * fragment clip travels in page coordinates and shifts with the scroll. */
pub(super) fn box_fragment(
    state: &State,
    fb: &mut PaintBuffer,
    f: &Fragment,
    sy: i32,
    bottom: i32,
) {
    let dy = TOP - state.scroll as i32;
    let clip = f.clip.map(|c| [c[0], c[1].saturating_add(dy), c[2], c[3].saturating_add(dy)]);
    /* The drop shadow paints first so the box and its content sit over it. */
    if let Some(s) = f.shadow.as_ref() {
        super::shadow::paint_shadow(fb, s, f.x, sy, f.w, f.h);
    }
    let bg = super::fade::fade(f.bg, f.alpha);
    if bg != 0 {
        fill_rounded(fb, f.x, sy, f.w, f.h, f.radius, bg, clip);
    }
    /* A decoded background image paints over the color and behind content. */
    super::bg_image::paint_bg_image(state, fb, f, sy, bottom, clip);
    super::borders::paint_borders(fb, f, sy, clip);
    /* Text and images draw only when their box sits fully inside the page
     * area and clip; the framebuffer has no clip for glyph or raster runs. */
    if sy < TOP || sy + f.h > bottom {
        return;
    }
    if let Some(c) = clip {
        if f.x < c[0] || f.x + f.w > c[2] || sy < c[1] || sy + f.h > c[3] {
            return;
        }
    }
    match &f.content {
        Content::None => {}
        Content::Text { .. } => super::paint_text::paint_text(fb, f, sy, clip),
        Content::Image { .. } => super::paint_image::paint_image(state, fb, f, sy, clip),
    }
}
