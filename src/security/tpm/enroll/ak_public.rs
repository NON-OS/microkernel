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

//! `TPM2_ReadPublic` of the attestation key: the public area the registrar
//! names in its challenge and checks the enrollment signature against.
//! TPM 2.0 Part 3, 12.4.

use alloc::vec::Vec;

use super::consts::TPM_CC_READ_PUBLIC;
use super::error::EnrollError;
use super::public::{checked_public, Public};
use crate::security::tpm::ak::cursor::Cursor;
use crate::security::tpm::ak::template::ak_template;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::{HEADER_LEN, TPM_ST_NO_SESSIONS};
use crate::security::tpm::machine_key::wire::{checked, frame};

pub(in crate::security::tpm) fn build_read_public(handle: u32) -> Vec<u8> {
    frame(TPM_ST_NO_SESSIONS, TPM_CC_READ_PUBLIC, &handle.to_be_bytes())
}

/// `outPublic`, then the name. The area must be the kernel's own AK template
/// with a P-256 point in `unique`: restricted signing, ECDSA over SHA-256,
/// fixed to this TPM. Any other key would sign whatever it was handed.
pub(in crate::security::tpm) fn parse_read_public(resp: &[u8]) -> Result<Public, EnrollError> {
    let r = checked(resp)?;
    let mut c = Cursor::at(r, HEADER_LEN);
    let n = c.u16()? as usize;
    let tpmt = c.take(n)?;
    let n = c.u16()? as usize;
    let name = c.take(n)?;
    /* The template is a TPM2B whose `unique` is two empty coordinates. */
    let template = ak_template();
    let shape =
        template.get(2..template.len().saturating_sub(4)).ok_or(TpmError::InvalidResponse)?;
    checked_public(tpmt, shape, &[32, 32], name)
}
