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

//! Absorbing input, in whatever sized pieces the caller has.

use super::types::{Sha256, BLOCK_BYTES};

impl Sha256 {
    /// Absorb `data`. The result depends only on the bytes and their order, never
    /// on how they were split across calls, which is what lets a 1.7 MB document be
    pub fn update(&mut self, data: &[u8]) {
        self.total = self.total.wrapping_add(data.len() as u64);
        let mut rest = data;

        // Top up a part-filled block first, so the fast path below always starts
        // on a block boundary.
        if self.buffered > 0 {
            let want = BLOCK_BYTES - self.buffered;
            let take = want.min(rest.len());
            self.block[self.buffered..self.buffered + take].copy_from_slice(&rest[..take]);
            self.buffered += take;
            rest = &rest[take..];
            if self.buffered < BLOCK_BYTES {
                return;
            }
            self.compress();
            self.buffered = 0;
        }

        // Whole blocks straight through the buffer.
        while rest.len() >= BLOCK_BYTES {
            self.block.copy_from_slice(&rest[..BLOCK_BYTES]);
            self.compress();
            rest = &rest[BLOCK_BYTES..];
        }

        // And the tail, kept for the next call or for the padding.
        if !rest.is_empty() {
            self.block[..rest.len()].copy_from_slice(rest);
            self.buffered = rest.len();
        }
    }
}
