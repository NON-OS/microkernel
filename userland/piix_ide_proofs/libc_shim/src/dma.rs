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

//! The DMA grant on host memory. The test picks the bus address and the
//! answer; the model reads what the engine would through `dma_at`.

use std::alloc::{alloc_zeroed, Layout};
use std::cell::RefCell;

use crate::script::note;

pub const MK_DMA_MAP_DMA32: u32 = 1 << 1;
pub const MK_DMA_MAP_COHERENT: u32 = 1 << 2;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DmaMapOut {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}

thread_local! {
    /// Refuse, or the bus address and the length to answer with (zero
    /// length answers the length asked for).
    static ANSWER: RefCell<Result<(u64, u64), i64>> = const { RefCell::new(Ok((0x0010_0000, 0))) };
    pub(crate) static MAPPED: RefCell<Vec<(u64, u64, u64)>> = const { RefCell::new(Vec::new()) };
}

pub fn script_dma(answer: Result<(u64, u64), i64>) {
    ANSWER.with(|a| *a.borrow_mut() = answer);
}

pub fn mk_dma_map(_d: u64, epoch: u64, len: u64, flags: u32, out: *mut DmaMapOut) -> i64 {
    assert_eq!(epoch, 7, "a grant under the claim's epoch");
    note("dma_map", flags as u64);
    let (dev, answered) = match ANSWER.with(|a| *a.borrow()) {
        Ok(a) => a,
        Err(e) => return e,
    };
    let length = if answered == 0 { len } else { answered };
    let layout = Layout::from_size_align(length as usize, 4096).expect("layout");
    // SAFETY: a non-zero, page-aligned layout; leaked for the thread's life.
    let va = unsafe { alloc_zeroed(layout) } as u64;
    MAPPED.with(|m| m.borrow_mut().push((dev, va, length)));
    // SAFETY: the caller hands a writable DmaMapOut, as the real call requires.
    unsafe { *out = DmaMapOut { user_va: va, device_addr: dev, length, grant_id: 70 } };
    0
}

pub fn mk_dma_unmap(grant: u64) -> i64 {
    note("dma_unmap", grant);
    0
}
