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

//! HMAC-SHA-256 over a message given in parts, computed in the caller.

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/*
 * The message comes in parts so HKDF can feed previous block, info and counter
 * without first gluing them into a fresh buffer. `None` only if the MAC refuses
 * the key, which HMAC does for no length: a long key is hashed first.
 */
/// The tag over the concatenation of `parts`, keyed with `key`.
pub fn tag(key: &[u8], parts: &[&[u8]]) -> Option<[u8; 32]> {
    let mac = keyed(key, parts)?;
    let mut out = [0u8; 32];
    out.copy_from_slice(&mac.finalize().into_bytes());
    Some(out)
}

/// Whether `expected` is the tag over `parts`, compared in constant time.
pub fn verify(key: &[u8], parts: &[&[u8]], expected: &[u8]) -> bool {
    match keyed(key, parts) {
        Some(mac) => mac.verify_slice(expected).is_ok(),
        None => false,
    }
}

fn keyed(key: &[u8], parts: &[&[u8]]) -> Option<HmacSha256> {
    let mut mac = <HmacSha256 as Mac>::new_from_slice(key).ok()?;
    for part in parts {
        mac.update(part);
    }
    Some(mac)
}
