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

//! One mapped register region, reached with volatile accesses that cannot
//! leave it.
//!
//! The range is promised once, at construction. After that every access is
//! checked against it and against its own width's alignment: a read outside
//! answers all ones (what a PCI read of nothing returns) and a write outside
//! is dropped. An offset worked out from a device-supplied value (a notify
//! offset, a device configuration field) therefore cannot reach memory the
//! region does not cover, whatever the device said.

use core::ptr::{read_volatile, write_volatile};

#[derive(Clone, Copy, Debug)]
pub struct Mmio {
    base: *mut u8,
    len: usize,
}

impl Mmio {
    /// # Safety
    ///
    /// `base..base + len` must stay valid for volatile reads and writes for
    /// as long as this value or any copy of it is used.
    pub const unsafe fn new(base: *mut u8, len: usize) -> Self {
        Self { base, len }
    }

    pub const fn len(self) -> usize {
        self.len
    }

    pub const fn is_empty(self) -> bool {
        self.len == 0
    }

    pub fn base(self) -> *mut u8 {
        self.base
    }

    fn slot(self, off: usize, width: usize) -> Option<*mut u8> {
        let end = off.checked_add(width)?;
        if end > self.len {
            return None;
        }
        // SAFETY: off + width <= len, inside the range `new` was promised.
        let p = unsafe { self.base.add(off) };
        if !(p as usize).is_multiple_of(width) {
            return None;
        }
        Some(p)
    }

    pub fn r8(self, off: usize) -> u8 {
        // SAFETY: `slot` returns only in-range, aligned addresses.
        self.slot(off, 1).map_or(u8::MAX, |p| unsafe { read_volatile(p) })
    }

    pub fn r16(self, off: usize) -> u16 {
        // SAFETY: as in `r8`.
        self.slot(off, 2).map_or(u16::MAX, |p| unsafe { read_volatile(p.cast()) })
    }

    pub fn r32(self, off: usize) -> u32 {
        // SAFETY: as in `r8`.
        self.slot(off, 4).map_or(u32::MAX, |p| unsafe { read_volatile(p.cast()) })
    }

    pub fn w8(self, off: usize, value: u8) {
        if let Some(p) = self.slot(off, 1) {
            // SAFETY: as in `r8`.
            unsafe { write_volatile(p, value) }
        }
    }

    pub fn w16(self, off: usize, value: u16) {
        if let Some(p) = self.slot(off, 2) {
            // SAFETY: as in `r8`.
            unsafe { write_volatile(p.cast(), value) }
        }
    }

    pub fn w32(self, off: usize, value: u32) {
        if let Some(p) = self.slot(off, 4) {
            // SAFETY: as in `r8`.
            unsafe { write_volatile(p.cast(), value) }
        }
    }
}
