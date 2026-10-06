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

/* A file's bytes taken into the family's copy, and read from it. */

use alloc::vec::Vec;

use crate::linux::abi::errno;

use super::super::super::{resolve, store};
use super::table::{held, now, with, Entry, CACHE, MAX_FILE};

/* Hold `path`: its bytes from the store, or none for a file being made. */
pub fn hold(path: &[u8], exists: bool) -> Result<(), i64> {
    if held(path) {
        return Ok(());
    }
    let key = resolve::key(path);
    let (data, mtime_ms) = match exists {
        true => {
            let at = store::stat_full(&key).map(|s| s.2).unwrap_or_else(|_| now());
            (store::read(&key, MAX_FILE as u32).map_err(|_| errno::EIO)?, at)
        }
        false => (Vec::new(), now()),
    };
    let stored = data.len() as u64;
    CACHE.0.borrow_mut().push(Entry {
        path: path.to_vec(),
        data,
        dirty: !exists,
        stored,
        in_store: exists,
        mtime_ms,
    });
    Ok(())
}

/*
 * Hold `path`, a file of `stored` bytes in the store, emptied: what a
 * truncating open makes of it, without reading the bytes it drops.
 */
pub fn hold_emptied(path: &[u8], stored: u64) -> Result<(), i64> {
    if held(path) {
        return super::change::resize(path, 0);
    }
    CACHE.0.borrow_mut().push(Entry {
        path: path.to_vec(),
        data: Vec::new(),
        dirty: true,
        stored,
        in_store: true,
        mtime_ms: now(),
    });
    Ok(())
}

pub fn read(path: &[u8], at: u64, len: usize) -> Option<Vec<u8>> {
    with(path, |e| {
        let from = (at as usize).min(e.data.len());
        e.data[from..(from + len).min(e.data.len())].to_vec()
    })
}
