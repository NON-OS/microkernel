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

//! A CTA-861 extension block (CTA-861-H section 7.3): tag 02, revision,
//! the offset `d` of its detailed timings, a flags byte, then the data
//! block collection from byte 4 up to `d`.

use crate::base::dtd::Detailed;
use crate::base::dtd_decode::LEN;
use crate::blocks::BLOCK;

pub const TAG: u8 = 0x02;

/// The extended tag codes this crate reads (CTA-861-H Table 54, data
/// block tag 7 with the code in the block's first byte).
pub const TAG_EXTENDED: u8 = 7;

/// The checked view of one CTA extension.
#[derive(Clone, Copy, Debug)]
pub struct Cta<'a>(&'a [u8; BLOCK]);

impl<'a> Cta<'a> {
    pub fn new(b: &'a [u8; BLOCK]) -> Option<Self> {
        (b[0] == TAG && b[1] >= 1).then_some(Cta(b))
    }

    pub fn bytes(&self) -> &'a [u8; BLOCK] {
        self.0
    }

    pub fn revision(&self) -> u8 {
        self.0[1]
    }

    /// Byte 3 bit 6, basic audio, defined from revision 2 on.
    pub fn basic_audio(&self) -> bool {
        self.revision() >= 2 && self.0[3] & 0x40 != 0
    }

    /// Where the detailed timings start. 0 says there are neither those
    /// nor data blocks; an offset past the checksum byte reads as 0.
    pub(crate) fn dtd_offset(&self) -> usize {
        match self.0[2] as usize {
            d @ 4..=127 => d,
            _ => 0,
        }
    }

    /// The detailed timings from `d` until a zero clock or byte 127; one
    /// that is no sound timing is passed over, not taken as the end.
    pub fn detailed(&self) -> impl Iterator<Item = Detailed> + 'a {
        let b: &'a [u8] = self.0;
        let first = self.dtd_offset().max(4);
        let n = if self.dtd_offset() == 0 { 0 } else { (BLOCK - 1 - first) / LEN };
        let slots = (0..n).map(move |i| &b[first + i * LEN..first + (i + 1) * LEN]);
        slots.take_while(|d| d[0] != 0 || d[1] != 0).filter_map(Detailed::decode)
    }
}
