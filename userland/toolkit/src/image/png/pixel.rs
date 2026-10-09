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

use crate::image::types::DecodeError;

use super::header::Header;
use super::palette::Palette;
use super::sample::{sample, to8};

fn argb(a: u32, r: u32, g: u32, b: u32) -> u32 {
    (a << 24) | (r << 16) | (g << 8) | b
}

/* Pixel `i` of an unfiltered row as ARGB8888. */
pub(super) fn pixel(
    h: &Header,
    pal: &Palette<'_>,
    row: &[u8],
    i: usize,
) -> Result<u32, DecodeError> {
    let d = h.bit_depth;
    let s = |k: usize| sample(row, i * h.channels + k, d);
    let keyed = |v: [u16; 3]| pal.key.iter().zip(v).all(|(k, v)| *k == Some(v));
    Ok(match h.color_type {
        0 => {
            let v = s(0);
            let g = to8(v, d);
            argb(if keyed([v; 3]) { 0 } else { 255 }, g, g, g)
        }
        2 => {
            let v = [s(0), s(1), s(2)];
            let a = if keyed(v) { 0 } else { 255 };
            argb(a, to8(v[0], d), to8(v[1], d), to8(v[2], d))
        }
        3 => {
            let idx = s(0) as usize;
            let e = idx.saturating_mul(3);
            let rgb = pal.plte.get(e..e + 3).ok_or(DecodeError::BadMagic)?;
            let a = pal.trns.get(idx).copied().unwrap_or(255) as u32;
            argb(a, rgb[0] as u32, rgb[1] as u32, rgb[2] as u32)
        }
        4 => {
            let g = to8(s(0), d);
            argb(to8(s(1), d), g, g, g)
        }
        /* 6, truecolor with alpha: the header admits no other type. */
        _ => argb(to8(s(3), d), to8(s(0), d), to8(s(1), d), to8(s(2), d)),
    })
}
