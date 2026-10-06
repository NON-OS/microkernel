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


//! AES-256, for the onion service layer.
//!
//! An onion service's virtual hop, its INTRODUCE1 payload and both layers of
//! its descriptor are AES-256 in counter mode (rend-spec-v3, and
//! relay_crypto_init with is_hs_v3 in the fork), where every other relay hop
//! is AES-128. Same block function, a longer schedule and four more rounds.

use crate::encrypt_block::encrypt_with;
use crate::sub_byte::sub_byte;
use crate::types::BLOCK_BYTES;

pub const KEY256_BYTES: usize = 32;
const ROUNDS: usize = 14;
const EXPANDED_BYTES: usize = BLOCK_BYTES * (ROUNDS + 1);
const RCON: [u8; 7] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40];

pub struct Aes256 {
    round_keys: [u8; EXPANDED_BYTES],
}

impl Aes256 {
    /// FIPS-197 section 5.2 with Nk = 8: every eighth word is rotated,
    /// substituted and given a round constant, and the word four after it is
    /// substituted as well.
    pub fn new(key: &[u8; KEY256_BYTES]) -> Self {
        let mut round_keys = [0u8; EXPANDED_BYTES];
        round_keys[..KEY256_BYTES].copy_from_slice(key);
        let mut at = KEY256_BYTES;
        while at < EXPANDED_BYTES {
            let mut word =
                [round_keys[at - 4], round_keys[at - 3], round_keys[at - 2], round_keys[at - 1]];
            if at.is_multiple_of(KEY256_BYTES) {
                word = [
                    sub_byte(word[1]) ^ RCON[at / KEY256_BYTES - 1],
                    sub_byte(word[2]),
                    sub_byte(word[3]),
                    sub_byte(word[0]),
                ];
            } else if at % KEY256_BYTES == 16 {
                word = [sub_byte(word[0]), sub_byte(word[1]), sub_byte(word[2]), sub_byte(word[3])];
            }
            for (offset, byte) in word.iter().enumerate() {
                round_keys[at + offset] = round_keys[at + offset - KEY256_BYTES] ^ byte;
            }
            at += 4;
        }
        Self { round_keys }
    }

    /// Encrypt one block in place.
    pub fn encrypt_block(&self, block: &mut [u8; BLOCK_BYTES]) {
        encrypt_with(&self.round_keys, ROUNDS, block);
    }
}

impl Drop for Aes256 {
    fn drop(&mut self) {
        for byte in self.round_keys.iter_mut() {
            /*
             * SAFETY: eK@nonos.systems. A key schedule, wiped as Aes128's is:
             * a plain store into a value being dropped may be optimised away.
             */
            unsafe { core::ptr::write_volatile(byte, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
