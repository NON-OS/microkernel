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

//! The IEEE 802.11 key derivation function over HMAC-SHA256 (IEEE Std
//! 802.11-2020, 12.7.1.6.2, "KDF-Hash-Length"). Each output block is
//! HMAC-SHA256(K, i || label || context || Length), where `i` is a 16-bit
//! little-endian counter starting at 1 and `Length` is the requested output in
//! bits, also 16-bit little-endian. The SHA-256 AKMs (SAE, PSK-SHA256) expand
//! the PTK with it, and SAE derives its KCK and PMK and the hunting-and-pecking
//! candidate with it. Checked through the SAE known-answer vectors.

use super::sha256::{hmac_sha256_parts, SHA256_LEN};

/// Fill `out` with KDF-SHA256(key, label, context) of `out.len() * 8` bits.
/// The output is limited to what the 16-bit Length field can name; a longer
/// request leaves `out` zeroed and returns false.
pub fn kdf_sha256(key: &[u8], label: &[u8], context: &[u8], out: &mut [u8]) -> bool {
    let Some(bits) = out.len().checked_mul(8).and_then(|b| u16::try_from(b).ok()) else {
        out.fill(0);
        return false;
    };
    let length = bits.to_le_bytes();
    let mut counter: u16 = 1;
    let mut off = 0usize;
    while off < out.len() {
        let block = hmac_sha256_parts(key, &[&counter.to_le_bytes(), label, context, &length]);
        let n = core::cmp::min(SHA256_LEN, out.len() - off);
        out[off..off + n].copy_from_slice(&block[..n]);
        off += n;
        counter = counter.wrapping_add(1);
    }
    true
}
