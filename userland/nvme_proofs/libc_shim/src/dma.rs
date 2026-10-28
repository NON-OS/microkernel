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

//! DMA grants as zeroed, page-aligned host memory. The device address is the
//! host address, so the test reads the queues where the driver points the
//! controller. Memory is never freed: an unmap is only recorded, so a test
//! can still read what a dropped queue held.

use std::alloc::{alloc_zeroed, Layout};
use std::cell::Cell;

use crate::calls::record;

#[repr(C)]
pub struct DmaMapOut {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}

thread_local! {
    static NEXT_GRANT: Cell<u64> = const { Cell::new(0x100) };
    static SKEW: Cell<(u64, u64)> = const { Cell::new((0, 0)) };
}

/// Have every later grant come `short` bytes short of the length asked,
/// and start `offset` bytes into its page, as a broker at fault might.
pub fn skew_grants(short: u64, offset: u64) {
    SKEW.with(|s| s.set((short, offset)));
}

pub fn mk_dma_map(_device: u64, _epoch: u64, length: u64, _flags: u32, out: &mut DmaMapOut) -> i64 {
    let (short, offset) = SKEW.with(Cell::get);
    let Ok(layout) = Layout::from_size_align((length + offset).max(1) as usize, 4096) else {
        return -22;
    };
    // SAFETY: the layout has a non-zero size.
    let at = unsafe { alloc_zeroed(layout) } as u64 + offset;
    let length = length.saturating_sub(short);
    let grant_id = NEXT_GRANT.with(|n| n.replace(n.get() + 1));
    *out = DmaMapOut { user_va: at, device_addr: at, length, grant_id };
    record("dma_map", grant_id);
    0
}

pub fn mk_dma_unmap(grant_id: u64) -> i64 {
    record("dma_unmap", grant_id)
}
