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

/*
 * CertificateVerify (RFC 8446 section 4.4.3): the server signs the
 * transcript up to its Certificate with the key in that certificate. It
 * is the only thing that ties the chain the server sent to the server on
 * this connection. Without it anyone can replay a real chain, finish the
 * handshake on keys of their own, and pass every other check.
 */

use alloc::vec::Vec;

const ECDSA_P256_SHA256: u16 = 0x0403;
const ECDSA_P384_SHA384: u16 = 0x0503;
const CONTEXT: &[u8] = b"TLS 1.3, server CertificateVerify";

/// Whether `body`, a CertificateVerify, is `leaf`'s signature over the
/// transcript `before` it (every handshake message up to Certificate).
pub fn check(leaf: &[u8], before: &[u8], body: &[u8]) -> bool {
    if body.len() < 4 {
        return false;
    }
    let scheme = u16::from_be_bytes([body[0], body[1]]);
    let len = u16::from_be_bytes([body[2], body[3]]) as usize;
    if 4 + len != body.len() {
        return false;
    }
    let Some(hash) = super::hash_sha256::hash_sha256(before) else { return false };
    let mut content = Vec::with_capacity(64 + CONTEXT.len() + 1 + hash.len());
    content.resize(64, 0x20);
    content.extend_from_slice(CONTEXT);
    content.push(0);
    content.extend_from_slice(&hash);
    let Some(spki) = super::cert_spki::cert_spki(leaf) else { return false };
    let Some(point) = super::spki_point::spki_point(spki) else { return false };
    let sig_der = &body[4..];
    match scheme {
        ECDSA_P256_SHA256 => super::cert_verify_ecdsa::p256(point, sig_der, &content),
        ECDSA_P384_SHA384 => super::cert_verify_ecdsa::p384(point, sig_der, &content),
        _ => false,
    }
}
