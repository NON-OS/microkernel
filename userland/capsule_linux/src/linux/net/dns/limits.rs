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

//! How much a resolver descriptor takes and keeps, as a datagram socket
//! bounds it. Pure, so the host proofs hold both bounds.

use crate::linux::abi::errno;

/// The largest query one write may carry: one UDP datagram over IPv4. A
/// longer one is EMSGSIZE, as Linux answers a datagram socket, and is never
/// read out of the guest, so its length cannot size this capsule's memory.
pub const MAX_QUERY: u64 = 65_507;

/// Answers kept for the program to read. A full queue drops the next one,
/// as a datagram socket's full receive queue does; the program asked and
/// will ask again.
pub const MAX_REPLIES: usize = 64;

/// The bytes a query of `len` is read as, or EMSGSIZE.
pub fn query_len(len: u64) -> Result<usize, i64> {
    match len {
        n if n > MAX_QUERY => Err(errno::EMSGSIZE),
        n => Ok(n as usize),
    }
}

/// Whether an answer is kept when `queued` already wait.
pub fn keeps(queued: usize) -> bool {
    queued < MAX_REPLIES
}
