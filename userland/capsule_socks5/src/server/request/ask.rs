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

//! What a caller asks for, read off the marker that opens its request.

use super::marks::*;

/// What a caller is asking for.
pub enum Ask<'a> {
    /// Carry these bytes, or if there are none, report what has come back.
    Stream(&'a [u8]),
    /// Begin a new conversation, discarding any tunnel still open.
    ///
    /// Handshake state is keyed on the caller, so without this a second
    /// request from the same capsule met a connection already relaying and
    /// its greeting was forwarded to the exit as stream bytes. The first
    /// request of a session worked and nothing after it could.
    Reset,
    /// Stream bytes under an exchange number, which the caller repeats when
    /// it did not receive the answer to that exchange.
    Numbered(u32, &'a [u8]),
    /// `Reset` of the named stream only.
    ResetOn(u32),
    /// `Numbered` on the named stream: stream, number, bytes.
    NumberedOn(u32, u32, &'a [u8]),
    /// How far the network has got (`STATUS_ASK`).
    Status,
}

/// Read what the caller is asking for, or `None` if it is not a shape we
/// speak.
///
/// The marker exists so that a request carrying no bytes can be sent at all.
/// A caller waiting on a mixnet reply has to ask repeatedly with nothing to
/// say, and the kernel refuses a zero length message, so without a byte to
/// carry there was no way to ask.
pub fn ask(request: &[u8]) -> Option<Ask<'_>> {
    match request.split_first() {
        Some((&STREAM_BYTES, rest)) => Some(Ask::Stream(rest)),
        Some((&STREAM_RESET, _)) => Some(Ask::Reset),
        Some((&STATUS_ASK, _)) => Some(Ask::Status),
        Some((&STREAM_NUMBERED, rest)) if rest.len() >= 4 => {
            let seq = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]);
            Some(Ask::Numbered(seq, &rest[4..]))
        }
        Some((&STREAM_RESET_ON, rest)) if rest.len() >= 4 => match u32_at(rest, 0) {
            0 => None,
            stream => Some(Ask::ResetOn(stream)),
        },
        Some((&STREAM_NUMBERED_ON, rest)) if rest.len() >= 8 => match u32_at(rest, 0) {
            0 => None,
            stream => Some(Ask::NumberedOn(stream, u32_at(rest, 4), &rest[8..])),
        },
        _ => None,
    }
}

/// The little endian u32 at `at`; the caller has checked the length.
fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}
