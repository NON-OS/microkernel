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

//! AES-CMAC (RFC 4493), the one MAC every LE Secure Connections function is
//! built from (Vol 3, Part H, 2.2.5). The message is given in parts so the
//! functions can pass their fields without first copying them together.

use alloc::vec::Vec;
use nonos_aes::Aes128;

const RB: u8 = 0x87;

/// One left shift of a 128-bit value, with Rb folded in when a bit falls
/// off (RFC 4493, 2.3).
fn double(v: [u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    for i in 0..16 {
        let next = if i < 15 { v[i + 1] >> 7 } else { 0 };
        out[i] = v[i] << 1 | next;
    }
    if v[0] & 0x80 != 0 {
        out[15] ^= RB;
    }
    out
}

/// AES-CMAC under `key` of the concatenation of `parts`.
pub fn aes_cmac(key: &[u8; 16], parts: &[&[u8]]) -> [u8; 16] {
    let aes = Aes128::new(key);
    let mut l = [0u8; 16];
    aes.encrypt_block(&mut l);
    let k1 = double(l);
    let msg: Vec<u8> = parts.concat();
    let full = !msg.is_empty() && msg.len().is_multiple_of(16);
    let blocks = msg.len().div_ceil(16).max(1);
    let mut x = [0u8; 16];
    for (i, chunk) in msg.chunks(16).chain(msg.is_empty().then_some(&[][..])).enumerate() {
        let mut block = [0u8; 16];
        block[..chunk.len()].copy_from_slice(chunk);
        if i + 1 == blocks {
            let sub = if full { k1 } else { double(k1) };
            if !full {
                block[chunk.len()] = 0x80;
            }
            block.iter_mut().zip(sub).for_each(|(b, s)| *b ^= s);
        }
        x.iter_mut().zip(block).for_each(|(a, b)| *a ^= b);
        aes.encrypt_block(&mut x);
    }
    x
}
