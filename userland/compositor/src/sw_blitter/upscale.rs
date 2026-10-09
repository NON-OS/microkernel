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

//! A rectangle of the canvas onto the screen, each pixel a square of `scale`
//! pixels: how a window drawn for the logical display reaches a panel with
//! `scale` times as many pixels each way.

use super::Surface;
use crate::state::damage::Rect;

/// Copy `rect` of `canvas` onto `screen` at `scale` times its position and
/// size, clipped to both. Returns the screen rectangle written, the one to
/// present, or `None` when none of it lands on the screen.
pub fn upscale(canvas: Surface, screen: Surface, rect: Rect, scale: u32) -> Option<Rect> {
    if scale == 0 || rect.x >= canvas.width || rect.y >= canvas.height {
        return None;
    }
    let w = rect.width.min(canvas.width - rect.x);
    let h = rect.height.min(canvas.height - rect.y);
    let (sx, sy) = (rect.x.checked_mul(scale)?, rect.y.checked_mul(scale)?);
    if w == 0 || h == 0 || sx >= screen.width || sy >= screen.height {
        return None;
    }
    let sw = w.checked_mul(scale)?.min(screen.width - sx);
    let sh = h.checked_mul(scale)?.min(screen.height - sy);
    for row in 0..sh {
        let src_va = canvas.row_start(rect.y + row / scale, rect.x, w)?;
        let dst_va = screen.row_start(sy + row, sx, sw)?;
        // SAFETY: `row_start` bounds checked the canvas row for `w` pixels and
        // the screen row for `sw`, and the canvas is never the screen when
        // `scale` is above one, so the two do not overlap.
        let src: &[u32] = unsafe { core::slice::from_raw_parts(src_va as *const u32, w as usize) };
        let dst: &mut [u32] =
            unsafe { core::slice::from_raw_parts_mut(dst_va as *mut u32, sw as usize) };
        for (i, px) in dst.iter_mut().enumerate() {
            *px = src[i / scale as usize];
        }
    }
    Some(Rect { x: sx, y: sy, width: sw, height: sh })
}
