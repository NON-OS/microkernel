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
 * The times the family set on its files with utimensat, which the store
 * cannot keep: it records one time, the last write's.
 *
 * A time set here stands until the file is written again, which moves the
 * store's own time past it, as a write moves mtime on Linux.
 */

use alloc::vec::Vec;
use core::cell::RefCell;

#[derive(Clone, Copy)]
pub struct Set {
    pub atime_ms: u64,
    pub mtime_ms: u64,
    /* The store's time when these were set: a later write replaces them. */
    pub written_ms: u64,
}

struct Times(RefCell<Vec<(Vec<u8>, Set)>>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Times {}

static TIMES: Times = Times(RefCell::new(Vec::new()));

pub fn of(path: &[u8]) -> Option<Set> {
    TIMES.0.borrow().iter().find(|(p, _)| p == path).map(|(_, s)| *s)
}

pub fn set(path: &[u8], times: Set) {
    let mut all = TIMES.0.borrow_mut();
    all.retain(|(p, _)| p != path);
    all.push((path.to_vec(), times));
}

pub fn forget(path: &[u8]) {
    TIMES.0.borrow_mut().retain(|(p, _)| p != path);
}

pub fn renamed(from: &[u8], to: &[u8]) {
    let mut all = TIMES.0.borrow_mut();
    all.retain(|(p, _)| p != to);
    if let Some((p, _)) = all.iter_mut().find(|(p, _)| p == from) {
        *p = to.to_vec();
    }
}
