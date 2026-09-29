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

use ab_glyph::{Font, OutlinedGlyph};

use super::raster::{box_area, MAX_GLYPH_AREA};
use super::rasterize::rasterize;
use super::store::{Key, Store};
use super::target::Target;

/* Where a cached glyph lands: its pen origin on the baseline, the colour,
and how coverage rounds when it is first rasterised (0.5 to nearest; 0.0
truncates, as the chrome cache always has). */
pub(super) struct Place {
    pub x: i32,
    pub y: i32,
    pub argb: u32,
    pub bias: f32,
}

/* Draw the glyph `key` names, from the cache when it is there, else from
`outline`, keeping the raster for the next frame. A glyph box over the
fixed cap is remembered as blank; one over this surface's own limit is
skipped without being rasterised or remembered. */
pub(super) fn draw_glyph(
    store: &mut Store,
    t: &mut Target,
    key: Key,
    outline: impl FnOnce() -> Option<OutlinedGlyph>,
    at: Place,
) {
    if let Some(slot) = store.get(&key) {
        if let Some(r) = slot.as_ref().filter(|r| r.area() <= t.glyph_limit()) {
            t.blit(r, at.x + r.min_x, at.y + r.min_y, at.argb);
        }
        return;
    }
    let Some(og) = outline() else { return store.put(key, None) };
    let bb = og.px_bounds();
    let area = box_area(bb.width(), bb.height());
    if area > MAX_GLYPH_AREA {
        return store.put(key, None);
    }
    if area > t.glyph_limit() {
        return;
    }
    let raster = rasterize(&og, at.bias);
    if let Some(r) = &raster {
        t.blit(r, at.x + r.min_x, at.y + r.min_y, at.argb);
    }
    store.put(key, raster);
}

/* The identity the cache knows a face by: its font data's address and length. */
pub(super) fn face_id<F: Font>(f: &F) -> (usize, usize) {
    let data = f.font_data();
    (data.as_ptr() as usize, data.len())
}
