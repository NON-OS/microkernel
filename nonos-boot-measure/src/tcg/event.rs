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

use super::banks::Banks;
use super::consts::{MAX_ALGS, MAX_EVENT_BYTES, TPM_ALG_SHA256};
use super::error::LogError;
use super::grow::Walk;
use super::read::{Cursor, Stop};

/// One `TCG_PCR_EVENT2`: its PCR, its type, its SHA-256 digest if it carries
/// one, and its length in the log.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Event {
    pub pcr: u32,
    pub kind: u32,
    pub sha256: Option<[u8; 32]>,
    pub len: usize,
}

/// Read the event at the start of `at`, which may hold only a prefix of it.
/// Every digest is under a bank the header declared, each bank at most once.
pub fn event_walk(banks: &Banks, at: &[u8]) -> Result<Walk<Event>, LogError> {
    let mut c = Cursor::new(at);
    let r = (|| {
        let (pcr, kind, count) = (c.u32()?, c.u32()?, c.u32()? as usize);
        if count > banks.len() {
            return Err(Stop::Bad(LogError::TooManyBanks));
        }
        let (mut sha256, mut seen) = (None, [0u16; MAX_ALGS]);
        for i in 0..count {
            let alg = c.u16()?;
            let size = banks.size_of(alg).ok_or(Stop::Bad(LogError::UnknownBank))?;
            if seen[..i].contains(&alg) {
                return Err(Stop::Bad(LogError::UnknownBank));
            }
            seen[i] = alg;
            let d = c.take(size)?;
            if alg == TPM_ALG_SHA256 {
                sha256 =
                    Some(<[u8; 32]>::try_from(d).map_err(|_| Stop::Bad(LogError::NoSha256Bank))?);
            }
        }
        let size = c.u32()? as usize;
        if size > MAX_EVENT_BYTES {
            return Err(Stop::Bad(LogError::EventTooLarge));
        }
        c.take(size)?;
        Ok((pcr, kind, sha256))
    })();
    Walk::of(r.map(|(pcr, kind, sha256)| Event { pcr, kind, sha256, len: c.at() }))
}

/// The event at the start of a log held whole.
pub fn event(banks: &Banks, at: &[u8]) -> Result<Event, LogError> {
    event_walk(banks, at)?.done().ok_or(LogError::Truncated)
}
