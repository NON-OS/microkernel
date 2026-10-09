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

//! The frames a SOCKS caller sends, told apart from API requests by their
//! first byte.

/// Stream bytes, of which there may be none.
const STREAM_BYTES: u8 = 0;

/// Forget this caller's conversation and start over.
const STREAM_RESET: u8 = 1;

/// Stream bytes in a numbered exchange: the marker, a u32 number (little
/// endian), then the bytes.
const STREAM_NUMBERED: u8 = 2;

/// A reset naming its stream: the marker, then a u32 stream (little endian,
/// never 0). See `who`.
const STREAM_RESET_ON: u8 = 3;

/// A numbered exchange naming its stream: the marker, a u32 stream (never
/// 0), a u32 number, then the bytes. Both numbers little endian.
const STREAM_NUMBERED_ON: u8 = 4;

/// How far the network has got: the marker alone, answered with
/// `reply::progress` and touching no conversation. Not 5, the SOCKS version
/// a greeting sent without a marker begins with. net.socks5 reads the same
/// marker the same way (its server/request.rs).
const STATUS_ASK: u8 = 6;

/// The first number of a conversation, after its reset.
pub const FIRST_SEQ: u32 = 1;

/// What a caller is asking for.
#[derive(Debug, PartialEq, Eq)]
pub enum Ask<'a> {
    Stream(&'a [u8]),
    Reset,
    Numbered(u32, &'a [u8]),
    /// `Reset` of the named stream only.
    ResetOn(u32),
    /// `Numbered` on the named stream: stream, number, bytes.
    NumberedOn(u32, u32, &'a [u8]),
    /// How far the network has got (`STATUS_ASK`).
    Status,
}

/// Whether a frame is for the SOCKS front. An API request opens with the
/// little endian magic, whose first byte is 0x31, so the two never meet.
pub fn is_socks(frame: &[u8]) -> bool {
    matches!(
        frame.first(),
        Some(
            &(STREAM_BYTES
                | STREAM_RESET
                | STREAM_NUMBERED
                | STREAM_RESET_ON
                | STREAM_NUMBERED_ON
                | STATUS_ASK)
        )
    )
}

/// Read a SOCKS frame, or `None` for one this front does not speak.
pub fn ask(frame: &[u8]) -> Option<Ask<'_>> {
    match frame.split_first() {
        Some((&STREAM_BYTES, rest)) => Some(Ask::Stream(rest)),
        Some((&STREAM_RESET, _)) => Some(Ask::Reset),
        Some((&STATUS_ASK, _)) => Some(Ask::Status),
        Some((&STREAM_NUMBERED, rest)) if rest.len() >= 4 => {
            Some(Ask::Numbered(u32_at(rest, 0), &rest[4..]))
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
