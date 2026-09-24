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

//! Counting good authority signatures over a consensus.

extern crate alloc;

use alloc::vec::Vec;

use crate::crypto::sha256_software;
use crate::directory::authority::REQUIRED_SIGNATURES;
use crate::directory::consensus::Consensus;

use super::cert::AuthorityCert;
use super::counted::accepts;

/// Why a consensus was not believed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum QuorumError {
    /*
     * How many signature lines the document carried, and how many verified against
     * a cert we hold. The two numbers separate a document nobody signed, one signed
     * by authorities whose certs we failed to fetch, and one with bad signatures.
     */
    NotEnough { lines: usize, verified: usize },
    Malformed,
}

/// Verify the consensus and report how many authorities signed it.
///
pub fn quorum(
    doc: &Consensus,
    body: &[u8],
    certs: &[(usize, AuthorityCert)],
) -> Result<usize, QuorumError> {
    let (from, to) = doc.signed;
    let span = body.get(from..to).ok_or(QuorumError::Malformed)?;
    /*
     * Hashed here rather than by the kernel. `crypto_hash` refuses anything over a
     * megabyte and a microdescriptor consensus signs about 1.7 MB, so every
     * consensus fetched failed its digest with errno 22 over a working transport.
     */
    let digest = sha256_software(span);
    let mut counted: Vec<usize> = Vec::new();
    for signature in doc.signatures.iter() {
        if let Some(index) = accepts(signature, &digest, certs) {
            if !counted.contains(&index) {
                counted.push(index);
            }
        }
    }
    if counted.len() < REQUIRED_SIGNATURES {
        return Err(QuorumError::NotEnough {
            lines: doc.signatures.len(),
            verified: counted.len(),
        });
    }
    Ok(counted.len())
}
