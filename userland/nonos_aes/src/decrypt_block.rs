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

//! The inverse cipher, for AES Key Unwrap (RFC 3394), which runs the block
//! function backwards. Counter mode and CMAC never need it.

use crate::inv_steps::{inv_mix_columns, inv_shift_rows};
use crate::inv_sub_byte::inv_sub_byte;
use crate::types::{Aes128, BLOCK_BYTES, ROUNDS};

impl Aes128 {
    /// Decrypt one block in place (FIPS 197, 5.3). Always the software
    /// path: the AES-NI decryption rounds want the middle round keys
    /// passed through InvMixColumns first, a second schedule held for an
    /// operation that runs a few hundred blocks per key exchange.
    pub fn decrypt_block(&self, block: &mut [u8; BLOCK_BYTES]) {
        add_round_key(&self.round_keys, block, ROUNDS);
        for round in (1..ROUNDS).rev() {
            inv_shift_rows(block);
            inv_sub_bytes(block);
            add_round_key(&self.round_keys, block, round);
            inv_mix_columns(block);
        }
        inv_shift_rows(block);
        inv_sub_bytes(block);
        add_round_key(&self.round_keys, block, 0);
    }
}

fn add_round_key(round_keys: &[u8], block: &mut [u8; BLOCK_BYTES], round: usize) {
    let at = round * BLOCK_BYTES;
    for (byte, key) in block.iter_mut().zip(round_keys[at..].iter()) {
        *byte ^= key;
    }
}

fn inv_sub_bytes(block: &mut [u8; BLOCK_BYTES]) {
    for byte in block.iter_mut() {
        *byte = inv_sub_byte(*byte);
    }
}
