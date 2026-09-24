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

//! Expanding a 16 byte key into eleven round keys.

use crate::sub_byte::sub_byte;
use crate::types::{Aes128, EXPANDED_BYTES, KEY_BYTES};

const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

impl Aes128 {
    pub fn new(key: &[u8; KEY_BYTES]) -> Self {
        let mut round_keys = [0u8; EXPANDED_BYTES];
        round_keys[..KEY_BYTES].copy_from_slice(key);
        let mut at = KEY_BYTES;
        while at < EXPANDED_BYTES {
            let mut word =
                [round_keys[at - 4], round_keys[at - 3], round_keys[at - 2], round_keys[at - 1]];
            if at.is_multiple_of(KEY_BYTES) {
                word = [
                    sub_byte(word[1]) ^ RCON[at / KEY_BYTES - 1],
                    sub_byte(word[2]),
                    sub_byte(word[3]),
                    sub_byte(word[0]),
                ];
            }
            for (offset, byte) in word.iter().enumerate() {
                round_keys[at + offset] = round_keys[at + offset - KEY_BYTES] ^ byte;
            }
            at += 4;
        }
        Self { round_keys }
    }
}
