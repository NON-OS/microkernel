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

/* A copy put in the store: at close, fsync and sync, and at exit. */

use alloc::vec::Vec;

use crate::linux::abi::errno;

use super::super::super::{resolve, store};
use super::table::CACHE;

/*
 * Put the copy of `path` in the store, if it changed; `keep` false lets
 * it go afterwards.
 */
pub fn flush(path: &[u8], keep: bool) -> Result<(), i64> {
    let mut all = CACHE.0.borrow_mut();
    let Some(i) = all.iter().position(|e| e.path == path) else {
        return Ok(());
    };
    if all[i].dirty {
        store::write(&resolve::key(path), &all[i].data).map_err(|_| errno::EIO)?;
        all[i].dirty = false;
    }
    if !keep {
        all.remove(i);
    }
    Ok(())
}

/* Every changed file to the store: sync, and a process's exit. */
pub fn flush_all() -> Result<(), i64> {
    let paths: Vec<Vec<u8>> =
        CACHE.0.borrow().iter().filter(|e| e.dirty).map(|e| e.path.clone()).collect();
    paths.iter().try_for_each(|p| flush(p, true))
}
