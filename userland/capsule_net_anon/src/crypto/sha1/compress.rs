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

//! The SHA-1 block function, eighty rounds over one 64 byte block.

use super::types::{Sha1, BLOCK_BYTES};

impl Sha1 {
    /// Absorb one 64 byte block into the state.
    pub(super) fn compress(&mut self, block: &[u8; BLOCK_BYTES]) {
        let mut w = [0u32; 80];
        for (index, word) in w.iter_mut().enumerate().take(16) {
            let at = index * 4;
            *word = u32::from_be_bytes([block[at], block[at + 1], block[at + 2], block[at + 3]]);
        }
        for index in 16..80 {
            let mixed = w[index - 3] ^ w[index - 8] ^ w[index - 14] ^ w[index - 16];
            w[index] = mixed.rotate_left(1);
        }
        let [mut a, mut b, mut c, mut d, mut e] = self.state;
        for (index, word) in w.iter().enumerate() {
            let (f, k) = round(index, b, c, d);
            let next = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(*word);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = next;
        }
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
    }
}

fn round(index: usize, b: u32, c: u32, d: u32) -> (u32, u32) {
    match index {
        0..=19 => ((b & c) | ((!b) & d), 0x5a82_7999),
        20..=39 => (b ^ c ^ d, 0x6ed9_eba1),
        40..=59 => ((b & c) | (b & d) | (c & d), 0x8f1b_bcdc),
        _ => (b ^ c ^ d, 0xca62_c1d6),
    }
}
