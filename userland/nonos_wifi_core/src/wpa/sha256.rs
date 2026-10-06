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

//! HMAC-SHA256 (RFC 2104 over FIPS 180-4), the hash every WPA3 derivation is
//! keyed with: SAE's pwd-seed, keyseed and confirm, the 802.11 KDF that expands
//! the PTK for the SHA-256 AKMs, and HKDF for hash-to-element. The message is
//! taken as a list of parts so a concatenation is MACed without allocating.

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// The SHA-256 output length in bytes.
pub const SHA256_LEN: usize = 32;

/// HMAC-SHA256 of the concatenation of `parts` under `key`. HMAC accepts a key
/// of any length (a long one is hashed first), so construction cannot fail;
/// the error arm exists only because the trait signature is fallible.
pub fn hmac_sha256_parts(key: &[u8], parts: &[&[u8]]) -> [u8; SHA256_LEN] {
    let mut out = [0u8; SHA256_LEN];
    let Ok(mut mac) = <HmacSha256 as Mac>::new_from_slice(key) else {
        return out;
    };
    for p in parts {
        mac.update(p);
    }
    out.copy_from_slice(&mac.finalize().into_bytes());
    out
}

/// HMAC-SHA256 of a single message.
pub fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; SHA256_LEN] {
    hmac_sha256_parts(key, &[msg])
}
