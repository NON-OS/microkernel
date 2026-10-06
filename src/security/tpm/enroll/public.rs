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

//! A public area the way the registrar takes it, and the check every one the
//! TPM returns passes before it is handed on.

use alloc::vec::Vec;

use super::consts::NAME_LEN;
use super::error::EnrollError;
use crate::crypto::hash::sha256;
use crate::security::tpm::ak::cursor::Cursor;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::TPM_ALG_SHA256;

/// `area` is the TPM2B_PUBLIC, size field first, byte for byte as the TPM
/// marshals it and as `tpm2_makecredential` reads it. `name` is
/// `TPM_ALG_SHA256 || SHA-256(TPMT_PUBLIC)`, what MakeCredential binds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Public {
    pub area: Vec<u8>,
    pub name: [u8; NAME_LEN],
}

/// `tpmt` must be `shape` and then `unique` fields of exactly `sizes`, and
/// `name`, the TPM's own, must be the hash of those bytes. Anything else is a
/// parse at the wrong offset or an object that is not the one asked for.
pub(super) fn checked_public(
    tpmt: &[u8],
    shape: &[u8],
    sizes: &[usize],
    name: &[u8],
) -> Result<Public, EnrollError> {
    if shape.is_empty() || !tpmt.starts_with(shape) {
        return Err(TpmError::InvalidResponse.into());
    }
    let mut c = Cursor::at(tpmt, shape.len());
    for &n in sizes {
        if c.u16()? as usize != n {
            return Err(TpmError::InvalidResponse.into());
        }
        c.skip(n)?;
    }
    let mut own = [0u8; NAME_LEN];
    own[..2].copy_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    own[2..].copy_from_slice(&sha256(tpmt));
    if c.position() != tpmt.len() || name != &own[..] {
        return Err(TpmError::InvalidResponse.into());
    }
    let mut area = Vec::with_capacity(2 + tpmt.len());
    area.extend_from_slice(&(tpmt.len() as u16).to_be_bytes());
    area.extend_from_slice(tpmt);
    Ok(Public { area, name: own })
}
