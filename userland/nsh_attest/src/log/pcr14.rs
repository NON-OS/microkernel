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

//! What PCR 14 may hold: capsule events alone, as the kernel writes them
//! (docs/design/k1-attest-doc-v4.md, "PCR 14"). The body is 65 bytes, the
//! certificate's digest, the verifier's digest and a kind byte, and the
//! event's SHA-256 must be the body's own, so the kind is under what the TPM
//! extended and cannot be changed after. The kind is one of the four
//! `Admitted` names, and a capsule names the path-only tag as its verifier
//! exactly when its path alone admitted it.

use nonos_hash::sha256;

use super::{Admitted, Capsule, Event, EV_NONOS_CAPSULE, PCR_CAPSULE};
use crate::LogFault;

/// What a development kernel hashes in place of a proof it does not have.
const PATH_ONLY_TAG: &[u8] = b"nonos capsule path-only";

/// The capsule a PCR 14 event records, `None` for an event of another PCR.
pub(super) fn capsule(e: &Event<'_>) -> Result<Option<Capsule>, LogFault> {
    if e.pcr != PCR_CAPSULE && e.kind != EV_NONOS_CAPSULE {
        return Ok(None);
    }
    if e.pcr != PCR_CAPSULE || e.kind != EV_NONOS_CAPSULE || e.body.len() != 65 {
        return Err(LogFault::Capsule);
    }
    if sha256(e.body) != e.sha256 {
        return Err(LogFault::Capsule);
    }
    let admitted = Admitted::from_kind(e.body[64]).ok_or(LogFault::Capsule)?;
    let half = |at: usize| e.body[at..at + 32].try_into().map_err(|_| LogFault::Capsule);
    let verifier: [u8; 32] = half(32)?;
    if (admitted == Admitted::PathOnly) != (verifier == sha256(PATH_ONLY_TAG)) {
        return Err(LogFault::Capsule);
    }
    Ok(Some(Capsule { cert: half(0)?, verifier, admitted }))
}
