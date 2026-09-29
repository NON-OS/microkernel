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

use super::gauss::blurred;
use super::rrect::RRect;
use super::Window;
use crate::browser::paint::grad::put_pixel;

/// An outer shadow: the box's rounded rect moved by (dx, dy) and grown by
/// the spread (`g` is [dx, dy, blur, spread]), blurred, and painted only
/// outside the border box, which never shows its own shadow through.
pub(super) fn paint(fb: &mut PaintBuffer, rect: &RRect, g: [f32; 4], color: u32, win: Window) {
    let s = rect.spread(g[0], g[1], g[3]);
    let (blur, ext) = (g[2], g[2] + 1.0);
    let x0 = ((s.x - ext) as i32).max(win[0]);
    let x1 = ((s.x + s.w + ext) as i32 + 1).min(win[2]);
    let y0 = ((s.y - ext) as i32).max(win[1]);
    let y1 = ((s.y + s.h + ext) as i32 + 1).min(win[3]);
    let (top, bot) = rect.caps();
    let (a, rgb) = ((color >> 24) as f32, color & 0x00ff_ffff);
    let (inner0, inner1) = (rect.x as i32, (rect.x + rect.w) as i32);
    for py in y0..y1 {
        let cy = py as f32 + 0.5;
        /* Between the corners the box interior is solid: skip it. */
        let straight = cy > rect.y + top && cy < rect.y + rect.h - bot;
        let mut px = x0;
        while px < x1 {
            if straight && px >= inner0 && px < inner1 {
                px = inner1;
                continue;
            }
            let cx = px as f32 + 0.5;
            let inside = (0.5 - rect.dist(cx, cy)).clamp(0.0, 1.0);
            if inside < 1.0 {
                let cov = blurred(s.dist(cx, cy), blur) * (1.0 - inside);
                let al = (a * cov + 0.5) as u32;
                if al > 0 {
                    put_pixel(fb, px, py, (al.min(255) << 24) | rgb);
                }
            }
            px += 1;
        }
    }
}
