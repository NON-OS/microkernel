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

//! Absorbing bytes, buffering whatever does not fill a block.

use super::types::{Sha1, BLOCK_BYTES};

impl Sha1 {
    /// Feed bytes in. A cell payload is 509 bytes, so this is always called
    /// with a length that leaves a partial block behind.
    pub fn update(&mut self, mut data: &[u8]) {
        self.total = self.total.wrapping_add(data.len() as u64);
        if self.buffered > 0 {
            let want = BLOCK_BYTES - self.buffered;
            let take = core::cmp::min(want, data.len());
            self.block[self.buffered..self.buffered + take].copy_from_slice(&data[..take]);
            self.buffered += take;
            data = &data[take..];
            if self.buffered < BLOCK_BYTES {
                return;
            }
            let full = self.block;
            self.compress(&full);
            self.buffered = 0;
        }
        while data.len() >= BLOCK_BYTES {
            let mut full = [0u8; BLOCK_BYTES];
            full.copy_from_slice(&data[..BLOCK_BYTES]);
            self.compress(&full);
            data = &data[BLOCK_BYTES..];
        }
        if !data.is_empty() {
            self.block[..data.len()].copy_from_slice(data);
            self.buffered = data.len();
        }
    }
}
