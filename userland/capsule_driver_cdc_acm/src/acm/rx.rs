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
//! What the device sent, kept until the client reads it. The bulk IN pipe
//! is read only while a whole read fits, so a client that falls behind
//! leaves the bytes in the device, which holds them off with NAKs, and
//! none is dropped here.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

pub const RX_CAP: usize = 16 * 1024;

#[derive(Default)]
pub struct Rx {
    q: VecDeque<u8>,
}

impl Rx {
    /// Whether a read of `n` bytes fits whole.
    pub fn fits(&self, n: usize) -> bool {
        RX_CAP - self.q.len() >= n
    }

    pub fn put(&mut self, b: &[u8]) {
        let n = b.len().min(RX_CAP - self.q.len());
        self.q.extend(&b[..n]);
    }

    pub fn take(&mut self, most: usize) -> Vec<u8> {
        let n = most.min(self.q.len());
        self.q.drain(..n).collect()
    }

    /// The bytes waiting for the client.
    pub fn queued(&self) -> usize {
        self.q.len()
    }

    pub fn clear(&mut self) {
        self.q.clear();
    }
}
