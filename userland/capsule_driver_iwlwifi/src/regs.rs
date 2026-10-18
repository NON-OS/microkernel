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

use nonos_libc::Deadline;

pub use crate::mmio::Mmio;

/// The register window. Every access is checked against its length: an
/// offset past it reads all ones, as a bus with nothing behind it does, and
/// a write there goes nowhere.
#[derive(Clone, Copy)]
pub struct Regs {
    base: usize,
    len: usize,
}

impl Regs {
    /// # Safety
    /// `base` must be the start of `len` bytes of register window mapped for
    /// this process (as `setup::mmio` maps BAR0) for as long as this lives.
    pub const unsafe fn new(base: u64, len: u64) -> Self {
        Self { base: base as usize, len: len as usize }
    }
    // The register at `off`, if four bytes there lie inside the window and
    // it is aligned.
    fn at(&self, off: usize) -> Option<usize> {
        let end = off.checked_add(4)?;
        (end <= self.len && off.is_multiple_of(4)).then(|| self.base + off)
    }
    pub fn read32(&self, off: usize) -> u32 {
        let Some(p) = self.at(off) else { return u32::MAX };
        // SAFETY: `new`'s caller vouched for `len` bytes at `base`, and `at`
        // put this aligned word inside them.
        unsafe { core::ptr::read_volatile(p as *const u32) }
    }
    pub fn write32(&self, off: usize, val: u32) {
        let Some(p) = self.at(off) else { return };
        // SAFETY: as for `read32`.
        unsafe { core::ptr::write_volatile(p as *mut u32, val) }
    }
    pub fn set_bits(&self, off: usize, bits: u32) {
        self.write32(off, self.read32(off) | bits);
    }
    /// Wait up to `ms` on the uptime clock for every bit of `mask` to read
    /// set. A count of reads would last as long as the bus takes per read,
    /// which differs from machine to machine.
    pub fn poll_set(&self, off: usize, mask: u32, ms: u64) -> bool {
        let deadline = Deadline::after_ms(ms);
        loop {
            if self.read32(off) & mask == mask {
                return true;
            }
            if deadline.expired() {
                return self.read32(off) & mask == mask;
            }
            core::hint::spin_loop();
        }
    }
}
