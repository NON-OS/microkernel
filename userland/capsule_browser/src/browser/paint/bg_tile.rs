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

use crate::browser::css::{BgSize, ObjectFit};
use crate::browser::image::{blit_rect, Decoded};
use crate::browser::layout::boxmodel::Fragment;

/* Paint an auto or length background layer: the tile keeps the image aspect at
 * its natural or scaled width and repeats across the box per background-repeat,
 * which is how CSS lays a pattern by default. `dest` is the box rect and
 * `clip` ([x0, y0, x1, y1)) the part of it that shows; only tiles meeting the
 * clip are drawn, and each draws only its visible pixels. */
pub(super) fn paint_tiles(
    fb: &mut PaintBuffer,
    img: &Decoded,
    f: &Fragment,
    dest: [i32; 4],
    clip: [i32; 4],
) {
    let [bx, by, bw, bh] = dest;
    if img.w == 0 || img.h == 0 || bw <= 0 || bh <= 0 {
        return;
    }
    let tw = match f.bg_size {
        BgSize::Px(px) => (px as i32).max(1),
        _ => img.w.min(i32::MAX as u32) as i32,
    };
    let th = ((tw as u64 * img.h as u64) / img.w as u64).clamp(1, i32::MAX as u64) as i32;
    let mut blit =
        |rect: [i32; 4], fit: ObjectFit| blit_rect(fb, img, rect, fit, f.alpha, Some(clip));
    if !f.bg_repeat {
        blit([bx, by, tw.min(bw), th.min(bh)], ObjectFit::Fill);
        return;
    }
    /* Bound the tile count so a one-pixel pattern cannot spin the painter. */
    let cols = (bw as i64 + tw as i64 - 1) / tw as i64;
    let rows = (bh as i64 + th as i64 - 1) / th as i64;
    if cols.saturating_mul(rows) > 4096 {
        blit([bx, by, bw, bh], ObjectFit::Cover);
        return;
    }
    let span = |lo: i32, hi: i32, at: i32, step: i32, n: i64| {
        let first = ((lo as i64 - at as i64) / step as i64).max(0);
        (first, ((hi as i64 - at as i64 + step as i64 - 1) / step as i64).min(n))
    };
    let (c0, c1) = span(clip[0], clip[2], bx, tw, cols);
    let (r0, r1) = span(clip[1], clip[3], by, th, rows);
    for r in r0..r1 {
        for c in c0..c1 {
            blit([bx + c as i32 * tw, by + r as i32 * th, tw, th], ObjectFit::Fill);
        }
    }
}
