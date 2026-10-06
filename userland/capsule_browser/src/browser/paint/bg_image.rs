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
use crate::browser::state::State;

use super::box_page::TOP;

/* Paint a fragment's background image, scaled to its box and behind content.
 * The url resolves against the page base like any other fetched image. Every
 * layer is limited to the box, its clip and the page rows below the chrome,
 * so a box only partly on screen paints the part that shows. */
pub(super) fn paint_bg_image(
    state: &State,
    fb: &mut PaintBuffer,
    f: &Fragment,
    sy: i32,
    bottom: i32,
    clip: Option<[i32; 4]>,
) {
    let Some(src) = f.bg_image.as_deref() else { return };
    let mut vis = [f.x, sy.max(TOP), f.x.saturating_add(f.w), sy.saturating_add(f.h).min(bottom)];
    if let Some(c) = clip {
        vis = [vis[0].max(c[0]), vis[1].max(c[1]), vis[2].min(c[2]), vis[3].min(c[3])];
    }
    if vis[0] >= vis[2] || vis[1] >= vis[3] {
        return;
    }
    /* A gradient renders directly; a url resolves and blits once decoded. */
    if super::grad::is_gradient(src) {
        super::grad::paint_gradient(fb, src, [f.x, sy, f.w, f.h], vis);
        return;
    }
    let Some(base) = state.base.as_ref() else { return };
    let abs = crate::browser::url::join(base, src);
    let Some(img) = state.images.ready(&abs) else { return };
    /* Sized from the natural size: the raster may be decoded smaller. */
    let nat = state.images.natural(&abs).unwrap_or((img.w, img.h));
    let tile = f.bg_layer.tile(nat, [f.x, sy, f.w.max(0), f.h.max(0)]);
    if f.mask {
        return super::mask::paint_mask(fb, img, f, tile, vis);
    }
    super::bg_tile::paint_tiles(fb, img, f.alpha, (tile, f.bg_layer.repeat), vis);
}
