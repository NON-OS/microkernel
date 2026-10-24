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

//! A reader over one body. Every read is bounds-checked and moves forward;
//! `finish` refuses a body with bytes left over, so a parser that forgets a
//! field cannot pass quietly. The typed reads are in `reader_fields`.

use crate::WireError;

pub struct Reader<'a> {
    pub(crate) buf: &'a [u8],
    pub(crate) pos: usize,
}

impl<'a> Reader<'a> {
    pub const fn new(buf: &'a [u8]) -> Self {
        Reader { buf, pos: 0 }
    }

    pub fn take(&mut self, n: usize) -> Result<&'a [u8], WireError> {
        let end = self.pos.checked_add(n).ok_or(WireError::Short)?;
        let part = self.buf.get(self.pos..end).ok_or(WireError::Short)?;
        self.pos = end;
        Ok(part)
    }

    pub fn array<const N: usize>(&mut self) -> Result<&'a [u8; N], WireError> {
        self.take(N)?.try_into().map_err(|_| WireError::Short)
    }

    /// Bytes the format reserves; anything but zero is refused.
    pub fn zero(&mut self, n: usize) -> Result<(), WireError> {
        match self.take(n)?.iter().all(|b| *b == 0) {
            true => Ok(()),
            false => Err(WireError::Reserved),
        }
    }

    /// A versioned body's start: magic, u16 version, two reserved bytes.
    /// A wrong magic or version is told apart from a malformed body.
    pub fn preamble(&mut self, magic: &[u8; 4], version: u16) -> Result<(), WireError> {
        if self.array::<4>()? != magic {
            return Err(WireError::Magic);
        }
        if u16::from_le_bytes(*self.array()?) != version {
            return Err(WireError::BodyVersion);
        }
        self.zero(2)
    }

    /// How many bytes are still unread.
    pub const fn left(&self) -> usize {
        self.buf.len() - self.pos
    }

    pub fn finish(self) -> Result<(), WireError> {
        match self.left() {
            0 => Ok(()),
            _ => Err(WireError::Trailing),
        }
    }
}
