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

//! `TPM2_PCR_Read` of PCR 4 in the SHA-256 bank. TPM 2.0 Part 3, 22.4.

use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::{TPM_ALG_SHA256, TPM_ST_NO_SESSIONS};
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::wire::{checked, digest_at, frame, u32_at};
use crate::security::tpm::machine_key::KeyError;

const TPM_CC_PCR_READ: u32 = 0x0000_017E;

/// One selection: the SHA-256 bank, three select bytes, bit 4 of the first.
const SELECT: [u8; 6] = [0x00, 0x0B, 3, 0x10, 0, 0];

/// PCR 4's SHA-256 value now.
pub fn pcr4() -> Result<[u8; 32], KeyError> {
    let mut body = 1u32.to_be_bytes().to_vec();
    body.extend_from_slice(&SELECT);
    let resp = run(&frame(TPM_ST_NO_SESSIONS, TPM_CC_PCR_READ, &body))?;
    let r = checked(&resp)?;
    /*
     * The update counter, then the selection the TPM read: it must be the one
     * asked for, since a bank the TPM does not keep comes back empty. Then
     * exactly one digest.
     */
    let read = r.get(18..24).ok_or(TpmError::InvalidResponse)?;
    if u32_at(r, 14)? != 1
        || read != SELECT
        || u16::from_be_bytes([read[0], read[1]]) != TPM_ALG_SHA256
    {
        return Err(TpmError::InvalidResponse.into());
    }
    if u32_at(r, 24)? != 1 {
        return Err(TpmError::InvalidResponse.into());
    }
    digest_at(r, 28)
}
