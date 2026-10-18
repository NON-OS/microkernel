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

//! The ring the request queue uses, in the grant's first page. Each request
//! is one chain from descriptor 0: everything the device reads first, then
//! everything it writes, as the specification orders them.

use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{fence, Ordering};

use crate::constants::layout::{
    AVAIL_F_NO_INTERRUPT, AVAIL_OFF, DESC_F_NEXT, DESC_F_WRITE, DESC_OFF, USED_OFF,
};

/// One buffer in a chain: its bus address, length, and whether the device
/// writes it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Seg {
    pub addr: u64,
    pub len: u32,
    pub write: bool,
}

pub struct Ring {
    va: u64,
    pub size: u16,
    avail_idx: u16,
    last_used: u16,
}

// SAFETY anchor for every unsafe here: `va` is the coherent DMA grant laid
// out by constants::layout, mapped for the driver's life, and every index is
// taken modulo `size`, which is at most QUEUE_MAX.
impl Ring {
    /// A fresh ring on a queue the device just enabled: no interrupts asked
    /// for, nothing offered, nothing used.
    pub fn new(va: u64, size: u16) -> Self {
        let ring = Self { va, size, avail_idx: 0, last_used: 0 };
        unsafe {
            write_volatile(ring.at::<u16>(AVAIL_OFF), AVAIL_F_NO_INTERRUPT);
            write_volatile(ring.at::<u16>(AVAIL_OFF + 2), 0);
        }
        ring
    }

    fn at<T>(&self, off: usize) -> *mut T {
        (self.va as usize + off) as *mut T
    }

    /// Write `chain` into descriptors 0.. and offer its head. The chain is
    /// at most QUEUE_MIN long, which every ring the device took holds.
    pub fn post(&mut self, chain: &[Seg]) {
        for (i, s) in chain.iter().enumerate() {
            let d = DESC_OFF + i * 16;
            let last = i + 1 == chain.len();
            let flags = if s.write { DESC_F_WRITE } else { 0 } | if last { 0 } else { DESC_F_NEXT };
            unsafe {
                write_volatile(self.at::<u64>(d), s.addr);
                write_volatile(self.at::<u32>(d + 8), s.len);
                write_volatile(self.at::<u16>(d + 12), flags);
                write_volatile(self.at::<u16>(d + 14), if last { 0 } else { i as u16 + 1 });
            }
        }
        let slot = (self.avail_idx % self.size) as usize;
        unsafe { write_volatile(self.at::<u16>(AVAIL_OFF + 4 + slot * 2), 0) };
        // The descriptors and the entry must be visible before the index
        // that publishes them.
        fence(Ordering::Release);
        self.avail_idx = self.avail_idx.wrapping_add(1);
        unsafe { write_volatile(self.at::<u16>(AVAIL_OFF + 2), self.avail_idx) };
    }

    /// Whether the device has returned a chain.
    pub fn returned(&self) -> bool {
        unsafe { read_volatile(self.at::<u16>(USED_OFF + 2)) != self.last_used }
    }

    /// The returned chain's head and the bytes the device wrote, once it has
    /// returned one.
    pub fn take(&mut self) -> Option<(u32, u32)> {
        if !self.returned() {
            return None;
        }
        // The element, the response and the data are the driver's only once
        // the index that covers them is seen.
        fence(Ordering::Acquire);
        let e = USED_OFF + 4 + (self.last_used % self.size) as usize * 8;
        let used =
            unsafe { (read_volatile(self.at::<u32>(e)), read_volatile(self.at::<u32>(e + 4))) };
        self.last_used = self.last_used.wrapping_add(1);
        Some(used)
    }
}
