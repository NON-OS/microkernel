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


//! SHA-384 and HMAC-SHA384 in the capsule, for one TLS 1.2 suite.
//!
//! A relay that speaks only TLS 1.2 is an old OpenSSL, and the one AEAD it
//! shares with the kernel's AEAD call is AES-256-GCM, whose TLS 1.2 suite
//! (ECDHE-RSA-AES256-GCM-SHA384) runs its PRF and Finished over SHA-384.
//! The hash syscall offers SHA-256 and SHA-512 but not SHA-384, which is
//! SHA-512 with other starting words and a shorter output, so it is here.

mod block;

pub use block::Sha384;

/// SHA-384 of `parts`, concatenated.
pub fn sha384(parts: &[&[u8]]) -> [u8; 48] {
    let mut h = Sha384::new();
    for part in parts {
        h.update(part);
    }
    h.finish()
}

/// HMAC-SHA384 (RFC 2104 over a 128-byte block) of `parts`, concatenated.
pub fn hmac_sha384(key: &[u8], parts: &[&[u8]]) -> [u8; 48] {
    let mut block = [0u8; 128];
    if key.len() > 128 {
        block[..48].copy_from_slice(&sha384(&[key]));
    } else {
        block[..key.len()].copy_from_slice(key);
    }
    let mut ipad = [0x36u8; 128];
    let mut opad = [0x5cu8; 128];
    for i in 0..128 {
        ipad[i] ^= block[i];
        opad[i] ^= block[i];
    }
    let mut inner = Sha384::new();
    inner.update(&ipad);
    for part in parts {
        inner.update(part);
    }
    let inner = inner.finish();
    let mut outer = Sha384::new();
    outer.update(&opad);
    outer.update(&inner);
    let out = outer.finish();
    // The padded key blocks hold the key; they go before returning.
    super::wipe::wipe(&mut block);
    super::wipe::wipe(&mut ipad);
    super::wipe::wipe(&mut opad);
    out
}
