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

//! Unkeyed BLAKE2b with an output of 1 to 64 bytes, RFC 7693 section 3.3,
//! the hash H that Argon2 is built on.

use super::blake2b_compress::compress;
use super::blake2b_consts::IV;
use crate::crypto::constant_time::secure_zero;

pub(super) struct Blake2b {
    h: [u64; 8],
    buf: [u8; 128],
    len: usize,
    total: u128,
    out_len: usize,
}

impl Blake2b {
    /// A hasher for an `out_len`-byte digest; `out_len` is clamped to 1..=64.
    pub(super) fn new(out_len: usize) -> Self {
        let out_len = out_len.clamp(1, 64);
        let mut h = IV;
        h[0] ^= 0x0101_0000 ^ out_len as u64;
        Self { h, buf: [0; 128], len: 0, total: 0, out_len }
    }

    pub(super) fn update(&mut self, mut data: &[u8]) {
        while !data.is_empty() {
            /*
             * A full buffer is compressed only once more input arrives: the
             * last block must be compressed with the final flag set.
             */
            if self.len == 128 {
                self.total += 128;
                compress(&mut self.h, &self.buf, self.total, false);
                self.len = 0;
            }
            let take = (128 - self.len).min(data.len());
            self.buf[self.len..self.len + take].copy_from_slice(&data[..take]);
            self.len += take;
            data = &data[take..];
        }
    }

    /// Write the digest into `out[..out_len]` and wipe the state.
    pub(super) fn finalize(mut self, out: &mut [u8]) {
        self.total += self.len as u128;
        self.buf[self.len..].fill(0);
        compress(&mut self.h, &self.buf, self.total, true);
        for (i, b) in out.iter_mut().take(self.out_len).enumerate() {
            *b = (self.h[i / 8] >> (8 * (i % 8))) as u8;
        }
        secure_zero(&mut self.buf);
        self.h = [0; 8];
    }
}
