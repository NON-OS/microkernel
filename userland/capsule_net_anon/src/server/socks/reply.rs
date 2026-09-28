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

//! How an answer to a SOCKS frame is marked.

extern crate alloc;

use alloc::vec::Vec;

/// The tunnel is open; any bytes that follow are stream bytes.
const REPLY_OPEN: u8 = 0;

/// The conversation is over; any bytes that follow are the last of it.
const REPLY_CLOSED: u8 = 1;

/// An answer, marker first. The marker makes "nothing yet" a real answer:
/// the kernel refuses an empty reply, and silence leaves the caller waiting
/// out its timeout for an answer already known.
pub fn encode(closed: bool, bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(1 + bytes.len());
    out.push(if closed { REPLY_CLOSED } else { REPLY_OPEN });
    out.extend_from_slice(bytes);
    out
}
