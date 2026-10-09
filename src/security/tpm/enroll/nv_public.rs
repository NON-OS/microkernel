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

//! `TPM2_NV_ReadPublic` of the certificate index: how long it is and who may
//! read it. TPM 2.0 Part 3, 31.6.

use alloc::vec::Vec;

use super::consts::{
    CERT_MAX, NV_AUTHREAD, NV_OWNERREAD, NV_TYPE_MASK, NV_WRITTEN, TPM_CC_NV_READ_PUBLIC,
};
use super::error::EnrollError;
use super::marshal::skip_tpm2b;
use crate::security::tpm::ak::cursor::Cursor;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::{HEADER_LEN, TPM_RH_OWNER, TPM_ST_NO_SESSIONS};
use crate::security::tpm::machine_key::wire::{checked, frame};

/// The index's data size, at most [`CERT_MAX`], and the handle whose empty
/// password authorizes the read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::security::tpm) struct NvSlot {
    pub size: usize,
    pub auth: u32,
}

pub(in crate::security::tpm) fn build_nv_read_public(index: u32) -> Vec<u8> {
    frame(TPM_ST_NO_SESSIONS, TPM_CC_NV_READ_PUBLIC, &index.to_be_bytes())
}

/// The index must be the one asked for, ordinary and written. The profile
/// gives the certificate index two reads with an empty password: by the
/// index itself (`AUTHREAD`), taken first, or by the owner (`OWNERREAD`).
pub(in crate::security::tpm) fn parse_nv_read_public(
    resp: &[u8],
    index: u32,
) -> Result<NvSlot, EnrollError> {
    let r = checked(resp)?;
    /* past the TPM2B_NV_PUBLIC's size: nvIndex, nameAlg, attributes */
    let mut c = Cursor::at(r, HEADER_LEN + 2);
    let named = c.u32()? == index;
    c.skip(2)?;
    let attrs = c.u32()?;
    skip_tpm2b(&mut c)?;
    let size = c.u16()? as usize;
    if !named || attrs & NV_TYPE_MASK != 0 || attrs & NV_WRITTEN == 0 || size == 0 {
        return Err(TpmError::InvalidResponse.into());
    }
    if size > CERT_MAX {
        return Err(EnrollError::OutOfBounds);
    }
    let auth = match (attrs & NV_AUTHREAD != 0, attrs & NV_OWNERREAD != 0) {
        (true, _) => index,
        (false, true) => TPM_RH_OWNER,
        (false, false) => return Err(TpmError::InvalidResponse.into()),
    };
    Ok(NvSlot { size, auth })
}
