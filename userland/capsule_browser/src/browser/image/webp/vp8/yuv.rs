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

use super::recon::Frame;
use super::yuv_rgb::rgb;
use super::yuv_up::up;

/// Emit the `w` x `h` picture as ARGB rows, chroma upsampled by the 9-3-3-1
/// triangle filter and converted with the integer BT.601 transform libwebp
/// decodes with, so rows match it; `alpha` (w x h) sets opacity.
pub(super) fn emit(
    f: &Frame,
    (w, h): (usize, usize),
    alpha: Option<&[u8]>,
    row: &mut dyn FnMut(usize, &[u32]),
) {
    let mut line = alloc::vec![0u32; w];
    let last_uv = (h - 1) / 2;
    for r in 0..h {
        /* The chroma row nearer this row, the other one, and whether it is
         * the lower row of a pair. */
        let (near, far, lower) = match r {
            0 => (0, 0, false),
            _ if r % 2 == 1 => ((r - 1) / 2, ((r + 1) / 2).min(last_uv), false),
            _ => (r / 2, r / 2 - 1, true),
        };
        let (top, cur) = if lower { (far, near) } else { (near, far) };
        let yrow = &f.y.px[r * f.y.stride..r * f.y.stride + w];
        for (x, px) in line.iter_mut().enumerate() {
            let (u, v) = (
                up(&f.u.px[top * f.u.stride..], &f.u.px[cur * f.u.stride..], x, w, lower),
                up(&f.v.px[top * f.v.stride..], &f.v.px[cur * f.v.stride..], x, w, lower),
            );
            let a = alpha.map_or(255, |a| a[r * w + x] as u32);
            *px = (a << 24) | rgb(yrow[x] as i32, u, v);
        }
        row(r, &line);
    }
}
