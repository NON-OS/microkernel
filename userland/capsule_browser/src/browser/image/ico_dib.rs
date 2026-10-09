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

use alloc::vec::Vec;

use crate::browser::image::store::Decoded;

/* Icon sides beyond this are not icons. */
const MAX_SIDE: usize = 1024;

/// An icon's DIB: a BITMAPINFOHEADER whose height counts the colour rows
/// and the 1-bit AND mask below them, a palette for 1/4/8-bit colour, then
/// bottom-up colour rows. Opacity is the 32-bit alpha, or the mask where a
/// set bit is transparent (also for 32-bit icons that carry no alpha).
pub(crate) fn decode_dib(d: &[u8]) -> Option<Decoded> {
    let le = |i: usize, n: usize| -> Option<usize> {
        let s = d.get(i..i + n)?;
        Some(s.iter().rev().fold(0usize, |a, &b| (a << 8) | b as usize))
    };
    let (hsz, w, h2, bpp, comp) = (le(0, 4)?, le(4, 4)?, le(8, 4)?, le(14, 2)?, le(16, 4)?);
    let h = h2 / 2;
    if hsz < 40 || w == 0 || h == 0 || w > MAX_SIDE || h > MAX_SIDE || !matches!(comp, 0 | 3) {
        return None;
    }
    let colors =
        if bpp <= 8 { le(32, 4).filter(|&c| c > 0 && c <= 256).unwrap_or(1 << bpp) } else { 0 };
    let pal = hsz + if comp == 3 && bpp == 32 { 12 } else { 0 };
    let (xor, stride) = (pal + colors * 4, (w * bpp).div_ceil(32) * 4);
    let (and, astride) = (xor + stride * h, w.div_ceil(32) * 4);
    let mut px = Vec::with_capacity(w * h);
    for y in 0..h {
        let row = (h - 1 - y) * stride + xor;
        for x in 0..w {
            let p = match bpp {
                32 => le(row + 4 * x, 4)? as u32,
                24 => 0xFF00_0000 | le(row + 3 * x, 3)? as u32,
                1 | 4 | 8 => {
                    let bit = x * bpp;
                    let idx =
                        (*d.get(row + bit / 8)? as usize >> (8 - bpp - bit % 8)) & ((1 << bpp) - 1);
                    0xFF00_0000 | le(pal + 4 * idx.min(colors.max(1) - 1), 3)? as u32
                }
                _ => return None,
            };
            px.push(p);
        }
    }
    let has_alpha = bpp == 32 && px.iter().any(|p| p >> 24 != 0);
    if !has_alpha {
        for (i, p) in px.iter_mut().enumerate() {
            let (x, y) = (i % w, i / w);
            let m = d.get(and + (h - 1 - y) * astride + x / 8).map_or(0, |b| b >> (7 - x % 8) & 1);
            *p = if m == 1 { 0 } else { *p | 0xFF00_0000 };
        }
    }
    Some(Decoded { w: w as u32, h: h as u32, px })
}
