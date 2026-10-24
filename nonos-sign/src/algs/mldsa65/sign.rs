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
use super::{ffi, guard, keygen, verify};
use crate::algs::traits::{KeyPair, Signer, Verifier};
use crate::error::SignError;
pub struct MlDsa65;
impl MlDsa65 {
    /// The deterministic signature of FIPS 204: no randomness is drawn, so
    /// the same key and message give the same bytes. The zero mode it uses
    /// is on only inside the lock, and never while a key is made.
    pub fn sign_reproducible(seed: &[u8], msg: &[u8]) -> Result<Vec<u8>, SignError> {
        sign_with(true, seed, msg)
    }
}
impl Signer for MlDsa65 {
    fn keygen_from_seed(seed: &[u8]) -> Result<KeyPair, SignError> {
        keygen::from_seed(seed)
    }

    fn keygen_random() -> Result<KeyPair, SignError> {
        keygen::random()
    }

    fn sign(seed: &[u8], msg: &[u8]) -> Result<Vec<u8>, SignError> {
        sign_with(false, seed, msg)
    }
}
impl Verifier for MlDsa65 {
    fn verify(pubkey: &[u8], msg: &[u8], sig: &[u8]) -> Result<bool, SignError> {
        verify::verify_raw(pubkey, msg, sig)
    }
}

fn sign_with(reproducible: bool, seed: &[u8], msg: &[u8]) -> Result<Vec<u8>, SignError> {
    if seed.len() != ffi::SECRET_KEY_BYTES {
        return Err(SignError::InvalidSeedLength {
            alg: "mldsa65",
            expected: ffi::SECRET_KEY_BYTES,
            actual: seed.len(),
        });
    }
    let mut sig = vec![0u8; ffi::SIGNATURE_BYTES];
    let mut siglen: usize = 0;
    let rc = guard::signature(reproducible, &mut sig, &mut siglen, msg, seed)?;
    if rc != 0 {
        return Err(SignError::PqcleanFailed("mldsa65 sign"));
    }
    if siglen != ffi::SIGNATURE_BYTES {
        return Err(SignError::InvalidSignatureLength {
            alg: "mldsa65",
            expected: ffi::SIGNATURE_BYTES,
            actual: siglen,
        });
    }
    Ok(sig)
}
