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

//! A DisplayID section in an EDID extension block, tag 0x70 (DisplayID
//! 2.0 section 2.2, unverified section number). From the block's byte 1:
//! version, payload length, use case, extension count, the payload's
//! data blocks, then the section's own checksum byte.

use crate::blocks::BLOCK;

pub const TAG: u8 = 0x70;

/// A section whose own checksum holds.
#[derive(Clone, Copy, Debug)]
pub struct Section<'a> {
    pub version: u8,
    payload: &'a [u8],
}

/// One data block: tag, revision byte and payload (section 2.3, the
/// three byte data block header).
#[derive(Clone, Copy, Debug)]
pub struct Block<'a> {
    pub tag: u8,
    pub revision: u8,
    pub body: &'a [u8],
}

impl<'a> Section<'a> {
    /// The section in an extension block. The section's bytes, five
    /// plus the payload, sum to zero; a length running past byte 126
    /// (127 is the EDID block's checksum) is refused.
    pub fn new(ext: &'a [u8; BLOCK]) -> Option<Self> {
        if ext[0] != TAG {
            return None;
        }
        let s = &ext[1..BLOCK - 1];
        let total = s[1] as usize + 5;
        let whole = s.get(..total)?;
        if whole.iter().fold(0u8, |a, &v| a.wrapping_add(v)) != 0 {
            return None;
        }
        Some(Section { version: s[0], payload: &whole[4..total - 1] })
    }

    /// DisplayID 2.x, whose tags this crate reads for timings.
    pub fn v2(&self) -> bool {
        self.version >> 4 == 2
    }

    /// The data blocks; one running past the payload ends the walk.
    pub fn blocks(&self) -> impl Iterator<Item = Block<'a>> {
        let p = self.payload;
        let mut at = 0;
        core::iter::from_fn(move || {
            let h = p.get(at..at + 3)?;
            let body = p.get(at + 3..at + 3 + h[2] as usize)?;
            at += 3 + body.len();
            Some(Block { tag: h[0], revision: h[1], body })
        })
    }
}
