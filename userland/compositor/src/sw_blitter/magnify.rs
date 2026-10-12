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

//! The canvas onto a screen through the magnifier's lens: screen pixel s
//! shows canvas pixel o + (4s + 2) / z, the one under its centre (as the
//! fractional scales do, `stretch`), so canvas pixel c covers screen pixels
//! from ((c - o) z + 1) / 4. A screen pixel past the canvas is black.

use super::Surface;
use crate::display::magnifier_rule::Lens;
use crate::state::damage::Rect;

const BLACK: u32 = 0xFF00_0000;

/// Draw the screen pixels canvas `rect` covers; the screen rectangle drawn,
/// or None when none of it is in the lens.
pub fn magnify(view: Surface, screen: Surface, rect: Rect, l: Lens) -> Option<Rect> {
    let z = l.z.max(1) as u64;
    let first = |c: u32, o: u32| if c <= o { 0 } else { ((c - o) as u64 * z + 1) / 4 };
    let (x0, y0) = (first(rect.x, l.ox), first(rect.y, l.oy));
    let x1 = first(rect.x.saturating_add(rect.width), l.ox).min(screen.width as u64);
    let y1 = first(rect.y.saturating_add(rect.height), l.oy).min(screen.height as u64);
    if x0 >= x1 || y0 >= y1 {
        return None;
    }
    let at = |s: u64, o: u32| o as u64 + (4 * s + 2) / z;
    for sy in y0..y1 {
        let Some(dst) = screen.row_start(sy as u32, x0 as u32, (x1 - x0) as u32) else { break };
        // SAFETY: `row_start` bounds checked the row for that many pixels.
        let out = unsafe { core::slice::from_raw_parts_mut(dst as *mut u32, (x1 - x0) as usize) };
        let cy = at(sy, l.oy);
        let src =
            (cy < view.height as u64).then(|| view.row_start(cy as u32, 0, view.width)).flatten();
        for (sx, px) in (x0..x1).zip(out.iter_mut()) {
            let cx = at(sx, l.ox);
            *px = match src {
                // SAFETY: the row was bounds checked for `view.width` pixels.
                Some(s) if cx < view.width as u64 => unsafe { *(s as *const u32).add(cx as usize) },
                _ => BLACK,
            };
        }
    }
    Some(Rect { x: x0 as u32, y: y0 as u32, width: (x1 - x0) as u32, height: (y1 - y0) as u32 })
}
