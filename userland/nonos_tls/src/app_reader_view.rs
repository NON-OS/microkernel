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

//! What an `AppReader` has read, and letting go of what was used.

use alloc::vec::Vec;

use super::app_reader::AppReader;

impl AppReader {
    /// Every application byte opened so far.
    pub fn plaintext(&self) -> &[u8] {
        &self.plain
    }

    pub fn into_plaintext(self) -> Vec<u8> {
        self.plain
    }

    /// True once a record failed to open; nothing further is read.
    pub fn is_broken(&self) -> bool {
        self.broken
    }

    /// Wire offset of the first record not yet opened.
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /*
     * A kept-alive connection carries one response after another. What came
     * before the next one is spent, and keeping it only made every later
     * read longer. The sequence number is not touched: it counts records on
     * the connection, not bytes in the buffer.
     */
    /// Drop the opened records from the front of `wire`, and the first
    /// `used` bytes of plaintext, so both start where the next response does.
    pub fn compact(&mut self, wire: &mut Vec<u8>, used: usize) {
        let opened = self.cursor.min(wire.len());
        wire.drain(..opened);
        self.cursor -= opened;
        let spent = used.min(self.plain.len());
        self.plain.drain(..spent);
    }
}
