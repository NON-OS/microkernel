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

//! Signatures and verification. A signature the C failed to finish is
//! wiped before the error is returned.

use crate::api::{SecretKey, Source, PUBLIC_LEN, SIG_LEN};
use crate::ffi::*;
use crate::rng::with_source;
use crate::secret::wipe;
use crate::Error;

/// Sign `msg` with an empty context, hedged with randomness from `rand`.
pub fn sign(sk: &SecretKey, msg: &[u8], rand: Source<'_>) -> Result<[u8; SIG_LEN], Error> {
    let mut sig = [0u8; SIG_LEN];
    let mut len = 0usize;
    // SAFETY: eK@nonos.systems - `sig` holds SIG_LEN, the most the C writes.
    let rc = with_source(rand, || unsafe {
        let (m, n) = (msg.as_ptr(), msg.len());
        PQCLEAN_MLDSA65_CLEAN_crypto_sign_signature(
            sig.as_mut_ptr(),
            &mut len,
            m,
            n,
            sk.as_bytes().as_ptr(),
        )
    });
    match rc {
        Some(0) if len == SIG_LEN => Ok(sig),
        other => {
            wipe(&mut sig);
            Err(if other.is_none() { Error::Randomness } else { Error::Failed })
        }
    }
}

/// Verification draws no randomness. It still takes its turn with the
/// other calls, so only one call is ever inside the C and its memory.
pub fn verify(pk: &[u8; PUBLIC_LEN], msg: &[u8], sig: &[u8; SIG_LEN]) -> bool {
    let mut none = |_: &mut [u8]| false;
    // SAFETY: eK@nonos.systems - the lengths are the fixed sizes the C reads.
    let rc = with_source(&mut none, || unsafe {
        let (m, n) = (msg.as_ptr(), msg.len());
        PQCLEAN_MLDSA65_CLEAN_crypto_sign_verify(sig.as_ptr(), SIG_LEN, m, n, pk.as_ptr())
    });
    rc == Some(0)
}
