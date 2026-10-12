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

//! The refresh rates a DisplayID panel takes: 1.3's video timing range
//! limits (tag 0x09: pixel clocks, line rates, then the lowest and highest
//! field rate in Hz at bytes 10 and 11) and 2.0's dynamic video timing range
//! limits (tag 0x25: pixel clocks, then the lowest field rate at byte 6 and
//! the highest at byte 7 with its two high bits in byte 8, bits 1-0, so a
//! panel past 255 Hz can say so). Recalled from the VESA documents,
//! unverified.

use super::section::Section;
use crate::edid::Edid;

pub const TAG_RANGE_V1: u8 = 0x09;
pub const TAG_RANGE_V2: u8 = 0x25;

/// The lowest and highest refresh rate in Hz.
pub fn decode(b: &[u8], v2: bool) -> Option<(u32, u32)> {
    let (min, max) = if v2 {
        let b = b.get(..9)?;
        (b[6] as u32, b[7] as u32 | ((b[8] as u32) & 3) << 8)
    } else {
        let b = b.get(..12)?;
        (b[10] as u32, b[11] as u32)
    };
    (min > 0 && min <= max).then_some((min, max))
}

impl Edid<'_> {
    /// The first DisplayID range limits block's refresh rates.
    pub fn displayid_refresh(&self) -> Option<(u32, u32)> {
        self.extensions().filter_map(Section::new).find_map(|s| {
            let (tag, v2) = if s.v2() { (TAG_RANGE_V2, true) } else { (TAG_RANGE_V1, false) };
            s.blocks().filter(|b| b.tag == tag).find_map(|b| decode(b.body, v2))
        })
    }
}
