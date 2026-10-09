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

//! `Queue` carries the pointers and bookkeeping the post/used
//! paths share. The struct is `Copy` so phase modules can hand it
//! across without taking a borrow on the outer `Driver`.

use crate::constants::QUEUE_SIZE;

#[derive(Debug, Clone, Copy)]
pub struct Queue {
    pub region_va: *mut u8,
    pub region_phys: u64,
    pub buf_va: *mut u8,
    pub buf_phys: u64,
    pub buf_len: u32,
    pub avail_offset: usize,
    /// Ring entries the device was told: available and used positions run
    /// modulo this.
    pub queue_size: u16,
    pub last_used: u16,
}

impl Queue {
    pub fn new(
        region_va: u64,
        region_phys: u64,
        buf_va: u64,
        buf_phys: u64,
        buf_len: u32,
        queue_size: u16,
    ) -> Self {
        const DESC_BYTES: usize = 16;
        Self {
            region_va: region_va as *mut u8,
            region_phys,
            buf_va: buf_va as *mut u8,
            buf_phys,
            buf_len,
            avail_offset: queue_size as usize * DESC_BYTES,
            queue_size,
            last_used: 0,
        }
    }

    pub const fn queue_size() -> u16 {
        QUEUE_SIZE
    }

    /// Physical address the device sees for the descriptor table.
    /// The legacy queue PFN write divides this by 4 KiB.
    pub fn region_phys(&self) -> u64 {
        self.region_phys
    }

    /// A ring position reduced to the ring. A zero size (which bring-up
    /// never hands over) is taken as one entry rather than divided by.
    pub fn ring_pos(&self, idx: u16) -> usize {
        (idx % self.queue_size.max(1)) as usize
    }
}
