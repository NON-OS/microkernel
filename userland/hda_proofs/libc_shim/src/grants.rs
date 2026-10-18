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

//! The grants a bring-up takes: the claim, the register window, the
//! interrupt and the DMA buffers. A test offers the window and the buffers
//! (host memory a device model reads and writes); the interrupt is refused,
//! so the driver polls. Every call is recorded, the unmaps among them: the
//! teardown is held to giving grants back only through the release.

use std::cell::RefCell;
use std::collections::VecDeque;

use crate::broker::record;
use crate::types::{DmaMapOut, IrqBindOut, MmioMapOut};

thread_local! {
    static WINDOW: RefCell<Option<(u64, u64)>> = const { RefCell::new(None) };
    static POOL: RefCell<VecDeque<(u64, u64, u64)>> = const { RefCell::new(VecDeque::new()) };
}

/// The register window BAR0 maps to: `len` bytes at `va`.
pub fn offer_window(va: u64, len: u64) {
    WINDOW.with(|w| *w.borrow_mut() = Some((va, len)));
}

/// The DMA buffers handed out in order, as (user va, device address,
/// length); a map past the last is refused as out of memory.
pub fn offer_dma(buffers: Vec<(u64, u64, u64)>) {
    POOL.with(|p| *p.borrow_mut() = buffers.into());
}

pub fn mk_device_claim(device_id: u64) -> i64 {
    record("device_claim", device_id);
    1
}

pub fn mk_mmio_map(_: u64, _: u64, bar: u32, _: u32, _: u64, _: u64, out: &mut MmioMapOut) -> i64 {
    record("mmio_map", bar as u64);
    let Some((user_va, length)) = WINDOW.with(|w| *w.borrow()) else { return -19 };
    *out = MmioMapOut { user_va, length, grant_id: 0x100 };
    0
}

pub fn mk_irq_bind(_: u64, _: u64, line: u32, _: u32, _: u32, _: &mut IrqBindOut) -> i64 {
    record("irq_bind", line as u64);
    -95
}

pub fn mk_dma_map(_: u64, _: u64, length: u64, _: u32, out: &mut DmaMapOut) -> i64 {
    record("dma_map", length);
    let Some((user_va, device_addr, length)) = POOL.with(|p| p.borrow_mut().pop_front()) else {
        return -12;
    };
    *out = DmaMapOut { user_va, device_addr, length, grant_id: device_addr >> 12 };
    0
}
