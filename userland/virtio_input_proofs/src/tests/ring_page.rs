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

//! A ring page in host memory, and the device's side of the used ring.

use crate::constants::queue::{EVENTS_OFF, RING_BYTES, USED_OFF};

pub struct Page(Vec<u64>);

impl Page {
    pub fn new() -> Self {
        Self(vec![0; RING_BYTES / 8])
    }
    pub fn va(&mut self) -> u64 {
        self.0.as_mut_ptr() as u64
    }
    pub fn u16(&mut self, off: usize) -> u16 {
        // SAFETY: inside the page; the ring's own accessors are volatile.
        unsafe { ((self.va() as usize + off) as *const u16).read() }
    }
    pub fn set(&mut self, off: usize, bytes: &[u8]) {
        let p = (self.va() as usize + off) as *mut u8;
        // SAFETY: inside the page.
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len()) };
    }
    /// The device returns buffer `id` as the `n`th used element.
    pub fn give(&mut self, n: u16, size: u16, id: u32, event: [u8; 8]) {
        self.set(EVENTS_OFF + id as usize * 8, &event);
        let e = USED_OFF + 4 + (n % size) as usize * 8;
        self.set(e, &[id.to_le_bytes(), 8u32.to_le_bytes()].concat());
        self.set(USED_OFF + 2, &n.wrapping_add(1).to_le_bytes());
    }
}
