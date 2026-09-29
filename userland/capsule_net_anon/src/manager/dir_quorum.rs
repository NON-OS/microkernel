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

//! Whether the authorities signed the consensus we fetched.

use crate::directory::authority::{AUTHORITIES, REQUIRED_SIGNATURES};
use crate::directory::consensus::Consensus;
use crate::directory::verify::{quorum, AuthorityCert, QuorumError};
use crate::trace;

pub(super) fn signed(doc: &Consensus, body: &[u8], certs: &[(usize, AuthorityCert)]) -> bool {
    match quorum(doc, body, certs) {
        Ok(count) => {
            trace::say_two(b"consensus signed by", count as u64, AUTHORITIES.len() as u64);
            true
        }
        Err(QuorumError::Malformed) => {
            // All three numbers, because printing two of them named a cause that
            // had not been checked.
            trace::say_two(b"consensus span from and to", doc.signed.0 as u64, doc.signed.1 as u64);
            trace::say_num(b"consensus body bytes", body.len() as u64);
            false
        }
        Err(QuorumError::NotEnough { lines, verified }) => {
            trace::say_two(
                b"consensus signatures, lines and verified",
                lines as u64,
                verified as u64,
            );
            trace::say_two(
                b"consensus quorum failed, held certs and needed",
                certs.len() as u64,
                REQUIRED_SIGNATURES as u64,
            );
            false
        }
    }
}
