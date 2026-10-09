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

use alloc::vec;

use super::axis::axis;
use super::window::cover_window;

// Scale `src` (ARGB rows, `sw` by `sh`) to fill `dw` by `dh` at its own
// aspect, cropping the overflow evenly. Rows go to `put_row` top to bottom.
pub fn scale_cover(
    src: &[u32],
    sw: u32,
    sh: u32,
    dw: u32,
    dh: u32,
    mut put_row: impl FnMut(u32, &[u32]),
) -> bool {
    let need = (sw as usize).checked_mul(sh as usize);
    if need.is_none_or(|n| n == 0 || src.len() < n) || dw == 0 || dh == 0 {
        return false;
    }
    let (x0, y0, cw, ch) = cover_window(sw, sh, dw, dh);
    let (Some(ax), Some(ay)) = (axis(sw, x0, cw, dw), axis(sh, y0, ch, dh)) else {
        return false;
    };
    let mut acc = vec![0u64; dw as usize * 3];
    let mut row = vec![0u32; dw as usize];
    for dy in 0..dh as usize {
        acc.iter_mut().for_each(|a| *a = 0);
        for ty in 0..ay.taps {
            let wy = ay.weight[dy * ay.taps + ty] as u64;
            let sy = ay.start[dy] as usize + ty;
            if wy == 0 || sy >= sh as usize {
                continue;
            }
            let line = &src[sy * sw as usize..(sy + 1) * sw as usize];
            super::cover_row::accumulate(line, &ax, wy, &mut acc);
        }
        for (px, a) in row.iter_mut().zip(acc.chunks(3)) {
            // Two 12-bit weights multiplied: 24 fractional bits, rounded.
            let ch = |v: u64| ((v + (1 << 23)) >> 24).min(255) as u32;
            *px = 0xFF00_0000 | (ch(a[0]) << 16) | (ch(a[1]) << 8) | ch(a[2]);
        }
        put_row(dy as u32, &row);
    }
    true
}
