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

use sha2::{Digest, Sha256};

use super::consts::{
    EV_EFI_BOOT_SERVICES_APPLICATION, EV_NO_ACTION, MAX_EVENTS, MAX_LOG_BYTES, PCR_BOOT_MANAGER,
};
use super::error::LogError;
use super::event::event;
use super::spec_id::spec_id;

/// PCR 4 as the log rebuilds it, the last application the firmware measured
/// into it, and how many events the log holds after its header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Replay {
    pub pcr4: [u8; 32],
    pub last_application: Option<[u8; 32]>,
    pub events: usize,
}

/*
 * Extend a zero PCR 4 with every PCR 4 event's SHA-256 digest in log order, as
 * the TPM did: `pcr = SHA-256(pcr || digest)`. EV_NO_ACTION events are logged
 * and never extended. The whole log must parse, to its last byte: a log that
 * cannot be read to its end cannot be replayed, and is refused.
 */
pub fn replay(log: &[u8]) -> Result<Replay, LogError> {
    if log.len() > MAX_LOG_BYTES {
        return Err(LogError::TooLarge);
    }
    let (banks, mut at) = spec_id(log)?;
    let mut out = Replay { pcr4: [0; 32], last_application: None, events: 0 };
    while let Some(rest) = log.get(at..).filter(|r| !r.is_empty()) {
        if out.events == MAX_EVENTS {
            return Err(LogError::TooManyEvents);
        }
        let e = event(&banks, rest)?;
        out.events += 1;
        at += e.len;
        if e.pcr != PCR_BOOT_MANAGER || e.kind == EV_NO_ACTION {
            continue;
        }
        let d = e.sha256.ok_or(LogError::MissingSha256)?;
        out.pcr4 = Sha256::new().chain_update(out.pcr4).chain_update(d).finalize().into();
        if e.kind == EV_EFI_BOOT_SERVICES_APPLICATION {
            out.last_application = Some(d);
        }
    }
    Ok(out)
}
