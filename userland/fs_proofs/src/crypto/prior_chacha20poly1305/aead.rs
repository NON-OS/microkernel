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

//! Prior key-stream XOR, tag and AEAD seal (RFC 8439 section 2.8).

use super::chacha20::chacha20_block;
use super::poly_new::Poly1305;

fn chacha20_xor(key: &[u8; 32], nonce: &[u8; 12], counter: u32, data: &mut [u8]) {
    let mut block = [0u8; 64];
    let mut block_counter = counter;
    let mut offset = 0;
    while offset < data.len() {
        chacha20_block(key, nonce, block_counter, &mut block);
        let to_xor = core::cmp::min(64, data.len() - offset);
        for i in 0..to_xor {
            data[offset + i] ^= block[i];
        }
        offset += to_xor;
        block_counter = block_counter.wrapping_add(1);
    }
}

pub fn poly1305_mac(msg: &[u8], key: &[u8; 32]) -> [u8; 16] {
    let mut poly = Poly1305::new(key);
    poly.update(msg);
    poly.finalize()
}

fn compute_tag(otk: &[u8; 32], aad: &[u8], ciphertext: &[u8]) -> [u8; 16] {
    let mut poly = Poly1305::new(otk);
    let zeros = [0u8; 16];
    poly.update(aad);
    poly.update(&zeros[..(16 - (aad.len() % 16)) % 16]);
    poly.update(ciphertext);
    poly.update(&zeros[..(16 - (ciphertext.len() % 16)) % 16]);
    let mut lengths = [0u8; 16];
    lengths[0..8].copy_from_slice(&(aad.len() as u64).to_le_bytes());
    lengths[8..16].copy_from_slice(&(ciphertext.len() as u64).to_le_bytes());
    poly.update(&lengths);
    poly.finalize()
}

pub fn aead_encrypt(key: &[u8; 32], nonce: &[u8; 12], aad: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let mut block0 = [0u8; 64];
    chacha20_block(key, nonce, 0, &mut block0);
    let otk: [u8; 32] = block0[..32].try_into().unwrap();
    let mut ciphertext = plaintext.to_vec();
    chacha20_xor(key, nonce, 1, &mut ciphertext);
    let tag = compute_tag(&otk, aad, &ciphertext);
    ciphertext.extend_from_slice(&tag);
    ciphertext
}
