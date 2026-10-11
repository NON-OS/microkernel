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

//! MMIO maps. A BAR is host memory a test sets; a map hands back its
//! address plus the offset asked for.

use std::cell::RefCell;

use super::broker::note;

#[repr(C)]
pub struct MmioMapOut {
    pub user_va: u64,
    pub length: u64,
    pub grant_id: u64,
}

thread_local! {
    static BARS: RefCell<[(u64, u64); 6]> = const { RefCell::new([(0, 0); 6]) };
}

pub fn set_bar(bar: usize, va: u64, len: u64) {
    BARS.with(|b| b.borrow_mut()[bar] = (va, len));
}

pub fn mk_mmio_map(
    _d: u64,
    _e: u64,
    bar: u32,
    _f: u32,
    off: u64,
    len: u64,
    out: *mut MmioMapOut,
) -> i64 {
    let (va, size) = BARS.with(|b| b.borrow()[bar as usize]);
    if off + len > size {
        return -22;
    }
    // SAFETY: the caller hands a writable MmioMapOut, as the real call requires.
    unsafe { *out = MmioMapOut { user_va: va + off, length: len, grant_id: 100 + bar as u64 } };
    note("mmio_map", bar as u64)
}
pub fn mk_mmio_unmap(grant: u64) -> i64 {
    note("mmio_unmap", grant)
}
