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
//! A rectangle of a canvas larger than the screen onto the screen, each
//! screen pixel the average of the canvas pixels it covers, so a thin
//! stroke of text turns lighter rather than vanishing as dropped pixels.

use super::Surface;
use crate::state::damage::Rect;

/// Shrink `rect` of `canvas` onto `screen`. Returns the screen rectangle
/// written, every pixel whose footprint meets `rect`, or None when none.
pub fn downscale(canvas: Surface, screen: Surface, rect: Rect) -> Option<Rect> {
    let (cw, ch) = (canvas.width as u64, canvas.height as u64);
    let (sw, sh) = (screen.width as u64, screen.height as u64);
    if sw == 0 || sh == 0 || rect.x >= canvas.width || rect.y >= canvas.height {
        return None;
    }
    let x1 = (rect.x as u64 + rect.width as u64).min(cw);
    let y1 = (rect.y as u64 + rect.height as u64).min(ch);
    let (sx0, sy0) = (rect.x as u64 * sw / cw, rect.y as u64 * sh / ch);
    let sx1 = (x1 * sw).div_ceil(cw).min(sw);
    let sy1 = (y1 * sh).div_ceil(ch).min(sh);
    if sx0 >= sx1 || sy0 >= sy1 {
        return None;
    }
    let n = (sx1 - sx0) as usize;
    for sy in sy0..sy1 {
        let rows = span(sy, sh, ch);
        let dst_va = screen.row_start(sy as u32, sx0 as u32, n as u32)?;
        // SAFETY: `row_start` bounds checked the screen row for `n` pixels,
        // and the canvas is never the screen when it is larger than it.
        let dst: &mut [u32] = unsafe { core::slice::from_raw_parts_mut(dst_va as *mut u32, n) };
        for (i, px) in dst.iter_mut().enumerate() {
            *px = average(canvas, span(sx0 + i as u64, sw, cw), rows)?;
        }
    }
    Some(Rect { x: sx0 as u32, y: sy0 as u32, width: n as u32, height: (sy1 - sy0) as u32 })
}

/// The canvas pixels screen pixel `s` of `n` covers out of `m`.
fn span(s: u64, n: u64, m: u64) -> (u64, u64) {
    let start = s * m / n;
    (start, ((s + 1) * m).div_ceil(n).clamp(start + 1, m.max(start + 1)))
}

fn average(canvas: Surface, (x0, x1): (u64, u64), (y0, y1): (u64, u64)) -> Option<u32> {
    let (mut r, mut g, mut b) = (0u64, 0u64, 0u64);
    for y in y0..y1 {
        let va = canvas.row_start(y as u32, x0 as u32, (x1 - x0) as u32)?;
        // SAFETY: `row_start` bounds checked the row for `x1 - x0` pixels.
        let row = unsafe { core::slice::from_raw_parts(va as *const u32, (x1 - x0) as usize) };
        for &p in row {
            r += ((p >> 16) & 0xFF) as u64;
            g += ((p >> 8) & 0xFF) as u64;
            b += (p & 0xFF) as u64;
        }
    }
    let k = (x1 - x0) * (y1 - y0);
    Some(0xFF00_0000 | (((r / k) as u32) << 16) | (((g / k) as u32) << 8) | (b / k) as u32)
}
