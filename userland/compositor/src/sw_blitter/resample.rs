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

pub use super::fit::fit_box;
use super::fit::fitted;
use super::Surface;
use crate::state::damage::Rect;

/// Draw `rect` of `src` onto `dst`; the output rectangle written.
pub fn resample(src: Surface, dst: Surface, rect: Rect) -> Option<Rect> {
    let (from, to) = ((src.width, src.height), (dst.width, dst.height));
    let (ox, oy, bw, bh) = fit_box(from, to);
    let d = fitted(from, to, rect)?;
    let (sw, sh, bw, bh) = (src.width as u64, src.height as u64, bw as u64, bh as u64);
    let (dx0, dy0, n) = ((d.x - ox) as u64, (d.y - oy) as u64, d.width as usize);
    for dy in dy0..dy0 + d.height as u64 {
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
    Some(d)
}
