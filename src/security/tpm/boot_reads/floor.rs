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

//! The loader's rollback floor, read as the loader reads it: how far its
//! counter at `0x01000020` has risen above the base at `0x01000021`, the
//! counter's value when this machine's floor began. Each is a `TPM2_NV_Read`
//! the index authorizes with an empty password, eight bytes from offset 0.
//! TPM 2.0 Part 3, 31.13. A TPM starts a new counter above every counter it
//! ever held, so the counter alone is no floor: the base makes it one.

use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::TPM_ST_SESSIONS;
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::wire::{checked, frame};
use crate::security::tpm::machine_key::KeyError;

const TPM_CC_NV_READ: u32 = 0x0000_014E;
const ROLLBACK_INDEX: u32 = 0x0100_0020;
const BASE_INDEX: u32 = 0x0100_0021;
const TPM_RS_PW: u32 = 0x4000_0009;
/// An index defined and never written (a counter never incremented) reads as this.
const TPM_RC_NV_UNINITIALIZED: u32 = 0x0000_014A;

/// The index's eight bytes, `None` for one never written.
fn read(index: u32) -> Result<Option<u64>, KeyError> {
    let mut body = [0u8; 25];
    body[0..4].copy_from_slice(&index.to_be_bytes());
    body[4..8].copy_from_slice(&index.to_be_bytes());
    body[8..12].copy_from_slice(&9u32.to_be_bytes());
    body[12..16].copy_from_slice(&TPM_RS_PW.to_be_bytes());
    /* Nonce, attributes and password all empty; then size 8 at offset 0. */
    body[21..23].copy_from_slice(&8u16.to_be_bytes());
    let resp = run(&frame(TPM_ST_SESSIONS, TPM_CC_NV_READ, &body))?;
    let r = match checked(&resp) {
        Err(KeyError::Refused(TPM_RC_NV_UNINITIALIZED)) => return Ok(None),
        other => other?,
    };
    if r.get(14..16) != Some(&[0, 8][..]) {
        return Err(TpmError::InvalidResponse.into());
    }
    let v = r.get(16..24).ok_or(TpmError::InvalidResponse)?;
    Ok(Some(u64::from_be_bytes([v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]])))
}

/// The floor: 0 for a counter never incremented. A counter without its base,
/// or a base above it, is no floor this loader keeps, and reads as an error.
pub fn rollback_floor() -> Result<u64, KeyError> {
    let Some(counter) = read(ROLLBACK_INDEX)? else {
        return Ok(0);
    };
    let base = read(BASE_INDEX)?.ok_or(TpmError::InvalidResponse)?;
    Ok(counter.checked_sub(base).ok_or(TpmError::InvalidResponse)?)
}
