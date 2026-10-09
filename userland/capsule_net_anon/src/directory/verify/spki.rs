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

//! Wrapping the PKCS#1 key a document carries into a SubjectPublicKeyInfo.

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

const RSA_ALGORITHM: &[u8] =
    &[0x30, 0x0d, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01, 0x05, 0x00];

/*
 * A directory document carries a bare PKCS#1 RSAPublicKey, and the pool's
 * verifier takes a SubjectPublicKeyInfo. The fingerprint an authority is
 * trusted by is the SHA-1 of the PKCS#1 form, not this one, so both forms of
 * the same key are needed: the inner bytes to check the identity, the wrapped
 * bytes to check the signature.
 */
pub fn wrap(pkcs1: &[u8]) -> Vec<u8> {
    let mut bits = Vec::with_capacity(pkcs1.len() + 8);
    bits.push(0x03);
    bits.extend_from_slice(&length(pkcs1.len() + 1));
    bits.push(0x00);
    bits.extend_from_slice(pkcs1);

    let mut body = Vec::with_capacity(RSA_ALGORITHM.len() + bits.len());
    body.extend_from_slice(RSA_ALGORITHM);
    body.extend_from_slice(&bits);

    let mut out = Vec::with_capacity(body.len() + 8);
    out.push(0x30);
    out.extend_from_slice(&length(body.len()));
    out.extend_from_slice(&body);
    out
}

/// A DER length, short form under 128 and long form above.
fn length(value: usize) -> Vec<u8> {
    if value < 0x80 {
        return vec![value as u8];
    }
    let bytes: Vec<u8> = value.to_be_bytes().into_iter().skip_while(|b| *b == 0).collect();
    let mut out = Vec::with_capacity(bytes.len() + 1);
    out.push(0x80 | bytes.len() as u8);
    out.extend_from_slice(&bytes);
    out
}
