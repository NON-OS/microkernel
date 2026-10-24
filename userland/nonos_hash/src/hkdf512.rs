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

//! HKDF over HMAC-SHA-512 (RFC 5869). `hkdf512_extract` turns a salt and
//! input keying material into a 64-byte pseudorandom key; `hkdf512_expand`
//! stretches that key, under an info string, into at most 255 blocks of
//! output. An empty salt means 64 zero bytes, which is what HMAC does with
//! an empty key, so no special case is needed.

use crate::hmac512::{hmac_sha512, HmacSha512};
use crate::wipe::wipe;

/// The most output one key can give: 255 blocks of 64 bytes.
pub const HKDF512_MAX: usize = 255 * 64;

/// More output was asked for than `HKDF512_MAX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HkdfTooLong;

pub fn hkdf512_extract(salt: &[u8], ikm: &[u8]) -> [u8; 64] {
    hmac_sha512(salt, ikm)
}

/// Fill `out` with output keying material. Nothing is written when `out`
/// is longer than `HKDF512_MAX`.
pub fn hkdf512_expand(prk: &[u8; 64], info: &[u8], out: &mut [u8]) -> Result<(), HkdfTooLong> {
    if out.len() > HKDF512_MAX {
        return Err(HkdfTooLong);
    }
    let mut block = [0u8; 64];
    for (i, chunk) in out.chunks_mut(64).enumerate() {
        let mut mac = HmacSha512::new(prk);
        if i > 0 {
            mac.update(&block);
        }
        mac.update(info);
        mac.update(&[i as u8 + 1]);
        block = mac.finalize();
        chunk.copy_from_slice(&block[..chunk.len()]);
    }
    wipe(&mut block);
    Ok(())
}
