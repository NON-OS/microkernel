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

//! `TPM2_NV_Read`, one chunk of the certificate index. TPM 2.0 Part 3, 31.13.

use alloc::vec::Vec;

use super::consts::TPM_CC_NV_READ;
use super::error::EnrollError;
use super::marshal::put_password;
use crate::security::tpm::ak::cursor::Cursor;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::{HEADER_LEN, TPM_ST_SESSIONS};
use crate::security::tpm::machine_key::wire::{checked, frame};

/// `auth` is the index itself or the owner, as `parse_nv_read_public` chose,
/// with the empty password; then `len` bytes from `offset`.
pub(in crate::security::tpm) fn build_nv_read(
    auth: u32,
    index: u32,
    len: u16,
    offset: u16,
) -> Vec<u8> {
    let mut body = Vec::with_capacity(25);
    body.extend_from_slice(&auth.to_be_bytes());
    body.extend_from_slice(&index.to_be_bytes());
    put_password(&mut body);
    body.extend_from_slice(&len.to_be_bytes());
    body.extend_from_slice(&offset.to_be_bytes());
    frame(TPM_ST_SESSIONS, TPM_CC_NV_READ, &body)
}

/// Exactly `len` bytes, after the parameter size. A shorter answer is refused
/// rather than taken as the end of the certificate.
pub(in crate::security::tpm) fn parse_nv_read(
    resp: &[u8],
    len: usize,
) -> Result<&[u8], EnrollError> {
    let r = checked(resp)?;
    let mut c = Cursor::at(r, HEADER_LEN + 4);
    if c.u16()? as usize != len {
        return Err(TpmError::InvalidResponse.into());
    }
    Ok(c.take(len)?)
}
