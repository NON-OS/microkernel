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

//! The claim and the maps the setup sequence asks the broker for: the claim
//! at epoch 1, BAR0 mapped onto the window a test names, and each DMA map a
//! grant at a made-up address the setup only records, its flags kept.

use std::cell::{Cell, RefCell};

use crate::release::MmioMapOut;

pub const MK_DMA_MAP_DMA32: u32 = 1 << 1;

#[derive(Clone, Copy, Debug, Default)]
pub struct DmaMapOut {
    pub user_va: u64,
    pub device_addr: u64,
    pub length: u64,
    pub grant_id: u64,
}

thread_local! {
    static WINDOW: Cell<(u64, u64)> = const { Cell::new((0, 0)) };
    static DMA_FLAGS: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
}

/// What the next BAR maps answer: `length` bytes at `user_va`.
pub fn map_window(user_va: u64, length: u64) {
    WINDOW.with(|w| w.set((user_va, length)));
}

/// The flags of every DMA map asked for on this thread, in order.
pub fn dma_flags() -> Vec<u32> {
    DMA_FLAGS.with(|f| f.borrow().clone())
}

pub fn mk_device_claim(_device_id: u64) -> i64 {
    1
}

pub fn mk_mmio_map(
    _d: u64,
    _e: u64,
    _b: u32,
    _f: u32,
    _o: u64,
    len: u64,
    out: &mut MmioMapOut,
) -> i64 {
    let (user_va, length) = WINDOW.with(|w| w.get());
    *out = MmioMapOut { user_va, length: length.min(len), grant_id: 20 };
    0
}

pub fn mk_dma_map(_d: u64, _e: u64, len: u64, flags: u32, out: &mut DmaMapOut) -> i64 {
    let n = DMA_FLAGS.with(|f| {
        f.borrow_mut().push(flags);
        f.borrow().len() as u64
    });
    *out = DmaMapOut { user_va: 0, device_addr: n << 28, length: len, grant_id: 30 + n };
    0
}
