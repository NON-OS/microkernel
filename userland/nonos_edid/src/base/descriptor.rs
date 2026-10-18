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

//! The four 18 byte descriptors at bytes 54-125 of the base block (E-EDID
//! 1.4 section 3.10): detailed timings, or display descriptors marked by a
//! zero pixel clock with their tag in byte 3 (section 3.10.3, Table 3.22).

use super::dtd::Detailed;
use super::dtd_decode::LEN;
use crate::edid::Edid;

pub const TAG_SERIAL: u8 = 0xFF;
pub const TAG_TEXT: u8 = 0xFE;
pub const TAG_RANGE: u8 = 0xFD;
pub const TAG_NAME: u8 = 0xFC;
pub const TAG_STANDARD: u8 = 0xFA;
pub const TAG_CVT: u8 = 0xF8;
pub const TAG_EST3: u8 = 0xF7;

const FIRST: usize = 54;

/// One base block descriptor.
#[derive(Clone, Copy, Debug)]
pub enum Descriptor<'a> {
    Timing(Detailed),
    /// A display descriptor: its tag and all 18 bytes.
    Display {
        tag: u8,
        bytes: &'a [u8],
    },
}

impl<'a> Edid<'a> {
    /// The base block's descriptors in order. A slot holding neither a
    /// usable timing nor a display descriptor is passed over.
    pub fn descriptors(&self) -> impl Iterator<Item = Descriptor<'a>> {
        let b: &'a [u8] = self.base();
        (0..4).filter_map(move |i| {
            let d = &b[FIRST + i * LEN..FIRST + (i + 1) * LEN];
            if d[0] == 0 && d[1] == 0 {
                Some(Descriptor::Display { tag: d[3], bytes: d })
            } else {
                Detailed::decode(d).map(Descriptor::Timing)
            }
        })
    }

    /// The bytes of the first display descriptor with `tag`.
    pub fn display_descriptor(&self, tag: u8) -> Option<&'a [u8]> {
        self.descriptors().find_map(|d| match d {
            Descriptor::Display { tag: t, bytes } if t == tag => Some(bytes),
            _ => None,
        })
    }
}
