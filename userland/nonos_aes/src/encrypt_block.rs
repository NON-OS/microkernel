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

//! The forward cipher: one block in place, ten rounds or fourteen.

use crate::mix_columns::mix_columns;
use crate::shift_rows::shift_rows;
use crate::sub_byte::sub_byte;
use crate::types::{Aes128, BLOCK_BYTES, ROUNDS};

impl Aes128 {
    /// Encrypt one block in place. CTR mode never decrypts, so there is no
    /// inverse cipher here and no inverse tables to carry.
    pub fn encrypt_block(&self, block: &mut [u8; BLOCK_BYTES]) {
        encrypt_with(&self.round_keys, ROUNDS, block);
    }
}

/// The forward cipher under an expanded key of `rounds + 1` round keys. One
/// body for both key sizes: AES-256 differs from AES-128 only in its schedule
/// and in running four more rounds.
pub(crate) fn encrypt_with(round_keys: &[u8], rounds: usize, block: &mut [u8; BLOCK_BYTES]) {
    if crate::hardware::in_use() && crate::hardware::encrypt(round_keys, rounds, block) {
        return;
    }
    add_round_key(round_keys, block, 0);
    for round in 1..rounds {
        sub_bytes(block);
        shift_rows(block);
        mix_columns(block);
        add_round_key(round_keys, block, round);
    }
    sub_bytes(block);
    shift_rows(block);
    add_round_key(round_keys, block, rounds);
}

fn add_round_key(round_keys: &[u8], block: &mut [u8; BLOCK_BYTES], round: usize) {
    let at = round * BLOCK_BYTES;
    for (byte, key) in block.iter_mut().zip(round_keys[at..].iter()) {
        *byte ^= key;
    }
}

fn sub_bytes(block: &mut [u8; BLOCK_BYTES]) {
    for byte in block.iter_mut() {
        *byte = sub_byte(*byte);
    }
}
