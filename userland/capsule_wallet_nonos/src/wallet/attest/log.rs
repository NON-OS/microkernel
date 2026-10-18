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

/*
 * The ReleaseAttest log: every ReleaseAttest this wallet's key
 * signed, so it never signs a second, different one for the same release.
 * A publisher who signed two would let an escrow buyer be shown one release
 * and paid for another. Pure: the file and the keyring are the caller's.
 */

use alloc::vec::Vec;

use super::types::{Attest, Decision, Record, Refusal};

fn same_release(a: &Attest, b: &Attest) -> bool {
    a.chain_id == b.chain_id && a.listing_id == b.listing_id && a.release_id == b.release_id
}

/// What may be done with `a`, given the log.
pub fn decide(log: &[Record], a: &Attest) -> Result<Decision, Refusal> {
    match log.iter().find(|r| same_release(&r.attest, a)) {
        Some(r) if r.attest == *a => Ok(Decision::Again(r.sig)),
        Some(_) => Err(Refusal::Conflict),
        None if log.len() >= super::MAX_ATTESTS => Err(Refusal::Full),
        None => Ok(Decision::Sign),
    }
}

/// The log with `a` signed as `sig` added; written before the signature is
/// handed on, so a crash can lose a signature but never forget one given.
pub fn signed(log: &[Record], a: &Attest, sig: [u8; 65]) -> Result<Vec<Record>, Refusal> {
    match decide(log, a)? {
        Decision::Again(_) => Ok(log.to_vec()),
        Decision::Sign => {
            let mut next = log.to_vec();
            next.push(Record { attest: *a, sig });
            Ok(next)
        }
    }
}
