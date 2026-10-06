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

//! Integers and strings as GGUF writes them: little-endian, and a string as
//! a u64 length followed by that many bytes.

use crate::cursor::Cursor;
use crate::error::GgufError;
use crate::source::ReadAt;
use crate::text::Text;

impl<S: ReadAt> Cursor<'_, S> {
    pub(crate) fn u32(&mut self) -> Result<u32, GgufError> {
        Ok(u32::from_le_bytes(self.take::<4>()?))
    }

    pub(crate) fn u64(&mut self) -> Result<u64, GgufError> {
        Ok(u64::from_le_bytes(self.take::<8>()?))
    }

    /// A string's length, refused above `max`, with the cursor left on its
    /// first byte.
    pub(crate) fn string_len(&mut self, max: u64) -> Result<u64, GgufError> {
        let at = self.pos;
        let len = self.u64()?;
        if len > max {
            return Err(GgufError::StringTooLong { at, len });
        }
        Ok(len)
    }

    /// Step over a string of at most `max` bytes.
    pub(crate) fn skip_string(&mut self, max: u64) -> Result<(), GgufError> {
        let len = self.string_len(max)?;
        self.skip(len)
    }

    /// Read a string of at most `max` bytes, keeping what fits in a Text.
    pub(crate) fn text(&mut self, max: u64) -> Result<Text, GgufError> {
        let len = self.string_len(max)?;
        let mut text = Text::empty(len);
        for i in 0..len {
            let [b] = self.take::<1>()?;
            text.keep(i, b);
        }
        Ok(text)
    }
}
