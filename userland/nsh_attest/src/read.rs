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

//! A big-endian reader over one document or TPM structure. Every read is
//! bounds-checked, a length is judged against its cap before the bytes it
//! covers are taken, and `finish` refuses anything left over.

use crate::FormatFault;

pub(crate) struct Be<'a> {
    pub(crate) buf: &'a [u8],
    pub(crate) pos: usize,
}

impl<'a> Be<'a> {
    pub(crate) const fn new(buf: &'a [u8]) -> Self {
        Be { buf, pos: 0 }
    }

    pub(crate) fn take(&mut self, n: usize) -> Result<&'a [u8], FormatFault> {
        let end = self.pos.checked_add(n).ok_or(FormatFault::Short)?;
        let part = self.buf.get(self.pos..end).ok_or(FormatFault::Short)?;
        self.pos = end;
        Ok(part)
    }

    pub(crate) fn array<const N: usize>(&mut self) -> Result<&'a [u8; N], FormatFault> {
        self.take(N)?.try_into().map_err(|_| FormatFault::Short)
    }

    pub(crate) fn finish(self) -> Result<(), FormatFault> {
        match self.buf.len() - self.pos {
            0 => Ok(()),
            _ => Err(FormatFault::Trailing),
        }
    }
}
