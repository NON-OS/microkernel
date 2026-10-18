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

//! The test's own reading of a request frame: the wire's magic and
//! version, and a length that fits both the frame and the wire's 256.

use super::kit::{Header, HDR_LEN, MAGIC, VERSION};

pub fn word(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

pub fn half(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

/// The request in `f` and its payload, if the test reads one there.
pub fn request(f: &[u8]) -> Option<(Header, &[u8])> {
    if word(f, 0)? != MAGIC || half(f, 4)? != VERSION {
        return None;
    }
    let (op, flags, output) = (half(f, 6)?, half(f, 8)?, half(f, 10)?);
    let (request_id, payload_len) = (word(f, 12)?, word(f, 16)?);
    let end = HDR_LEN.checked_add(payload_len as usize).filter(|&e| e <= 256)?;
    Some((Header { op, flags, output, request_id, payload_len }, f.get(HDR_LEN..end)?))
}
