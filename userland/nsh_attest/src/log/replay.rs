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

//! The replay: every PCR starts at zero (PCR 0 at its startup locality),
//! and each extending event, in log order, sets `pcr = SHA-256(pcr ||
//! digest)`, as the TPM did. `EV_NO_ACTION` is logged and never extended.
//! The whole log must parse to its last byte; a log over the cap is
//! refused, never cut short.

use nonos_hash::Sha256;

use super::locality::startup_locality;
use super::pcr14::capsule;
use super::pcr4::pcr4_event;
use super::replayed::Replayed;
use super::{event, header, EV_EFI_BOOT_SERVICES_APPLICATION, EV_NO_ACTION, MAX_EVENTS};
use super::{PCRS, PCR_BOOT_MANAGER};
use crate::consts::MAX_LOG;
use crate::LogFault;

pub fn replay(log: &[u8]) -> Result<Replayed, LogFault> {
    if log.len() > MAX_LOG {
        return Err(LogFault::TooLarge);
    }
    let (banks, mut at) = header(log)?;
    let mut out = Replayed::default();
    let mut touched = [false; PCRS];
    while let Some(rest) = log.get(at..).filter(|r| !r.is_empty()) {
        if out.events == MAX_EVENTS {
            return Err(LogFault::TooManyEvents);
        }
        let e = event(&banks, rest)?;
        (out.events, at) = (out.events + 1, at + e.len);
        if e.pcr == PCR_BOOT_MANAGER {
            pcr4_event(&e)?;
        }
        out.capsules.extend(capsule(&e)?);
        if e.kind == EV_NO_ACTION {
            if let Some(l) = startup_locality(&e, touched[0])? {
                out.pcrs[0][31] = l;
                touched[0] = true;
            }
            continue;
        }
        let mut h = Sha256::new();
        h.update(&out.pcrs[e.pcr]);
        h.update(&e.sha256);
        out.pcrs[e.pcr] = h.finalize();
        touched[e.pcr] = true;
        if e.pcr == PCR_BOOT_MANAGER && e.kind == EV_EFI_BOOT_SERVICES_APPLICATION {
            out.last_application = Some(e.sha256);
            out.applications.push(e.sha256);
        }
    }
    Ok(out)
}
