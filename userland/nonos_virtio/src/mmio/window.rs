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

//! The range is promised once, at construction. After that every access is
//! checked against it and against its own width's alignment: a read outside
//! answers all ones (what a PCI read of nothing returns) and a write outside
//! is dropped. So an offset worked out from a device-supplied value (notify
//! offset, device configuration) cannot reach memory outside the region.

use core::mem::size_of;

#[derive(Clone, Copy, Debug)]
pub struct Mmio {
    base: *mut u8,
    len: usize,
    pub(super) registers: bool,
}

impl Mmio {
    /// A view of memory, DMA buffers and rings included: plain volatile
    /// accesses, ordered where the caller places a barrier.
    ///
    /// # Safety
    ///
    /// `base..base + len` must stay valid for volatile reads and writes for
    /// as long as this value or any copy of it is used.
    pub const unsafe fn new(base: *mut u8, len: usize) -> Self {
        Self { base, len, registers: false }
    }

    /// A device's registers: each store follows every earlier store to DMA
    /// memory, and each load completes before any later access (`order`).
    ///
    /// # Safety
    ///
    /// As for `new`.
    pub const unsafe fn registers(base: *mut u8, len: usize) -> Self {
        Self { base, len, registers: true }
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

    pub(super) fn slot<T>(self, off: usize) -> Option<*mut T> {
        let end = off.checked_add(size_of::<T>())?;
        if end > self.len {
            return None;
        }
        // SAFETY: off + width <= len, inside the range `new` was promised.
        let p = unsafe { self.base.add(off) };
        (p as usize).is_multiple_of(size_of::<T>()).then_some(p.cast())
    }
}
