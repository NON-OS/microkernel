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

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub const PIPE_BUF_SIZE: usize = 65536;

pub struct PipeBuffer {
    pub data: Vec<u8>,
    pub head: AtomicUsize,
    pub tail: AtomicUsize,
    pub capacity: usize,
    readers: AtomicUsize,
    writers: AtomicUsize,
}

impl Default for PipeBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl PipeBuffer {
    pub fn new() -> Self {
        Self {
            data: alloc::vec![0u8; PIPE_BUF_SIZE],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            capacity: PIPE_BUF_SIZE,
            readers: AtomicUsize::new(1),
            writers: AtomicUsize::new(1),
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, i32> {
        if self.readers.load(Ordering::SeqCst) == 0 {
            return Err(-32);
        }
        let head = self.head.load(Ordering::SeqCst);
        let tail = self.tail.load(Ordering::SeqCst);
        let available =
            if head >= tail { self.capacity - head + tail - 1 } else { tail - head - 1 };
        if available == 0 {
            return Err(-11);
        }
        let to_write = buf.len().min(available);
        for (i, &byte) in buf.iter().take(to_write).enumerate() {
            self.data[(head + i) % self.capacity] = byte;
        }
        self.head.store((head + to_write) % self.capacity, Ordering::SeqCst);
        Ok(to_write)
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, i32> {
        let head = self.head.load(Ordering::SeqCst);
        let tail = self.tail.load(Ordering::SeqCst);
        let available = if head >= tail { head - tail } else { self.capacity - tail + head };
        if available == 0 {
            if self.writers.load(Ordering::SeqCst) == 0 {
                return Ok(0);
            }
            return Err(-11);
        }
        let to_read = buf.len().min(available);
        for (i, slot) in buf.iter_mut().take(to_read).enumerate() {
            *slot = self.data[(tail + i) % self.capacity];
        }
        self.tail.store((tail + to_read) % self.capacity, Ordering::SeqCst);
        Ok(to_read)
    }

    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::SeqCst);
        let tail = self.tail.load(Ordering::SeqCst);
        if head >= tail {
            head - tail
        } else {
            self.capacity - tail + head
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn available_write(&self) -> usize {
        self.capacity - self.len() - 1
    }
    pub fn add_reader(&self) {
        self.readers.fetch_add(1, Ordering::SeqCst);
    }
    /* Stops at zero instead of wrapping to usize::MAX. Every caller holds the
    pipe's mutex, so the load and the store do not race. */
    pub fn remove_reader(&self) {
        let n = self.readers.load(Ordering::SeqCst);
        if n > 0 {
            self.readers.store(n - 1, Ordering::SeqCst);
        }
    }
    pub fn add_writer(&self) {
        self.writers.fetch_add(1, Ordering::SeqCst);
    }
    pub fn remove_writer(&self) {
        let n = self.writers.load(Ordering::SeqCst);
        if n > 0 {
            self.writers.store(n - 1, Ordering::SeqCst);
        }
    }
    pub fn has_readers(&self) -> bool {
        self.readers.load(Ordering::SeqCst) > 0
    }
    pub fn has_writers(&self) -> bool {
        self.writers.load(Ordering::SeqCst) > 0
    }
}
