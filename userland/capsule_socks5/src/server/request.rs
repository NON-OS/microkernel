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

/// Whatever follows is stream bytes for the tunnel, of which there may be
/// none.
pub const STREAM_BYTES: u8 = 0;

/// Forget whatever conversation this caller had and start over.
pub const STREAM_RESET: u8 = 1;

/// Stream bytes carried in a numbered exchange: the marker, a u32 number
/// (little endian), then the bytes. See `kept` for why.
pub const STREAM_NUMBERED: u8 = 2;

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
        Some((&STREAM_NUMBERED, rest)) if rest.len() >= 4 => {
            let seq = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]);
            Some(Ask::Numbered(seq, &rest[4..]))
        }
        _ => None,
    }
}

/// The stream bytes of a request, for callers that only handle that shape.
pub fn stream_bytes(request: &[u8]) -> Option<&[u8]> {
    match ask(request)? {
        Ask::Stream(bytes) | Ask::Numbered(_, bytes) => Some(bytes),
        Ask::Reset => None,
    }
}
