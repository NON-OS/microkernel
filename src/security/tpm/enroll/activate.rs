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

//! `TPM2_ActivateCredential`: the TPM unwraps the registrar's credential only
//! when the EK it was made to is this TPM's and the key it names is loaded
//! here beside it. TPM 2.0 Part 3, 12.5.

use alloc::vec::Vec;

use super::activated::Activated;
use super::consts::{ENCRYPTED_SECRET_MAX, ID_OBJECT_MAX, TPM_CC_ACTIVATE_CREDENTIAL, TPM_RS_PW};
use super::error::EnrollError;
use super::marshal::{auth, put_tpm2b};
use crate::security::tpm::ak::cursor::Cursor;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::{HEADER_LEN, TPM_ST_SESSIONS};
use crate::security::tpm::machine_key::wire::{checked, frame};

/// Neither input empty, and neither longer than the structure it fills.
pub(in crate::security::tpm) fn check_challenge(
    blob: &[u8],
    secret: &[u8],
) -> Result<(), EnrollError> {
    let fits = |b: &[u8], max: usize| !b.is_empty() && b.len() <= max;
    if fits(blob, ID_OBJECT_MAX) && fits(secret, ENCRYPTED_SECRET_MAX) {
        Ok(())
    } else {
        Err(EnrollError::OutOfBounds)
    }
}

/// `blob` and `secret` are the bodies of the registrar's TPM2B_ID_OBJECT and
/// TPM2B_ENCRYPTED_SECRET, without their size fields. Two authorizations: the
/// empty password for the AK, whose role here is admin and which carries no
/// admin policy, then the policy session for the EK.
pub(in crate::security::tpm) fn build_activate(
    ak: u32,
    ek: u32,
    session: u32,
    blob: &[u8],
    secret: &[u8],
) -> Result<Vec<u8>, EnrollError> {
    check_challenge(blob, secret)?;
    let mut body = Vec::with_capacity(34 + blob.len() + secret.len());
    body.extend_from_slice(&ak.to_be_bytes());
    body.extend_from_slice(&ek.to_be_bytes());
    body.extend_from_slice(&18u32.to_be_bytes());
    body.extend_from_slice(&auth(TPM_RS_PW));
    body.extend_from_slice(&auth(session));
    put_tpm2b(&mut body, blob);
    put_tpm2b(&mut body, secret);
    Ok(frame(TPM_ST_SESSIONS, TPM_CC_ACTIVATE_CREDENTIAL, &body))
}

/// certInfo, after the parameter size: the registrar's secret, nonempty and
/// at most a digest long.
pub(in crate::security::tpm) fn parse_activate(resp: &[u8]) -> Result<Activated, EnrollError> {
    let r = checked(resp)?;
    let mut c = Cursor::at(r, HEADER_LEN + 4);
    let n = c.u16()? as usize;
    Activated::copy(c.take(n)?).ok_or(TpmError::InvalidResponse.into())
}
