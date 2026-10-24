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

//! The first event: a `TCG_PCClientPCREvent` in the old SHA-1 shape, PCR 0,
//! `EV_NO_ACTION`, a zero digest, and a body that is exactly the
//! `TCG_EfiSpecIDEvent` naming the banks every later event carries.

use super::le::Le;
use super::{Banks, EV_NO_ACTION, MAX_BANKS, MAX_DIGEST, MAX_EVENT_BODY, TPM_ALG_SHA256};
use crate::LogFault;

const SPEC_ID: &[u8; 16] = b"Spec ID Event03\0";

/// The banks, and the header's length in bytes. A header that declares no
/// bank is refused below, as one without a SHA-256 bank.
pub fn header(log: &[u8]) -> Result<(Banks, usize), LogFault> {
    let mut c = Le::new(log);
    let (pcr, kind) = (c.u32()?, c.u32()?);
    let digest = c.take(20)?;
    if pcr != 0 || kind != EV_NO_ACTION || digest.iter().any(|b| *b != 0) {
        return Err(LogFault::Header);
    }
    let banks = spec_id(c.body(MAX_EVENT_BODY)?)?;
    Ok((banks, c.pos))
}

fn spec_id(body: &[u8]) -> Result<Banks, LogFault> {
    let mut b = Le::new(body);
    let (sig, _class) = (b.take(16)?, b.u32()?);
    let (minor, major, _errata, uintn) = (b.u8()?, b.u8()?, b.u8()?, b.u8()?);
    if sig != SPEC_ID || minor != 0 || major != 2 || !(1..=2).contains(&uintn) {
        return Err(LogFault::Header);
    }
    let n = b.u32()? as usize;
    if n > MAX_BANKS {
        return Err(LogFault::Bank);
    }
    let mut out = Banks { algs: [(0, 0); MAX_BANKS], n: 0 };
    for i in 0..n {
        let (alg, size) = (b.u16()?, b.u16()?);
        if size == 0 || size as usize > MAX_DIGEST || out.size_of(alg).is_some() {
            return Err(LogFault::Bank);
        }
        (out.algs[i], out.n) = ((alg, size), i + 1);
    }
    let vendor = b.u8()? as usize;
    b.take(vendor)?;
    if b.left() != 0 || out.size_of(TPM_ALG_SHA256) != Some(32) {
        return Err(if b.left() != 0 { LogFault::Header } else { LogFault::Bank });
    }
    Ok(out)
}
