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

//! The pages a window's pixels are kept in (scene_pixels.rs): asked of the
//! kernel whole and given back whole, never the family's heap.

use nonos_libc::{mk_mmap, mk_munmap};

const PROT_READ_WRITE: i32 = 0x3;
const MAP_PRIVATE_ANON: i32 = 0x22;

/// `len` bytes of zeroed pages: where they start, or None when the kernel
/// gives none.
pub fn map(len: usize) -> Option<usize> {
    let at = mk_mmap(core::ptr::null_mut(), len, PROT_READ_WRITE, MAP_PRIVATE_ANON, -1, 0);
    let at = at as isize;
    (at > 0).then_some(at as usize)
}

/// The `len` bytes `map` gave at `at`, unmapped.
pub fn unmap(at: usize, len: usize) {
    let _ = mk_munmap(at as *mut u8, len);
}
