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

//! Hashing through the crypto pool, and signature checking in this capsule.

use nonos_ed25519::{verify, Signature};
use nonos_libc::crypto_hash;

use super::CryptoError;

/// The pool's algorithm selector for SHA-256.
const ALGO_SHA256: u64 = 1;

pub fn sha256(data: &[u8]) -> Result<[u8; 32], CryptoError> {
    let mut out = [0u8; 32];
    let n = crypto_hash(ALGO_SHA256, data.as_ptr(), data.len(), out.as_mut_ptr(), out.len());
    if n == out.len() as i64 {
        Ok(out)
    } else {
        Err(CryptoError::Digest)
    }
}

/// Verify an Ed25519 signature. Used on the relay's link certificates and on
/// the identity certificate that binds them to the relay's long term key.
/*
 * A library call, as the market's is: the kernel keeps ed25519 for the boot
 * chain only and offers no syscall for it.
 */
pub fn ed25519_verify(public: &[u8; 32], message: &[u8], signature: &[u8; 64]) -> bool {
    verify(public, message, &Signature::from_bytes(signature))
}
