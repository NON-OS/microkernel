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

//! Geist, JetBrains Mono and the NØNOS mark, pre-rendered by
//! assets/fonts/make_atlas.py and embedded whole.
//! Every read is bounds-checked: a short or damaged atlas draws nothing.

use super::bytes::le;

pub(super) static ATLAS: &[u8] = include_bytes!("../../../assets/fonts/nonos.nxf");

const FACE_AT: usize = 8;
const FACE_LEN: usize = 16;

#[derive(Clone, Copy)]
pub struct Face {
    pub px: u32,
    pub ascent: u32,
    pub line: u32,
    pub(super) count: usize,
    pub(super) table: usize,
}

impl Face {
    pub const EMPTY: Face = Face { px: 0, ascent: 0, line: 0, count: 0, table: 0 };
}

pub(super) fn u16_at(at: usize) -> Option<u16> {
    le(ATLAS, at, 2).map(|v| v as u16)
}

pub(super) fn u32_at(at: usize) -> Option<u32> {
    le(ATLAS, at, 4)
}

/// Face `i` of the atlas, in the order make_atlas.py writes them.
pub fn face(i: usize) -> Option<Face> {
    if ATLAS.get(..4)? != b"NXF1" || i >= *ATLAS.get(4)? as usize {
        return None;
    }
    let at = FACE_AT + i * FACE_LEN;
    let (ascent, line) = (u16_at(at + 2)? as u32, u16_at(at + 4)? as u32);
    let (count, table) = (u16_at(at + 6)? as usize, u32_at(at + 8)? as usize);
    Some(Face { px: *ATLAS.get(at)? as u32, ascent, line, count, table })
}
