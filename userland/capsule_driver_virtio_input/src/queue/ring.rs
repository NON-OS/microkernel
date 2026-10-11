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

//! The event queue's split ring. Every descriptor points at its own 8-byte
//! buffer for good; a buffer the device filled is read, then offered again.

use core::ptr::write_volatile;
use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::{
    AVAIL_F_NO_INTERRUPT, AVAIL_OFF, DESC_F_WRITE, DESC_OFF, EVENTS_OFF, EVENT_BYTES,
};

pub struct EventRing {
    pub va: u64,
    pub dev: u64,
    pub size: u16,
    /// The next available index to publish and the next used one to read.
    pub avail_idx: u16,
    pub last_used: u16,
}

// SAFETY anchor for every unsafe here and in used.rs: `va` is the coherent
// DMA page laid out by constants::queue, and every index is taken modulo
// `size`, which is at most QUEUE_MAX.
impl EventRing {
    pub fn new(va: u64, dev: u64, size: u16) -> Self {
        Self { va, dev, size, avail_idx: 0, last_used: 0 }
    }

    pub(super) fn at<T>(&self, off: usize) -> *mut T {
        (self.va as usize + off) as *mut T
    }

    /// Every descriptor aimed at its buffer, and every one offered.
    pub fn prime(&mut self) {
        for i in 0..self.size {
            let d = DESC_OFF + i as usize * 16;
            let buf = self.dev + (EVENTS_OFF + i as usize * EVENT_BYTES) as u64;
            unsafe {
                write_volatile(self.at::<u64>(d), buf);
                write_volatile(self.at::<u32>(d + 8), EVENT_BYTES as u32);
                write_volatile(self.at::<u16>(d + 12), DESC_F_WRITE);
                write_volatile(self.at::<u16>(d + 14), 0);
            }
        }
        unsafe { write_volatile(self.at::<u16>(AVAIL_OFF), AVAIL_F_NO_INTERRUPT) };
        for i in 0..self.size {
            self.offer(i);
        }
    }

    /// Put descriptor `id` back on the available ring.
    pub fn offer(&mut self, id: u16) {
        let slot = (self.avail_idx % self.size) as usize;
        unsafe { write_volatile(self.at::<u16>(AVAIL_OFF + 4 + slot * 2), id) };
        // The entry must be visible before the index that publishes it.
        fence(Ordering::Release);
        self.avail_idx = self.avail_idx.wrapping_add(1);
        unsafe { write_volatile(self.at::<u16>(AVAIL_OFF + 2), self.avail_idx) };
    }
}
