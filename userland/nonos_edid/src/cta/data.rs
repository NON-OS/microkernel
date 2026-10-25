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

//! The data block collection of a CTA-861 extension (CTA-861-H section
//! 7.4): from byte 4 up to the DTD offset, each block a header byte with
//! the tag in bits 7-5 and the payload length in bits 4-0.

use super::blocks::{Cta, TAG_EXTENDED};

/// One data block: its tag (bits 7-5 of the header byte), the extended
/// tag when the tag is 7, and the payload after those.
#[derive(Clone, Copy, Debug)]
pub struct DataBlock<'a> {
    pub tag: u8,
    pub extended: Option<u8>,
    pub body: &'a [u8],
}

impl<'a> Cta<'a> {
    /// The data blocks, which revision 3 introduced; a block running
    /// past `d` ends the walk rather than being read short.
    pub fn data_blocks(&self) -> impl Iterator<Item = DataBlock<'a>> {
        let b: &'a [u8] = self.bytes();
        let end = if self.revision() >= 3 { self.dtd_offset() } else { 0 };
        let mut at = 4;
        core::iter::from_fn(move || {
            let h = *b.get(at).filter(|_| at < end)? as usize;
            let body = b.get(at + 1..at + 1 + (h & 0x1F)).filter(|_| at + 1 + (h & 0x1F) <= end)?;
            at += 1 + body.len();
            let tag = (h >> 5) as u8;
            match (tag, body.split_first()) {
                (TAG_EXTENDED, Some((&x, rest))) => {
                    Some(DataBlock { tag, extended: Some(x), body: rest })
                }
                _ => Some(DataBlock { tag, extended: None, body }),
            }
        })
    }
}
