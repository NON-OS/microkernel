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

//! The schedule region: one coherent DMA map below 4 GiB, as EHCI's 32-bit
//! addresses need; every access volatile and bounded to the region.

use crate::env::Mem;
use crate::sched::layout::REGION;
use core::ptr::{read_volatile, write_volatile};
use nonos_libc::DmaMapOut;

pub struct Region {
    va: *mut u8,
    bus: u32,
}

impl Region {
    /// None unless the map covers the layout and ends below 4 GiB.
    pub fn new(map: &DmaMapOut) -> Option<Self> {
        let end = map.device_addr.checked_add(REGION as u64)?;
        let fits = map.length >= REGION as u64 && end <= 1 << 32 && map.user_va != 0;
        fits.then_some(Region { va: map.user_va as *mut u8, bus: map.device_addr as u32 })
    }

    fn at(&self, off: usize, len: usize) -> *mut u8 {
        assert!(off.checked_add(len).is_some_and(|e| e <= REGION));
        // SAFETY: `off + len` is within the mapped REGION bytes.
        unsafe { self.va.add(off) }
    }
}

impl Mem for Region {
    fn bus(&self, off: usize) -> u32 {
        self.bus + off as u32
    }

    fn rd(&self, off: usize) -> u32 {
        // SAFETY: in the region, and every word the layout uses is aligned.
        unsafe { read_volatile(self.at(off, 4) as *const u32) }
    }

    fn wr(&self, off: usize, v: u32) {
        // SAFETY: as for `rd`.
        unsafe { write_volatile(self.at(off, 4) as *mut u32, v) }
    }

    fn read_bytes(&self, off: usize, out: &mut [u8]) {
        let p = self.at(off, out.len());
        for (i, b) in out.iter_mut().enumerate() {
            // SAFETY: `at` checked the whole span.
            *b = unsafe { read_volatile(p.add(i)) };
        }
    }

    fn write_bytes(&self, off: usize, src: &[u8]) {
        let p = self.at(off, src.len());
        for (i, &b) in src.iter().enumerate() {
            // SAFETY: `at` checked the whole span.
            unsafe { write_volatile(p.add(i), b) };
        }
    }
}
