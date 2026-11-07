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

//! Reading a range of a file.

use alloc::vec::Vec;

use super::Files;
use crate::errno::{Errno, ENOMEM};
use crate::host::Objects;
use crate::limits::CHUNK;
use crate::store::read_full;

/// Up to `len` bytes of `ino` from `offset`, short only at its end.
pub fn read<O: Objects>(
    f: &mut Files<O>,
    ino: u64,
    offset: u64,
    len: usize,
) -> Result<Vec<u8>, Errno> {
    let size = f.tree.get(ino)?.size;
    let end = offset.saturating_add(len as u64).min(size);
    let mut out = Vec::new();
    if end <= offset {
        return Ok(out);
    }
    out.try_reserve_exact((end - offset) as usize).map_err(|_| ENOMEM)?;
    out.resize((end - offset) as usize, 0);
    let mut at = offset;
    while at < end {
        let index = at / CHUNK as u64;
        let within = (at % CHUNK as u64) as usize;
        let take = ((end - at) as usize).min(CHUNK - within);
        let dst = &mut out[(at - offset) as usize..][..take];
        // The buffer starts zeroed: a hole, or a chunk shorter than the
        // range, leaves zeros where it holds nothing.
        if let Some(bytes) = f.dirty.get(ino, index) {
            if let Some(src) = bytes.get(within..) {
                let n = src.len().min(take);
                dst[..n].copy_from_slice(&src[..n]);
            }
        } else {
            let id = f.tree.get(ino)?.chunks.get(index as usize).copied().unwrap_or(0);
            if id != 0 {
                read_full(f.objects, id, within as u64, dst)?;
            }
        }
        at += take as u64;
    }
    Ok(out)
}
