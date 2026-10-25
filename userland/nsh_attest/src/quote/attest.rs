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

//! `TPMS_ATTEST` for a quote (TPM 2.0 Part 2, 10.12.12), read to its last
//! byte: magic, type, qualifiedSigner, extraData, clockInfo,
//! firmwareVersion, then `TPMS_QUOTE_INFO`: one or more PCR selections and
//! the PCR digest.

use super::{Attest, Selection, MAX_HASHED, MAX_SELECT, MAX_SELECTIONS};
use super::{ST_ATTEST_QUOTE, TPM_GENERATED};
use crate::read::Be;
use crate::{FormatFault, QuoteFault, Refusal};

impl<'a> Attest<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, Refusal> {
        let mut r = Be::new(bytes);
        if r.u32().map_err(malformed)? != TPM_GENERATED {
            return Err(Refusal::Quote(QuoteFault::NotTpmGenerated));
        }
        if r.u16().map_err(malformed)? != ST_ATTEST_QUOTE {
            return Err(Refusal::Quote(QuoteFault::NotAQuote));
        }
        let attest = rest(&mut r).map_err(malformed)?;
        r.finish().map_err(malformed)?;
        Ok(attest)
    }
}

fn rest<'a>(r: &mut Be<'a>) -> Result<Attest<'a>, FormatFault> {
    let signer = r.sized(false, 0, MAX_HASHED)?;
    let extra = r.sized(false, 0, MAX_HASHED)?;
    let _clock = (r.u64()?, r.u32()?, r.u32()?, r.flag()?);
    let _firmware = r.u64()?;
    let count = r.u32()?;
    if count == 0 || count > MAX_SELECTIONS {
        return Err(FormatFault::Value);
    }
    let (hash, bitmap) = select(r)?;
    for _ in 1..count {
        select(r)?;
    }
    let pcr_digest = r.sized(false, 0, MAX_HASHED)?;
    Ok(Attest { signer, extra, selection: Selection { count, hash, bitmap }, pcr_digest })
}

/// One `TPMS_PCR_SELECTION`: the bank, then a bitmap of at most four bytes.
fn select<'a>(r: &mut Be<'a>) -> Result<(u16, &'a [u8]), FormatFault> {
    let hash = r.u16()?;
    let size = r.u8()? as usize;
    if size > MAX_SELECT {
        return Err(FormatFault::Oversize);
    }
    Ok((hash, r.take(size)?))
}

fn malformed(_: FormatFault) -> Refusal {
    Refusal::Quote(QuoteFault::Malformed)
}
