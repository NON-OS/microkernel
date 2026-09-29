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

/* Paint a background image layer: the tile placed by background-size and
 * background-position ([x, y, w, h]), once, or repeated from there in both
 * directions across the box per background-repeat. `clip` ([x0, y0, x1,
 * y1)) is the part of the box that shows; only tiles meeting it are drawn,
 * and each draws only its visible pixels. */
pub(super) fn paint_tiles(
    fb: &mut PaintBuffer,
    img: &Decoded,
    alpha: u8,
    (tile, repeat): ([i32; 4], bool),
    clip: [i32; 4],
) {
    let [tx, ty, tw, th] = tile;
    if img.w == 0 || img.h == 0 || tw <= 0 || th <= 0 {
        return;
    }
    let mut blit =
        |rect: [i32; 4], fit: ObjectFit| blit_rect(fb, img, rect, fit, alpha, Some(clip));
    if !repeat {
        return blit(tile, ObjectFit::Fill);
    }
    /* The first tile at or before each clip edge, stepping from the anchor. */
    let first = |lo: i32, at: i32, step: i32| {
        at as i64 + (lo as i64 - at as i64).div_euclid(step as i64) * step as i64
    };
    let (x0, y0) = (first(clip[0], tx, tw), first(clip[1], ty, th));
    let cols = (clip[2] as i64 - x0 + tw as i64 - 1) / tw as i64;
    let rows = (clip[3] as i64 - y0 + th as i64 - 1) / th as i64;
    /* Bound the tile count so a one-pixel pattern cannot spin the painter. */
    if cols.saturating_mul(rows) > 4096 {
        return blit([clip[0], clip[1], clip[2] - clip[0], clip[3] - clip[1]], ObjectFit::Cover);
    }
    for r in 0..rows.max(0) {
        for c in 0..cols.max(0) {
            let (x, y) = (x0 + c * tw as i64, y0 + r * th as i64);
            blit([x as i32, y as i32, tw, th], ObjectFit::Fill);
        }
    }
}
