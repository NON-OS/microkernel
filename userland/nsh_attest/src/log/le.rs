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

//! A little-endian cursor over the log: the TCG structures are in the
//! platform's byte order, unlike the TPM's own.

use crate::LogFault;

pub(super) struct Le<'a> {
    buf: &'a [u8],
    pub(super) pos: usize,
}

impl<'a> Le<'a> {
    pub(super) const fn new(buf: &'a [u8]) -> Self {
        Le { buf, pos: 0 }
    }

    pub(super) fn take(&mut self, n: usize) -> Result<&'a [u8], LogFault> {
        let end = self.pos.checked_add(n).ok_or(LogFault::Short)?;
        let part = self.buf.get(self.pos..end).ok_or(LogFault::Short)?;
        self.pos = end;
        Ok(part)
    }

    pub(super) fn u8(&mut self) -> Result<u8, LogFault> {
        Ok(self.take(1)?[0])
    }

    pub(super) fn u16(&mut self) -> Result<u16, LogFault> {
        let b = self.take(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    pub(super) fn u32(&mut self) -> Result<u32, LogFault> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// A u32 size, judged against `max` before the bytes are taken.
    pub(super) fn body(&mut self, max: usize) -> Result<&'a [u8], LogFault> {
        let n = self.u32()? as usize;
        if n > max {
            return Err(LogFault::EventTooLarge);
        }
        self.take(n)
    }

    pub(super) fn left(&self) -> usize {
        self.buf.len() - self.pos
    }
}
