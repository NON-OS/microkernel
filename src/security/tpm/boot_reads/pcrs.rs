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

//! `TPM2_PCR_Read` of several PCRs in the SHA-256 bank. A TPM may answer
//! with fewer digests than it was asked for, and says which it read in
//! `pcrSelectionOut`, so the read is repeated for the rest until every
//! value has come back. A round that reads nothing (the bank is not
//! allocated) or reads a PCR not asked for is refused. TPM 2.0 Part 3, 22.4.

use alloc::vec::Vec;

use super::pcrs_round::round;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::KeyError;

/// More rounds than any TPM needs: each reads at least one PCR, and a
/// selection of eight is done in eight.
const ROUNDS: usize = 8;

/// The SHA-256 value of each PCR in `pcrs`, in that order. Every index is
/// below 24 and none is listed twice.
pub fn read_pcrs<const N: usize>(pcrs: &[u8; N]) -> Result<[[u8; 32]; N], KeyError> {
    read_with(pcrs, round)
}

/// The same, over any one-round read: the TPM's, or a test's.
pub(in crate::security::tpm) fn read_with<const N: usize>(
    pcrs: &[u8; N],
    mut round: impl FnMut(u32) -> Result<Vec<(u8, [u8; 32])>, KeyError>,
) -> Result<[[u8; 32]; N], KeyError> {
    let mut left = 0u32;
    for &p in pcrs {
        if p >= 24 || left & (1 << p) != 0 {
            return Err(TpmError::InvalidResponse.into());
        }
        left |= 1 << p;
    }
    let mut out = [[0u8; 32]; N];
    for _ in 0..ROUNDS {
        if left == 0 {
            break;
        }
        let got = round(left)?;
        if got.is_empty() {
            return Err(TpmError::InvalidResponse.into());
        }
        for (pcr, value) in got {
            let at = pcrs.iter().position(|p| *p == pcr).ok_or(TpmError::InvalidResponse)?;
            if left & (1 << pcr) == 0 {
                return Err(TpmError::InvalidResponse.into());
            }
            left &= !(1 << pcr);
            out[at] = value;
        }
    }
    match left {
        0 => Ok(out),
        _ => Err(TpmError::InvalidResponse.into()),
    }
}
