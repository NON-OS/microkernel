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

//! The data buffer, which the driver touches only while no request holds
//! it, and empties with volatile writes the compiler keeps.

use nonos_virtio::Mmio;

use super::layout::Queue;

/// Zero the first `len` bytes of `m`, at most its length.
pub(super) fn wipe(m: Mmio, len: usize) {
    let len = len.min(m.len());
    let words = len / 4 * 4;
    for at in (0..words).step_by(4) {
        m.w32(at, 0);
    }
    for at in words..len {
        m.w8(at, 0);
    }
}

impl Queue {
    /// Put a write's data in the buffer. False, with nothing copied, while
    /// a request holds the buffer or when the data is larger than it.
    pub fn fill(&mut self, src: &[u8]) -> bool {
        let m = self.data.view;
        if !self.idle() || src.len() > m.len() {
            return false;
        }
        let mut words = src.chunks_exact(4);
        for (i, w) in words.by_ref().enumerate() {
            m.w32(4 * i, u32::from_le_bytes([w[0], w[1], w[2], w[3]]));
        }
        let tail = src.len() - words.remainder().len();
        for (i, b) in words.remainder().iter().enumerate() {
            m.w8(tail + i, *b);
        }
        true
    }

    /// Copy the answered read's first `out.len()` bytes out. False while a
    /// request holds the buffer or when `out` is larger than it.
    pub fn take(&self, out: &mut [u8]) -> bool {
        let (m, n) = (self.data.view, out.len());
        if !self.idle() || n > m.len() {
            return false;
        }
        let mut words = out.chunks_exact_mut(4);
        for (i, w) in words.by_ref().enumerate() {
            w.copy_from_slice(&m.r32(4 * i).to_le_bytes());
        }
        let rest = words.into_remainder();
        let tail = n - rest.len();
        for (i, b) in rest.iter_mut().enumerate() {
            *b = m.r8(tail + i);
        }
        true
    }
}
