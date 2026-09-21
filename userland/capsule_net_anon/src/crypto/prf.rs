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

//! HMAC and HKDF, framed the way the crypto pool takes them.

extern crate alloc;

use alloc::vec::Vec;
use nonos_libc::{crypto_hkdf_sha256, crypto_hmac_sha256};

use super::CryptoError;

/// HMAC-SHA256 with the pool doing the work. ntor calls this three times per
/// handshake under three different keys, which is why it takes the key first.
pub fn hmac_sha256(key: &[u8], data: &[u8], out: &mut [u8; 32]) -> Result<(), CryptoError> {
    let n =
        crypto_hmac_sha256(key.as_ptr(), key.len(), data.as_ptr(), data.len(), out.as_mut_ptr());
    if n == out.len() as i64 {
        Ok(())
    } else {
        Err(CryptoError::Mac)
    }
}

/// HKDF-SHA256, extract then expand, which is what the network calls
/// KDF-RFC5869 and what the relay on the far side of an ntor handshake runs.
pub fn hkdf_sha256(
    salt: &[u8],
    ikm: &[u8],
    info: &[u8],
    out: &mut [u8],
) -> Result<(), CryptoError> {
    let frame = frame(out.len(), salt, ikm, info)?;
    let n = crypto_hkdf_sha256(frame.as_ptr(), frame.len(), out.as_mut_ptr(), out.len());
    if n == out.len() as i64 {
        Ok(())
    } else {
        Err(CryptoError::Kdf)
    }
}

fn frame(out_len: usize, salt: &[u8], ikm: &[u8], info: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let widths = [out_len, salt.len(), ikm.len(), info.len()];
    if widths.iter().any(|w| *w > u16::MAX as usize) {
        return Err(CryptoError::Kdf);
    }
    let mut out = Vec::with_capacity(8 + salt.len() + ikm.len() + info.len());
    for width in widths {
        out.extend_from_slice(&(width as u16).to_le_bytes());
    }
    out.extend_from_slice(salt);
    out.extend_from_slice(ikm);
    out.extend_from_slice(info);
    Ok(out)
}
