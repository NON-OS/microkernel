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

//! `TPM2_Hash` of the message under the endorsement hierarchy. A restricted
//! key signs only against the ticket this returns, which says the TPM hashed
//! the digest itself and the message did not begin with `TPM_GENERATED_VALUE`.
//! TPM 2.0 Part 3, 15.4.

use alloc::vec::Vec;

use super::consts::{AK_SIGN_LABEL, DIGEST_MAX, TPM_CC_HASH, TPM_RH_ENDORSEMENT, TPM_ST_HASHCHECK};
use super::error::EnrollError;
use super::marshal::put_tpm2b;
use crate::security::tpm::ak::cursor::Cursor;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::{
    DIGEST_LEN, HEADER_LEN, TPM_ALG_SHA256, TPM_RH_NULL, TPM_ST_NO_SESSIONS,
};
use crate::security::tpm::machine_key::wire::{checked, digest_at, frame};

/// The AK's enrollment label, then `msg`: the bytes the signature covers.
pub(in crate::security::tpm) fn build_hash(msg: &[u8; DIGEST_LEN]) -> Vec<u8> {
    let mut data = Vec::with_capacity(AK_SIGN_LABEL.len() + DIGEST_LEN);
    data.extend_from_slice(AK_SIGN_LABEL);
    data.extend_from_slice(msg);
    let mut body = Vec::with_capacity(2 + data.len() + 6);
    put_tpm2b(&mut body, &data);
    body.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    body.extend_from_slice(&TPM_RH_ENDORSEMENT.to_be_bytes());
    frame(TPM_ST_NO_SESSIONS, TPM_CC_HASH, &body)
}

/// The digest, and the TPMT_TK_HASHCHECK whole, as Sign takes it back. A null
/// ticket is the TPM declining to vouch for the message: `Unsignable`.
pub(in crate::security::tpm) fn parse_hash(
    resp: &[u8],
) -> Result<([u8; DIGEST_LEN], Vec<u8>), EnrollError> {
    let r = checked(resp)?;
    let digest = digest_at(r, HEADER_LEN)?;
    let at = HEADER_LEN + 2 + DIGEST_LEN;
    let mut c = Cursor::at(r, at);
    let tag = c.u16()?;
    let hierarchy = c.u32()?;
    let n = c.u16()? as usize;
    c.skip(n)?;
    if tag != TPM_ST_HASHCHECK || n > DIGEST_MAX {
        return Err(TpmError::InvalidResponse.into());
    }
    if hierarchy == TPM_RH_NULL || n == 0 {
        return Err(EnrollError::Unsignable);
    }
    if hierarchy != TPM_RH_ENDORSEMENT {
        return Err(TpmError::InvalidResponse.into());
    }
    let ticket = r.get(at..c.position()).ok_or(TpmError::InvalidResponse)?;
    Ok((digest, ticket.to_vec()))
}
