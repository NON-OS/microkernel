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

//! Fake DMA memory, reached only through the raw pointer taken once at
//! construction, as device memory is: a fresh borrow between accesses would
//! invalidate the queue's view under the aliasing model, and Miri checks it.

use nonos_virtio::Mmio;

pub struct Mem(*mut [u64]);

impl Drop for Mem {
    fn drop(&mut self) {
        // SAFETY: from Box::into_raw in `new`, and dropped only here.
        drop(unsafe { Box::from_raw(self.0) });
    }
}

impl Mem {
    pub fn new(len: usize) -> Self {
        Self(Box::into_raw(vec![0u64; len.div_ceil(8)].into_boxed_slice()))
    }

    pub fn len(&self) -> usize {
        self.0.len() * 8
    }

    /// The driver's view. Every test drops the queue holding it before
    /// the memory.
    pub fn view(&self) -> Mmio {
        // SAFETY: the allocation is `len` bytes and outlives every queue.
        unsafe { Mmio::new(self.0 as *mut u8, self.len()) }
    }

    pub fn get(&self, at: usize) -> u8 {
        assert!(at < self.len());
        // SAFETY: inside the allocation, checked above.
        unsafe { (self.0 as *mut u8).add(at).read() }
    }

    pub fn put(&self, at: usize, bytes: &[u8]) {
        assert!(at + bytes.len() <= self.len());
        for (i, b) in bytes.iter().enumerate() {
            // SAFETY: inside the allocation, checked above.
            unsafe { (self.0 as *mut u8).add(at + i).write(*b) };
        }
    }

    pub fn u16_at(&self, at: usize) -> u16 {
        u16::from_le_bytes([self.get(at), self.get(at + 1)])
    }

    pub fn u32_at(&self, at: usize) -> u32 {
        u32::from_le_bytes([self.get(at), self.get(at + 1), self.get(at + 2), self.get(at + 3)])
    }
}
