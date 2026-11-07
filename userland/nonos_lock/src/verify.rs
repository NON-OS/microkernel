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

//! Making a verifier from a passphrase and checking a passphrase against
//! one. The tag is compared in constant time and every derived copy is
//! wiped before it is let go.

use crate::argon2::{argon2id, Argon2Error, Params};
use crate::record::{Record, PARAMS, SALT_LEN, TAG_LEN};
use crate::wipe::{ct_eq, secure_zero};

/// Argon2id of `passphrase` under `salt`, 32 bytes. `between` runs after
/// each segment, so a long derivation can yield.
pub fn derive(
    passphrase: &[u8],
    salt: &[u8; SALT_LEN],
    params: Params,
    between: &mut dyn FnMut(),
) -> Result<[u8; TAG_LEN], Argon2Error> {
    let mut tag = [0u8; TAG_LEN];
    match argon2id(passphrase, salt, params, &mut tag, between) {
        Ok(()) => Ok(tag),
        Err(e) => {
            secure_zero(&mut tag);
            Err(e)
        }
    }
}

/// A new record for `passphrase` under the fresh `salt`, at `PARAMS`, with
/// no wrong entries.
pub fn make(
    passphrase: &[u8],
    salt: &[u8; SALT_LEN],
    between: &mut dyn FnMut(),
) -> Result<Record, Argon2Error> {
    let tag = derive(passphrase, salt, PARAMS, between)?;
    Ok(Record { params: PARAMS, salt: *salt, tag, failures: 0 })
}

/// Whether `passphrase` is the one `record` was made from.
pub fn matches(
    record: &Record,
    passphrase: &[u8],
    between: &mut dyn FnMut(),
) -> Result<bool, Argon2Error> {
    let mut tag = derive(passphrase, &record.salt, record.params, between)?;
    let same = ct_eq(&tag, &record.tag);
    secure_zero(&mut tag);
    Ok(same)
}
