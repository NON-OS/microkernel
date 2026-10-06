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

/* Copies that follow their names, and what the copies add up to. */

use alloc::vec::Vec;

use super::table::CACHE;

/*
 * The bytes the family's copies hold in memory, which /proc/meminfo and
 * sysinfo report as the page cache and shared memory a tmpfs file takes.
 */
pub fn bytes() -> u64 {
    CACHE.0.borrow().iter().map(|e| e.data.len() as u64).sum()
}

/*
 * What the family's copies add to the store's bytes once written, less
 * what they take away.
 */
pub fn growth() -> i64 {
    let all = CACHE.0.borrow();
    all.iter().map(|e| e.data.len() as i64 - e.stored as i64).sum()
}

/* The copies the store has no name for yet, each a name once written. */
pub fn unstored() -> u64 {
    CACHE.0.borrow().iter().filter(|e| !e.in_store).count() as u64
}

/* The name went away or moved: the copy follows it. */
pub fn forget(path: &[u8]) {
    CACHE.0.borrow_mut().retain(|e| e.path != path);
}

pub fn renamed(from: &[u8], to: &[u8]) {
    let mut all = CACHE.0.borrow_mut();
    all.retain(|e| e.path != to);
    if let Some(e) = all.iter_mut().find(|e| e.path == from) {
        e.path = to.to_vec();
    }
}

/*
 * The files directly in `dir` that the family holds, which a listing must
 * show although the store may not have them yet.
 */
pub fn names_in(dir: &[u8]) -> Vec<alloc::string::String> {
    let dir = if dir == b"/" { &b""[..] } else { dir };
    let all = CACHE.0.borrow();
    let leaves = all.iter().filter_map(|e| e.path.strip_prefix(dir)?.strip_prefix(b"/"));
    leaves
        .filter(|l| !l.contains(&b'/'))
        .filter_map(|l| alloc::string::String::from_utf8(l.to_vec()).ok())
        .collect()
}
