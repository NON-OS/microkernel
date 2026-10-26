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

//! The PCM the audio server sent and the device has not played yet.

use alloc::vec;
use alloc::vec::Vec;

/// About 85 ms at 48 kHz stereo: four of the audio server's pieces, so a
/// write is taken while the one before plays, and a stop is heard soon.
pub const QUEUE_BYTES: usize = 16 * 1024;

pub struct PcmQueue {
    buf: Vec<u8>,
    head: usize,
    len: usize,
}

impl PcmQueue {
    /// Made once, by the run loop; `Default` (fresh.rs) is the same.
    pub fn new() -> Self {
        Self { buf: vec![0u8; QUEUE_BYTES], head: 0, len: 0 }
    }

    pub fn fits(&self, n: usize) -> bool {
        QUEUE_BYTES - self.len >= n
    }

    pub fn clear(&mut self) {
        (self.head, self.len) = (0, 0);
    }

    /// Takes all of `data` or none of it.
    pub fn push(&mut self, data: &[u8]) -> bool {
        if !self.fits(data.len()) {
            return false;
        }
        let tail = (self.head + self.len) % QUEUE_BYTES;
        let first = data.len().min(QUEUE_BYTES - tail);
        self.buf[tail..tail + first].copy_from_slice(&data[..first]);
        self.buf[..data.len() - first].copy_from_slice(&data[first..]);
        self.len += data.len();
        true
    }

    /// Fills `out` from the queue and the rest with silence, so a stream
    /// that ran short keeps its clock. Returns the bytes that were queued.
    pub fn pop_into(&mut self, out: &mut [u8]) -> usize {
        let n = self.len.min(out.len());
        let first = n.min(QUEUE_BYTES - self.head);
        out[..first].copy_from_slice(&self.buf[self.head..self.head + first]);
        out[first..n].copy_from_slice(&self.buf[..n - first]);
        out[n..].fill(0);
        self.head = (self.head + n) % QUEUE_BYTES;
        self.len -= n;
        n
    }
}
