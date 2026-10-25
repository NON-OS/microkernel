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

//! Where a picture lands fitted whole inside an output of another size, as
//! a mirror shows it: its box, shape kept and centred, and where any
//! rectangle of it lands, grown outward to whole output pixels. Pure.

use crate::state::damage::Rect;

/// Where the picture sits on an output: x, y, width, height.
pub fn fit_box(src: (u32, u32), dst: (u32, u32)) -> (u32, u32, u32, u32) {
    let (sw, sh, dw, dh) = (src.0 as u64, src.1 as u64, dst.0 as u64, dst.1 as u64);
    if sw == 0 || sh == 0 {
        return (0, 0, 0, 0);
    }
    let (bw, bh) = if dw * sh <= dh * sw { (dw, sh * dw / sw) } else { (sw * dh / sh, dh) };
    (((dw - bw) / 2) as u32, ((dh - bh) / 2) as u32, bw as u32, bh as u32)
}

/// The output rectangle `rect` of the picture covers; None when none.
pub fn fitted(src: (u32, u32), dst: (u32, u32), rect: Rect) -> Option<Rect> {
    let (ox, oy, bw, bh) = fit_box(src, dst);
    if bw == 0 || bh == 0 || rect.x >= src.0 || rect.y >= src.1 {
        return None;
    }
    let (sw, sh, bw, bh) = (src.0 as u64, src.1 as u64, bw as u64, bh as u64);
    let x1 = (rect.x as u64 + rect.width as u64).min(sw);
    let y1 = (rect.y as u64 + rect.height as u64).min(sh);
    let (dx0, dy0) = (rect.x as u64 * bw / sw, rect.y as u64 * bh / sh);
    let (dx1, dy1) = ((x1 * bw).div_ceil(sw).min(bw), (y1 * bh).div_ceil(sh).min(bh));
    let (x, y) = (ox + dx0 as u32, oy + dy0 as u32);
    let (width, height) = (dx1.saturating_sub(dx0) as u32, dy1.saturating_sub(dy0) as u32);
    (width > 0 && height > 0).then_some(Rect { x, y, width, height })
}
