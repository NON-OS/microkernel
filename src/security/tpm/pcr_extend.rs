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

//! `TPM2_PCR_Extend` (TPM 2.0 Part 3, 22.2): one PCR, one digest for each
//! bank given, under the empty password a PCR's authorization takes. A bank
//! the TPM has not allocated is left as it is; one it does not implement is
//! refused by the TPM, and the extend with it.

use alloc::vec::Vec;

use super::machine_key::consts::TPM_ST_SESSIONS;
use super::machine_key::run::run;
use super::machine_key::wire::{checked, frame};
use super::machine_key::KeyError;

const TPM_CC_PCR_EXTEND: u32 = 0x0000_0182;
const TPM_RS_PW: u32 = 0x4000_0009;

/// The command's body: the PCR's handle, the password session, then the
/// `TPML_DIGEST_VALUES`.
pub fn extend_body(pcr: u32, digests: &[(u16, &[u8])]) -> Vec<u8> {
    let mut b = pcr.to_be_bytes().to_vec();
    b.extend_from_slice(&9u32.to_be_bytes());
    b.extend_from_slice(&TPM_RS_PW.to_be_bytes());
    b.extend_from_slice(&[0, 0, 0, 0, 0]);
    b.extend_from_slice(&(digests.len() as u32).to_be_bytes());
    for (alg, d) in digests {
        b.extend_from_slice(&alg.to_be_bytes());
        b.extend_from_slice(d);
    }
    b
}

pub fn pcr_extend(pcr: u32, digests: &[(u16, &[u8])]) -> Result<(), KeyError> {
    let resp = run(&frame(TPM_ST_SESSIONS, TPM_CC_PCR_EXTEND, &extend_body(pcr, digests)))?;
    checked(&resp).map(|_| ())
}
