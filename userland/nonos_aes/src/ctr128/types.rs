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

//! A keystream that counts across the whole block and keeps its place.

use crate::types::{Aes128, BLOCK_BYTES, KEY_BYTES};

/*
 * One keystream per direction per hop, counting from zero.
 *
 * The counter is the whole 128 bit block, big endian, which is what the
 * relay on the other side is running. A 64 bit counter with a fixed nonce
 * prefix agrees with it for the first 2^64 blocks and is still the wrong
 * cipher, so the width is stated rather than inherited.
 *
 * The keystream is also not block aligned to the data. A hop encrypts 509
 * byte payloads, which is not a multiple of 16, so a cell leaves the
 * keystream mid block and the next cell has to carry on from exactly there.
 * That is what `held` is for.
 */
pub struct Ctr128Be {
    pub(super) cipher: Aes128,
    pub(super) counter: [u8; BLOCK_BYTES],
    pub(super) held: [u8; BLOCK_BYTES],
    pub(super) used: usize,
}

impl Ctr128Be {
    /// A fresh keystream under `key`, counter at zero, as the relay starts it.
    pub fn new(key: &[u8; KEY_BYTES]) -> Self {
        Self {
            cipher: Aes128::new(key),
            counter: [0u8; BLOCK_BYTES],
            held: [0u8; BLOCK_BYTES],
            used: BLOCK_BYTES,
        }
    }
}
