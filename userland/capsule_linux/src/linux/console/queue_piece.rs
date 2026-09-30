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

//! What the terminal's input queue holds and gives: pieces of bytes and
//! ends of file, and what one read takes. Pure, like the queue.

use alloc::vec::Vec;

/// A message that is exactly this one byte is an end of file (Ctrl-D).
pub const EOF: u8 = 0x04;
/// The most kept unread; past it the terminal's inbox holds the rest.
pub const MOST: usize = 64 << 10;

pub enum Piece {
    /// Bytes, and how many of them a read has taken already.
    Bytes(Vec<u8>, usize),
    Eof,
}

/// What one read takes.
#[derive(PartialEq, Eq, Debug)]
pub enum Taken {
    Bytes(Vec<u8>),
    Eof,
    Empty,
}
