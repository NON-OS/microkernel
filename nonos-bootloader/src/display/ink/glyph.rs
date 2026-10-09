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

//! One glyph of a face: where it sits, how far it moves the pen, and its
//! coverage, read from the atlas with every offset checked.

use super::atlas::{u16_at, u32_at, Face, ATLAS};

const GLYPH_LEN: usize = 16;

#[derive(Clone, Copy)]
pub struct Glyph {
    pub left: i32,
    pub top: i32,
    pub w: u32,
    pub h: u32,
    pub adv64: u32,
    pub(super) data: usize,
}

/// The glyph for byte `code`, if the atlas has one.
pub fn glyph(f: &Face, code: u8) -> Option<Glyph> {
    (0..f.count).map(|i| f.table + i * GLYPH_LEN).find(|&at| ATLAS.get(at) == Some(&code)).and_then(
        |at| {
            Some(Glyph {
                left: u16_at(at + 2)? as i16 as i32,
                top: u16_at(at + 4)? as i16 as i32,
                w: u16_at(at + 6)? as u32,
                h: u16_at(at + 8)? as u32,
                adv64: u16_at(at + 10)? as u32,
                data: u32_at(at + 12)? as usize,
            })
        },
    )
}

/// Coverage of pixel (x, y) of `g`, 0 to 15; 0 outside the atlas.
pub fn coverage(g: &Glyph, x: u32, y: u32) -> u32 {
    let i = (y * g.w + x) as usize;
    let b = ATLAS.get(g.data + i / 2).copied().unwrap_or(0) as u32;
    if i % 2 == 0 {
        b >> 4
    } else {
        b & 15
    }
}
