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

//! Calling the crypto pool for an RSA signature over a bare digest.

extern crate alloc;

use alloc::vec::Vec;

use super::frame::{call, port};

const SCHEME_UNPREFIXED: u8 = 2;

/// Digest length selectors the pool accepts under that scheme.
pub const HASH_SHA256: u8 = 0;
pub const HASH_SHA1: u8 = 3;

/// Verify `signature` over `digest` under `spki`.
///
pub fn rsa_verify(hashid: u8, spki: &[u8], signature: &[u8], digest: &[u8]) -> bool {
    let Some(port) = port() else { return false };
    let mut body = Vec::with_capacity(6 + spki.len() + signature.len() + digest.len());
    body.push(SCHEME_UNPREFIXED);
    body.push(hashid);
    body.extend_from_slice(&(spki.len() as u16).to_le_bytes());
    body.extend_from_slice(spki);
    body.extend_from_slice(&(signature.len() as u16).to_le_bytes());
    body.extend_from_slice(signature);
    body.extend_from_slice(digest);
    call(port, &body)
}
