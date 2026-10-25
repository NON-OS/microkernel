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

//! One `TPM2_PCR_Read`: the PCRs left to read, as a bitmap, and the values
//! the TPM gave back, each with the PCR `pcrSelectionOut` says it is.

use alloc::vec::Vec;

use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::consts::{TPM_ALG_SHA256, TPM_ST_NO_SESSIONS};
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::wire::{checked, digest_at, frame, u32_at};
use crate::security::tpm::machine_key::KeyError;

const TPM_CC_PCR_READ: u32 = 0x0000_017E;
/// A `TPM2B_DIGEST` of SHA-256: its size, then 32 bytes.
const DIGEST_FIELD: usize = 2 + 32;

/// `left` holds bit `n` for PCR `n`, below 24.
pub(super) fn round(left: u32) -> Result<Vec<(u8, [u8; 32])>, KeyError> {
    let want = [left as u8, (left >> 8) as u8, (left >> 16) as u8];
    let mut body = 1u32.to_be_bytes().to_vec();
    body.extend_from_slice(&TPM_ALG_SHA256.to_be_bytes());
    body.push(3);
    body.extend_from_slice(&want);
    let resp = run(&frame(TPM_ST_NO_SESSIONS, TPM_CC_PCR_READ, &body))?;
    parse(checked(&resp)?, left)
}

/// The answer to a read of `left`: which PCRs it read, at most those asked
/// for, and exactly one SHA-256 digest for each.
pub(in crate::security::tpm) fn parse(
    r: &[u8],
    left: u32,
) -> Result<Vec<(u8, [u8; 32])>, KeyError> {
    /*
     * The update counter at 10, then one selection: the bank, three select
     * bytes, which PCRs it read. Then the digests, one for each, in PCR order.
     */
    if r.get(..2) != Some(&TPM_ST_NO_SESSIONS.to_be_bytes()[..]) {
        return Err(TpmError::InvalidResponse.into());
    }
    let sel = r.get(18..24).ok_or(TpmError::InvalidResponse)?;
    if u32_at(r, 14)? != 1 || u16::from_be_bytes([sel[0], sel[1]]) != TPM_ALG_SHA256 || sel[2] != 3
    {
        return Err(TpmError::InvalidResponse.into());
    }
    let read = u32::from(sel[3]) | u32::from(sel[4]) << 8 | u32::from(sel[5]) << 16;
    let n = read.count_ones() as usize;
    if read & !left != 0 || u32_at(r, 24)? as usize != n || r.len() != 28 + n * DIGEST_FIELD {
        return Err(TpmError::InvalidResponse.into());
    }
    let mut out = Vec::with_capacity(n);
    for (k, pcr) in (0..24u8).filter(|p| read & (1 << p) != 0).enumerate() {
        out.push((pcr, digest_at(r, 28 + k * DIGEST_FIELD)?));
    }
    Ok(out)
}
