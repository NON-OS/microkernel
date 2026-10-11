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

//! Transmit ring state. `post` hands the slot at `tail` to the part;
//! `reclaim` walks `clean` over descriptors the part has given back, and
//! `full` refuses a post that would land on one it still owns. One slot
//! always stays empty, so `tail == clean` means empty and never full.

use crate::constants::queue::BUFFER_LEN;

use super::layout::Desc;

pub struct TxRing {
    pub ring_va: u64,
    pub ring_device_addr: u64,
    pub pool_va: u64,
    pub pool_device_addr: u64,
    pub tail: u16,
    /// Oldest descriptor not yet seen done.
    pub clean: u16,
}

impl TxRing {
    pub fn new(ring: (u64, u64), pool: (u64, u64)) -> Self {
        let (ring_va, ring_device_addr) = ring;
        let (pool_va, pool_device_addr) = pool;
        Self { ring_va, ring_device_addr, pool_va, pool_device_addr, tail: 0, clean: 0 }
    }

    /// # Safety
    /// `idx` is below TX_DESC_COUNT; the ring is the control grant's
    /// transmit half, TX_DESC_COUNT descriptors long.
    pub unsafe fn descriptor(&self, idx: u16) -> *mut Desc {
        (self.ring_va as *mut Desc).add(idx as usize)
    }

    pub fn buffer_device_addr(&self, idx: u16) -> u32 {
        (self.pool_device_addr + idx as u64 * BUFFER_LEN as u64) as u32
    }

    pub fn buffer_va(&self, idx: u16) -> u64 {
        self.pool_va + idx as u64 * BUFFER_LEN as u64
    }
}
