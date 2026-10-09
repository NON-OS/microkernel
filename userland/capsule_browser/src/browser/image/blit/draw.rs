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

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;

use super::super::store::Decoded;
use super::sample::{bilinear, put, taps};

/* Bilinear-blit the source rect [sx0, sy0, sw, sh] of `img` into the
 * destination rect [dx, dy, dw, dh]. The destination is cut to the clip and
 * the framebuffer once and only that part is visited, so the cost follows
 * the visible area, not the box. Source taps and weights come from the
 * unclipped origin (per column once per blit, per row once per row), so a
 * clipped draw samples exactly as an unclipped one; 1:1 copies skip the
 * filter. */
pub(super) fn draw(
    fb: &mut PaintBuffer,
    img: &Decoded,
    dest: [i32; 4],
    src: [u32; 4],
    alpha: u8,
    clip: Option<[i32; 4]>,
) {
    let [dx, dy, dw, dh] = dest;
    let [sx0, sy0, sw, sh] = src;
    let w = img.w as usize;
    if dw <= 0 || dh <= 0 || sw == 0 || sh == 0 || img.px.len() < w.saturating_mul(img.h as usize) {
        return;
    }
    let Some([x0, y0, x1, y1]) = visible(dest, clip, fb) else { return };
    let at = |s: u32, i: i32, span: u32, d: i32| {
        (s as u64 * 256 + i as u64 * span as u64 * 256 / d as u64) as u32
    };
    let cols: Vec<[u32; 3]> = (x0..x1).map(|px| taps(at(sx0, px - dx, sw, dw), img.w)).collect();
    let copy = sw as i32 == dw && sh as i32 == dh;
    for py in y0..y1 {
        let [r0, r1, fy] = taps(at(sy0, py - dy, sh, dh), img.h).map(|v| v as usize);
        let (row0, row1) = (&img.px[r0 * w..r0 * w + w], &img.px[r1 * w..r1 * w + w]);
        for (px, &[c0, c1, fx]) in (x0..x1).zip(cols.iter()) {
            let (c0, c1) = (c0 as usize, c1 as usize);
            let argb = if copy {
                row0[c0]
            } else {
                bilinear([row0[c0], row0[c1], row1[c0], row1[c1]], fx, fy as u32)
            };
            put(fb, px as u32, py as u32, argb, alpha);
        }
    }
}

/* The destination rect [x, y, w, h] cut to the clip [x0, y0, x1, y1) and the
 * framebuffer, as [x0, y0, x1, y1); None when nothing shows. */
fn visible(d: [i32; 4], clip: Option<[i32; 4]>, fb: &PaintBuffer) -> Option<[i32; 4]> {
    let c = clip.unwrap_or([i32::MIN, i32::MIN, i32::MAX, i32::MAX]);
    let x0 = d[0].max(c[0]).max(0);
    let y0 = d[1].max(c[1]).max(0);
    let x1 = (d[0] as i64 + d[2] as i64).min(c[2] as i64).min(fb.width as i64) as i32;
    let y1 = (d[1] as i64 + d[3] as i64).min(c[3] as i64).min(fb.height as i64) as i32;
    (x0 < x1 && y0 < y1).then_some([x0, y0, x1, y1])
}
