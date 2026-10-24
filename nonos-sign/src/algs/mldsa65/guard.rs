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

//! Every call into the C, one at a time, refused if any draw it made failed
//! (the C ignores that). One lock covers them all, so the zero mode of a
//! reproducible signature is never on while another thread makes a key.

use super::ffi;
use crate::error::SignError;
use std::sync::Mutex;

pub(super) static PQ: Mutex<()> = Mutex::new(());

/// Run `call` with the zero mode set as `reproducible` asks, then refuse
/// its result if any draw failed. The mode is always off afterwards.
fn guarded(reproducible: bool, call: impl FnOnce() -> i32) -> Result<i32, SignError> {
    let _turn = PQ.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    // SAFETY: eK@nonos.systems - the switches and the note are plain ints
    // in the C, touched only while PQ is held.
    unsafe {
        ffi::NONOS_mldsa65_take_random_failure();
        ffi::NONOS_mldsa65_deterministic_random(i32::from(reproducible));
    }
    let rc = call();
    // SAFETY: eK@nonos.systems - as above.
    let failed = unsafe {
        ffi::NONOS_mldsa65_deterministic_random(0);
        ffi::NONOS_mldsa65_take_random_failure()
    };
    match failed {
        0 => Ok(rc),
        _ => Err(SignError::PqcleanFailed("mldsa65: the system randomness failed")),
    }
}

/// A key pair, never in the zero mode.
pub(super) fn keypair(pk: &mut [u8], sk: &mut [u8]) -> Result<i32, SignError> {
    // SAFETY: eK@nonos.systems - the callers size both buffers to the C's.
    guarded(false, || unsafe {
        ffi::PQCLEAN_MLDSA65_CLEAN_crypto_sign_keypair(pk.as_mut_ptr(), sk.as_mut_ptr())
    })
}

pub(super) fn signature(
    reproducible: bool,
    sig: &mut [u8],
    siglen: &mut usize,
    msg: &[u8],
    sk: &[u8],
) -> Result<i32, SignError> {
    // SAFETY: eK@nonos.systems - `sig` is SIGNATURE_BYTES, `sk` checked.
    guarded(reproducible, || unsafe {
        let (m, n) = (msg.as_ptr(), msg.len());
        ffi::PQCLEAN_MLDSA65_CLEAN_crypto_sign_signature(
            sig.as_mut_ptr(),
            siglen,
            m,
            n,
            sk.as_ptr(),
        )
    })
}
