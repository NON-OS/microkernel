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

use crate::browser::layout::boxmodel::{BoxDocument, Fragment};
use crate::browser::state::State;

use super::box_page::TOP;
use super::fill_page::fill_page;

/* What shows where a page paints no canvas of its own. */
const NO_CANVAS: u32 = 0xFFFF_FFFF;

/* The canvas under the page: white, then the root's (or body's) background
 * color over the whole page area, below the content too, then its image.
 * The image is laid over the visible window, since a box only partly on
 * screen cannot take a background image. */
pub(super) fn paint_canvas(state: &State, doc: &BoxDocument, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width as i32, fb.height as i32 - TOP);
    /* An opaque canvas covers the fallback white; one with any transparency
     * composites over it. */
    if doc.canvas_bg >> 24 != 0xff {
        fb.fill_rect(0, TOP as u32, fb.width, h.max(0) as u32, NO_CANVAS);
    }
    if doc.canvas_bg >> 24 != 0 {
        fill_page(fb, 0, TOP, w, h, doc.canvas_bg, None);
    }
    let Some(img) = doc.canvas_bg_image.as_ref() else { return };
    let f = Fragment {
        y: state.scroll as i32,
        w,
        h,
        bg_image: Some(img.url.clone()),
        bg_layer: img.layer,
        ..Fragment::BLANK
    };
    super::bg_image::paint_bg_image(state, fb, &f, TOP, TOP + h, None);
}
