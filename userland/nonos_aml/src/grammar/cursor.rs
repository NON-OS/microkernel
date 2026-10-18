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

//! A read position inside one object. Every read is checked against `end`,
//! the end of the innermost package around it, so no production can read into
//! its parent's neighbour.

use crate::error::{Error, ErrorKind};

#[derive(Clone, Copy, Debug)]
pub struct Cursor<'a> {
    pub bytes: &'a [u8],
    pub pos: u32,
    pub end: u32,
}

impl<'a> Cursor<'a> {
    /// `end` is clamped to the bytes, so no caller can widen a cursor past
    /// the table it came from.
    pub fn new(bytes: &'a [u8], pos: u32, end: u32) -> Self {
        let end = end.min(bytes.len() as u32);
        Self { bytes, pos: pos.min(end), end }
    }

    pub fn done(&self) -> bool {
        self.pos >= self.end
    }

    pub fn peek(&self) -> Option<u8> {
        (self.pos < self.end).then(|| self.bytes[self.pos as usize])
    }

    /// Steps over `n` bytes and returns where they started.
    pub fn take(&mut self, n: u32) -> Result<u32, Error> {
        let start = self.pos;
        let next = start.checked_add(n).filter(|e| *e <= self.end);
        self.pos = next.ok_or(Error::new(ErrorKind::Truncated, start))?;
        Ok(start)
    }

    pub fn u8(&mut self) -> Result<u8, Error> {
        let at = self.take(1)?;
        Ok(self.bytes[at as usize])
    }

    /// A little-endian integer of `n` bytes (WordData, DWordData, QWordData).
    pub fn le(&mut self, n: u32) -> Result<u64, Error> {
        let at = self.take(n)? as usize;
        let bytes = &self.bytes[at..at + n as usize];
        Ok(bytes.iter().rev().fold(0u64, |v, b| (v << 8) | *b as u64))
    }

    /// The cursor for a package that ends at `end`, refused if that is past
    /// the end of the package holding it.
    pub fn inner(&self, end: u32) -> Result<Cursor<'a>, Error> {
        if end > self.end {
            return Err(Error::new(ErrorKind::PastParent, self.pos));
        }
        Ok(Cursor { bytes: self.bytes, pos: self.pos, end })
    }
}
