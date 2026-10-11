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

//! Reads and writes in pieces of IO_MAX: an escrow offer's file is near a
//! MiB, and one IPC message to the store holds at most that with
//! its header. Each piece follows the last at the file's position.

use alloc::vec::Vec;

use super::frame::Answer;
use super::ops;

pub(super) const IO_MAX: usize = 60 * 1024;

pub(super) fn read_all(fd: u32, len: usize) -> Answer<Vec<u8>> {
    // `len` may come from the file's own head: reserve one piece, never what
    // it claims. A file shorter than that ends at its first short piece.
    let mut out = Vec::with_capacity(len.min(IO_MAX));
    while out.len() < len {
        let piece = ops::read(fd, (len - out.len()).min(IO_MAX))?;
        out.extend_from_slice(&piece);
    }
    Ok(out)
}

pub(super) fn write_all(fd: u32, data: &[u8]) -> Answer<()> {
    data.chunks(IO_MAX).try_for_each(|piece| ops::write(fd, piece))
}
