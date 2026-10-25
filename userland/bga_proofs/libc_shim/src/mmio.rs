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

//! The BAR mappings, per thread.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::claim::give_back;
use crate::{seal, watch};

pub struct MmioMapOut {
    pub user_va: u64,
    pub length: u64,
    pub grant_id: u64,
}

/// libc's write-combining flag (L17), as the driver passes it for BAR0.
pub const MK_MMIO_MAP_WC: u32 = 1 << 0;

/// Each BAR's grant is this plus its index.
pub(crate) const GRANT_BASE: u64 = 0x100;

thread_local! {
    static BARS: RefCell<HashMap<u32, (u64, u64)>> = RefCell::new(HashMap::new());
}

/// Map BAR `index` at `va`, `length` bytes long as the broker reports it.
pub fn map_bar(index: u8, va: u64, length: u64) {
    BARS.with(|b| b.borrow_mut().insert(index as u32, (va, length)));
}

/// Where BAR `bar` is mapped and how long the broker says it is.
pub(crate) fn mapping(bar: u32) -> Option<(u64, u64)> {
    BARS.with(|b| b.borrow().get(&bar).copied())
}

pub fn mk_mmio_map(
    _device_id: u64,
    _epoch: u64,
    bar: u32,
    _flags: u32,
    _offset: u64,
    _length: u64,
    out: &mut MmioMapOut,
) -> i64 {
    let Some((va, length)) = mapping(bar) else { return -19 };
    *out = MmioMapOut { user_va: va, length, grant_id: GRANT_BASE + bar as u64 };
    0
}

pub fn mk_mmio_unmap(grant: u64) -> i64 {
    let rc = give_back("mmio_unmap", grant);
    if watch::watches(grant) {
        seal::seal();
    }
    rc
}
