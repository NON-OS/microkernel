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

//! HKDF over HMAC-SHA256 (RFC 5869). SAE hash-to-element seeds the password
//! element from HKDF-Extract(ssid, password) and expands it with HKDF-Expand,
//! and the SAE keyseed is an HKDF-Extract of the shared secret (IEEE Std
//! 802.11-2020, 12.4.4.2.3 and 12.4.5.4). Checked through the SAE vectors.

use super::sha256::{hmac_sha256_parts, SHA256_LEN};

/// HKDF-Extract(salt, ikm): the pseudo-random key HMAC-SHA256(salt, ikm), with
/// the input keying material given as parts that are concatenated.
pub fn extract(salt: &[u8], ikm: &[&[u8]]) -> [u8; SHA256_LEN] {
    hmac_sha256_parts(salt, ikm)
}

/// HKDF-Expand(prk, info, out.len()). RFC 5869 caps the output at 255 blocks;
/// a longer request leaves `out` zeroed and returns false.
pub fn expand(prk: &[u8], info: &[u8], out: &mut [u8]) -> bool {
    if out.len() > 255 * SHA256_LEN {
        out.fill(0);
        return false;
    }
    let mut prev = [0u8; SHA256_LEN];
    let mut prev_len = 0usize;
    let mut counter: u8 = 1;
    let mut off = 0usize;
    while off < out.len() {
        let block = hmac_sha256_parts(prk, &[&prev[..prev_len], info, &[counter]]);
        let n = core::cmp::min(SHA256_LEN, out.len() - off);
        out[off..off + n].copy_from_slice(&block[..n]);
        prev = block;
        prev_len = SHA256_LEN;
        off += n;
        counter = counter.wrapping_add(1);
    }
    true
}
