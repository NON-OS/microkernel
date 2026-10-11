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

//! Receive ring state: where the descriptors and the pool live, `head`, the
//! next descriptor the part fills, and the registers for the receive poll
//! demand. Priming is in rx_prime.rs and taking a frame in rx_consume.rs.

use crate::constants::queue::BUFFER_LEN;
use crate::regs::Regs;

use super::layout::Desc;

pub struct RxRing {
    pub ring_va: u64,
    pub ring_device_addr: u64,
    pub pool_va: u64,
    pub pool_device_addr: u64,
    pub head: u16,
    pub regs: Regs,
}

impl RxRing {
    pub fn new(ring: (u64, u64), pool: (u64, u64), regs: Regs) -> Self {
        let (ring_va, ring_device_addr) = ring;
        let (pool_va, pool_device_addr) = pool;
        Self { ring_va, ring_device_addr, pool_va, pool_device_addr, head: 0, regs }
    }

    /// # Safety
    /// `idx` is below RX_DESC_COUNT; the ring is the control grant's
    /// receive half, RX_DESC_COUNT descriptors long.
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
