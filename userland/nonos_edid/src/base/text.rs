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

//! The strings in display descriptors: the monitor's name, its serial and
//! free text (E-EDID 1.4 sections 3.10.3.1, 3.10.3.2 and 3.10.3.4).

use super::descriptor::{TAG_NAME, TAG_SERIAL};
use super::dtd_decode::LEN;
use crate::edid::Edid;

/// Up to 13 bytes, ended by 0x0A and padded with spaces. Bytes are kept
/// as the panel sent them; a caller printing one replaces what it must.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Text {
    bytes: [u8; 13],
    len: u8,
}

impl Text {
    /// The string in a display descriptor's bytes 5-17.
    pub fn from_descriptor(d: &[u8]) -> Self {
        let mut bytes = [0u8; 13];
        let body = d.get(5..LEN).unwrap_or(&[]);
        let mut len = body.iter().position(|&c| c == 0x0A).unwrap_or(body.len());
        while len > 0 && body[len - 1] == b' ' {
            len -= 1;
        }
        bytes[..len].copy_from_slice(&body[..len]);
        Text { bytes, len: len as u8 }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len as usize]
    }
}

impl Edid<'_> {
    /// The product name descriptor (tag 0xFC), when the panel has one.
    pub fn name(&self) -> Option<Text> {
        self.display_descriptor(TAG_NAME).map(Text::from_descriptor)
    }

    /// The serial number string descriptor (tag 0xFF).
    pub fn serial_text(&self) -> Option<Text> {
        self.display_descriptor(TAG_SERIAL).map(Text::from_descriptor)
    }
}
