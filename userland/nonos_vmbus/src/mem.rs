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

//! The pages a channel shares with the host. The host writes them while the
//! driver reads, so every access is a copy through this trait, never a
//! reference: the driver's own view lives in its own memory, and each index
//! the host owns is read once, then checked before use.

use core::sync::atomic::{fence, AtomicU32, Ordering};

pub trait Shared {
    /// The bytes shared, from 0.
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Copy `out.len()` bytes from `at`. `at + out.len() <= len()`.
    fn read(&self, at: usize, out: &mut [u8]);
    /// Copy `data` to `at`. `at + data.len() <= len()`.
    fn write(&mut self, at: usize, data: &[u8]);
    /// A ring index or flag the other side writes, read with acquire order.
    fn load32(&self, at: usize) -> u32;
    /// A ring index or flag the other side reads, written with release order.
    fn store32(&mut self, at: usize, v: u32);
    /// A full barrier between this side's writes and its next reads.
    fn barrier(&self) {
        fence(Ordering::SeqCst);
    }
}

/// Shared pages at a pointer: the driver's DMA grant, mapped into it.
pub struct Pages {
    base: *mut u8,
    len: usize,
}

impl Pages {
    /// # Safety
    /// `base` points to `len` bytes, mapped for the life of the value and
    /// written by no one else but the host.
    pub unsafe fn new(base: *mut u8, len: usize) -> Self {
        Self { base, len }
    }
}

impl Shared for Pages {
    fn len(&self) -> usize {
        self.len
    }

    fn read(&self, at: usize, out: &mut [u8]) {
        assert!(at.checked_add(out.len()).is_some_and(|e| e <= self.len));
        for (i, b) in out.iter_mut().enumerate() {
            // SAFETY: in bounds, checked above; the host may write it at any
            // time, so each byte is read once, volatile.
            *b = unsafe { core::ptr::read_volatile(self.base.add(at + i)) };
        }
    }

    fn write(&mut self, at: usize, data: &[u8]) {
        assert!(at.checked_add(data.len()).is_some_and(|e| e <= self.len));
        for (i, b) in data.iter().enumerate() {
            // SAFETY: in bounds, checked above.
            unsafe { core::ptr::write_volatile(self.base.add(at + i), *b) };
        }
    }

    fn load32(&self, at: usize) -> u32 {
        assert!(at.is_multiple_of(4) && at + 4 <= self.len);
        // SAFETY: in bounds and aligned (the grant is page aligned).
        unsafe { &*(self.base.add(at) as *const AtomicU32) }.load(Ordering::Acquire)
    }

    fn store32(&mut self, at: usize, v: u32) {
        assert!(at.is_multiple_of(4) && at + 4 <= self.len);
        // SAFETY: as `load32`.
        unsafe { &*(self.base.add(at) as *const AtomicU32) }.store(v, Ordering::Release)
    }
}

/// Shared pages in a plain buffer, for the host tests and the models.
pub struct Buffer(pub alloc::vec::Vec<u8>);

impl Shared for Buffer {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn read(&self, at: usize, out: &mut [u8]) {
        out.copy_from_slice(&self.0[at..at + out.len()]);
    }

    fn write(&mut self, at: usize, data: &[u8]) {
        self.0[at..at + data.len()].copy_from_slice(data);
    }

    fn load32(&self, at: usize) -> u32 {
        u32::from_le_bytes(self.0[at..at + 4].try_into().unwrap())
    }

    fn store32(&mut self, at: usize, v: u32) {
        self.0[at..at + 4].copy_from_slice(&v.to_le_bytes());
    }
}
