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
 * What the store may hold: every file lives in this capsule's heap.
 *
 * A write grew its file with an infallible resize, and a copy cloned a file
 * whole, so any client that could write could ask for more than the heap
 * held, and the allocation failure aborted the store for every capsule on
 * the machine. The 2048 names were shared with no limit per client either,
 * so one client could leave nobody able to create a file.
 *
 * Now every byte a request adds is reserved fallibly and counted against the
 * machine's budget (budget_size.rs), leaving the heap's rest to the server; past it,
 * or when the heap refuses, the request is Full (ENOSPC) and the store goes
 * on. One owner creates at most NAMES_PER_OWNER names. Owner 0's files (the
 * kernel's, the seed, the packages staged from disk at boot) are held to
 * neither and not counted: clients share the whole budget beside them.
 */

use alloc::vec::Vec;

use super::budget_size::limit;
use super::types::{Store, StoreError, StoreResult, MAX_FILES, MAX_FILE_BYTES};

/// The least every machine gives every client file together.
pub const DATA_BYTES_MAX: usize = 160 << 20;
/// The names one owner may hold.
pub const NAMES_PER_OWNER: usize = MAX_FILES / 4;
const KERNEL: u32 = 0;

impl Store {
    /// Bytes every client file but `except` holds allocated.
    pub fn held_except(&self, except: Option<usize>) -> usize {
        self.files
            .iter()
            .enumerate()
            .filter(|(i, f)| Some(*i) != except && f.owner != KERNEL)
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
        let fits = others.checked_add(want).is_some_and(|total| total <= limit());
        if fits && moved(data, want) {
            return Ok(());
        }
    }
    Err(StoreError::Full)
}

/// Move `data` into an allocation of `capacity` bytes, made fallibly, and
/// clear the bytes left behind before their block goes back to the heap. A
/// reserve on the Vec itself would let the allocator copy and free the old
/// block with the file's bytes still in it.
fn moved(data: &mut Vec<u8>, capacity: usize) -> bool {
    let mut next = Vec::new();
    if next.try_reserve_exact(capacity).is_err() {
        return false;
    }
    next.extend_from_slice(data);
    super::zeroize::zeroize(data);
    *data = next;
    true
}

/// Room past a file's length worth giving back: less is left as it is, so
/// a small file is not copied on every close.
const TRIM_SLACK: usize = 64 << 10;

/// The room past `data`'s length given back, once its writer closes it.
/// A file written a piece at a time grows by doubling, so up to as much
/// again as it holds stays allocated, and the budget counts what is
/// allocated: packs written 60 KiB at a time, 1.4 GiB of them, held the
/// whole 2 GiB of an 8 GiB machine, and the next install was refused as
/// Full. The bytes move to an allocation of their length, made fallibly;
/// when the heap has no such room the file keeps the one it has. The old
/// bytes are cleared before they go, as a removed file's are.
pub(super) fn trim(data: &mut Vec<u8>) {
    if data.capacity() - data.len() < TRIM_SLACK {
        return;
    }
    let len = data.len();
    let _ = moved(data, len);
}

/// A copy of `src` made within the budget, or Full.
pub(super) fn copy_of(src: &[u8], others: usize) -> StoreResult<Vec<u8>> {
    let mut data = Vec::new();
    grow(&mut data, src.len(), others)?;
    data.extend_from_slice(src);
    Ok(data)
}
