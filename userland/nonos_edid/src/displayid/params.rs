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

//! The panel's native size from DisplayID's display parameters block: 1.3
//! tag 0x01, 2.0 tag 0x21. Both open with the image size, horizontal then
//! vertical, two bytes each in 0.1 mm (2.0's revision bit 7 set: 1 mm),
//! then the native pixel counts, two bytes each (stored less one in 2.0).
//! This names a native resolution past the base block's 4095 x 4095 and
//! its whole-millimetre size. Recalled from the VESA documents, unverified.

use super::section::Section;
use crate::edid::Edid;

pub const TAG_PARAMS_V1: u8 = 0x01;
pub const TAG_PARAMS_V2: u8 = 0x21;

/// The native picture and the image size in millimetres.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Native {
    pub width: u32,
    pub height: u32,
    pub width_mm: u32,
    pub height_mm: u32,
}

pub fn decode(b: &[u8], v2: bool, revision: u8) -> Option<Native> {
    let b = b.get(..8)?;
    let w = |i: usize| u16::from_le_bytes([b[i], b[i + 1]]) as u32;
    let mm = |v: u32| if v2 && revision & 0x80 != 0 { v } else { v / 10 };
    let less = v2 as u32;
    let n =
        Native { width: w(4) + less, height: w(6) + less, width_mm: mm(w(0)), height_mm: mm(w(2)) };
    (n.width > less && n.height > less).then_some(n)
}

impl Edid<'_> {
    /// The first DisplayID display parameters block's native size.
    pub fn displayid_native(&self) -> Option<Native> {
        self.extensions().filter_map(Section::new).find_map(|s| {
            let tag = if s.v2() { TAG_PARAMS_V2 } else { TAG_PARAMS_V1 };
            let v2 = s.v2();
            s.blocks().filter(|b| b.tag == tag).find_map(|b| decode(b.body, v2, b.revision))
        })
    }
}
