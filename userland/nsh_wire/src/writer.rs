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

//! A writer into a caller's buffer, for the encoders. It never grows the
//! buffer; a frame that does not fit is `NoRoom` and nothing is half sent,
//! because the caller sends only what `done` reports. The typed writes are
//! in `writer_fields`.

use crate::WireError;

pub struct Writer<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> Writer<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Writer { buf, pos: 0 }
    }

    pub fn put(&mut self, bytes: &[u8]) -> Result<(), WireError> {
        let end = self.pos.checked_add(bytes.len()).ok_or(WireError::NoRoom)?;
        let dst = self.buf.get_mut(self.pos..end).ok_or(WireError::NoRoom)?;
        dst.copy_from_slice(bytes);
        self.pos = end;
        Ok(())
    }

    /// How many bytes have been written.
    pub const fn done(&self) -> usize {
        self.pos
    }
}
