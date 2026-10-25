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

//! The TPM banks an event can carry, and the body's digest in each.

use alloc::vec::Vec;

use crate::crypto::hash::sha384::sha384;
use crate::crypto::hash::{sha256, sha512};

pub const ALG_SHA1: u16 = 0x0004;
pub const ALG_SHA256: u16 = 0x000B;
pub const ALG_SHA384: u16 = 0x000C;
pub(crate) const ALG_SHA512: u16 = 0x000D;

/// The body's digest under `alg`, or `None` for a bank this kernel cannot
/// hash: then it cannot extend that bank, and does not extend at all.
pub fn digest(alg: u16, body: &[u8]) -> Option<Vec<u8>> {
    match alg {
        ALG_SHA1 => Some(sha1(body).to_vec()),
        ALG_SHA256 => Some(sha256(body).to_vec()),
        ALG_SHA384 => Some(sha384(body).to_vec()),
        ALG_SHA512 => Some(sha512(body).to_vec()),
        _ => None,
    }
}

/// A TPM's SHA-1 bank takes a SHA-1 digest. Nothing here rests on SHA-1's
/// strength: a verifier replays and trusts the SHA-256 bank alone.
#[allow(deprecated)]
fn sha1(body: &[u8]) -> [u8; 20] {
    crate::crypto::hash::sha1(body)
}
