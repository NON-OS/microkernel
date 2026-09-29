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

use nonos_toolkit::decorations::{frame_rect, radius, BORDER_PX, FRAME_BORDER, TRANSPARENT};
use nonos_toolkit::paint::radius::{clamp_radius, coverage};

use crate::paint::PaintBuffer;

/* The part of `frame_finish::finish` that lands inside the content area:
 * the rounded bottom corners. A partial repaint redraws content rows, so
 * the corner pixels among rows [y0, y1) are fresh and take the transparent
 * cut and the border arc once, exactly as a full paint gives them. The
 * straight border edges sit outside the content and are left alone. */
pub(super) fn finish_band(fb: &mut PaintBuffer, maximized: bool, y0: u32, y1: u32) {
    let f = frame_rect(fb.width, fb.height, maximized);
    let r = clamp_radius(f.w, f.h, radius(maximized));
    let t = BORDER_PX;
    if r == 0 || f.w <= t * 2 || f.h <= t * 2 {
        return;
    }
    let (iw, ih) = (f.w - t * 2, f.h - t * 2);
    let ir = clamp_radius(iw, ih, r.saturating_sub(t));
    let (a, rgb) = ((FRAME_BORDER >> 24) & 0xFF, FRAME_BORDER & 0x00FF_FFFF);
    let len = fb.pixels.len();
    for row in f.h - r..f.h {
        let py = f.y + row;
        if py < y0 || py >= y1 {
            continue;
        }
        for col in (0..r).chain(f.w - r..f.w) {
            let outer = coverage(col, row, f.w, f.h, r);
            if outer == 0 {
                let idx = (py * fb.stride_words + f.x + col) as usize;
                if idx < len {
                    fb.pixels[idx] = TRANSPARENT;
                }
                continue;
            }
            let inside = row >= t && row < f.h - t && col >= t && col < f.w - t;
            let inner = if inside { coverage(col - t, row - t, iw, ih, ir) } else { 0 };
            let cov = outer.saturating_sub(inner);
            if cov != 0 {
                fb.blend_px(f.x + col, py, ((a * cov / 255) << 24) | rgb);
            }
        }
    }
}
