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

//! The key schedule the way it was computed before: every HMAC and hash a
//! syscall answered by the crypto pool's own handlers.

use alloc::vec::Vec;

pub fn hash(data: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    assert_eq!(nonos_libc::crypto_hash(1, data.as_ptr(), data.len(), out.as_mut_ptr(), 32), 32);
    out
}

pub fn hmac(key: &[u8], data: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    let (k, d) = (key.as_ptr(), data.as_ptr());
    assert_eq!(nonos_libc::crypto_hmac_sha256(k, key.len(), d, data.len(), out.as_mut_ptr()), 32);
    out
}

/// HKDF-Expand-Label, RFC 8446 section 7.1, one HMAC syscall per block.
pub fn label(secret: &[u8; 32], name: &[u8], context: &[u8], len: usize) -> Vec<u8> {
    let mut info = (len as u16).to_be_bytes().to_vec();
    info.push(6 + name.len() as u8);
    info.extend_from_slice(b"tls13 ");
    info.extend_from_slice(name);
    info.push(context.len() as u8);
    info.extend_from_slice(context);
    let (mut out, mut prev, mut counter) = (Vec::new(), Vec::new(), 1u8);
    while out.len() < len {
        let block = hmac(secret, &[&prev[..], &info, &[counter]].concat());
        out.extend_from_slice(&block);
        prev = block.to_vec();
        counter += 1;
    }
    out.truncate(len);
    out
}

/// The next stage's secret: Derive-Secret(base, "derived", "") then Extract.
pub fn next_stage(base: &[u8; 32], ikm: &[u8; 32]) -> [u8; 32] {
    let derived = label(base, b"derived", &hash(&[]), 32);
    hmac(&derived, ikm)
}
