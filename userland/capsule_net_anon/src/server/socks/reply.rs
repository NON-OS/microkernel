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

use super::tunnel::Progress;

/// The tunnel is open; any bytes that follow are stream bytes.
pub const REPLY_OPEN: u8 = 0;

/// The conversation is over; any bytes that follow are the last of it.
pub const REPLY_CLOSED: u8 = 1;

/// No conversation is held for that stream, and the exchange asked is not
/// the first of one: it was lost when this capsule restarted, or ended and
/// was forgotten. Said apart from a close, which is the far end's, so the
/// caller does not blame the site. The marker alone.
pub const REPLY_LOST: u8 = 2;

/// The answer to a status ask: this marker, a format version, whether a
/// stream can be opened now, the step the network is at, and how many there
/// are (`tunnel::Progress`). Five bytes, as net.socks5 gives them.
pub const REPLY_STATUS: u8 = 3;

/// The format of a status answer.
pub const STATUS_VERSION: u8 = 1;

/// The answer to a status ask.
pub fn progress(p: Progress) -> Vec<u8> {
    Vec::from([REPLY_STATUS, STATUS_VERSION, u8::from(p.ready), p.step, p.steps])
}

/// An answer, marker first. The marker makes "nothing yet" a real answer:
/// the kernel refuses an empty reply, and silence leaves the caller waiting
/// out its timeout for an answer already known.
pub fn encode(closed: bool, bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(1 + bytes.len());
    out.push(if closed { REPLY_CLOSED } else { REPLY_OPEN });
    out.extend_from_slice(bytes);
    out
}
