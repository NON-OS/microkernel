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

//! Setting a file's size.

use super::Files;
use crate::errno::{Errno, EFBIG};
use crate::host::Objects;
use crate::limits::{CHUNK, MAX_FILE};
use crate::node::chunks_for;

/// Cuts or grows `ino` to `size`. Bytes past a cut are gone from the chunk
/// it ends in too, so growing it again shows zeros.
pub fn truncate<O: Objects>(f: &mut Files<O>, ino: u64, size: u64, now: u64) -> Result<(), Errno> {
    if size > MAX_FILE {
        return Err(EFBIG);
    }
    let old = f.tree.get(ino)?.size;
    if size < old {
        let keep = chunks_for(size);
        let within = (size % CHUNK as u64) as usize;
        if within != 0 {
            let last = keep as u64 - 1;
            let has =
                f.dirty.get(ino, last).is_some() || f.tree.get(ino)?.chunks[last as usize] != 0;
            if has {
                f.chunk_mut(ino, last)?.truncate(within);
            }
        }
        f.dirty.forget(ino, keep as u64);
        let node = f.tree.get_mut(ino)?;
        let cut = node.chunks.split_off(keep);
        f.store.pending.extend(cut.into_iter().filter(|id| *id != 0));
    }
    let node = f.tree.get_mut(ino)?;
    node.size = size;
    node.chunks.resize(chunks_for(size), 0);
    (node.mtime, node.ctime) = (now, now);
    Ok(())
}
