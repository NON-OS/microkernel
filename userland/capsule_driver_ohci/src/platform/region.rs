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
//! The schedule region: one coherent DMA map below 4 GiB, as OHCI's 32-bit
//! addresses need; every access volatile and bounded to the region (span.rs):
//! one outside it reads all ones and writes nothing.

use super::span::Span;
use crate::env::{Mem, OutOfRegion};
use crate::sched::layout::REGION;
use core::ptr::{read_volatile, write_volatile};
use nonos_libc::DmaMapOut;

pub struct Region {
    va: *mut u8,
    bus: u32,
    span: Span,
}

impl Region {
    /// None unless the map covers the layout and ends below 4 GiB.
    pub fn new(map: &DmaMapOut) -> Option<Self> {
        let end = map.device_addr.checked_add(REGION as u64)?;
        let fits = map.length >= REGION as u64 && end <= 1 << 32 && map.user_va != 0;
        let (va, bus) = (map.user_va as *mut u8, map.device_addr as u32);
        fits.then_some(Region { va, bus, span: Span::new(REGION) })
    }

    fn at(&self, off: usize, len: usize) -> Result<*mut u8, OutOfRegion> {
        let off = self.span.check(off, len)?;
        // SAFETY: `off + len` is within the mapped REGION bytes.
        Ok(unsafe { self.va.add(off) })
    }
}

impl Mem for Region {
    fn bus(&self, off: usize) -> u32 {
        self.bus.wrapping_add(off as u32)
    }
    fn rd(&self, off: usize) -> u32 {
        // SAFETY: in the region, and every word the layout uses is aligned.
        self.at(off, 4).map_or(!0, |p| unsafe { read_volatile(p as *const u32) })
    }
    fn wr(&self, off: usize, v: u32) {
        // SAFETY: as for `rd`.
        let _ = self.at(off, 4).map(|p| unsafe { write_volatile(p as *mut u32, v) });
    }
    fn read_bytes(&self, off: usize, out: &mut [u8]) {
        let Ok(p) = self.at(off, out.len()) else { return out.fill(!0) };
        // SAFETY: `at` checked the whole span.
        out.iter_mut().enumerate().for_each(|(i, b)| *b = unsafe { read_volatile(p.add(i)) });
    }
    fn write_bytes(&self, off: usize, src: &[u8]) {
        let Ok(p) = self.at(off, src.len()) else { return };
        // SAFETY: `at` checked the whole span.
        src.iter().enumerate().for_each(|(i, &b)| unsafe { write_volatile(p.add(i), b) });
    }
    fn fault(&self) -> Result<(), OutOfRegion> {
        self.span.fault()
    }
}
