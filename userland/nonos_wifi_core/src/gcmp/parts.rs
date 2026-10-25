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

//! GCM's pieces (NIST SP 800-38D, section 7.1): the pre-counter block J0
//! for a 96-bit IV, counter mode from inc32(J0), and the tag
//! E(K, J0) xor GHASH(H, A, C) with H = E(K, 0^128).

use super::aes256::Aes256;
use super::ghash::Ghash;

pub fn ctr(aes: &Aes256, j0: &[u8; 16], data: &mut [u8]) {
    let mut cb = *j0;
    for chunk in data.chunks_mut(16) {
        let c = u32::from_be_bytes([cb[12], cb[13], cb[14], cb[15]]).wrapping_add(1);
        cb[12..].copy_from_slice(&c.to_be_bytes());
        let mut ks = cb;
        aes.encrypt_block(&mut ks);
        for (d, k) in chunk.iter_mut().zip(ks.iter()) {
            *d ^= k;
        }
    }
}

pub fn tag(aes: &Aes256, j0: &[u8; 16], aad: &[u8], ct: &[u8]) -> [u8; 16] {
    let mut h = [0u8; 16];
    aes.encrypt_block(&mut h);
    let mut g = Ghash::new(&h);
    g.update_padded(aad);
    g.update_padded(ct);
    let mut t = g.finish(aad.len(), ct.len());
    let mut e = *j0;
    aes.encrypt_block(&mut e);
    for (a, b) in t.iter_mut().zip(e.iter()) {
        *a ^= b;
    }
    t
}

pub fn j0(iv: &[u8; 12]) -> [u8; 16] {
    let mut j = [0u8; 16];
    j[..12].copy_from_slice(iv);
    j[15] = 1;
    j
}
