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

//! `TPM2_Sign` with the attestation key, against the hash ticket.
//! TPM 2.0 Part 3, 20.2.

use alloc::vec::Vec;

use super::consts::{TPM_ALG_ECDSA, TPM_CC_SIGN};
use super::error::EnrollError;
use super::marshal::{put_password, put_tpm2b};
use crate::security::tpm::ak::cursor::Cursor;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::{
    DIGEST_LEN, HEADER_LEN, TPM_ALG_NULL, TPM_ALG_SHA256, TPM_ST_SESSIONS,
};
use crate::security::tpm::machine_key::wire::{checked, frame};

/// The AK's empty password; inScheme null, so the key's own scheme, ECDSA
/// over SHA-256; then the ticket `parse_hash` returned, unchanged.
pub(in crate::security::tpm) fn build_sign(
    ak: u32,
    digest: &[u8; DIGEST_LEN],
    ticket: &[u8],
) -> Vec<u8> {
    let mut body = Vec::with_capacity(4 + 13 + 2 + DIGEST_LEN + 2 + ticket.len());
    body.extend_from_slice(&ak.to_be_bytes());
    put_password(&mut body);
    put_tpm2b(&mut body, digest);
    body.extend_from_slice(&TPM_ALG_NULL.to_be_bytes());
    body.extend_from_slice(ticket);
    frame(TPM_ST_SESSIONS, TPM_CC_SIGN, &body)
}

/// r then s, each left-padded to 32 bytes, since a part may drop leading
/// zeros. Any scheme but ECDSA over SHA-256 is not the AK's and is refused.
pub(in crate::security::tpm) fn parse_sign(resp: &[u8]) -> Result<[u8; 64], EnrollError> {
    let r = checked(resp)?;
    let mut c = Cursor::at(r, HEADER_LEN + 4);
    if c.u16()? != TPM_ALG_ECDSA || c.u16()? != TPM_ALG_SHA256 {
        return Err(TpmError::InvalidResponse.into());
    }
    let mut sig = [0u8; 64];
    for half in sig.chunks_exact_mut(32) {
        let n = c.u16()? as usize;
        if n == 0 || n > 32 {
            return Err(TpmError::InvalidResponse.into());
        }
        half[32 - n..].copy_from_slice(c.take(n)?);
    }
    Ok(sig)
}
