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

//! What PCR 4 may hold. The TPM extends only an event's digest, never its
//! type, so a type is only a claim: an application relabelled as an action
//! would replay the same. PCR 4 therefore admits four types alone, and a
//! separator or action must carry the SHA-256 of its own body, so the
//! only digests in PCR 4 the client cannot recompute are applications,
//! and every one of those is held to the release's loader or the
//! machine's enrolment.

use nonos_hash::sha256;

use super::{Event, EV_EFI_BOOT_SERVICES_APPLICATION, EV_NO_ACTION};
use crate::LogFault;

pub const EV_SEPARATOR: u32 = 0x0000_0004;
pub const EV_EFI_ACTION: u32 = 0x8000_0007;

pub(super) fn pcr4_event(e: &Event<'_>) -> Result<(), LogFault> {
    match e.kind {
        EV_NO_ACTION | EV_EFI_BOOT_SERVICES_APPLICATION => Ok(()),
        EV_SEPARATOR | EV_EFI_ACTION if sha256(e.body) == e.sha256 => Ok(()),
        EV_SEPARATOR | EV_EFI_ACTION => Err(LogFault::Pcr4Digest),
        _ => Err(LogFault::Pcr4Event),
    }
}
