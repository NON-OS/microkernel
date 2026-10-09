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

use crate::browser::fonts::NO_CLIP;
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
    /* Outer shadows paint first so the box sits over them; inset ones
     * go over the background and under the borders. */
    super::shadow::paint_shadow(fb, f, sy, clip, false);
    let bg = super::fade::fade(f.bg, f.alpha);
    /* A masked box shows its color only through the mask. */
    if bg != 0 && !f.mask {
        for c in super::round_clip::bands(clip, f.clip_r) {
            fill_rounded(fb, f.x, sy, f.w, f.h, f.radius, bg, c);
        }
    }
    super::shadow::paint_shadow(fb, f, sy, clip, true);
    /* A decoded background image paints over the color and behind content. */
    for c in super::round_clip::bands(clip, f.clip_r) {
        super::bg_image::paint_bg_image(state, fb, f, sy, bottom, c);
    }
    super::borders::paint_borders(fb, f, sy, clip);
    /* Text and images draw clipped to the page area and the fragment clip,
     * so a run or a picture cut by the scroll edge shows its visible part. */
    let [x0, y0, x1, y1] = clip.unwrap_or(NO_CLIP);
    let vis = [x0, y0.max(TOP), x1, y1.min(bottom)];
    if vis[1] >= vis[3] || vis[0] >= vis[2] || sy >= vis[3] || sy + f.h <= vis[1] {
        return;
    }
    match &f.content {
        Content::None => {}
        Content::Text { .. } => super::paint_text::paint_text(fb, f, sy, vis),
        Content::Image { .. } => {
            for c in super::round_clip::bands(clip, f.clip_r) {
                let [x0, y0, x1, y1] = c.unwrap_or(vis);
                let c = [x0.max(vis[0]), y0.max(vis[1]), x1.min(vis[2]), y1.min(vis[3])];
                if c[0] < c[2] && c[1] < c[3] {
                    super::paint_image::paint_image(state, fb, f, sy, c);
                }
            }
        }
    }
}
