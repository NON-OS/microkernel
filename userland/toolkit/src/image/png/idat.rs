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

use crate::image::types::DecodeError;

use super::chunks::chunk_at;
use super::deflate::ByteSource;

/* The zlib stream split across consecutive IDAT chunks, read in place: the
 * next chunk is entered when the current one runs out, and the stream ends
 * at the first chunk that is not IDAT. */
pub(super) struct IdatBytes<'a> {
    input: &'a [u8],
    pos: usize,
    end: usize,
    next: usize,
}

impl<'a> IdatBytes<'a> {
    pub fn new(input: &'a [u8], first: usize) -> Result<Self, DecodeError> {
        let (_, _, next) = chunk_at(input, first)?;
        Ok(Self { input, pos: first + 8, end: next - 4, next })
    }
}

impl ByteSource for IdatBytes<'_> {
    fn next_byte(&mut self) -> Option<u8> {
        while self.pos >= self.end {
            let (tag, _, next) = chunk_at(self.input, self.next).ok()?;
            if tag != b"IDAT" {
                return None;
            }
            self.pos = self.next + 8;
            self.end = next - 4;
            self.next = next;
        }
        let b = *self.input.get(self.pos)?;
        self.pos += 1;
        Some(b)
    }
}
