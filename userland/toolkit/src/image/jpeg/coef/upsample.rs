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

/// A component's samples at output scale: `w` x `h` real samples in a
/// buffer of row stride `stride`, and its subsampling (fx, fy) against the
/// output grid.
pub struct Samples<'a> {
    pub px: &'a [u8],
    pub stride: usize,
    pub w: usize,
    pub h: usize,
    pub fx: usize,
    pub fy: usize,
}

/// Output row `y` of the component, `out.len()` samples wide. Halved
/// sampling uses libjpeg's triangle ("fancy") filter, 3/4 of the nearer
/// sample and 1/4 of the next, with its rounding; other ratios repeat.
pub fn row(s: &Samples, y: usize, out: &mut [u8]) {
    let at = |r: usize, c: usize| s.px[r.min(s.h - 1) * s.stride + c.min(s.w - 1)] as u32;
    let (sy, odd) = (y / s.fy, y % 2 == 1);
    /* The vertical neighbour of the fancy filter: above for an even row. */
    let far = if odd { sy + 1 } else { sy.saturating_sub(1) };
    let col = |c: usize| match s.fy {
        2 => at(sy, c) * 3 + at(far, c),
        _ => at(sy, c) * 4,
    };
    match (s.fx, s.fy) {
        (1, 1) => out.iter_mut().enumerate().for_each(|(x, o)| *o = at(sy, x) as u8),
        (1, 2) => out.iter_mut().enumerate().for_each(|(x, o)| {
            *o = ((col(x) + if odd { 2 } else { 1 }) >> 2) as u8;
        }),
        (2, fy @ (1 | 2)) => {
            /* Column sums carry weight 4 either way; h2v1 rounds +1/+2 in
             * quarters, h2v2 +8/+7 in sixteenths. */
            let bias = if fy == 1 { [4, 8] } else { [8, 7] };
            for (x, o) in out.iter_mut().enumerate() {
                let (c, right) = (x / 2, x % 2 == 1);
                let edge = (c == 0 && !right) || (c + 1 >= s.w && right);
                let near = col(c);
                let v = if edge {
                    (near * 4 + if right { 7 } else { 8 }) >> 4
                } else {
                    let other = if right { col(c + 1) } else { col(c - 1) };
                    (near * 3 + other + bias[right as usize]) >> 4
                };
                *o = v.min(255) as u8;
            }
        }
        _ => out.iter_mut().enumerate().for_each(|(x, o)| *o = at(sy, x / s.fx) as u8),
    }
}
