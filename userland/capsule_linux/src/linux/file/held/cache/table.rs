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

/* The family's copies, one per path, and the lookups on them. */

use alloc::vec::Vec;
use core::cell::RefCell;

/* The most a family may hold of one file while writing it. */
pub const MAX_FILE: usize = 8 << 20;

pub(super) struct Entry {
    pub(super) path: Vec<u8>,
    pub(super) data: Vec<u8>,
    pub(super) dirty: bool,
}

pub(super) struct Cache(pub(super) RefCell<Vec<Entry>>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Cache {}

pub(super) static CACHE: Cache = Cache(RefCell::new(Vec::new()));

pub(super) fn with<T>(path: &[u8], f: impl FnOnce(&mut Entry) -> T) -> Option<T> {
    CACHE.0.borrow_mut().iter_mut().find(|e| e.path == path).map(f)
}

pub fn held(path: &[u8]) -> bool {
    with(path, |_| ()).is_some()
}

pub fn size(path: &[u8]) -> Option<u64> {
    with(path, |e| e.data.len() as u64)
}
