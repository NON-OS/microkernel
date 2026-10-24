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

//! The two calls that draw randomness: a key pair, and a hedged signature.
//! Each refuses its result when any draw it made failed, since the C never
//! checks. A refused key pair's secret key is wiped as it drops; a refused
//! signature is wiped before the error is returned.

use super::constants::{PUBLICKEY_BYTES, SECRETKEY_BYTES, SIGNATURE_BYTES};
use super::ffi;
use super::types::{
    MlDsa65Error, MlDsa65KeyPair, MlDsa65PublicKey, MlDsa65SecretKey, MlDsa65Signature,
};
use crate::crypto::pqclean_support::drew_cleanly;

pub fn ml_dsa_65_keypair() -> Result<MlDsa65KeyPair, MlDsa65Error> {
    let mut pk = MlDsa65PublicKey { bytes: [0u8; PUBLICKEY_BYTES] };
    let mut sk = MlDsa65SecretKey { bytes: [0u8; SECRETKEY_BYTES] };
    // SAFETY: eK@nonos.systems - both buffers are the sizes the C writes.
    let (rc, clean) =
        drew_cleanly(|| unsafe { ffi::keypair(pk.bytes.as_mut_ptr(), sk.bytes.as_mut_ptr()) });
    match (clean, rc) {
        (false, _) => Err(MlDsa65Error::RandomnessFailed),
        (true, 0) => Ok(MlDsa65KeyPair { public_key: pk, secret_key: sk }),
        (true, _) => Err(MlDsa65Error::FfiError),
    }
}

pub fn ml_dsa_65_sign(sk: &MlDsa65SecretKey, msg: &[u8]) -> Result<MlDsa65Signature, MlDsa65Error> {
    let mut sig = MlDsa65Signature { bytes: [0u8; SIGNATURE_BYTES] };
    let mut siglen: usize = 0;
    // SAFETY: eK@nonos.systems - `sig` holds SIGNATURE_BYTES, the most the
    // C writes; `sk` is a whole secret key.
    let (rc, clean) = drew_cleanly(|| unsafe {
        let (m, n) = (msg.as_ptr(), msg.len());
        ffi::sign(sig.bytes.as_mut_ptr(), &mut siglen, m, n, sk.bytes.as_ptr())
    });
    let err = match (clean, rc, siglen) {
        (true, 0, SIGNATURE_BYTES) => return Ok(sig),
        (false, _, _) => MlDsa65Error::RandomnessFailed,
        _ => MlDsa65Error::FfiError,
    };
    for b in &mut sig.bytes {
        // SAFETY: eK@nonos.systems - a volatile store to a byte we own, so
        // the wipe is not dropped as dead.
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    Err(err)
}
