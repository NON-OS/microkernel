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

/// A reset naming its stream: the marker, then a u32 stream (little endian,
/// never 0). See `who` for why a caller names streams.
pub const STREAM_RESET_ON: u8 = 3;

/// A numbered exchange naming its stream: the marker, a u32 stream (never
/// 0), a u32 number, then the bytes. Both numbers little endian.
pub const STREAM_NUMBERED_ON: u8 = 4;

/// How far the proxy has got with its network: the marker alone. Answered
/// with `reply::progress`, never with stream bytes, and it touches no
/// conversation, so a caller may ask it while a CONNECT waits. A proxy older
/// than this marker answers it as a reset of stream 0, which the browser,
/// the one caller that asks, never uses. Not 5: that is the SOCKS version,
/// the first byte of a greeting sent without a marker.
pub const STATUS_ASK: u8 = 6;

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

/// The first number of a conversation, after its reset.
pub const FIRST_SEQ: u32 = 1;

/// Whether exchange `seq` asks for a conversation this proxy lost: not the
/// first of one, while no conversation is `held` for its stream and no
/// answer is `kept` for that number.
pub fn lost(seq: u32, held: bool, kept: bool) -> bool {
    seq != FIRST_SEQ && !held && !kept
}

/// The little endian u32 at `at`; the caller has checked the length.
fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

/// The stream bytes of a request, for callers that only handle that shape.
pub fn stream_bytes(request: &[u8]) -> Option<&[u8]> {
    match ask(request)? {
        Ask::Stream(bytes) | Ask::Numbered(_, bytes) | Ask::NumberedOn(_, _, bytes) => Some(bytes),
        Ask::Reset | Ask::ResetOn(_) | Ask::Status => None,
    }
}
