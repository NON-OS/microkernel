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

/// An inset shadow: inside the padding box, everything outside a hole
/// that is the padding box moved by (dx, dy) and shrunk by the spread
/// (`g` is [dx, dy, blur, spread]), with the hole's edge blurred.
pub(super) fn paint(
    fb: &mut PaintBuffer,
    rect: &RRect,
    border: [u32; 4],
    g: [f32; 4],
    color: u32,
    win: Window,
) {
    let Some(pad) = rect.padding_box(border) else { return };
    let hole = pad.spread(g[0], g[1], -g[3]);
    let (blur, m) = (g[2], g[2] + 1.0);
    let (x0, x1) = ((pad.x as i32).max(win[0]), ((pad.x + pad.w) as i32 + 1).min(win[2]));
    let (y0, y1) = ((pad.y as i32).max(win[1]), ((pad.y + pad.h) as i32 + 1).min(win[3]));
    let (top, bot) = hole.caps();
    let (clear0, clear1) = ((hole.x + m) as i32 + 1, (hole.x + hole.w - m) as i32);
    let (a, rgb) = ((color >> 24) as f32, color & 0x00ff_ffff);
    for py in y0..y1 {
        let cy = py as f32 + 0.5;
        /* Well inside the hole the shadow is zero: skip that span. */
        let clear = cy > hole.y + top + m && cy < hole.y + hole.h - bot - m;
        let mut px = x0;
        while px < x1 {
            if clear && px >= clear0 && px < clear1 {
                px = clear1;
                continue;
            }
            let cx = px as f32 + 0.5;
            let inside = (0.5 - pad.dist(cx, cy)).clamp(0.0, 1.0);
            let cov = (1.0 - blurred(hole.dist(cx, cy), blur)) * inside;
            let al = (a * cov + 0.5) as u32;
            if al > 0 {
                put_pixel(fb, px, py, (al.min(255) << 24) | rgb);
            }
            px += 1;
        }
    }
}
