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

//! Key data as the access point sends it: padded to the key-wrap block size
//! and wrapped under the KEK.

use nonos_wifi_core::ccmp::aes::Aes128;

/// RFC 3394 AES key wrap: the AP's half of the group-key delivery.
pub fn aes_wrap(kek: &[u8; 16], plain: &[u8]) -> Vec<u8> {
    assert!(plain.len().is_multiple_of(8) && !plain.is_empty());
    let n = plain.len() / 8;
    let aes = Aes128::new(kek);
    let mut a = [0xA6u8; 8];
    let mut r: Vec<[u8; 8]> = (0..n).map(|i| plain[i * 8..i * 8 + 8].try_into().unwrap()).collect();
    for j in 0..6u64 {
        for (i, ri) in r.iter_mut().enumerate() {
            let mut block = [0u8; 16];
            block[..8].copy_from_slice(&a);
            block[8..].copy_from_slice(&ri[..]);
            aes.encrypt_block(&mut block);
            let t = (n as u64) * j + (i as u64) + 1;
            a.copy_from_slice(&block[..8]);
            for (k, ak) in a.iter_mut().enumerate() {
                *ak ^= (t >> (56 - 8 * k)) as u8;
            }
            ri.copy_from_slice(&block[8..]);
        }
    }
    let mut out = a.to_vec();
    for block in &r {
        out.extend_from_slice(block);
    }
    out
}

/// Pad key data to the key-wrap block size with 0xDD then zeros.
pub fn pad(mut kd: Vec<u8>) -> Vec<u8> {
    if !kd.len().is_multiple_of(8) || kd.len() < 16 {
        kd.push(0xDD);
        while !kd.len().is_multiple_of(8) || kd.len() < 16 {
            kd.push(0);
        }
    }
    kd
}
