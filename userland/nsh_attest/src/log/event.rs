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

//! One `TCG_PCR_EVENT2`: PCR, type, one digest under every bank the header
//! declared (each once, in any order), then the body.

use super::le::Le;
use super::{Banks, MAX_BANKS, MAX_EVENT_BODY, PCRS, TPM_ALG_SHA256};
use crate::LogFault;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event<'a> {
    pub pcr: usize,
    pub kind: u32,
    pub sha256: [u8; 32],
    pub body: &'a [u8],
    /// The event's length in the log.
    pub len: usize,
}

/// The event at the start of `at`.
pub fn event<'a>(banks: &Banks, at: &'a [u8]) -> Result<Event<'a>, LogFault> {
    let mut c = Le::new(at);
    let pcr = c.u32()? as usize;
    if pcr >= PCRS {
        return Err(LogFault::PcrIndex);
    }
    let kind = c.u32()?;
    if c.u32()? as usize != banks.n {
        return Err(LogFault::Bank);
    }
    let (mut sha256, mut seen) = (None, [0u16; MAX_BANKS]);
    for i in 0..banks.n {
        let alg = c.u16()?;
        let size = banks.size_of(alg).ok_or(LogFault::Bank)?;
        if seen[..i].contains(&alg) {
            return Err(LogFault::Bank);
        }
        seen[i] = alg;
        let d = c.take(size)?;
        if alg == TPM_ALG_SHA256 {
            sha256 = d.try_into().ok();
        }
    }
    let sha256 = sha256.ok_or(LogFault::Bank)?;
    let body = c.body(MAX_EVENT_BODY)?;
    Ok(Event { pcr, kind, sha256, body, len: c.pos })
}
