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

//! Padding and reading out, either ending the hash or peeking at it.

use super::types::{Sha1, BLOCK_BYTES, DIGEST_BYTES};

impl Sha1 {
    /// Consume the hash and return its digest.
    pub fn finish(mut self) -> [u8; DIGEST_BYTES] {
        let bits = self.total.wrapping_mul(8);
        self.absorb_padding();
        let mut tail = [0u8; BLOCK_BYTES];
        let split = BLOCK_BYTES - 8;
        if self.buffered > split {
            /*
             * 0x80 left no room for the length, so this block closes
             * without it and the length lands in the next one.
             */
            tail[..self.buffered].copy_from_slice(&self.block[..self.buffered]);
            self.compress(&tail);
            tail = [0u8; BLOCK_BYTES];
        } else {
            tail[..self.buffered].copy_from_slice(&self.block[..self.buffered]);
        }
        tail[split..].copy_from_slice(&bits.to_be_bytes());
        self.compress(&tail);
        let mut out = [0u8; DIGEST_BYTES];
        for (index, word) in self.state.iter().enumerate() {
            out[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        out
    }

    /// The digest the hash would produce now, leaving it able to absorb more.
    ///
    pub fn peek(&self) -> [u8; DIGEST_BYTES] {
        self.clone().finish()
    }

    fn absorb_padding(&mut self) {
        self.block[self.buffered] = 0x80;
        self.buffered += 1;
    }
}
