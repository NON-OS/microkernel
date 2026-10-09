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

//! AES-128-CMAC (RFC 4493, NIST SP 800-38B). The EAPOL-Key MIC for the SAE and
//! PSK-SHA256 AKMs is AES-128-CMAC under the KCK (IEEE Std 802.11-2020,
//! 12.7.3, key descriptor version 0 and 3), where the WPA2-PSK MIC was
//! HMAC-SHA1. The message is taken as parts so a frame with its MIC field
//! zeroed is MACed in place. Checked against the RFC 4493 vectors.

use super::aes::Aes128;

/// The CMAC tag length in bytes.
pub const CMAC_LEN: usize = 16;

// Double a block in GF(2^128): shift left one bit, and if the bit shifted out
// was set, fold in the constant 0x87.
fn dbl(b: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    let mut carry = 0u8;
    for i in (0..16).rev() {
        out[i] = (b[i] << 1) | carry;
        carry = b[i] >> 7;
    }
    // Branch-free: the mask is all ones exactly when the top bit was set.
    out[15] ^= 0x87 & 0u8.wrapping_sub(carry);
    out
}

/// AES-128-CMAC of the concatenation of `parts` under `key`.
pub fn aes_cmac_parts(key: &[u8; 16], parts: &[&[u8]]) -> [u8; CMAC_LEN] {
    let aes = Aes128::new(key);
    let mut l = [0u8; 16];
    aes.encrypt_block(&mut l);
    let k1 = dbl(&l);
    let k2 = dbl(&k1);

    let total: usize = parts.iter().map(|p| p.len()).sum();
    // The number of 16-byte blocks; an empty message is one padded block.
    let blocks = if total == 0 { 1 } else { total.div_ceil(16) };
    let last_complete = total != 0 && total.is_multiple_of(16);

    let mut x = [0u8; 16];
    let mut block = [0u8; 16];
    let mut fill = 0usize;
    let mut done = 0usize;
    for p in parts {
        for &byte in p.iter() {
            block[fill] = byte;
            fill += 1;
            // Hold back the final block: it is masked with a subkey first.
            if fill == 16 && done + 1 < blocks {
                for (xi, bi) in x.iter_mut().zip(block.iter()) {
                    *xi ^= *bi;
                }
                aes.encrypt_block(&mut x);
                done += 1;
                fill = 0;
            }
        }
    }
    let subkey = if last_complete {
        &k1
    } else {
        // Pad the incomplete final block with 10*.
        block[fill] = 0x80;
        for b in block.iter_mut().skip(fill + 1) {
            *b = 0;
        }
        &k2
    };
    for i in 0..16 {
        x[i] ^= block[i] ^ subkey[i];
    }
    aes.encrypt_block(&mut x);
    x
}

/// AES-128-CMAC of a single message.
pub fn aes_cmac(key: &[u8; 16], msg: &[u8]) -> [u8; CMAC_LEN] {
    aes_cmac_parts(key, &[msg])
}
