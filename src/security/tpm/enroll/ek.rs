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

//! `TPM2_CreatePrimary(TPM_RH_ENDORSEMENT, template)` for the EK, and its
//! public area and name out of the answer. TPM 2.0 Part 3, 24.1.

use alloc::vec::Vec;

use super::consts::TPM_RH_ENDORSEMENT;
use super::error::EnrollError;
use super::kind::EkKind;
use super::marshal::{put_password, put_tpm2b, skip_tpm2b};
use super::public::{checked_public, Public};
use super::template::{ek_shape, ek_template};
use crate::security::tpm::ak::cursor::Cursor;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::{TPM_CC_CREATE_PRIMARY, TPM_ST_SESSIONS};
use crate::security::tpm::machine_key::wire::{checked, frame, u32_at};

/// Header, object handle, parameter size: where `outPublic` begins.
const OUT_PUBLIC_AT: usize = 10 + 4 + 4;

/// Authorized by the endorsement hierarchy's empty password. inSensitive is
/// empty, as the profile requires; no outside info, no creation PCRs.
pub(in crate::security::tpm) fn build_create_ek(kind: EkKind) -> Vec<u8> {
    let template = ek_template(kind);
    let mut body = Vec::with_capacity(32 + template.len());
    body.extend_from_slice(&TPM_RH_ENDORSEMENT.to_be_bytes());
    put_password(&mut body);
    body.extend_from_slice(&[0, 4, 0, 0, 0, 0]);
    put_tpm2b(&mut body, &template);
    body.extend_from_slice(&[0; 6]);
    frame(TPM_ST_SESSIONS, TPM_CC_CREATE_PRIMARY, &body)
}

/// `outPublic`, then creationData, creationHash and creationTicket stepped
/// over, then the name, which must close the parameter area exactly. The
/// handle ahead of them is `parse_create`'s to read.
pub(in crate::security::tpm) fn parse_ek(resp: &[u8], kind: EkKind) -> Result<Public, EnrollError> {
    let r = checked(resp)?;
    let end = OUT_PUBLIC_AT as u64 + u64::from(u32_at(r, OUT_PUBLIC_AT - 4)?);
    let mut c = Cursor::at(r, OUT_PUBLIC_AT);
    let n = c.u16()? as usize;
    let tpmt = c.take(n)?;
    skip_tpm2b(&mut c)?;
    skip_tpm2b(&mut c)?;
    /* the ticket's tag and hierarchy, then its digest */
    c.skip(6)?;
    skip_tpm2b(&mut c)?;
    let n = c.u16()? as usize;
    let name = c.take(n)?;
    if c.position() as u64 != end {
        return Err(TpmError::InvalidResponse.into());
    }
    checked_public(tpmt, &ek_shape(kind), kind.unique(), name)
}
