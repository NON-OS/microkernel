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

/*
 * What the store may hold: every file lives in this capsule's 192 MiB heap.
 *
 * A write grew its file with an infallible resize, and a copy cloned a file
 * whole, so any client that could write could ask for more than the heap
 * held, and the allocation failure aborted the store for every capsule on
 * the machine. The 2048 names were shared with no limit per client either,
 * so one client could leave nobody able to create a file.
 *
 * Now every byte a request adds is reserved fallibly and counted against
 * DATA_BYTES_MAX, all files' allocations together, which leaves the rest of
 * the heap for names, handles, the journal and the request buffers; past it,
 * or when the heap refuses, the request is Full (ENOSPC) and the store goes
 * on. One owner creates at most NAMES_PER_OWNER names. The kernel's own
 * files, the seed and the packages staged from disk (owner 0) are not held to
 * either: they come in at boot, before any client asks for room.
 */

use alloc::vec::Vec;

use super::types::{Store, StoreError, StoreResult, MAX_FILES, MAX_FILE_BYTES};

/// Every file's allocation together.
pub const DATA_BYTES_MAX: usize = 160 << 20;
/// The names one owner may hold.
pub const NAMES_PER_OWNER: usize = MAX_FILES / 4;
const KERNEL: u32 = 0;

impl Store {
    /// Bytes every file but `except` holds allocated.
    pub fn held_except(&self, except: Option<usize>) -> usize {
        self.files
            .iter()
            .enumerate()
            .filter(|(i, _)| Some(*i) != except)
            .fold(0usize, |sum, (_, f)| sum.saturating_add(f.data.capacity()))
    }

    /// How many names `owner` created.
    pub fn names_of(&self, owner: u32) -> usize {
        self.files.iter().filter(|f| f.owner == owner).count()
    }

    /// Whether `owner` may create `more` names.
    pub(super) fn may_name(&self, owner: u32, more: usize) -> bool {
        owner == KERNEL || self.names_of(owner).saturating_add(more) <= NAMES_PER_OWNER
    }
}

/// Make room in `data` for `new_len` bytes, with `others` bytes held by every
/// other file. Grows by doubling where the budget and the heap allow it, so a
/// file written in small pieces is not copied on every piece, and to the
/// exact length where only that fits; Full where neither does.
pub(super) fn grow(data: &mut Vec<u8>, new_len: usize, others: usize) -> StoreResult<()> {
    if new_len <= data.capacity() {
        return Ok(());
    }
    if new_len > MAX_FILE_BYTES {
        return Err(StoreError::Full);
    }
    let doubled = data.capacity().saturating_mul(2).clamp(new_len, MAX_FILE_BYTES);
    for want in [doubled, new_len] {
        let fits = others.checked_add(want).is_some_and(|total| total <= DATA_BYTES_MAX);
        if fits && data.try_reserve_exact(want - data.len()).is_ok() {
            return Ok(());
        }
    }
    Err(StoreError::Full)
}

/// A copy of `src` made within the budget, or Full.
pub(super) fn copy_of(src: &[u8], others: usize) -> StoreResult<Vec<u8>> {
    let mut data = Vec::new();
    grow(&mut data, src.len(), others)?;
    data.extend_from_slice(src);
    Ok(data)
}
