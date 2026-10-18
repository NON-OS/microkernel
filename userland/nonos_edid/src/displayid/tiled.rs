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

//! Tiled display topology, one panel as a grid of outputs (DisplayID 2.0
//! tag 0x28, 1.3 tag 0x12, one 22 byte payload). Bit placement unverified.

use super::section::Section;
use crate::edid::Edid;

pub const TAG_TILED_V2: u8 = 0x28;
pub const TAG_TILED_V1: u8 = 0x12;

/// Where this output's tile sits and how big the whole panel is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tile {
    pub h_tiles: u32,
    pub v_tiles: u32,
    pub h_location: u32,
    pub v_location: u32,
    /// This tile's picture.
    pub width: u32,
    pub height: u32,
    /// Byte 0 bit 7: the tiles share one enclosure.
    pub one_enclosure: bool,
    /// The topology ID every tile of one panel repeats, to group outputs.
    pub vendor: [u8; 3],
    pub product: u16,
    pub serial: u32,
}

impl Tile {
    /// Byte 1: tile counts less one, byte 2: location, low nibbles; byte
    /// 3 two high bits of each; 4-7 size less one; 13-21 topology ID.
    pub fn decode(b: &[u8]) -> Option<Self> {
        let b = b.get(..22)?;
        let n = |lo: u8, hi: u8| lo as u32 | ((hi as u32) & 3) << 4;
        let t = Tile {
            h_tiles: n(b[1] >> 4, b[3] >> 6) + 1,
            v_tiles: n(b[1] & 0xF, b[3] >> 4) + 1,
            h_location: n(b[2] >> 4, b[3] >> 2),
            v_location: n(b[2] & 0xF, b[3]),
            width: u16::from_le_bytes([b[4], b[5]]) as u32 + 1,
            height: u16::from_le_bytes([b[6], b[7]]) as u32 + 1,
            one_enclosure: b[0] & 0x80 != 0,
            vendor: [b[13], b[14], b[15]],
            product: u16::from_le_bytes([b[16], b[17]]),
            serial: u32::from_le_bytes([b[18], b[19], b[20], b[21]]),
        };
        (t.h_location < t.h_tiles && t.v_location < t.v_tiles).then_some(t)
    }
}

impl Edid<'_> {
    /// The first tiled topology block in any DisplayID extension.
    pub fn tile(&self) -> Option<Tile> {
        let want = |s: &Section<'_>| if s.v2() { TAG_TILED_V2 } else { TAG_TILED_V1 };
        self.extensions().filter_map(Section::new).find_map(|s| {
            let tag = want(&s);
            s.blocks().filter(|b| b.tag == tag).find_map(|b| Tile::decode(b.body))
        })
    }
}
