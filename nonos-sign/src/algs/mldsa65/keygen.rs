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

use crate::algs::traits::KeyPair;
use crate::error::SignError;

use super::{ffi, guard};

pub fn from_seed(_seed: &[u8]) -> Result<KeyPair, SignError> {
    Err(SignError::SeedKeygenUnsupported("mldsa65"))
}

pub fn random() -> Result<KeyPair, SignError> {
    let mut pk = vec![0u8; ffi::PUBLIC_KEY_BYTES];
    let mut sk = vec![0u8; ffi::SECRET_KEY_BYTES];
    let rc = guard::keypair(&mut pk, &mut sk).inspect_err(|_| sk.fill(0))?;
    if rc != 0 {
        return Err(SignError::PqcleanFailed("mldsa65 keypair"));
    }
    Ok(KeyPair { pubkey: pk, seed: sk })
}
