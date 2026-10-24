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

//! The two ML-KEM calls that draw randomness: a key pair and an
//! encapsulation. Each refuses its result when any draw it made failed,
//! since the C never checks. A refused key pair's secret key is wiped as it
//! drops; a refused encapsulation's shared secret is wiped before the error
//! is returned.

use super::SHAREDSECRET_BYTES;
use super::{ffi, ok, KyberCiphertext, KyberError, KyberKeyPair, KyberPublicKey};
use super::{KyberSecretKey, CIPHERTEXT_BYTES, PUBLICKEY_BYTES, SECRETKEY_BYTES};
use crate::crypto::pqclean_support::drew_cleanly;

pub fn kyber_keygen() -> Result<KyberKeyPair, KyberError> {
    let mut pk = KyberPublicKey { bytes: [0u8; PUBLICKEY_BYTES] };
    let mut sk = KyberSecretKey { bytes: [0u8; SECRETKEY_BYTES] };
    // SAFETY: eK@nonos.systems - both buffers are the sizes the C writes.
    let (rc, clean) =
        drew_cleanly(|| unsafe { ffi::keypair(pk.bytes.as_mut_ptr(), sk.bytes.as_mut_ptr()) });
    if !clean {
        return Err(KyberError::RandomnessFailed);
    }
    ok(rc)?;
    Ok(KyberKeyPair { public_key: pk, secret_key: sk })
}

pub fn kyber_encaps(
    pk: &KyberPublicKey,
) -> Result<(KyberCiphertext, [u8; SHAREDSECRET_BYTES]), KyberError> {
    let mut ct = KyberCiphertext { bytes: [0u8; CIPHERTEXT_BYTES] };
    let mut ss = [0u8; SHAREDSECRET_BYTES];
    // SAFETY: eK@nonos.systems - `ct` and `ss` are the sizes the C writes.
    let (rc, clean) = drew_cleanly(|| unsafe {
        ffi::encaps(ct.bytes.as_mut_ptr(), ss.as_mut_ptr(), pk.bytes.as_ptr())
    });
    if clean && rc == 0 {
        return Ok((ct, ss));
    }
    for b in &mut ss {
        // SAFETY: eK@nonos.systems - a volatile store to a byte we own, so
        // the wipe is not dropped as dead.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    Err(if clean { KyberError::FfiError } else { KyberError::RandomnessFailed })
}
