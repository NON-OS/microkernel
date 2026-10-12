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

//! The register window as the driver reaches it: the mapped BAR in the
//! capsule, a model in the proofs.

pub trait Regs {
    fn r32(&self, off: usize) -> u32;
    fn w32(&self, off: usize, value: u32);
}

/// The broker-mapped window. An access past it reads zero and is dropped.
#[derive(Clone, Copy)]
pub struct Mmio {
    base: u64,
    len: usize,
}

impl Mmio {
    pub const fn new(base: u64, len: usize) -> Self {
        Self { base, len }
    }

    fn at(&self, off: usize) -> Option<*mut u32> {
        (off.is_multiple_of(4) && off.checked_add(4)? <= self.len)
            .then(|| (self.base as usize + off) as *mut u32)
    }
}

impl Regs for Mmio {
    fn r32(&self, off: usize) -> u32 {
        // SAFETY: `at` keeps the access aligned and inside the window the
        // broker mapped, which stays mapped while the driver lives.
        self.at(off).map_or(0, |p| unsafe { core::ptr::read_volatile(p) })
    }

    fn w32(&self, off: usize, value: u32) {
        if let Some(p) = self.at(off) {
            // SAFETY: as above; the adapter may act on the store at once.
            unsafe { core::ptr::write_volatile(p, value) };
        }
    }
}
