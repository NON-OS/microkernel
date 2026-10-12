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

//! AES-128-CMAC (RFC 4493, NIST SP 800-38B), the system's one CMAC. The
//! Wi-Fi core's EAPOL-Key MIC and BIP and every LE Secure Connections
//! function are built on it. The message is taken in parts, so a caller
//! MACs fields or a frame with its MIC zeroed without copying them first.

use crate::types::{Aes128, BLOCK_BYTES};

/// The tag length.
pub const CMAC_BYTES: usize = 16;

// Double in GF(2^128) (RFC 4493, 2.3): shift left one bit and fold in
// 0x87 when a bit falls off. The fold is masked, not branched on, because
// the input is the cipher's output under the key.
fn double(b: &[u8; BLOCK_BYTES]) -> [u8; BLOCK_BYTES] {
    let mut out = [0u8; BLOCK_BYTES];
    let mut carry = 0u8;
    for i in (0..BLOCK_BYTES).rev() {
        out[i] = (b[i] << 1) | carry;
        carry = b[i] >> 7;
    }
    out[BLOCK_BYTES - 1] ^= 0x87 & 0u8.wrapping_sub(carry);
    out
}

/// AES-CMAC under `key` of the concatenation of `parts`. Streams the
/// parts, holding back each full block until it is known not to be last,
/// so nothing is allocated.
pub fn aes_cmac(key: &[u8; 16], parts: &[&[u8]]) -> [u8; CMAC_BYTES] {
    let aes = Aes128::new(key);
    let mut l = [0u8; BLOCK_BYTES];
    aes.encrypt_block(&mut l);
    let k1 = double(&l);
    let total: usize = parts.iter().map(|p| p.len()).sum();
    let (mut x, mut block) = ([0u8; BLOCK_BYTES], [0u8; BLOCK_BYTES]);
    let mut fill = 0usize;
    for &byte in parts.iter().flat_map(|p| p.iter()) {
        if fill == BLOCK_BYTES {
            x.iter_mut().zip(block).for_each(|(a, b)| *a ^= b);
            aes.encrypt_block(&mut x);
            fill = 0;
        }
        block[fill] = byte;
        fill += 1;
    }
    // A full last block takes K1; a short one, or the empty message, is
    // padded 10* and takes K2.
    let subkey = if fill == BLOCK_BYTES && total != 0 { k1 } else { double(&k1) };
    if fill < BLOCK_BYTES {
        block[fill] = 0x80;
        block[fill + 1..].fill(0);
    }
    x.iter_mut().zip(block).zip(subkey).for_each(|((a, b), k)| *a ^= b ^ k);
    aes.encrypt_block(&mut x);
    x
}
