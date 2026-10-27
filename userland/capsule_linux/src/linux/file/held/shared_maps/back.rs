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

//! A writer's pages written back to its file: only the pages whose bytes
//! differ from the file's, and only up to the file's end. They go into the
//! family's copy as any write does, but no map is shown them: the range of
//! a writer is shown by no other map (rules.rs), and its own has them.

use super::super::super::{resolve, store};
use super::super::cache;
use super::entry::Entry;
use super::spans::{back_range, differing};
use crate::linux::guest::Guest;

/// Compared a piece at a time, so a large map costs a bounded buffer.
const PIECE: u64 = 64 << 10;

/// How long `path` is now: the family's copy, or the store's file.
fn end_of(path: &[u8]) -> Option<u64> {
    cache::size(path).or_else(|| store::stat(&resolve::key(path)).ok().map(|(s, _)| s))
}

/// What writer `e` changed of its file's bytes `want`, written back;
/// whether a page went.
pub(super) fn back(e: &Entry, want: (u64, u64)) -> bool {
    if !e.writer || e.orphan {
        return false;
    }
    /*
     * A file longer than the family may hold is never taken into a copy:
     * the copy would be cut short, and the cut written to the store.
     */
    let end = end_of(&e.path).filter(|&end| end <= cache::MAX_FILE as u64);
    let Some((lo, hi)) = end.and_then(|end| back_range(e, want, end)) else {
        return false;
    };
    if cache::hold(&e.path, true).is_err() {
        return false;
    }
    let (mut at, mut went) = (lo, false);
    while at < hi {
        let n = (hi - at).min(PIECE);
        let mut map = alloc::vec![0u8; n as usize];
        /* A process already gone has nothing left to give back. */
        if !Guest::read_into(e.pid, e.addr(at), &mut map) {
            return went;
        }
        let file = cache::read(&e.path, at, n as usize).unwrap_or_default();
        for (a, b) in differing(&map, &file) {
            went |= cache::put(&e.path, at + a as u64, &map[a..b]).is_ok();
        }
        at += n;
    }
    went
}
