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

//! Padding and the final digest, FIPS 180-4 section 5.1.1.

use super::types::{Sha256, BLOCK_BYTES, DIGEST_BYTES};

impl Sha256 {
    /// The digest of everything absorbed so far.
    ///
    pub fn finish(mut self) -> [u8; DIGEST_BYTES] {
        /*
         * A single 1 bit, then zeroes, then the length in bits as a 64 bit big
         * endian count. The length is of the message, so it is taken before any
         * padding goes in.
         */
        let bits = self.total.wrapping_mul(8);
        self.pad_byte(0x80);
        while self.buffered != BLOCK_BYTES - 8 {
            self.pad_byte(0x00);
        }
        for byte in bits.to_be_bytes() {
            self.pad_byte(byte);
        }

        let mut out = [0u8; DIGEST_BYTES];
        for (i, word) in self.state.iter().enumerate() {
            out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        out
    }

    fn pad_byte(&mut self, byte: u8) {
        self.block[self.buffered] = byte;
        self.buffered += 1;
        if self.buffered == BLOCK_BYTES {
            self.compress();
            self.buffered = 0;
        }
    }
}

/// The digest of one contiguous slice, for callers with the whole input to hand.
pub fn digest(data: &[u8]) -> [u8; DIGEST_BYTES] {
    let mut hash = Sha256::new();
    hash.update(data);
    hash.finish()
}
