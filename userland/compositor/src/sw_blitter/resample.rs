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

//! The screen onto another output of any size, as a mirror shows it: the
//! picture fitted whole inside the output, its shape kept and centred, the
//! bars beside it left as the output's primary came (black), each output
//! pixel the screen pixel nearest its centre. Only the part a damaged
//! rectangle covers is written.

use super::Surface;
use crate::state::damage::Rect;

/// Where the screen's picture sits on an output: x, y, width, height.
pub fn fit_box(src: (u32, u32), dst: (u32, u32)) -> (u32, u32, u32, u32) {
    let (sw, sh, dw, dh) = (src.0 as u64, src.1 as u64, dst.0 as u64, dst.1 as u64);
    if sw == 0 || sh == 0 {
        return (0, 0, 0, 0);
    }
    let (bw, bh) = if dw * sh <= dh * sw { (dw, sh * dw / sw) } else { (sw * dh / sh, dh) };
    (((dw - bw) / 2) as u32, ((dh - bh) / 2) as u32, bw as u32, bh as u32)
}

/// Draw `rect` of `src` onto `dst`; the output rectangle written.
pub fn resample(src: Surface, dst: Surface, rect: Rect) -> Option<Rect> {
    let (ox, oy, bw, bh) = fit_box((src.width, src.height), (dst.width, dst.height));
    let (sw, sh) = (src.width as u64, src.height as u64);
    if bw == 0 || bh == 0 || rect.x >= src.width || rect.y >= src.height {
        return None;
    }
    let x1 = (rect.x as u64 + rect.width as u64).min(sw);
    let y1 = (rect.y as u64 + rect.height as u64).min(sh);
    let (bw, bh) = (bw as u64, bh as u64);
    let (dx0, dy0) = (rect.x as u64 * bw / sw, rect.y as u64 * bh / sh);
    let (dx1, dy1) = ((x1 * bw).div_ceil(sw).min(bw), (y1 * bh).div_ceil(sh).min(bh));
    if dx0 >= dx1 || dy0 >= dy1 {
        return None;
    }
    let n = (dx1 - dx0) as usize;
    for dy in dy0..dy1 {
        let sy = ((dy * 2 + 1) * sh / (bh * 2)) as u32;
        let src_va = src.row_start(sy, 0, src.width)?;
        let dst_va = dst.row_start(oy + dy as u32, ox + dx0 as u32, n as u32)?;
        // SAFETY: `row_start` bounds checked both rows; a mirror's output is
        // another driver primary, never the screen.
        let (s, d) = unsafe {
            let s = core::slice::from_raw_parts(src_va as *const u32, src.width as usize);
            (s, core::slice::from_raw_parts_mut(dst_va as *mut u32, n))
        };
        for (i, px) in d.iter_mut().enumerate() {
            *px = s[(((dx0 + i as u64) * 2 + 1) * sw / (bw * 2)) as usize];
        }
    }
    let (x, y) = (ox + dx0 as u32, oy + dy0 as u32);
    Some(Rect { x, y, width: n as u32, height: (dy1 - dy0) as u32 })
}
