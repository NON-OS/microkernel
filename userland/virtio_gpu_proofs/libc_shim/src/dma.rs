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
//! DMA maps as host memory: zeroed, as the kernel's are, the device address
//! the host address, and every grant kept per thread until unmapped or the
//! test resets, so a test sees which are still held.

use std::cell::RefCell;

pub const MK_DMA_MAP_HIGH: u32 = 1 << 0;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DmaMapOut {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}

thread_local! {
    static GRANTS: RefCell<Vec<(u64, Box<[u64]>)>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_dma_map(_device: u64, _epoch: u64, length: u64, _flags: u32, out: *mut DmaMapOut) -> i64 {
    let words = vec![0u64; length.div_ceil(8) as usize].into_boxed_slice();
    let base = words.as_ptr() as u64;
    let grant_id = GRANTS.with(|g| {
        let mut g = g.borrow_mut();
        let id = g.iter().map(|e| e.0).max().unwrap_or(0) + 1;
        g.push((id, words));
        id
    });
    // SAFETY: the caller passes its own out-struct.
    unsafe { *out = DmaMapOut { user_va: base, device_addr: base, length, grant_id } };
    0
}

pub fn mk_dma_unmap(grant_id: u64) -> i64 {
    GRANTS.with(|g| {
        let mut g = g.borrow_mut();
        let before = g.len();
        g.retain(|e| e.0 != grant_id);
        if g.len() < before {
            0
        } else {
            -22
        }
    })
}

/// Each grant still held, as (id, first address, bytes).
pub fn dma_spans() -> Vec<(u64, u64, u64)> {
    GRANTS.with(|g| {
        g.borrow().iter().map(|e| (e.0, e.1.as_ptr() as u64, e.1.len() as u64 * 8)).collect()
    })
}

/// Every grant of this thread given back, its memory freed.
pub fn dma_reset() {
    GRANTS.with(|g| g.borrow_mut().clear());
}
