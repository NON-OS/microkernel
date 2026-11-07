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

//! Writing a range of a file.

use super::Files;
use crate::errno::{Errno, EFBIG};
use crate::host::Objects;
use crate::limits::{CHUNK, MAX_FILE};
use crate::node::chunks_for;

/// Writes `data` into `ino` at `offset`, growing it as needed. A write the
/// volume stops part way returns what landed; one that landed nothing
/// returns the volume's error.
pub fn write<O: Objects>(
    f: &mut Files<O>,
    ino: u64,
    offset: u64,
    data: &[u8],
    now: u64,
) -> Result<usize, Errno> {
    let end = offset.checked_add(data.len() as u64).ok_or(EFBIG)?;
    if end > MAX_FILE {
        return Err(EFBIG);
    }
    let mut done = 0usize;
    while done < data.len() {
        let at = offset + done as u64;
        let index = at / CHUNK as u64;
        let within = (at % CHUNK as u64) as usize;
        let take = (data.len() - done).min(CHUNK - within);
        let node = f.tree.get_mut(ino)?;
        let grown = node.size.max(at + take as u64);
        node.chunks.resize(chunks_for(grown), 0);
        let chunk = match f.chunk_mut(ino, index) {
            Ok(chunk) => chunk,
            Err(e) if done == 0 => return Err(e),
            Err(_) => break,
        };
        if chunk.len() < within + take {
            chunk.resize(within + take, 0);
        }
        chunk[within..within + take].copy_from_slice(&data[done..done + take]);
        let node = f.tree.get_mut(ino)?;
        node.size = grown;
        done += take;
    }
    let node = f.tree.get_mut(ino)?;
    (node.mtime, node.ctime) = (now, now);
    // A chunk list grown for a piece that never landed goes back to the size.
    node.chunks.truncate(chunks_for(node.size));
    Ok(done)
}
