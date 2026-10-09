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

/*
 * SHA-256 (FIPS 180-4) that can be put down and picked up. Its whole state
 * is the eight words after the last full block, the count of bytes taken and
 * the block still filling, so a stream cut by a reboot hashes on from its
 * last saved state. The compression is the `sha2` crate's own.
 */

use sha2::compress256;
use sha2::digest::generic_array::GenericArray;

const IV: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];
pub(super) struct PinHash {
    pub(super) h: [u32; 8],
    pub(super) total: u64,
    pub(super) buf: [u8; 64],
}

impl PinHash {
    pub(super) fn new() -> Self {
        PinHash { h: IV, total: 0, buf: [0; 64] }
    }

    /* Bytes hashed so far. */
    pub(super) fn taken(&self) -> u64 {
        self.total
    }

    pub(super) fn update(&mut self, mut data: &[u8]) {
        while !data.is_empty() {
            let at = (self.total % 64) as usize;
            let take = (64 - at).min(data.len());
            self.buf[at..at + take].copy_from_slice(&data[..take]);
            (self.total, data) = (self.total + take as u64, &data[take..]);
            if at + take == 64 {
                compress256(&mut self.h, &[*GenericArray::from_slice(&self.buf)]);
            }
        }
    }

    pub(super) fn finish(mut self) -> [u8; 32] {
        let bits = self.total.wrapping_mul(8);
        let pad = 1 + (119 - self.total % 64) % 64;
        let mut tail = [0u8; 72];
        tail[0] = 0x80;
        self.update(&tail[..pad as usize]);
        self.update(&bits.to_be_bytes());
        let mut out = [0u8; 32];
        out.chunks_mut(4).zip(self.h).for_each(|(o, w)| o.copy_from_slice(&w.to_be_bytes()));
        out
    }
}
