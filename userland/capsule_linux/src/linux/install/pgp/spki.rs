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

//! An RSA public key as a SubjectPublicKeyInfo, the form the crypto service
//! takes: SEQUENCE { SEQUENCE { rsaEncryption, NULL }, BIT STRING {
//! SEQUENCE { INTEGER n, INTEGER e } } }.

use alloc::vec::Vec;

const RSA_ALGORITHM: [u8; 15] =
    [0x30, 0x0D, 0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01, 0x05, 0x00];

fn tlv(tag: u8, body: &[u8], out: &mut Vec<u8>) {
    out.push(tag);
    let len = body.len();
    match len {
        0..=0x7F => out.push(len as u8),
        0x80..=0xFF => out.extend([0x81, len as u8]),
        _ => out.extend([0x82, (len >> 8) as u8, len as u8]),
    }
    out.extend_from_slice(body);
}

/// A positive INTEGER: leading zeros dropped, one put back if the top bit is set.
fn integer(v: &[u8], out: &mut Vec<u8>) {
    let v = &v[v.iter().position(|&b| b != 0).unwrap_or(v.len())..];
    let mut body = Vec::with_capacity(v.len() + 1);
    if v.first().is_none_or(|&b| b & 0x80 != 0) {
        body.push(0);
    }
    body.extend_from_slice(v);
    tlv(0x02, &body, out);
}

/// None for a key too large for two length bytes; no real one is.
pub fn rsa_spki(n: &[u8], e: &[u8]) -> Option<Vec<u8>> {
    if n.len() > 0x2000 || e.len() > 0x100 {
        return None;
    }
    let mut ints = Vec::new();
    integer(n, &mut ints);
    integer(e, &mut ints);
    let mut key = Vec::new();
    tlv(0x30, &ints, &mut key);
    let mut bits = alloc::vec![0u8];
    bits.extend(key);
    let mut body = RSA_ALGORITHM.to_vec();
    tlv(0x03, &bits, &mut body);
    let mut out = Vec::new();
    tlv(0x30, &body, &mut out);
    Some(out)
}
