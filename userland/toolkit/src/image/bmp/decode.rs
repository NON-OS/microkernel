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

use crate::image::types::{DecodeError, ImageSize};

use super::header::{parse, Layout};
use super::masks::channel;

/* Decode an uncompressed or bit-field BMP (1, 4, 8, 16, 24 or 32 bpp, bottom-up
 * or top-down) into ARGB8888. An alpha channel that is zero everywhere is the
 * reserved byte most writers leave clear, not transparency: it decodes opaque. */
pub fn decode_bmp_argb8888(input: &[u8], out: &mut [u32]) -> Result<ImageSize, DecodeError> {
    let l = parse(input)?;
    let (w, h) = (l.size.width as usize, l.size.height as usize);
    let count = w.saturating_mul(h);
    if count > out.len() {
        return Err(DecodeError::OutputTooSmall);
    }
    let last_row = w.saturating_mul(l.bpp).div_ceil(8);
    let need = l.data_off.saturating_add(l.stride.saturating_mul(h - 1)).saturating_add(last_row);
    if need > input.len() {
        return Err(DecodeError::Truncated);
    }
    let mut alpha = 0u32;
    for (y, dst) in out[..count].chunks_exact_mut(w).enumerate() {
        let src_row = if l.top_down { y } else { h - 1 - y };
        let row = &input[l.data_off + src_row * l.stride..];
        for (x, px) in dst.iter_mut().enumerate() {
            *px = pixel(&l, row, x)?;
            alpha |= *px >> 24;
        }
    }
    if alpha == 0 {
        out[..count].iter_mut().for_each(|p| *p |= 0xFF00_0000);
    }
    Ok(l.size)
}

fn pixel(l: &Layout<'_>, row: &[u8], x: usize) -> Result<u32, DecodeError> {
    let at = |i: usize| row.get(i).copied().map(u32::from).ok_or(DecodeError::Truncated);
    match l.bpp {
        1 | 4 | 8 => {
            let bit = x * l.bpp;
            let idx = (at(bit / 8)? >> (8 - l.bpp - bit % 8)) & ((1 << l.bpp) - 1);
            let e = l
                .palette
                .get(idx as usize * 4..idx as usize * 4 + 3)
                .ok_or(DecodeError::BadMagic)?;
            Ok(0xFF00_0000 | (e[2] as u32) << 16 | (e[1] as u32) << 8 | e[0] as u32)
        }
        24 => Ok(0xFF00_0000 | at(3 * x + 2)? << 16 | at(3 * x + 1)? << 8 | at(3 * x)?),
        /* 16 or 32: `readable` admits no other depth. */
        _ => {
            let n = l.bpp / 8;
            let v = (0..n)
                .try_fold(0u32, |v, k| Ok::<u32, DecodeError>(v | at(n * x + k)? << (8 * k)))?;
            let [r, g, b, a] = l.masks.map(|m| channel(v, m));
            let a = if l.masks[3] == 0 { 255 } else { a };
            Ok(a << 24 | r << 16 | g << 8 | b)
        }
    }
}
