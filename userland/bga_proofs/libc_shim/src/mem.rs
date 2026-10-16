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

//! The memory the driver maps for its surface, per thread: a test lends the
//! buffer an anonymous mapping hands out, and every unmap is given back.

use std::cell::Cell;

use crate::claim::give_back;

thread_local! {
    static LENT: Cell<(u64, usize)> = const { Cell::new((0, 0)) };
}

/// The next anonymous mapping of at most `len` bytes is the memory at `va`.
pub fn lend(va: u64, len: usize) {
    LENT.with(|l| l.set((va, len)));
}

pub fn mk_mmap(
    _addr: *mut u8,
    len: usize,
    _prot: i32,
    _flags: i32,
    _fd: i32,
    _offset: i64,
) -> *mut u8 {
    let (va, lent) = LENT.with(Cell::get);
    if va == 0 || len > lent {
        return -12i64 as *mut u8;
    }
    va as *mut u8
}

pub fn mk_munmap(addr: *mut u8, _len: usize) -> i64 {
    give_back("munmap", addr as u64)
}
