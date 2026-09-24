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

//! Wrapping a PKCS#1 RSAPublicKey into the SubjectPublicKeyInfo the pool takes.

extern crate alloc;

use alloc::vec::Vec;

const RSA_ALGORITHM: &[u8] =
    &[0x30, 0x0d, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01, 0x05, 0x00];

/*
 * A directory document carries a bare PKCS#1 RSAPublicKey, and the pool's
 * verifier takes a SubjectPublicKeyInfo. The two are not interchangeable, and
 * the fingerprint a certificate is trusted by is taken over the PKCS#1 form,
 * not this one, so both forms are needed on the same key: the inner bytes to
 * check the identity, the wrapped bytes to check the signature.
 */
/// Wrap `pkcs1` as a SubjectPublicKeyInfo.
pub fn wrap_pkcs1(pkcs1: &[u8]) -> Vec<u8> {
    let mut bit_string = Vec::with_capacity(pkcs1.len() + 8);
    bit_string.push(0x03);
    bit_string.extend_from_slice(&length(pkcs1.len() + 1));
    // The unused-bits count, always zero for a whole number of bytes.
    bit_string.push(0x00);
    bit_string.extend_from_slice(pkcs1);

    let mut body = Vec::with_capacity(RSA_ALGORITHM.len() + bit_string.len());
    body.extend_from_slice(RSA_ALGORITHM);
    body.extend_from_slice(&bit_string);

    let mut out = Vec::with_capacity(body.len() + 8);
    out.push(0x30);
    out.extend_from_slice(&length(body.len()));
    out.extend_from_slice(&body);
    out
}

/// A DER length, short form under 128 and long form above it.
fn length(value: usize) -> Vec<u8> {
    if value < 0x80 {
        return alloc::vec![value as u8];
    }
    let bytes: Vec<u8> = value.to_be_bytes().into_iter().skip_while(|b| *b == 0).collect();
    let mut out = Vec::with_capacity(bytes.len() + 1);
    out.push(0x80 | bytes.len() as u8);
    out.extend_from_slice(&bytes);
    out
}
