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

//! This machine's own build root, enrolled by a person in first-boot setup.
//!
//! Main's route asked a capsule to request a root and a person to type back a
//! code printed on the serial console, which no desktop user ever sees. Setup
//! is the trusted path instead: it alone holds `EnrolDevRoot`, it grants the
//! local root as a named step, and the token it keeps restores that consent on
//! later boots without asking again.

use super::authority::Authority;
use super::error::EnrolError;
use super::table::TABLE;
use crate::capabilities::Capability;
use crate::security::attest_registry::registry_complete;
use crate::security::local_build::{consent_token, root};

/// Enrol the local root and return the token that restores it, if this
/// machine can keep one.
pub fn grant_local_root(caller_caps: u64) -> Result<(Authority, Option<[u8; 32]>), EnrolError> {
    if caller_caps & Capability::EnrolDevRoot.bit() == 0 {
        return Err(EnrolError::Denied);
    }
    let authority = enrol()?;
    Ok((authority, root().and_then(|r| consent_token(&r))))
}

/// Enrol the local root again from a token a grant returned. A token that is
/// not this machine's for this root is refused.
pub fn restore_local_root(token: &[u8; 32]) -> Result<Authority, EnrolError> {
    let want = root().and_then(|r| consent_token(&r)).ok_or(EnrolError::NotConfirmed)?;
    let differs = want.iter().zip(token.iter()).fold(0u8, |acc, (a, b)| acc | (a ^ b));
    if differs != 0 {
        return Err(EnrolError::NotConfirmed);
    }
    enrol()
}

/// Stop running what this machine installs. Narrowing, so it asks only for
/// the same right a grant does, not for a second person.
pub fn revoke_local_root(caller_caps: u64) -> Result<(), EnrolError> {
    if caller_caps & Capability::EnrolDevRoot.bit() == 0 {
        return Err(EnrolError::Denied);
    }
    let local = root().ok_or(EnrolError::EmptyRoot)?;
    TABLE.lock().remove(&local);
    crate::sys::serial::println(b"[DEV-ROOT] local root withdrawn");
    Ok(())
}

fn enrol() -> Result<Authority, EnrolError> {
    if !registry_complete() {
        return Err(EnrolError::RegistryIncomplete);
    }
    let local = root().ok_or(EnrolError::EmptyRoot)?;
    let slot = TABLE.lock().insert(local).ok_or(EnrolError::NoSlots)?;
    crate::sys::serial::println(b"[DEV-ROOT] local root enrolled; installed software may run");
    Ok(Authority::Developer(slot))
}
