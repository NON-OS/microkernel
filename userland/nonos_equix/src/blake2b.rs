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


//! BLAKE2b with the parameter block's salt, as RFC 7693 and the BLAKE2 paper
//! define it. No key: neither HashX's seed expansion nor the onion service's
//! effort check is keyed.
//!
//! HashX sets the salt to "HashX v1" so its seed expansion cannot collide with
//! any other use of BLAKE2b; the effort check uses the default, all-zero salt.

const IV: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

const SIGMA: [[usize; 16]; 12] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
];

/// An unkeyed BLAKE2b in progress, with its output length fixed at the start.
pub struct Blake2b {
    h: [u64; 8],
    t: u128,
    block: [u8; 128],
    filled: usize,
    out_len: usize,
}

impl Blake2b {
    /// A hash of `out_len` bytes, 1 to 64, under a 16-byte salt. A length
    /// outside that range is clamped into it, since every caller in this crate
    /// passes a constant.
    pub fn new(out_len: usize, salt: &[u8; 16]) -> Self {
        let out_len = out_len.clamp(1, 64);
        let mut h = IV;
        // Parameter block word 0: digest length, key length 0, fanout 1,
        // depth 1. Words 4 and 5 are the salt; the rest stay zero.
        h[0] ^= 0x0101_0000 ^ out_len as u64;
        h[4] ^= u64::from_le_bytes(word(salt, 0));
        h[5] ^= u64::from_le_bytes(word(salt, 8));
        Self { h, t: 0, block: [0; 128], filled: 0, out_len }
    }

    pub fn update(&mut self, mut data: &[u8]) {
        while !data.is_empty() {
            // The last block is compressed in `finish` with the final flag,
            // so a full block is held back until more data arrives.
            if self.filled == 128 {
                self.t += 128;
                let block = self.block;
                self.compress(&block, false);
                self.filled = 0;
            }
            let take = (128 - self.filled).min(data.len());
            self.block[self.filled..self.filled + take].copy_from_slice(&data[..take]);
            self.filled += take;
            data = &data[take..];
        }
    }

    /// The digest, written into the front of `out`. Bytes past the output
    /// length are left as they were.
    pub fn finish(mut self, out: &mut [u8]) {
        self.t += self.filled as u128;
        let mut block = self.block;
        block[self.filled..].fill(0);
        self.compress(&block, true);
        let mut bytes = [0u8; 64];
        for (chunk, word) in bytes.chunks_exact_mut(8).zip(self.h.iter()) {
            chunk.copy_from_slice(&word.to_le_bytes());
        }
        let n = self.out_len.min(out.len());
        out[..n].copy_from_slice(&bytes[..n]);
    }

    fn compress(&mut self, block: &[u8; 128], last: bool) {
        let mut m = [0u64; 16];
        for (i, word) in m.iter_mut().enumerate() {
            *word = u64::from_le_bytes(word_at(block, i * 8));
        }
        let mut v = [0u64; 16];
        v[..8].copy_from_slice(&self.h);
        v[8..].copy_from_slice(&IV);
        v[12] ^= self.t as u64;
        v[13] ^= (self.t >> 64) as u64;
        if last {
            v[14] = !v[14];
        }
        for s in SIGMA.iter() {
            mix(&mut v, 0, 4, 8, 12, m[s[0]], m[s[1]]);
            mix(&mut v, 1, 5, 9, 13, m[s[2]], m[s[3]]);
            mix(&mut v, 2, 6, 10, 14, m[s[4]], m[s[5]]);
            mix(&mut v, 3, 7, 11, 15, m[s[6]], m[s[7]]);
            mix(&mut v, 0, 5, 10, 15, m[s[8]], m[s[9]]);
            mix(&mut v, 1, 6, 11, 12, m[s[10]], m[s[11]]);
            mix(&mut v, 2, 7, 8, 13, m[s[12]], m[s[13]]);
            mix(&mut v, 3, 4, 9, 14, m[s[14]], m[s[15]]);
        }
        for i in 0..8 {
            self.h[i] ^= v[i] ^ v[i + 8];
        }
    }
}

fn mix(v: &mut [u64; 16], a: usize, b: usize, c: usize, d: usize, x: u64, y: u64) {
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(x);
    v[d] = (v[d] ^ v[a]).rotate_right(32);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(24);
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(y);
    v[d] = (v[d] ^ v[a]).rotate_right(16);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(63);
}

fn word(bytes: &[u8; 16], at: usize) -> [u8; 8] {
    let mut w = [0u8; 8];
    w.copy_from_slice(&bytes[at..at + 8]);
    w
}

fn word_at(bytes: &[u8; 128], at: usize) -> [u8; 8] {
    let mut w = [0u8; 8];
    w.copy_from_slice(&bytes[at..at + 8]);
    w
}
