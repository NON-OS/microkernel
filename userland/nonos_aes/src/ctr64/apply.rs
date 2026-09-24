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

//! Applying the low 64 bit keystream, byte by byte across blocks.

use crate::types::BLOCK_BYTES;

use super::types::Ctr64Be;

impl Ctr64Be {
    /// XOR the keystream into `data`, continuing where the last call stopped.
    /// Encryption and decryption are the same operation.
    pub fn apply(&mut self, data: &mut [u8]) {
        for byte in data.iter_mut() {
            if self.used == BLOCK_BYTES {
                self.refill();
            }
            *byte ^= self.held[self.used];
            self.used += 1;
        }
    }

    fn refill(&mut self) {
        self.held = self.counter;
        self.cipher.encrypt_block(&mut self.held);
        self.used = 0;
        self.bump();
    }

    /*
     * Increment the low 64 bits big endian, wrapping within them. The high
     * half is the nonce prefix and never moves: a 128 bit counter would agree
     * for the first 2^64 blocks and still be the wrong cipher for Sphinx.
     */
    fn bump(&mut self) {
        let mut low = [0u8; 8];
        low.copy_from_slice(&self.counter[8..]);
        let next = u64::from_be_bytes(low).wrapping_add(1);
        self.counter[8..].copy_from_slice(&next.to_be_bytes());
    }
}
