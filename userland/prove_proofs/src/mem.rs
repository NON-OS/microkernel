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

//! `MkMmap` and `MkMunmap` as the kernel answers them: fresh ranges from a
//! cursor that only moves up, at most a gigabyte a call, an errno in the
//! pointer when refused. A test may answer one call itself or leave a gap.

use std::cell::RefCell;

const MAX_MMAP_SIZE: usize = 1 << 30;
const EINVAL: isize = -22;
/// One `mk_mmap` call's arguments: address, length, prot, flags, fd, offset.
pub type Map = (usize, usize, i32, i32, i32, i64);

#[derive(Default)]
struct Kernel {
    cursor: usize,
    maps: Vec<Map>,
    /// The call, counted from 1, answered with this raw value instead.
    answer: Option<(usize, isize)>,
    /// The call, counted from 1, that lands a page past the last range.
    gap: Option<usize>,
    unmapped: Vec<(usize, usize)>,
}

thread_local! {
    static KERNEL: RefCell<Kernel> = RefCell::new(Kernel::default());
}

/// A fresh kernel whose cursor starts at `base`.
pub fn reset(base: usize, answer: Option<(usize, isize)>, gap: Option<usize>) {
    KERNEL.set(Kernel { cursor: base, answer, gap, ..Kernel::default() });
}

/// What the calls since `reset` were: every map, every unmap.
pub fn calls() -> (Vec<Map>, Vec<(usize, usize)>) {
    KERNEL.with_borrow(|k| (k.maps.clone(), k.unmapped.clone()))
}

pub fn mk_mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut u8 {
    KERNEL.with_borrow_mut(|k| {
        k.maps.push((addr as usize, len, prot, flags, fd, off));
        let n = k.maps.len();
        if !addr.is_null() || len == 0 || len > MAX_MMAP_SIZE {
            return EINVAL as *mut u8;
        }
        if let Some((_, raw)) = k.answer.filter(|a| a.0 == n) {
            return raw as *mut u8;
        }
        if k.gap == Some(n) {
            k.cursor += 4096;
        }
        let at = k.cursor;
        k.cursor += len;
        at as *mut u8
    })
}

pub fn mk_munmap(addr: *mut u8, len: usize) -> i64 {
    KERNEL.with_borrow_mut(|k| k.unmapped.push((addr as usize, len)));
    0
}
