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

//! The event page: the model's own memory, mapped as the broker would map
//! BAR1's first page.

use std::cell::Cell;

use crate::script::{answer, note};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MmioMapOut {
    pub user_va: u64,
    pub length: u64,
    pub grant_id: u64,
}

thread_local! {
    static MEMORY: Cell<u64> = const { Cell::new(0) };
}

/// Where the model keeps the page the driver maps.
pub fn set_memory(va: u64) {
    MEMORY.with(|m| m.set(va));
}

pub fn mk_mmio_map(
    _d: u64,
    epoch: u64,
    bar: u32,
    _f: u32,
    off: u64,
    len: u64,
    out: *mut MmioMapOut,
) -> i64 {
    assert_eq!((epoch, off), (7, 0), "the page at the start of the BAR, under the claim");
    note("mmio_map", bar as u64);
    let r = answer().mmio;
    if r < 0 {
        return r;
    }
    let va = MEMORY.with(|m| m.get());
    // SAFETY: the caller hands a writable MmioMapOut, as the real call requires.
    unsafe { *out = MmioMapOut { user_va: va, length: len, grant_id: 60 } };
    0
}
pub fn mk_mmio_unmap(grant: u64) -> i64 {
    note("mmio_unmap", grant);
    0
}
