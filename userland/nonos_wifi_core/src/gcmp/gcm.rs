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

//! GCM with AES-256 and a 96-bit IV (NIST SP 800-38D, section 7): H is
//! the cipher of the zero block, J0 is IV || 0^31 || 1, the payload is
//! counter mode from inc32(J0), and the 128-bit tag is E(J0) xor
//! GHASH(A, C). Opening compares the tag in constant time and releases
//! no plaintext unless it matches.

use alloc::vec::Vec;

use super::aes256::Aes256;
use super::parts::{ctr, j0, tag};

pub const TAG_LEN: usize = 16;

/// The ciphertext followed by its tag.
pub fn seal(key: &[u8; 32], iv: &[u8; 12], aad: &[u8], plain: &[u8]) -> Vec<u8> {
    let (aes, j) = (Aes256::new(key), j0(iv));
    let mut out = plain.to_vec();
    ctr(&aes, &j, &mut out);
    let t = tag(&aes, &j, aad, &out);
    out.extend_from_slice(&t);
    out
}

/// The plaintext of `ct || tag`, or `None` if the tag does not verify.
pub fn open(key: &[u8; 32], iv: &[u8; 12], aad: &[u8], sealed: &[u8]) -> Option<Vec<u8>> {
    let n = sealed.len().checked_sub(TAG_LEN)?;
    let (ct, t) = sealed.split_at(n);
    let (aes, j) = (Aes256::new(key), j0(iv));
    let want = tag(&aes, &j, aad, ct);
    let diff = want.iter().zip(t.iter()).fold(0u8, |d, (a, b)| d | (a ^ b));
    if diff != 0 {
        return None;
    }
    let mut out = ct.to_vec();
    ctr(&aes, &j, &mut out);
    Some(out)
}
