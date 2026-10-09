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
 * The permission bits of the family's files, which the store does not keep.
 *
 * A file or directory the family makes gets the mode it was made with, less
 * the umask, as on Linux, and chmod changes it; each lives as long as the
 * family, which is as long as its private directories do. The shared tree
 * is read-only to a guest, so its modes never change: a directory is 0755,
 * and a file is 0755 too, because the store cannot say which of its files
 * are programs and a program must be executable.
 */

use alloc::vec::Vec;
use core::cell::RefCell;

struct Modes(RefCell<Vec<(Vec<u8>, u32)>>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Modes {}

static MODES: Modes = Modes(RefCell::new(Vec::new()));

pub const SHARED: u32 = 0o755;
/* A private file or directory that predates the family's record of it. */
pub const FILE: u32 = 0o644;
pub const DIR: u32 = 0o755;

pub fn of(path: &[u8]) -> Option<u32> {
    MODES.0.borrow().iter().find(|(p, _)| p == path).map(|(_, m)| *m)
}

pub fn set(path: &[u8], mode: u32) {
    let mut all = MODES.0.borrow_mut();
    all.retain(|(p, _)| p != path);
    all.push((path.to_vec(), mode & 0o7777));
}

pub fn forget(path: &[u8]) {
    MODES.0.borrow_mut().retain(|(p, _)| p != path);
}

/* The name moved, and everything below it with it. */
pub fn renamed(from: &[u8], to: &[u8]) {
    let mut all = MODES.0.borrow_mut();
    all.retain(|(p, _)| p != to);
    for (p, _) in all.iter_mut() {
        if p.starts_with(from) && matches!(p.get(from.len()), None | Some(b'/')) {
            let mut moved = to.to_vec();
            moved.extend_from_slice(&p[from.len()..]);
            *p = moved;
        }
    }
}
