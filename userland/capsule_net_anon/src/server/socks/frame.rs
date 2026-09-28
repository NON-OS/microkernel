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

/// What a caller is asking for.
#[derive(Debug, PartialEq, Eq)]
pub enum Ask<'a> {
    Stream(&'a [u8]),
    Reset,
    Numbered(u32, &'a [u8]),
}

/// Whether a frame is for the SOCKS front. An API request opens with the
/// little endian magic, whose first byte is 0x31, so the two never meet.
pub fn is_socks(frame: &[u8]) -> bool {
    matches!(frame.first(), Some(&(STREAM_BYTES | STREAM_RESET | STREAM_NUMBERED)))
}

/// Read a SOCKS frame, or `None` for one this front does not speak.
pub fn ask(frame: &[u8]) -> Option<Ask<'_>> {
    match frame.split_first() {
        Some((&STREAM_BYTES, rest)) => Some(Ask::Stream(rest)),
        Some((&STREAM_RESET, _)) => Some(Ask::Reset),
        Some((&STREAM_NUMBERED, rest)) if rest.len() >= 4 => {
            let seq = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]);
            Some(Ask::Numbered(seq, &rest[4..]))
        }
        _ => None,
    }
}
