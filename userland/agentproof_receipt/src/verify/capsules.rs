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

//! PCR 14 replayed from the receipt's capsules alone, as nsh_attest replays
//! it from the log (log/pcr14.rs): each body is the SHA-256 the kernel
//! extended, so every byte of it is under the quote once the replay from
//! zero gives the quoted PCR 14. A kind is one of the four `Admitted` names,
//! and a body names the path-only tag as its verifier exactly when its path
//! alone admitted it.

use alloc::vec::Vec;

use nonos_hash::sha256;
use nsh_attest::log::Admitted;

use super::Refusal;
use crate::napr::CAPSULE;

const PATH_ONLY_TAG: &[u8] = b"nonos capsule path-only";

/// What admitted each capsule, if `capsules` replays to `quoted`.
pub fn replay(capsules: &[u8], quoted: &[u8; 32]) -> Result<Vec<Admitted>, Refusal> {
    let mut pcr = [0u8; 32];
    let mut kinds = Vec::new();
    for body in capsules.chunks_exact(CAPSULE) {
        let admitted = Admitted::from_kind(body[64]).ok_or(Refusal::Capsules)?;
        let path_only = body[32..64] == sha256(PATH_ONLY_TAG);
        if (admitted == Admitted::PathOnly) != path_only {
            return Err(Refusal::Capsules);
        }
        pcr = sha256(&[&pcr[..], &sha256(body)].concat());
        kinds.push(admitted);
    }
    (pcr == *quoted).then_some(kinds).ok_or(Refusal::Capsules)
}
