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

//! NameString, section 20.2.2: a root or parent prefixes, then 0-255 NameSegs.

use super::{is_lead_char, is_name_char, Cursor};
use crate::error::{Error, ErrorKind};

/// A NameString as it sits in the table; its segments are contiguous.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NamePath {
    pub root: bool,
    pub up: u8, // ParentPrefixChars ('^') before the path
    pub at: u32,
    pub count: u8,
}

impl NamePath {
    pub fn is_null(&self) -> bool {
        !self.root && self.up == 0 && self.count == 0
    }

    pub fn seg(&self, bytes: &[u8], i: u8) -> [u8; 4] {
        let at = self.at as usize + 4 * i as usize;
        [bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]
    }
}

/// Reads a NameString. RootChar and ParentPrefixChar are exclusive (the
/// grammar has no `\^`), and a MultiNamePath's SegCount runs from 1 to 255.
pub fn name_string(c: &mut Cursor<'_>) -> Result<NamePath, Error> {
    let bad = Error::new(ErrorKind::Name, c.pos);
    let root = c.peek() == Some(b'\\');
    let mut up = 0u8;
    if root {
        c.take(1)?;
    }
    while !root && c.peek() == Some(b'^') {
        c.take(1)?;
        // A path climbing 256 levels names nothing a namespace can hold.
        up = up.checked_add(1).ok_or(bad)?;
    }
    let count = match c.peek().ok_or(Error::new(ErrorKind::Truncated, c.pos))? {
        0x00 => c.take(1).map(|_| 0)?,
        0x2E => c.take(1).map(|_| 2)?,
        0x2F => {
            c.take(1)?;
            Some(c.u8()?).filter(|n| *n != 0).ok_or(bad)?
        }
        b if is_lead_char(b) => 1,
        _ => return Err(bad),
    };
    let at = c.take(4 * count as u32)?;
    for (i, b) in c.bytes[at as usize..c.pos as usize].iter().enumerate() {
        let ok = if i % 4 == 0 { is_lead_char(*b) } else { is_name_char(*b) };
        if !ok {
            return Err(Error::new(ErrorKind::Name, at + i as u32));
        }
    }
    Ok(NamePath { root, up, at, count })
}
