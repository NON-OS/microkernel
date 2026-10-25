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

//! `TPMT_SIGNATURE` as the AK makes it: ECDSA over SHA-256, then r and s,
//! each a `TPM2B` of 1 to 32 bytes, and nothing after them. Any other
//! scheme or hash is refused: the AK's template allows only this one.

use crate::read::Be;
use crate::{FormatFault, Refusal};

pub const TPM_ALG_ECDSA: u16 = 0x0018;
pub const TPM_ALG_SHA256: u16 = 0x000B;

/// r and s, each left-padded to 32 bytes. A TPM may send either shorter
/// when its leading bytes are zero; the value is the same.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EcdsaSig {
    pub r: [u8; 32],
    pub s: [u8; 32],
}

impl EcdsaSig {
    pub fn parse(bytes: &[u8]) -> Result<Self, Refusal> {
        read(bytes).map_err(|_| Refusal::Signature)?.ok_or(Refusal::Signature)
    }
}

fn read(bytes: &[u8]) -> Result<Option<EcdsaSig>, FormatFault> {
    let mut b = Be::new(bytes);
    if b.u16()? != TPM_ALG_ECDSA || b.u16()? != TPM_ALG_SHA256 {
        return Ok(None);
    }
    let r = padded(b.sized(false, 1, 32)?);
    let s = padded(b.sized(false, 1, 32)?);
    b.finish()?;
    Ok(Some(EcdsaSig { r, s }))
}

fn padded(v: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    out[32 - v.len()..].copy_from_slice(v);
    out
}
