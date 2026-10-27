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

//! What may stand in the table. One writer per range of a file: a map that
//! is or may become a writer shares no byte of its file with another map,
//! in any process of the family, its own included. Everything else leans on
//! it: a write-back has no other map to show its bytes to. Pure.

use alloc::vec::Vec;

use super::entry::{overlap, Entry};
use super::spans::outside;
use crate::linux::abi::errno;

/// The most maps the table holds, so its memory is bounded.
pub const MAX_MAPS: usize = 64;

/// Whether `a` and `b` may not both stand.
pub fn conflict(a: &Entry, b: &Entry) -> bool {
    let writes = a.writer || a.may_write || b.writer || b.may_write;
    !a.orphan && b.shows(&a.path) && writes && overlap(a.file(), b.file()).is_some()
}

/// The table once `[at, at + span)` of `pid` is unmapped, or ENOMEM when a
/// split would pass MAX_MAPS, as Linux refuses a munmap past
/// vm.max_map_count.
pub fn unmap(all: &[Entry], pid: u32, at: u64, span: u64) -> Result<Vec<Entry>, i64> {
    let kept = cut(all, pid, at, span);
    if kept.len() > MAX_MAPS.max(all.len()) {
        return Err(errno::ENOMEM);
    }
    Ok(kept)
}

/// The table once `[at, at + span)` of `pid` is unmapped, split or not.
pub fn cut(all: &[Entry], pid: u32, at: u64, span: u64) -> Vec<Entry> {
    let left =
        |e: &Entry| if e.pid == pid { outside(e, at, span) } else { [Some(e.clone()), None] };
    all.iter().flat_map(left).flatten().collect()
}

/// Whether `new` may join: ENOSYS against the one writer rule, ENOMEM past
/// MAX_MAPS. MAP_FIXED (`fixed`) unmaps `new`'s span first.
pub fn admits(all: &[Entry], new: &Entry, fixed: bool) -> Result<(), i64> {
    let rest = if fixed { cut(all, new.pid, new.at, new.span) } else { all.to_vec() };
    if rest.iter().any(|e| conflict(e, new)) {
        return Err(errno::ENOSYS);
    }
    if rest.len() >= MAX_MAPS {
        return Err(errno::ENOMEM);
    }
    Ok(())
}
