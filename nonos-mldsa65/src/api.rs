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

//! Key pairs and secret keys. The `SecretKey` a caller holds is wiped when
//! it is dropped. That is all that is wiped: the C leaves the key's seed and
//! working values on its own stack, and a value that moves may leave a copy
//! behind. A caller that needs more keeps the key in one place for its whole
//! life and signs there, as nsh.keys does.

use crate::ffi::*;
use crate::rng::with_source;
pub use crate::secret::SecretKey;

pub const PUBLIC_LEN: usize = 1952;
pub const SECRET_LEN: usize = 4032;
pub const SIG_LEN: usize = 3309;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The randomness could not be drawn.
    Randomness,
    /// The C reported a failure.
    Failed,
}

pub(crate) type Source<'a> = &'a mut dyn FnMut(&mut [u8]) -> bool;

pub fn keypair(rand: Source<'_>) -> Result<([u8; PUBLIC_LEN], SecretKey), Error> {
    let mut pk = [0u8; PUBLIC_LEN];
    let mut sk = SecretKey([0u8; SECRET_LEN]);
    // SAFETY: eK@nonos.systems - both buffers are the sizes the C writes.
    let rc = with_source(rand, || unsafe {
        PQCLEAN_MLDSA65_CLEAN_crypto_sign_keypair(pk.as_mut_ptr(), sk.0.as_mut_ptr())
    });
    match rc {
        Some(0) => Ok((pk, sk)),
        Some(_) => Err(Error::Failed),
        None => Err(Error::Randomness),
    }
}
